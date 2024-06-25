use std::{io::Write, time::Instant};

use datagen::Files;
use goober::{FeedForwardNetwork, OutputLayer, SparseVector};
use javelin::{PolicyNetwork, SubNet};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use crate::policy_data_loader::PolicyDataLoader;

const BATCH_SIZE: usize = 16_384;
const BATCHES_PER_SUPERBATCH: usize = 1536;
const EXPORT_PATH: &'static str = "../../resources/training/checkpoints/";

pub struct PolicyTrainer;
impl PolicyTrainer {
    pub fn train(name: &'static str, threads: usize, superbatches: usize, mut learning_rate: f32, lr_drop: usize) {
        let mut train_data = Files::new();
        let _ = train_data.load_policy();
        let mut policy = rand_init();
        let throughput = superbatches * BATCHES_PER_SUPERBATCH * BATCH_SIZE;

        println!("Network Name: {name}");
        println!("Export Path: {}", format!("{}{}.net", EXPORT_PATH, name).as_str());
        println!("Thread Count: {threads}");
        println!("Loaded Positions: {}", train_data.policy_data.len());
        println!("Superbatches: {superbatches}");
        println!("LR Drop: {lr_drop}");
        println!("Start LR: {learning_rate}");
        println!("Epochs {:.2}\n", throughput as f64 / train_data.policy_data.len() as f64);

        let mut momentum = boxed_and_zeroed::<PolicyNetwork>();
        let mut velocity = boxed_and_zeroed::<PolicyNetwork>();

        let mut running_error = 0.0;
        let mut superbatch_index = 0;
        let mut batch_index = 0;
        let mut data_chunk_start_index = 0;

        'training: loop {
            let data_chunk_end_index =
            (data_chunk_start_index + 1536 * BATCH_SIZE).min(train_data.policy_data.len());
            let mut policy_data = PolicyDataLoader::prepare_policy_dataset(&&train_data.policy_data[data_chunk_start_index..data_chunk_end_index].to_vec());
            data_chunk_start_index = data_chunk_end_index % train_data.policy_data.len();
            policy_data.shuffle(&mut thread_rng());
            let timer = Instant::now();

            for (index, batch) in policy_data.chunks(BATCH_SIZE).enumerate() {
                let mut grad = boxed_and_zeroed();
                running_error += gradient_batch(threads, &policy, &mut grad, batch);
                let adj = 1.0 / batch.len() as f32;
                update(&mut policy, &grad, adj, learning_rate, &mut momentum, &mut velocity);

                batch_index += 1;
                let l: usize = policy_data.len();
                print!(
                    "> Superbatch {}/{superbatches} Batch {}/{BATCHES_PER_SUPERBATCH} - {index} - {l} Speed {:.0}\r",
                    superbatch_index + 1,
                    batch_index % BATCHES_PER_SUPERBATCH,
                    (index * BATCH_SIZE) as f32 / timer.elapsed().as_secs_f32()
                );
                let _ = std::io::stdout().flush();

                if batch_index % BATCHES_PER_SUPERBATCH == 0 {
                    superbatch_index += 1;
                    println!(
                        "> Superbatch {superbatch_index}/{superbatches} Running Loss {}",
                        running_error / (BATCHES_PER_SUPERBATCH * BATCH_SIZE) as f32
                    );
                    running_error = 0.0;

                    if superbatch_index % lr_drop == 0 {
                        learning_rate *= 0.1;
                        println!("Dropping LR to {learning_rate}");
                    }

                    export(&policy, format!("{}{}-sb{superbatch_index}.net", EXPORT_PATH, name).as_str());

                    if superbatch_index == superbatches {
                        break 'training;
                    }
                }
            }
        }

        loop {}
    }
}

fn gradient_batch(
    threads: usize,
    policy: &PolicyNetwork,
    grad: &mut PolicyNetwork,
    batch: &[(SparseVector, Vec<(usize, usize, f32, usize)>)],
) -> f32 {
    let size = (batch.len() / threads).max(1);
    let mut errors = vec![0.0; threads];

    std::thread::scope(|s| {
        batch
            .chunks(size)
            .zip(errors.iter_mut())
            .map(|(chunk, error)| {
                s.spawn(move || {
                    let mut inner_grad = boxed_and_zeroed();
                    for entry in chunk {
                        update_single_grad(entry, policy, &mut inner_grad, error);
                    }
                    inner_grad
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|p| p.join().unwrap())
            .for_each(|part| add_without_explicit_lifetime(grad, &part));
    });

    errors.iter().sum::<f32>()
}

fn update(
    policy: &mut PolicyNetwork,
    grad: &PolicyNetwork,
    adj: f32,
    learning_rate: f32,
    momentum: &mut PolicyNetwork,
    velocity: &mut PolicyNetwork,
) {
    for (i, subnet_pair) in policy.subnets.iter_mut().enumerate() {
        for (j, subnet) in subnet_pair.iter_mut().enumerate(){
            subnet.adam(&grad.subnets[i][j], &mut momentum.subnets[i][j], &mut velocity.subnets[i][j], adj, learning_rate);
        }
    }
}

fn update_single_grad(
    (entry_input, entry_moves): &(SparseVector, Vec<(usize, usize, f32, usize)>),
    policy: &PolicyNetwork,
    grad: &mut PolicyNetwork,
    error: &mut f32,
) {
    let mut policies = Vec::with_capacity(entry_moves.len());

    let mut max = f32::NEG_INFINITY;
    let mut total = 0.0;

    for &(from_index, to_index, expected_policy, see) in entry_moves {
        let from_out = policy.subnets[from_index][0].out_with_layers(&entry_input);
        let to_out = policy.subnets[64 + to_index][see].out_with_layers(&entry_input);
        let policy_value = from_out.output_layer().dot(&to_out.output_layer());

        max = max.max(policy_value);
        policies.push((from_index, to_index, from_out, to_out, policy_value, expected_policy, see));
    }

    for (_, _, _, _, policy_value, _, _) in policies.iter_mut() {
        *policy_value = (*policy_value - max).exp();
        total += *policy_value;
    }
    for (from_index, to_index, from_out, to_out, policy_value, expected_value, see) in policies {
        let policy_value = policy_value / total;
        let error_factor = policy_value - expected_value;

        *error -= expected_value * policy_value.ln();

        let factor = error_factor;

        policy.subnets[from_index][0].backprop(
            &entry_input,
            &mut grad.subnets[from_index][0],
            factor * to_out.output_layer(),
            &from_out,
        );

        policy.subnets[64 + to_index][see].backprop(
            &entry_input,
            &mut grad.subnets[64 + to_index][see],
            factor * from_out.output_layer(),
            &to_out,
        );
    }
}

fn rand_init() -> Box<PolicyNetwork> {
    let mut policy = boxed_and_zeroed::<PolicyNetwork>();

    let mut rng = rand::thread_rng();
    for subnet_pair in policy.subnets.iter_mut() {
        for subnet in subnet_pair.iter_mut() {
            *subnet = SubNet::from_fn(|| (rng.gen_range(0, u32::MAX) as f32 / u32::MAX as f32) * 0.2);
        }
    }

    policy
}

fn add_without_explicit_lifetime(lhs: &mut PolicyNetwork, rhs: &PolicyNetwork) {
    for (i_pair, j_pair) in lhs.subnets.iter_mut().zip(rhs.subnets.iter()) {
        for (i, j) in i_pair.iter_mut().zip(j_pair.iter()) {
            *i += j;
        }
    }
}

fn boxed_and_zeroed<T>() -> Box<T> {
    unsafe {
        let layout = std::alloc::Layout::new::<T>();
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Box::from_raw(ptr.cast())
    }
}

fn export(net: &Box<PolicyNetwork>, path: &str) {
    let size = std::mem::size_of::<PolicyNetwork>();

    let mut file = std::fs::File::create(path).unwrap();

    unsafe {
        let slice: *const u8 = std::slice::from_ref(net.as_ref()).as_ptr().cast();
        let struct_bytes: &[u8] = std::slice::from_raw_parts(slice, size);
        file.write_all(struct_bytes).expect("Failed to write data!");
    }
}

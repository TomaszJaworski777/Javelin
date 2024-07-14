use bullet::{inputs, lr, optimiser, outputs, wdl, LocalSettings, Loss, TrainerBuilder, TrainingSchedule};

pub struct ValueTrainer;
impl ValueTrainer {
    pub fn run() {
        let mut trainer = TrainerBuilder::default()
        .optimiser(optimiser::AdamW)
        .single_perspective()
        .input(inputs::ChessBucketsMirrored::default())
        .output_buckets(outputs::Single)
        .feature_transformer(512)
        .activate(bullet::Activation::SCReLU)
        .add_layer(1)
        .build();

        let schedule = TrainingSchedule {
            net_id: "value_11".to_string(),
            eval_scale: 400.0,
            ft_regularisation: 0.0,
            batch_size: 16_384,
            batches_per_superbatch: 4096,
            start_superbatch: 1,
            end_superbatch: 80,
            wdl_scheduler: wdl::ConstantWDL { value: 1.0 },
            lr_scheduler: lr::StepLR {
                start: 0.001,
                gamma: 0.1,
                step: 25,
            },
            loss_function: Loss::SigmoidMSE,
            save_rate: 10,
            optimiser_settings: optimiser::AdamWParams {
                decay: 0.01,
                beta1: 0.9,
                beta2: 0.999,
                min_weight: -1.98,
                max_weight: 1.98,
            },
        };

        let settings = LocalSettings {
            threads: 7,
            data_file_paths: vec!["../../resources/data/bullet_data.data"],
            test_set: None,
            output_directory: "../../resources/training/checkpoints",
        };

        trainer.run(&schedule, &settings);
    }
}

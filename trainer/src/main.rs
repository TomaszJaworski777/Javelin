mod core_net_struct;
mod value_net;

use crate::value_net::ValueNet;

fn main() {
    let value_net = ValueNet::new();

    let mut inputs = Vec::<f32>::new();
    for _ in 0..768 {
        inputs.push(0.5);
    }

    println!("Output: {:?}", value_net.evaluate(inputs));
    value_net.save();
    value_net.export();
}

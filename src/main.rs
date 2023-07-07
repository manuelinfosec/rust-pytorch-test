// A large number of tensors are created either on the CPU or GPU...
// ...and can be used to monitor the main memory usage or GPU memory...
// ..at the same time

// C++ LibTorch Bindings
use tch::Tensor;

fn main() {
    // Collect arguments from the command line
    let a: Vec<String> = std::env::args().collect();

    // Determine the target device based on the provided arguments
    let device = match a.iter().map(|x| x.as_str()).collect::<Vec<_>>().as_slice() {
        [_] => tch::Device::Cpu,      // If only one argument is provided, use CPU device
        [_, "cpu"] => tch::Device::Cpu,   // If second argument is "cpu", use CPU device
        [_, "gpu"] => tch::Device::Cuda(0),  // If second argument is "gpu", use GPU device (specifically, the first GPU)
        _ => panic!("Usage main cpu | gpu"),  // If arguments don't match the expected patterns, panic with usage message
    };

    let slice = vec![0; 1_000_000];  // Create a vector filled with zeros, containing 1 million elements

    // Iterate from 1 to 99,999 (100,000 iterations)
    for number in 1..100_000 {
        // Create a tensor by copying the content of the slice and move it to the specified device
        let t = Tensor::of_slice(&slice).to_device(device);

        // Print the iteration number and the size of the tensor
        println!("{number} {:?} ", t.size());

        // Check memory usage at regular intervals
        if i % 1000 == 0 {
            // Retrieve memory statistics in kilobytes
            memory.refresh().unwrap();

            // Print memory usage in megabytes
            println!("Memory usage: {} MB", memory.current().rss() / 1024);
        }
    }
}

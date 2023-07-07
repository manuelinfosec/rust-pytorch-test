# PyTorch Stress Test with Rust

This project demonstrates a simple example of memory leak detection using the tch crate in Rust. It repeatedly creates
tensors and monitors memory usage to identify potential memory leaks.

## Prerequisites

Before running the code, ensure that you have the following dependencies installed:

- Rust programming language (https://www.rust-lang.org/tools/install)
- LibTorch library (dependency of tch crate)

## LibTorch Installation

### Linux

1. Download the LibTorch library package from the official PyTorch website: https://pytorch.org/get-started/locally/

2. Extract the downloaded package. For example, using the command:

```agsl
tar -xvf libtorch-cxx11-abi-shared-with-deps-*.zip
```

3. Set the `LIBTORCH` environment variable to the extracted library path:

```agsl
export LIBTORCH=/path/to/libtorch
```

### macOS

1. Download the LibTorch library package from the official PyTorch website: https://pytorch.org/get-started/locally/

2. Extract the downloaded package. For example, using the command:

```agsl
unzip libtorch-macos-*.zip
```

### Windows

1. Download the LibTorch library package from the official PyTorch website: https://pytorch.org/get-started/locally/

2. Extract the downloaded package to a desired location, e.g., `C:\libtorch`

3. Set the `LIBTORCH` environment variable to the extracted library path:

## Building the Project

1. Clone the repository:

```agsl
git clone https://github.com/manuelinfosec/rust-pytorch-test
```

2. Navigate to the project directory:

```
cd rust-pytorch-test
```

3. Build the project using Cargo:

```agsl
cargo build --release
```

## Running the Code

To run the code and detect memory leaks, use the following command:

```agsl
cargo run --release
```

The output will display the iteration number and tensor size. If memory monitoring is enabled, it will also show memory
usage at regular intervals.

## Contributing

Contributions are welcome! If you find any issues or want to suggest improvements, please create an issue or submit a
pull request.

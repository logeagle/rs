# Logeagle (Rust Version)

Logeagle is a Rust tool for continuously processing log data and converting it into Parquet format.

## Overview

Log files are valuable sources of information, but they can be challenging to handle efficiently due to their size and structure. Logeagle simplifies this process by providing a robust solution to read log files, clean the data, and convert it into Parquet format. This allows for easy analysis and storage using various data processing tools.

## Features

- **Continuous Processing:** Logeagle continuously reads log data from specified files and converts it into Parquet format.
- **Data Cleaning:** It cleans log data by removing unnecessary characters and formatting inconsistencies.
- **Parallelism:** Utilizes Rust's concurrency features for efficient data processing.

## Getting Started

To use Logeagle, follow these steps:

1. Clone the repository: `git clone https://github.com/logeagle/rs.git`
2. Navigate to the project directory: `cd rs`
3. Build the project: `cargo build --release`
4. Run Logeagle: `cargo run --release`

## Usage

Logeagle accepts log files in various formats, such as Apache access logs, nginx error logs, and custom log formats. You can specify the input log files and output directory using command-line arguments or modify the paths directly in the source code.

Example:

```bash
cargo run --release
```

## Contributing

Contributions to Logeagle are welcome! If you have any ideas for improvements, bug fixes, or new features, feel free to open an issue or submit a pull request on [GitHub](https://github.com/logeagle/rs).

## License

Logeagle is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for more information.

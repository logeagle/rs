use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

use arrow::array::StringArray;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::{WriterProperties, WriterVersion};

// Function to read log file and return a Vec<String> containing each line
fn read_log_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().flatten().collect())
}

// Function to write data to Apache Parquet format
fn write_to_parquet_file(file_path: &str, schema: SchemaRef, data: Vec<String>) {
    let file = File::create(file_path).expect("Failed to create Parquet file");
    let props = WriterProperties::builder()
        .set_compression(Compression::UNCOMPRESSED)
        .set_writer_version(WriterVersion::PARQUET_2_0)
        .build();

    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props))
        .expect("Failed to create ArrowWriter");

    let array = StringArray::from(data);
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![Arc::new(array)],
    ).expect("Failed to create record batch");

    writer.write(&batch).expect("Failed to write data to ArrowWriter");
    writer.close().expect("Failed to close ArrowWriter");
}

fn main() {
    // Get username using whoami command
    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");
    let username = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Define paths for log files and output directory
    let access_log_path = "/var/log/nginx/access.log";
    let error_log_path = "/var/log/nginx/error.log";
    let output_directory = format!("/home/{}/logeagle", username);

    // Read data from log files
    let access_log_data = match read_log_file(access_log_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to read access log: {}", err);
            Vec::new()
        }
    };

    let error_log_data = match read_log_file(error_log_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to read error log: {}", err);
            Vec::new()
        }
    };

    // Create output directory if it doesn't exist
    if !Path::new(&output_directory).exists() {
        std::fs::create_dir_all(&output_directory).expect("Failed to create output directory");
    }

    // Define schema for Parquet file
    let schema = Arc::new(
        arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("line", arrow::datatypes::DataType::Utf8, false)
        ])
    );

    // Write data to Parquet files
    write_to_parquet_file(
        &format!("{}/access.parquet", &output_directory),
        schema.clone(),
        access_log_data,
    );

    write_to_parquet_file(
        &format!("{}/error.parquet", &output_directory),
        schema,
        error_log_data,
    );

    println!("Data has been successfully converted and saved to Parquet format.");
}

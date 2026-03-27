// Part 3: Rust Async File I/O
// Cargo.toml dependencies
// [dependencies]
// tokio = { version = "1.49.0", features = ["full"] }

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

const FILENAME: &str = "example.txt";

#[derive(Debug, Default)]
struct LogStats {
    total_lines: usize,
    errors: usize,
    warnings: usize,
    info: usize,
    longest_line_length: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Async File I/O Examples ===");

    // Create sample file
    create_sample_file().await?;

    // Run examples
    read_entire_file().await?;
    buffered_reading().await?;
    write_to_file().await?;
    buffered_writing().await?;
    process_file_chunks().await?;

    let stats = analyze_log_file(FILENAME).await?;
    println!("\nAnalyzing log file: {:#?}", stats);

    Ok(())
}

// Create a sample file
async fn create_sample_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Sample File ===");

    let mut file = File::create(FILENAME).await?;

    file.write_all(b"INFO: Application started\n").await?;
    file.write_all(b"INFO: Loading configuration\n").await?;
    file.write_all(b"WARN: Cache miss for 'user_123'\n").await?;
    file.write_all(b"ERROR: Failed to connect to the database\n")
        .await?;
    file.write_all(b"INFO: Trying again...\n").await?;
    file.write_all(b"INFO: Successfully connected\n").await?;

    println!("Created sample file\n");

    Ok(())
}

// Example 1: Simple file reading
async fn read_entire_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Example 1: Reading the entire file ===");

    let mut file = File::open(FILENAME).await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    println!("File contents:\n{}", contents);

    Ok(())
}

// Example 2: Buffered reading line by line
async fn buffered_reading() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Example 2: Buffered Reading ===");

    let file = File::open(FILENAME).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut line_num = 1;

    while let Some(line) = lines.next_line().await? {
        println!("Line: {}: {}", line_num, line);
        line_num += 1;
    }

    Ok(())
}

// Example 3: Write to a file
async fn write_to_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Example 3: Writing to a file ===");

    let mut file = File::create("output.txt").await?;

    file.write_all(b"Writing from Rust async\n").await?;
    file.write_all(b"Rust async examples\n").await?;

    println!("Written to output.txt file");

    Ok(())
}

// Example 4: Buffered writing (more efficient way)
async fn buffered_writing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Example 4: Buffered writing ===");

    let file = File::create("buffered_output.txt").await?;
    let mut writer = BufWriter::new(file);

    for i in 1..1000 {
        writer
            .write_all(format!("Line: {}\n", i).as_bytes())
            .await?;
    }

    println!("Written 1000 lines");

    Ok(())
}

// Example 5: Processing large files in chunks
async fn process_file_chunks() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Example 5: Processing large file into chunks ===");

    let file = File::open(FILENAME).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; 64];
    let mut total_bytes_read = 0;

    loop {
        let bytes_read = reader.read(&mut buffer).await?;

        if bytes_read == 0 {
            break; // EOF
        }

        total_bytes_read += bytes_read;

        println!("Read bytes: {}, total: {}", bytes_read, total_bytes_read);

        let chunks = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Chunks:\n{}", chunks);
    }

    println!("\nTotal bytes read: {}", total_bytes_read);

    Ok(())
}

// Example 6: Real world log file
async fn analyze_log_file(filename: &str) -> Result<LogStats, Box<dyn std::error::Error>> {
    println!("\n=== Example 6: Analyze log file ===");

    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut stats = LogStats::default();

    while let Some(line) = lines.next_line().await? {
        stats.total_lines += 1;

        if line.contains("ERROR") {
            stats.errors += 1;
        } else if line.contains("INFO") {
            stats.info += 1;
        } else {
            stats.warnings += 1;
        }

        if line.len() > stats.longest_line_length {
            stats.longest_line_length = line.len();
        }
    }

    Ok(stats)
}

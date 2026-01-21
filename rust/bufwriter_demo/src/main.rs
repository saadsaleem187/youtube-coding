use std::{fs::File, io::{BufWriter, Write}, time::Instant};

fn main() -> std::io::Result<()> {
    let num_lines = 10_000;
    let filename_without_bufwriter = "without_bufwriter.txt";
    let filename_with_bufwriter = "with_bufwriter.txt";

    println!("\nWriting {} lines to files...", num_lines);

    println!("\n--- Writing WITHOUT BufWriter ---");
    let time1 = write_without_bufwriter(filename_without_bufwriter, num_lines)?;
    println!("Time taken: {:.2} microseconds ({:.2}ms)", time1, time1 as f64 / 1000.0);

    println!("\n--- Writing WITH BufWriter ---");
    let time2 = write_with_bufwriter(filename_with_bufwriter, num_lines)?;
    println!("Time taken: {:.2} microseconds ({:.2}ms)", time2, time2 as f64 / 1000.0);

    println!("\n--- Performance Comparison ---");
    let speedup = time1 as f64 / time2 as f64;
    println!("BufWriter is {:.2}x faster", speedup);
    println!("Time saved: {:.2}ms", (time1 - time2) as f64 / 1000.0);

    if time1 > time2 {
        let percent_faster = ((time1 - time2) as f64 / time1 as f64) * 100.0;
        println!("BufWriter reduced time by {:.1}%", percent_faster);
    }

    // Clean up
    std::fs::remove_file(filename_without_bufwriter)?;
    std::fs::remove_file(filename_with_bufwriter)?;

    Ok(())
}

fn write_without_bufwriter(filename: &str, lines: usize) -> std::io::Result<u128> {
    let start = Instant::now();
    let mut file = File::create(filename)?;

    for i in 0..lines {
        writeln!(file, "Line: {i}")?;
    }

    file.flush()?;

    let duration = start.elapsed().as_micros();

    Ok(duration)
}

fn write_with_bufwriter(filename: &str, lines: usize) -> std::io::Result<u128> {
    let start = Instant::now();
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for i in 0..lines {
        writeln!(writer, "Line: {i}")?;
    }

    writer.flush()?;

    let duration = start.elapsed().as_micros();

    Ok(duration)
}

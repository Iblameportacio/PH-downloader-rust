use clap::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about = "Native media grabber CLI tool")]
struct Args {
    /// Video URL to fetch
    #[arg(short, long)]
    url: Option<String>,

    /// Where to dump the downloaded file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Video quality preset (e.g., best, worst)
    #[arg(short, long, default_value = "best")]
    quality: String,
}

fn main() {
    // Grab CLI arguments
    let mut args = Args::parse();

    // Trigger interactive mode if no URL is passed
    if args.url.is_none() {
        println!("=== Heavy-Duty Video Downloader ===");

        let mut url_input = String::new();
        print!("Drop the video URL here: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut url_input).unwrap();
        let url_input = url_input.trim().to_string();

        if url_input.is_empty() {
            eprintln!("Error: URL can't be empty, chief.");
            return;
        }
        args.url = Some(url_input);

        let mut out_input = String::new();
        print!("Target folder (hit Enter for default './descargas'): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut out_input).unwrap();
        let out_input = out_input.trim();

        if !out_input.is_empty() {
            args.output = Some(PathBuf::from(out_input));
        }
    }

    let url = args.url.unwrap();
    let quality = args.quality;

    // Set up output path destination
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let mut current_dir = env::current_dir().unwrap();
            current_dir.push("descargas");
            current_dir
        }
    };

    // Spin up the directory if it's missing
    if let Err(e) = fs::create_dir_all(&output_path) {
        eprintln!("Error creating directory: {}", e);
        return;
    }

    // Build the naming template inside the target folder
    let mut template = output_path.clone();
    template.push("%(title)s.%(ext)s");
    let template_str = template.to_string_lossy().into_owned();

    println!("Firing up download from: {}", url);
    println!("Saving files to: {}", output_path.display());
    println!("Selected quality: {}", quality);
    println!("--------------------------------------------------");

    // Call the system's yt-dlp binary directly
    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg(&quality)
        .arg("-o")
        .arg(&template_str)
        .arg("--no-playlist")
        .arg(&url)
        // Inherit I/O streams so the real-time progress bar shows up clean in your shell
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status(); // Block thread execution until the actual download finishes

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("\n--------------------------------------------------");
            println!("All done! Process finished successfully.");
            println!("Go check your file at: {}", output_path.display());
        }
        Ok(exit_status) => {
            eprintln!("\n`yt-dlp` bailed out with an error code: {}", exit_status);
        }
        Err(e) => {
            eprintln!("\nFailed to run `yt-dlp`. Are you sure it's installed? Error: {}", e);
        }
    }
}

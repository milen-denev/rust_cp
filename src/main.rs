use clap::{Parser, ArgAction};
use std::fs;
use std::io;
use std::path::Path;
use std::io::Write;

/// A tool that mimics `cp` command in Linux
#[derive(Parser)]
struct Cli {
    /// Source file or directory path
    source: String,

    /// Destination file or directory path
    destination: String,

    /// Copy directories recursively
    #[arg(short = 'r', long = "recursive", action = ArgAction::SetTrue)]
    recursive: bool,

    /// Verbose mode, print details of the copying process
    #[arg(short = 'v', long = "verbose", action = ArgAction::SetTrue)]
    verbose: bool,

    /// Interactive mode, prompt before overwriting files
    #[arg(short = 'i', long = "interactive", action = ArgAction::SetTrue)]
    interactive: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let source_path = Path::new(&args.source);
    let dest_path = Path::new(&args.destination);

    // Check if source exists
    if !source_path.exists() {
        eprintln!("Source path does not exist: {}", args.source);
        std::process::exit(1);
    }

    if source_path.is_dir() {
        // Handle directory copying with the recursive flag
        if !args.recursive {
            eprintln!("Source is a directory. Use the -r flag to copy directories recursively.");
            std::process::exit(1);
        }
        copy_directory(&source_path, &dest_path, &args)?;
    } else {
        // Copy single file
        copy_file(&source_path, &dest_path, &args)?;
    }

    Ok(())
}

/// Copy a single file with optional verbose and interactive modes
fn copy_file(source: &Path, destination: &Path, args: &Cli) -> io::Result<()> {
    // Handle interactive mode: confirm overwrite
    if args.interactive && destination.exists() {
        print!("Overwrite {}? [y/N]: ", destination.display());
        io::stdout().flush()?;
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        if !answer.trim().eq_ignore_ascii_case("y") {
            println!("Not overwriting {}", destination.display());
            return Ok(());
        }
    }

    // Perform the file copy
    fs::copy(source, destination)?;

    if args.verbose {
        println!("Copied {} to {}", source.display(), destination.display());
    }

    Ok(())
}

/// Recursively copy directories with optional verbose and interactive modes
fn copy_directory(source: &Path, destination: &Path, args: &Cli) -> io::Result<()> {
    // Create the destination directory if it doesn't exist
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dest_entry_path = destination.join(entry_name);

        if entry_path.is_dir() {
            // Recursive call for sub-directories
            copy_directory(&entry_path, &dest_entry_path, args)?;
        } else {
            // Copy individual files
            copy_file(&entry_path, &dest_entry_path, args)?;
        }
    }

    if args.verbose {
        println!("Recursively copied directory {} to {}", source.display(), destination.display());
    }

    Ok(())
}

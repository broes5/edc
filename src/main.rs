use clap::Parser;
use std::env;
use std::fs;
use std::io;
use std::io::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(
	about = "De-capitalises all uppercase or mixed-case file extensions in the current
working directory (by default).",
	author,
	version
)]
struct Cli {
	#[arg(help = "Where to look for uppercase extensions")]
	path: Option<Vec<PathBuf>>,

	#[arg(short, long, help = "Recursively check through sub-directories")]
	recursive: bool,

	#[arg(
		short,
		long,
		conflicts_with = "quiet",
		help = "Print files as they have their extensions de-capitalised"
	)]
	verbose: bool,

	#[arg(
		short,
		long,
		conflicts_with = "verbose",
		help = "Don't produce any output (extensions won't be de-capitalised 
if it'll cause cause a pre-existing file to be overwritten)"
	)]
	quiet: bool,

	#[arg(short, long, help = "Prompt before fixing extension")]
	interactive: bool,
}

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();
	let mut pathbufv: Vec<PathBuf> = Vec::new();
	match cli.path {
		Some(unchecked_pathbufv) => {
			for pathbuf in unchecked_pathbufv {
				if pathbuf.as_path().is_dir() {
					if !cli.recursive {
						let dir_entries = fs::read_dir(&pathbuf).unwrap();
						for entry in dir_entries {
							pathbufv.push(entry.unwrap().path());
						}
					} else {
						for entry in WalkDir::new(&pathbuf)
							.into_iter()
							.filter(|thing| thing.as_ref().unwrap().file_type().is_file())
						{
							pathbufv.push(entry.unwrap().path().to_path_buf());
						}
					}
				} else if pathbuf.as_path().is_file() {
					pathbufv.push(pathbuf);
				} else {
					let mut stderr = io::stderr().lock();
					writeln!(
						stderr,
						"edc: cannot access '{}': No such file or directory",
						pathbuf.display()
					)?;
				}
			}
		}
		None => {
			if !cli.recursive {
				let current_dir_entries = fs::read_dir(env::current_dir()?).unwrap();
				for entry in current_dir_entries {
					pathbufv.push(entry.unwrap().path());
				}
			} else {
				for entry in WalkDir::new(env::current_dir()?)
					.into_iter()
					.filter(|thing| thing.as_ref().unwrap().file_type().is_file())
				{
					pathbufv.push(entry.unwrap().path().to_path_buf());
				}
			}
		}
	}

    // Remove lowercase extensions.
    let mut index = 0;
    for pathbuf in pathbufv.clone() {
		if let Some(extension) = pathbuf.clone().extension() {
		    if extension.to_str().unwrap().chars().any(char::is_uppercase) {
                index += 1;
                continue 
            } else {
                pathbufv.remove(index);
            }
        } else {
            pathbufv.remove(index);
        }
    }

    // If they want, ask the user if they want each file's extension de-capitalised.
    // Remove the ones they don't want de-capitalised.
	if cli.interactive {
		pathbufv.retain(|pathbuf| ask(pathbuf));
	}
			
	decap(pathbufv, cli.verbose, cli.quiet)?;
	Ok(())
}

fn ask(file_path: &Path) -> bool {
	let mut stdin = io::stdin().lock();
	print!("edc: de-capitalise {} [y/N]? ", file_path.display());
	io::stdout().flush().expect("Flush failed!");
	let mut answer = String::new();
	stdin
		.read_line(&mut answer)
		.expect("Something went wrong while reading user input!");
	answer.as_str().starts_with('y') || answer.as_str().starts_with('Y')
}

fn find_new_name(overwrite_file_path: &Path, original_file_path: &Path) -> Option<PathBuf> {
	let mut tries = 1;
	let mut new_file_path = overwrite_file_path.to_path_buf();
	let extension = overwrite_file_path.extension().unwrap();
	while new_file_path.exists() {
		let mut prefix = overwrite_file_path.file_stem().unwrap().to_os_string();
		prefix.push(format!(" ({}).", tries));
		prefix.push(extension);
		new_file_path.set_file_name(prefix);
		tries += 1;
	}

	let mut stdin = io::stdin().lock();
	print!(
		"\nedc: de-capitalising '{}' would cause a pre-existing file or files to be overwritten.
Rename file to '{}' [Y/n]? ",
		original_file_path.display(),
		new_file_path.file_name().unwrap().to_str().unwrap()
	);
	io::stdout().flush().expect("Flush failed!");
	let mut answer = String::new();
	stdin
		.read_line(&mut answer)
		.expect("Something went wrong while reading user input!");
	if answer.as_str().starts_with('n') || answer.as_str().starts_with('N') {
		None
	} else {
		Some(new_file_path.to_path_buf())
	}
}

fn decap(
	pathbufv: Vec<PathBuf>,
	verbose: bool,
	quiet: bool,
) -> std::io::Result<()> {
	if !quiet {
		let mut count = 0;
		if verbose {
            // De-capitalise extensions. Check if de-capitalising the file's extension will
            // cause another file to be overwritten, and, if the user wants the file to be
            // renamed. Ignore the ones they don't want renamed. Print what the files get
            // renamed to.
            for mut pathbuf in pathbufv {
				let pathbuf_old = pathbuf.clone();
				pathbuf.set_extension(pathbuf.extension().unwrap().to_str().unwrap().to_lowercase());
			    if pathbuf.exists() {
			    	match find_new_name(&pathbuf, &pathbuf_old) {
			    		Some(new_file_path) => pathbuf = new_file_path,
			    		None => continue
			    	}
			    }
				fs::rename(&pathbuf_old, &pathbuf)?;
				println!(
					"{} → {}",
					pathbuf_old.display(),
					pathbuf.file_name().unwrap().to_str().unwrap()
				);
				count += 1;
            }
		} else {
            // The same as above, except don't print what the files get renamed to.
            for mut pathbuf in pathbufv {
				let pathbuf_old = pathbuf.clone();
				pathbuf.set_extension(pathbuf.extension().unwrap().to_str().unwrap().to_lowercase());
			    if pathbuf.exists() {
			    	match find_new_name(&pathbuf, &pathbuf_old) {
			    		Some(new_file_path) => pathbuf = new_file_path,
			    		None => continue
			    	}
			    }
				fs::rename(&pathbuf_old, &pathbuf)?;
				count += 1;
            }
		}
		if count != 1 {
			println!(
				"\nA total of \x1b[1m{}\x1b[22m file extensions were de-capitalised.",
				count
			);
		} else {
			println!("\nA total of \x1b[1m1\x1b[22m file extension was de-capitalised.");
		}
	} else {
        // De-capitalise extensions. So long as it doesn't cause files to be overwritten.
        for mut pathbuf in pathbufv {
			let pathbuf_old = pathbuf.clone();
			pathbuf.set_extension(pathbuf.extension().unwrap().to_str().unwrap().to_lowercase());

		    if !pathbuf.exists() {      // It'd be really shitty if edc sometimes silently overwrote files.
			    fs::rename(&pathbuf_old, &pathbuf)?;
		    }
        }
	}
	Ok(())
}

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.is_empty() || args.len() != 2 {
        system_error("The syntax of the command is incorrect.");
    }

    let subject = &args[1];
    let current_directory = std::env::current_dir()?;

    match subject.as_str() {
        "*.*" => {
            match confirmation(&current_directory) {
                true => {
                    for entry in current_directory.read_dir()? {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                std::fs::remove_file(path)?;
                            }
                            else if path.is_dir() {
                                std::fs::remove_dir_all(path)?;
                            }
                        }
                    }
                },
                false => system_error(""),
            }
            
        }
        subject => {
            let path = Path::new(&subject);
            if !path.exists() {
                system_error("This file or directory does not exist.");
            }


            match confirmation(&path) {
                true => {
                    if path.is_file() {
                        std::fs::remove_file(path)?;
                    }
                    else if path.is_dir() {
                        std::fs::remove_dir_all(path)?;
                    }
                }
                _ => system_error("")
            }
        }
    }

    println!("Removal process complete.");
    return Ok(());
}

fn system_error(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(-1);
}

fn confirmation<P>(path: &P) -> bool 
where 
    P: AsRef<Path> {

    eprint!("Are you sure you want to delete '{}'? [y/n]: ", path.as_ref().display());
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();

    let choice = result.as_bytes()[0].to_ascii_lowercase();
    match choice {
        b'y' => true,
        _ => false
    }
}

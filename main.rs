use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const IGNORE_DIRS: &[&str] = &[".git"];


fn get_dir_name(path: &Path) -> &str {
    return path.file_name()
               .and_then(|name| name.to_str())
               .unwrap_or_default()
}

fn skip_dir(path: &Path) -> bool {
    let dir_name = get_dir_name(path);
    IGNORE_DIRS.iter().any(|ignored| *ignored == dir_name) 
}

fn list_all_pass_files(dir: &Path) -> Result<HashMap<PathBuf, Vec<PathBuf>>> {
    let mut all_files = HashMap::new();
    let mut files_here = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if skip_dir(&path) {
                continue;
            }
            all_files.extend(list_all_pass_files(&path)?);
        } else {
            files_here.push(path);
        }
    }

    all_files.insert(dir.to_path_buf(), files_here);
    return Ok(all_files)
}

fn write_to_csv(
        files_by_dir: &HashMap<PathBuf,
        Vec<PathBuf>>, output: &mut fs::File) -> Result<()> {
    writeln!(output, "Title,Url,Username,Password");
    
    for (dir, files) in files_by_dir {
        write_dir_to_csv(dir, files)?;
    }

    Ok(()) 
}

fn write_dir_to_csv(dir: &PathBuf, files: &Vec<PathBuf>) -> Result<()> {
    let dir_name = get_dir_name(dir);

    for file in files {
        let file_name = get_dir_name(file);
        println!("{} @ {}", file_name, dir_name)
    }

    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let [_, dir_arg, output_arg] = args.as_slice() else {
        return Err(format!(
            "Usage: {} <password store directory> output.csv", args[0]).into())
    };

    let pass_dir = Path::new(&dir_arg);
    if !pass_dir.is_dir() {
        return Err(format!("'{}' is not a directory.", pass_dir.display()).into())
    }

    let mut output_csv = fs::File::create(Path::new(output_arg))?;

    let files_by_dir = list_all_pass_files(pass_dir)?;
    //for (dir, files) in &files_by_dir {
    //    println!("{}:", dir.display());
    //    for file in files {
    //        println!("{}", file.display());
    //    }
    //}

    write_to_csv(&files_by_dir, &mut output_csv)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

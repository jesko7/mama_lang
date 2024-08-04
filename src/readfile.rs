use std::io::{self, BufRead, Read, Write};
use std::iter::zip;
use std::path::Path;
use std::env;
use std::fs::{self, File};

pub fn get_files_with_extension<P: AsRef<Path>>(dir: P, ext: &str) -> io::Result<Vec<std::path::PathBuf>> {
    let mut files_with_extension = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == ext {
                    files_with_extension.push(path);
                }
            }
        } else if path.is_dir() {
            let mut sub_dir_files = get_files_with_extension(&path, ext)?;
            files_with_extension.append(&mut sub_dir_files);
        }
    }

    Ok(files_with_extension)
}

pub fn read(endung: &str) -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    let current_dir = match env::current_dir() {
        Ok(path) => {
            path
        },
        Err(e) => {
            panic!("Fehler beim Abrufen des aktuellen Verzeichnisses: {}", e);
        }
    };

    let files_result = get_files_with_extension(current_dir, endung);
    let files;
    let mut file_path = "".to_string();
    if files_result.is_err() {
        panic!("couldnt get .{endung} files: {:?}", files_result.err());
    }
    else {
        files = files_result.unwrap();

        if files.len() == 0 {
            panic!("no .{endung} files found")
        }

        let mut names: Vec<String> = vec![];
        let mut paths: Vec<String> = vec![];

        for file_buf in files {
            let path: String = file_buf.as_path().display().to_string();
            paths.push(path.clone());
            let path_names = &path.split(r"/".chars().collect::<Vec<char>>()[0]).collect::<Vec<&str>>();
            let name = path_names.clone()[path_names.clone().len() - 1].split('.').collect::<Vec<&str>>()[0];
            names.push(name.to_string());

            if name == "main" {
                file_path = path;
                println!("running main.{endung} ...\n-------------------------------------------------------------------------------------------\n");
            }
        }

        if file_path.is_empty() {
            println!("WARING: no main.{endung} program found\n-------------------------------------------------------------------------------------------");

            if names.len() == 1 {
                file_path = paths.clone()[0].clone();
                println!("running {}.{}\n-------------------------------------------------------------------------------------------\n", names[0], endung)

            } else {
                println!("found {} .{endung} programs:", names.len());
                for (name, path) in zip(names.clone(), paths.clone()) {
                    println!("  {name}.{endung} - {path}");
                }
                println!("");
                let idx;
                loop {
                    let mut programm_name  = input("-------------------------------------------------------------------------------------------\nplease enter the name of the program that should be executed (exit to stop): ").trim_end().to_string();
                    if programm_name == "exit" {
                        panic!("exited");
                    }
                    if !programm_name.contains(&(".".to_owned() + endung)) {
                        programm_name += &(".".to_owned() + endung);
                    }

                    if names.clone().contains(&programm_name.strip_suffix(&(".".to_owned() + endung)).unwrap().to_string()) {
                        idx = names.iter().position(|x| x.clone() == programm_name.strip_suffix(&(".".to_owned() + endung)).unwrap().to_string()).unwrap();
                        break;
                    } else {
                        println!("file: {} not found\n-------------------------------------------------------------------------------------------", programm_name);
                    }
                }
                
                file_path = paths.clone()[idx].clone();
                println!("-------------------------------------------------------------------------------------------\nrunning {}.{}\n-------------------------------------------------------------------------------------------\n", names[idx], endung)
            }
        }
    }

    let path = Path::new(&file_path);

    let file = File::open(&path).expect("couldnt load file");

    let mut reader: io::BufReader<File> = io::BufReader::new(file);
    let mut lines_string = String::new();

    let _ = reader.read_to_string(&mut lines_string);

    let mut linestoread = vec![];

    for line in lines_string.split(';') {
        let inner_lines = line.split_inclusive("{");
        for inner_line in inner_lines {
            let inner_inner_lines = inner_line.split_inclusive("}");

            for inner_inner_line in inner_inner_lines {
                linestoread.push(inner_inner_line);
            }
        }
    }

    for line in linestoread {
            lines.push(line.trim().to_string().replace("\n", ""));
    }

    lines
}

fn input<T>(inp: T) -> String
where 
T: std::fmt::Display
{
    let mut input = String::new();
    print!("{}", inp);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("bro");
    input
}

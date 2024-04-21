use catppuccin::PALETTE;
use glob::glob;
use std::io;
use std::fs;
use std::env;
use std::path::PathBuf;
use regex::Regex;
use unidecode::unidecode;

const fn ansi(color: &catppuccin::Color) -> ansi_term::Colour {
    ansi_term::Colour::RGB(color.rgb.r, color.rgb.g, color.rgb.b)
}

fn main() {
    // use these args, otherwise use the working directory
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    if args.len() > 1 {
        for arg in &args {
            println!("{}", arg);
        }
        let paths: Vec<PathBuf> = 
        args[1..]
            .iter()
            .map(|arg| PathBuf::from(arg))
            .collect();
        convert_files(paths);
    } else {
        println!("No files specified, scan working directory for files to convert? (Y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim().to_lowercase();
        if input == "y" || input == "" {
            // get file names in working directory
            println!("Scanning working directory for files to convert...");
            let files = glob("*").unwrap();
            convert_files(files.map(|f| f.unwrap()).collect());
        } else {
            println!("Exiting...");
        };
    }


    
}

fn convert_files(files: Vec<PathBuf>) {
    println!("{}",ansi_term::Colour::Blue.bold().paint("Processing files..."));
    // dbg!(&files);
    for file in files {
        if file.metadata().unwrap().is_file() {
            println!(
                "{} {:?}", 
                ansi_term::Colour::Blue.bold().paint("Processing file"),
                file
            );
            let contents;
            match fs::read_to_string(&file) {
                Ok(c) => {
                    contents = c;
                    // println!("{}", contents);
                },
                Err(e) => {
                    println!(
                        "{}: {}, error: {}",
                        ansi_term::Colour::Red.paint("Failed to read file"),
                        file.display(),
                        e
                    );
                    continue;
                },
            }
            // println!("{}", contents);

            let flavor = detect_flavor(&contents);
            match flavor {
                Some(flavor) => {
                    println!(
                        "{}: {}",
                        ansi_term::Colour::Green.paint("Detected flavor"),
                        flavor
                    );
                    convert(&file, &contents, flavor);
                    check(&contents, flavor);
                },
                None => println!("No flavor detected"),
            }
        } else {
            println!("\nSkipping directory: {:?}", file);
            continue;
        }
    }
}

fn detect_flavor(contents: &str) -> Option<catppuccin::FlavorName> {
    for flavor in &PALETTE {
        for color in flavor.colors.iter() {
            let hex_color = color.hex.to_string();
            // check if contents includes color hex
            if contents.to_lowercase().contains(&hex_color) {
                println!("{}: {} ({} {}) ",
                    ansi_term::Colour::Green.paint("Detected color"),
                    color.name,
                    ansi(color).reverse().paint("  "),
                    hex_color
                );
                return Some(flavor.name);
            }
        }
    }
    None
}

fn convert(path: &PathBuf, contents: &str, flavorname: catppuccin::FlavorName) {
    // dbg!(&contents);
    for flavor in &PALETTE {
        let mut new_contents = contents.to_string();
        if flavor.name == flavorname {
            continue;
        }
        println!("{}: {}",
            ansi_term::Colour::Yellow.paint("Converting to flavor"),
            flavor.name
        );
        
        for c in flavor.colors.iter() {
            let equiv_color = PALETTE.get_flavor(flavorname).colors[c.name];
            let re = Regex::new(&format!(r"(?i){}", equiv_color.hex.to_string())).unwrap();
            if new_contents.to_lowercase().contains(&equiv_color.hex.to_string()) {
                println!(
                    "{} → {} | {} → {} ({})",
                    ansi(&equiv_color).reverse().paint("  "),
                    ansi(&c).reverse().paint("  "),
                    equiv_color.hex.to_string(),
                    c.hex.to_string(),
                    c.name,
                );
                new_contents = re.replace_all(&new_contents, c.hex.to_string()).to_string();
            }
        }
        // dbg!(&new_contents);

        // write new contents to file
        let new_name = if let Some(extension) = path.extension() {
            format!(
                "../{}/{}.{}",
                unidecode(&flavor.name.to_string()).to_string().to_lowercase(),
                path.file_stem().unwrap().to_str().unwrap(),
                extension.to_str().unwrap()
            )
        } else {
            format!(
                "../{}/{}",
                unidecode(&flavor.name.to_string()).to_string().to_lowercase(),
                path.file_stem().unwrap().to_str().unwrap()
            )
        };
        let new_path = path.with_file_name(new_name);

        let new_dir = new_path.parent().unwrap();
        fs::create_dir_all(&new_dir).expect("Failed to create directory");

        match fs::write(&new_path, new_contents) {
            Ok(_) => println!(
                "{}: {}",
                ansi_term::Colour::Green.paint("File written"),
                new_path.display()
            ),
            Err(e) => println!(
                "{} {}: {}",
                ansi_term::Colour::Red.paint("Failed to write file to"),
                new_path.display(),
                e
            ),
        }
    }
}

fn check(contents: &str, flavorname: catppuccin::FlavorName) -> Vec<String> {
    // Get all hex codes in `contents` of type hex
    let hex_codes: Vec<String> = contents
        .split_whitespace()
        .filter(|word| word.starts_with("#"))
        .map(|word| word.to_string().to_lowercase())
        .collect();
    // dbg!(&hex_codes);

    let mut unknown_colors = Vec::new();

    for hex_code in hex_codes {
        let flavor_colors = PALETTE.get_flavor(flavorname).colors;
        if !flavor_colors.iter().any(|color| color.hex.to_string() == hex_code) {
            println!(
                "{} Hex code {} is not contained in the palette",
                ansi_term::Colour::Red.paint("Warning:"),
                hex_code
            );
            unknown_colors.push(hex_code);
        }
    }

    unknown_colors
}

#[cfg(test)]
#[path = "./main_tests.rs"]
mod tests;
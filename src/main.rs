use catppuccin::PALETTE;
use glob::glob;
use std::io;
use std::fs;
use std::env;
use std::path::PathBuf;

const fn ansi(color: &catppuccin::Color) -> ansi_term::Colour {
    ansi_term::Colour::RGB(color.rgb.r, color.rgb.g, color.rgb.b)
}

fn main() {
// for flavor in &PALETTE {
    //     let heading = format!(
    //         "{} ({})",
    //         flavor.name,
    //         if flavor.dark { "dark" } else { "light" }
    //     );
    //     println!(
    //         "{}\n",
    //         ansi_term::Style::new().underline().bold().paint(heading)
    //     );

    //     for color in flavor {
    //         let name = format!(
    //             "{}{}",
    //             color.name,
    //             if color.accent { " (accent)" } else { "" }
    //         );
    //         let rgb = format!(
    //             "rgb({:3}, {:3}, {:3})",
    //             color.rgb.r, color.rgb.g, color.rgb.b
    //         );
    //         let hsl = format!(
    //             "hsl({:3.0}, {:5.3}, {:5.3})",
    //             color.hsl.h, color.hsl.s, color.hsl.l
    //         );
    //         println!(
    //             "{} {:18} →  {:6}  {:18}  {:18}",
    //             ansi(color).reverse().paint("  "),
    //             name,
    //             color.hex,
    //             rgb,
    //             hsl,
    //         );
    //     }
    //     println!();
    // }

    /////////////////////////////////////////////////////////////////
    
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
    dbg!(&contents);
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
            if new_contents.to_lowercase().contains(&equiv_color.hex.to_string()) {
                println!(
                    "{} → {} | {} → {} ({})",
                    ansi(&equiv_color).reverse().paint("  "),
                    ansi(&c).reverse().paint("  "),
                    equiv_color.hex.to_string(),
                    c.hex.to_string(),
                    c.name,
                );
                new_contents = new_contents.replace(&equiv_color.hex.to_string(), &c.hex.to_string());
            }
        }
        dbg!(&new_contents);

        // write new contents to file
        let new_name = format!(
            "../{}/{}.{}",
            flavor.name.to_string().to_lowercase(),
            path.file_stem().unwrap().to_str().unwrap(),
            path.extension().unwrap().to_str().unwrap()
        );
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
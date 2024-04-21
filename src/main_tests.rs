#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs::File;
    use std::io::Write;
    use catppuccin::ColorName;
    use tempfile::tempdir;

    #[test]
    fn test_convert_files() {
        // Step 1: Create a temporary directory and a file in it with specific content.
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        let contents = "Unknown hex: #ffffff | Pink and green: #f5bdE6 #A6DA95";
        let flavor_name = detect_flavor(&contents).unwrap();
        writeln!(file, "{}", &contents).unwrap();

        // Step 2: Call the `convert_files` function with the path to the created file.
        convert_files(vec![file_path.clone()]);

        // Step 3: Check that the expected files were created in the correct directories.
        for flavor in &PALETTE {
            if flavor.name == flavor_name {
                continue;
            }
            let expected_path = file_path.with_file_name(format!(
                "../{}/test_file.txt",
                flavor.name.to_string().to_lowercase()
            ));
            assert!(expected_path.exists(), "File {} was not created", expected_path.display());
        }

        // Step 4: Check that the content of the created files is as expected.
        for flavor in &PALETTE {
            if flavor.name == flavor_name {
                continue;
            }
            let expected_path = file_path.with_file_name(format!(
                "../{}/test_file.txt",
                flavor.name.to_string().to_lowercase()
            ));
            let expected_contents = format!(
                "Unknown hex: #ffffff | Pink and green: {} {}\n",
                flavor.colors[ColorName::Pink].hex.to_string(),
                flavor.colors[ColorName::Green].hex.to_string()
            );
            let actual_contents = std::fs::read_to_string(&expected_path).unwrap();
            assert_eq!(actual_contents, expected_contents);
        }

        // Step 5: Clean up the temporary directory.
        dir.close().unwrap();
    }

    #[test]
    fn test_check() {
        // Step 1: Create a temporary directory and a file in it with specific content.
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        let contents = "Unknown hex: #ffffff #4A1555 | Pink and green: #f5bdE6 #a6Da95";
        let flavor_name = detect_flavor(&contents).unwrap();
        writeln!(file, "{}", &contents).unwrap();

        // Step 2: Call the `check` function with the path to the created file.
        let unknown_colors = check(&contents, flavor_name);

        // Step 3: Check that the expected unknown colors were detected.
        let expected_unknown_colors = vec!["#ffffff", "#4a1555"];
        assert_eq!(unknown_colors, expected_unknown_colors);

        // Step 4: Clean up the temporary directory.
        dir.close().unwrap();
    }
}
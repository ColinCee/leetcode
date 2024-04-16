use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the problem title from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a problem title.");
        return;
    }
    let problem_title = &args[1];

    // Extract the problem number and name from the title
    let parts: Vec<&str> = problem_title.split(". ").collect();
    if parts.len() != 2 {
        println!("Invalid problem title format. Expected format: 'Number. Problem Name'");
        return;
    }
    let problem_number = parts[0].parse::<i32>().expect("Invalid problem number");
    let problem_name = parts[1].to_lowercase().replace(" ", "_");

    // Generate the module name and paths
    let module_name = format!("p{problem_number:04}_{problem_name}");
    let module_path = format!("src/{module_name}");
    let mod_file_path = format!("{module_path}/mod.rs");
    let bin_file_path = format!("src/bin/p{problem_number:04}.rs");

    // Create the module directory and mod.rs file
    fs::create_dir_all(&module_path).expect("Failed to create module directory");
    fs::write(&mod_file_path, "pub fn solve() {\n    // Implement the solution here\n}\n").expect("Failed to create mod.rs file");

    // Create the binary file
    let bin_file_content = format!(
        "use leetcode::{module_name};\n\nfn main() {{\n    {module_name}::solve();\n}}\n"
    );
    fs::write(&bin_file_path, bin_file_content).expect("Failed to create binary file");

    // Update lib.rs
    let lib_file_path = "src/lib.rs";
    let lib_file_content = fs::read_to_string(&lib_file_path).expect("Failed to read lib.rs");
    let lines: Vec<&str> = lib_file_content.lines().collect();
    let mut new_lines = Vec::new();
    let mut module_inserted = false;

    for line in lines {
        if line.starts_with("pub mod ") && !module_inserted {
            let existing_module_name = line.trim_start_matches("pub mod ").trim_end_matches(';').to_string();
            println!("{} - {}", module_name, existing_module_name);
            if module_name < existing_module_name {
                new_lines.push(format!("pub mod {module_name};"));
                module_inserted = true;
            }
        }
        new_lines.push(line.to_string());
    }

    let new_lib_file_content = new_lines.join("\n");
    fs::write(&lib_file_path, new_lib_file_content).expect("Failed to update lib.rs");

    println!("Generated module and binary files for problem: {problem_title}");
}

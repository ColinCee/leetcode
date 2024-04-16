use std::env;
use std::fs;

fn main() {
    // Get the problem title from command line arguments
    let problem_title = get_problem_title();

    // Extract the problem number and name from the title
    if let Some((problem_number, problem_name)) = parse_problem_title(&problem_title) {
        // Generate the module name and paths
        let module_name = format!("p{:04}_{}", problem_number, problem_name);
        let module_path = format!("src/{}", module_name);
        let mod_file_path = format!("{}/mod.rs", module_path);
        let bin_file_path = format!("src/bin/p{:04}.rs", problem_number);

        // Create the module directory and mod.rs file
        create_module_files(&module_path, &mod_file_path);

        // Create the binary file
        create_binary_file(&bin_file_path, &module_name);

        // Update lib.rs
        update_lib_file(&module_name);

        println!("Generated module and binary files for problem: {}", problem_title);
    } else {
        println!("Invalid problem title format. Expected format: 'Number. Problem Name'");
    }
}

fn get_problem_title() -> String {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(title) => title.to_string(),
        None => {
            println!("Please provide a problem title.");
            std::process::exit(1);
        }
    }
}

fn parse_problem_title(problem_title: &str) -> Option<(i32, String)> {
    let parts: Vec<&str> = problem_title.split(". ").collect();
    if let [problem_number, problem_name] = parts[..] {
        match problem_number.parse() {
            Ok(num) => Some((num, problem_name.to_lowercase().replace(" ", "_"))),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn create_module_files(module_path: &str, mod_file_path: &str) {
    fs::create_dir_all(module_path).expect("Failed to create module directory");
    fs::write(mod_file_path, "pub fn solve() {\n    // Implement the solution here\n}\n")
        .expect("Failed to create mod.rs file");
}

fn create_binary_file(bin_file_path: &str, module_name: &str) {
    let bin_file_content = format!(
        "use leetcode::{};\n\nfn main() {{\n    {}::solve();\n}}\n",
        module_name, module_name
    );
    fs::write(bin_file_path, bin_file_content).expect("Failed to create binary file");
}

fn update_lib_file(module_name: &str) {
    let lib_file_path = "src/lib.rs";
    let lib_file_content = fs::read_to_string(&lib_file_path).expect("Failed to read lib.rs");
    let mut lines: Vec<String> = lib_file_content.lines().map(|line| line.to_string()).collect();
    let insert_index = lines
        .iter()
        .position(|line| line.starts_with("pub mod ") && line[8..] > *module_name)
        .unwrap_or(lines.len());
    lines.insert(insert_index, format!("pub mod {};", module_name));
    let new_lib_file_content = lines.join("\n");
    fs::write(&lib_file_path, new_lib_file_content).expect("Failed to update lib.rs");
}

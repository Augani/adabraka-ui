use std::path::Path;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("Current working directory: {:?}", cwd);

    // Test the paths
    let test_paths = vec![
        "crates/adabraka-ui/assets/icons/heart.svg",
        "assets/icons/heart.svg",
        "src/icons/heart.svg",
    ];

    for path in test_paths {
        let full_path = Path::new(path);
        let exists = full_path.exists();
        println!("  Path: {} → {}", path, if exists { "✓ EXISTS" } else { "✗ MISSING" });
    }
}

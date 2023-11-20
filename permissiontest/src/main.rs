use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::Duration;

fn main() {
    let file_path = "../test.md";

    let metadata = fs::metadata(&file_path).expect("Failed to retrieve file metadata");

    let mut original = metadata.permissions();

    let mut permissions = original.clone();

    permissions.set_readonly(true); 

    fs::set_permissions(Path::new(&file_path), permissions)
        .expect("Failed to set file permissions");

    println!("File permissions set to read-only for: {}", file_path);

    std::thread::sleep(Duration::from_secs(30));

    fs::set_permissions(Path::new(&file_path), original)
        .expect("Failed to set file permissions");

    println!("File permissions set to normal");

}


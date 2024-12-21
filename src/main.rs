mod decoder;
mod processor;

use processor::process_bin;
use std::env;
use std::fs;

/*
 * 137  -> 10001001  -> 100010|d|w  -> 100010 (mov) | d = 0     | w = 1
 * 217  -> 11011001  -> mod|reg|r/m -> mod = 11     | reg = 011 | r/m = 001
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Expected 1 argument, found {}", args.len() - 1);
        return;
    }

    let file_path: &str = &args[1].as_str();
    println!("; Read from the binary\n");

    let contents: Vec<u8> = fs::read(&file_path).expect("Error reading file");
    for bytes in contents.chunks(8) {
        for byte in bytes {
            print!("{:08b} ", byte);
        }
        print!("\n")
    }
    println!("; raw data\n===\n");

    let processed: String = process_bin(&contents);
    println!("{}", processed);
}

#[cfg(test)]
mod tests {
    use std::{ffi::c_void, fs::File, io::Write, process::Command};

    use super::*;

    fn normalize_asm(content: &str) -> String {
        content
            .split("\n")
            .filter(|line| !line.starts_with(";"))
            .filter(|line| !line.chars().all(|c| c.is_whitespace()))
            .collect::<Vec<&str>>()
            .join("\n")
    }

    fn rebuild_asm(content: &String, bin_file_path: &String) -> String {
        if fs::metadata("./out").is_err() {
            fs::create_dir_all("./out").expect("Error creating './out' directory");
        }

        let test_name = bin_file_path.split("/").last().unwrap();

        let asm_file_path = format!("./out/test_{}.asm", test_name);
        let out_file_path = format!("./out/test_{}.o", test_name);

        let mut asm_file = File::create(&asm_file_path).unwrap();
        asm_file.write(content.as_bytes()).unwrap();

        Command::new("nasm")
            .args([&asm_file_path, "-o", &out_file_path])
            .output()
            .expect("Failed to build asm");

        out_file_path
    }

    fn compare_bin(bin_file: &str) {
        let original_bin_content: Vec<u8> = fs::read(bin_file).expect("Error reading file");
        let original_bin_content_processed: String = process_bin(&original_bin_content);

        let original_bin_content_processed_rebuild: String =
            rebuild_asm(&original_bin_content_processed, &bin_file.to_string());
        let rebuilded_bin_content: Vec<u8> =
            fs::read(original_bin_content_processed_rebuild).expect("Error reading file");

        if original_bin_content.len() != rebuilded_bin_content.len() {
            assert!(false)
        }

        unsafe {
            if libc::memcmp(
                original_bin_content.as_ptr() as *const c_void,
                rebuilded_bin_content.as_ptr() as *const c_void,
                original_bin_content.len(),
            ) == 0
            {
                assert!(true)
            }
        }
    }

    fn compare_asm(asm_file: &str, bin_file: &str) {
        let original_asm_content: String =
            fs::read_to_string(asm_file).expect("Error reading file");
        let original_asm_content_normalized: String = normalize_asm(&original_asm_content);

        let original_bin_content: Vec<u8> = fs::read(bin_file).expect("Error reading file");
        let original_bin_content_processed: String = process_bin(&original_bin_content);
        let original_bin_content_processed_normalized: String =
            normalize_asm(&original_bin_content_processed);

        assert_eq!(
            original_asm_content_normalized,
            original_bin_content_processed_normalized
        );
    }

    #[test]
    fn asm_cmp_listing_0037_single_register_mov() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov";

        compare_asm(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn asm_cmp_listing_0038_many_register_mov() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov";

        compare_asm(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn asm_cmp_listing_0039_more_movs() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0039_more_movs.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0039_more_movs";

        compare_asm(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn asm_cmp_listing_0040_challenge_movs() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0040_challenge_movs.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0040_challenge_movs";

        compare_asm(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn bin_cmp_listing_0037_single_register_mov() {
        const BIN_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov";

        compare_bin(BIN_FILE_PATH)
    }

    #[test]
    fn bin_cmp_listing_0038_many_register_mov() {
        const BIN_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov";

        compare_bin(BIN_FILE_PATH)
    }

    #[test]
    fn bin_cmp_listing_0039_more_movs() {
        const BIN_FILE_PATH: &str = "./vendor/listing_0039_more_movs";

        compare_bin(BIN_FILE_PATH)
    }

    #[test]
    fn bin_cmp_listing_0040_challenge_movs() {
        const BIN_FILE_PATH: &str = "./vendor/listing_0040_challenge_movs";

        compare_bin(BIN_FILE_PATH)
    }

    #[test]
    #[ignore]
    fn bin_cmp_listing_0041_add_sub_cmp_jnz() {
        const BIN_FILE_PATH: &str = "./vendor/listing_0041_add_sub_cmp_jnz";

        compare_bin(BIN_FILE_PATH)
    }

    #[test]
    #[ignore]
    fn bin_cmp_listing_0042_completionist_decode() {
        const BIN_FILE_PATH: &str = "./vendor/listing_0042_completionist_decode";

        compare_bin(BIN_FILE_PATH)
    }
}

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
    use super::*;

    fn normalize_asm(content: &str) -> String {
        content
            .split("\n")
            .filter(|line| !line.starts_with(";"))
            .filter(|line| !line.chars().all(|c| c.is_whitespace()))
            .collect::<Vec<&str>>()
            .join("\n")
    }

    fn compare(asm_file: &str, bin_file: &str) {
        let original_asm_content: String =
            fs::read_to_string(asm_file).expect("Error reading file");
        let original_asm_content_normalized: String = normalize_asm(&original_asm_content);

        let bin_to_asm_content: Vec<u8> = fs::read(bin_file).expect("Error reading file");
        let bin_to_asm_content_processed: String = process_bin(&bin_to_asm_content);
        let bin_to_asm_content_normalized: String = normalize_asm(&bin_to_asm_content_processed);

        assert_eq!(
            original_asm_content_normalized,
            bin_to_asm_content_normalized
        );
    }

    #[test]
    fn listing_0037_single_register_mov() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov";

        compare(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn listing_0038_single_register_mov() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov";

        compare(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn listing_0039_more_movs() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0039_more_movs.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0039_more_movs";

        compare(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn listing_0040_more_movs() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0040_challenge_movs.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0040_challenge_movs";

        compare(ASM_FILE_PATH, BIN_FILE_PATH)
    }
}

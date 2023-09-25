#[cfg(test)]
mod tests {
    use std::path::Path;
    use tiger::digest::Digest;
    use tiger::Tiger;

    #[test]
    fn path_exists() {
        let root = env!("CARGO_MANIFEST_DIR");
        println!("{root}");
        let path = Path::new(root);
        let file_path = path.join("sounds").join("shabeko_dayo.wav");
        println!("{}", file_path.display());
        assert!(file_path.exists());
    }

    #[test]
    fn digest_str() {
        let id = "99999999999999999999999999";
        let digest = Tiger::digest(id.as_bytes());
        let digest_str = format!("{digest:X}");
        assert_eq!(
            digest_str,
            "7EABF4E47410D6A9FCF10B802CE754E5357120F7081B840B"
        );
    }
}

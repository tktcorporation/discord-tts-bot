pub struct Path {
    value: std::path::PathBuf,
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Path {
    pub fn new() -> Self {
        let value: std::path::PathBuf = super::root_path().join("sounds");
        Path { value }
    }

    pub fn welcome_audio_path(&self) -> std::path::PathBuf {
        self.value.join("shabeko_dayo.wav")
    }
}

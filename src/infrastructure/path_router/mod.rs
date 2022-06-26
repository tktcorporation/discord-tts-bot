mod guild_path;
pub use guild_path::GuildPath;
mod shared_sound;
pub use shared_sound::Path as SharedSoundPath;

use std::path::{Path, PathBuf};

fn root_path() -> PathBuf {
    let base = env!("CARGO_MANIFEST_DIR");
    Path::new(base).into()
}
fn tmp_path() -> PathBuf {
    root_path().join("tmp")
}

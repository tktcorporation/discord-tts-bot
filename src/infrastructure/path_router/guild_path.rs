use serenity::model::id;
use std::path::PathBuf;
use tiger::{digest::Digest, Tiger};

#[derive(Debug, PartialEq, Eq)]
pub struct GuildPath {
    value: PathBuf,
}

impl GuildPath {
    pub fn new(guild_id: &id::GuildId) -> Self {
        let root: std::path::PathBuf = super::tmp_path();
        let digest = Tiger::digest(guild_id.to_string().as_bytes());
        let guild_id_digest_str = format!("{:X}", digest);
        let guild_path = root.join(guild_id_digest_str);
        std::fs::create_dir_all(&guild_path).expect("fail to create a dir of guild path");
        guild_path.into()
    }
}

impl From<PathBuf> for GuildPath {
    fn from(item: PathBuf) -> GuildPath {
        GuildPath { value: item }
    }
}

impl From<GuildPath> for PathBuf {
    fn from(item: GuildPath) -> PathBuf {
        item.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn to_pathbuf() {
        let path = Path::new("root").join("sounds");
        let guild_path = GuildPath { value: path };
        assert_eq!("root/sounds", PathBuf::from(guild_path).to_str().unwrap());
    }

    #[test]
    fn to_guild_path() {
        let path = Path::new("root").join("sounds");
        let guild_path: GuildPath = path.into();
        assert_eq!(guild_path.value.to_str().unwrap(), "root/sounds");
    }
}

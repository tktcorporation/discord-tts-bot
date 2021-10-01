use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct GuildPath {
    value: PathBuf,
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

use serenity::Error as SerenityError;
use songbird::error::JoinError;
use songbird::tracks::TrackError;
use std::error::Error as StdError;
use std::fmt;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    NotInVoiceChannel,
    ErrorSourcingFfmpeg,
    AlreadyJoined,
    JoinError(JoinError),
    SerenityError(SerenityError),
    TrackError(TrackError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotInVoiceChannel => f.write_str("Not in a voice channel."),
            Error::ErrorSourcingFfmpeg => f.write_str("Error sourcing ffmpeg."),
            Error::AlreadyJoined => f.write_str("Already joined. I'm busy!"),
            Error::JoinError(e) => e.fmt(f),
            Error::SerenityError(e) => e.fmt(f),
            Error::TrackError(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

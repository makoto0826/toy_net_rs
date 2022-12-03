use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invaild size")]
    InvalidSize,
    #[error("invaild packet")]
    InvalidPacket,

    #[error("io error:{error}")]
    IoError {
        #[from]
        #[source]
        error: ::std::io::Error,
    },
}

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum BootspecError {
    #[error("failed to synthesize: {0}")]
    Synthesize(#[from] SynthesizeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0} had an invalid file name")]
    InvalidFileName(PathBuf),
    #[error("{0} contained invalid UTF8")]
    InvalidUtf8(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum SynthesizeError {
    #[error("unsupported schema version {0}")]
    UnsupportedVersion(u64),
    #[error("failed to canonicalize {path}: {err}")]
    Canonicalize {
        path: PathBuf,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to read {path}: {err}")]
    ReadPath {
        path: PathBuf,
        #[source]
        err: std::io::Error,
    },
    #[error("could not find kernel version dir in {0}")]
    MissingKernelVersionDir(PathBuf),
}

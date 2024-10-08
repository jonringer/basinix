use std::{str::Utf8Error, error::Error as StdError, fmt, process::ExitStatus};

use log::error;
use reqwest;
use derive_more::{From, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
trait ErrorLog {
    fn log_if_error(&self);
}

impl<T> ErrorLog for Result<T> {
    fn log_if_error(&self) {
        if let Err(e) = &self {
            error!("{}", e);
        }
    }
}

#[derive(Debug, derive_more::Error, From, Display)]
pub enum Error {
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    SerdeJson(serde_json::Error),
    Utf8Error(Utf8Error),
    Basinix(BasinixError),
}

/// Basinix-specific Errors
#[derive(From)]
pub struct BasinixError {
    inner: Box<Inner>,
}

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    source: Option<BoxError>,
}

impl BasinixError {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> BasinixError
    where
      E: Into<BoxError>
    {
        BasinixError {
            inner: Box::new(Inner {
                kind,
                source: source.map(|e| e.into()),
            }),
        }
    }
}

impl fmt::Debug for BasinixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("basinix::shared::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref message) = self.inner.source {
            builder.field("message", message);
        }

        builder.finish()
    }
}

impl fmt::Display for BasinixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner.kind {
            Kind::GitError => f.write_str("git error")?,
            Kind::ProcessError(exit_status) => f.write_str(&format!("process error: {}", exit_status))?,
        };

        if let Some(ref e) = self.inner.source {
            write!(f, ": {}", e)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    GitError,
    ProcessError(ExitStatus),
}

impl StdError for BasinixError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

// constructors

pub fn git_error<E: Into<BoxError>>(e: E) -> BasinixError {
    BasinixError::new(Kind::GitError, Some(e))
}

pub fn process_error(exit_status: ExitStatus) -> BasinixError {
    BasinixError::new(Kind::ProcessError(exit_status), None::<BasinixError>)
}

use curl::easy;
use curl::easy::HttpVersion;
use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

#[derive(Clone, Debug)]
struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        let code = e.code() as i32; // due to windows build
        let description = match e.extra_description() {
            None => e.description().to_string(),
            Some(s) => s.to_string(),
        };
        Error(format!("{code}: {description}"))
    }
}

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let url = &args[1];

    let mut handle = easy::Easy::new();

    handle.url(url)?;
    handle.nobody(true)?;
    handle.http_version(HttpVersion::V3)?;
    handle.verbose(true)?;
    handle.timeout(Duration::from_secs(20))?;

    let transfer = handle.transfer();
    transfer.perform()?;

    Ok(())
}

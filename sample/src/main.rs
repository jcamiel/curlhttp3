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

    handle.reset();

    handle.buffer_size(102400)?;
    handle.url(url)?;
    handle.progress(false)?;
    handle.nobody(true)?;
    handle.useragent("sample/0.0.1")?;
    handle.max_redirections(50)?;
    handle.http_version(HttpVersion::V3)?;
    handle.fetch_filetime(true)?;
    handle.verbose(true)?;

    handle.cookie_file("")?;
    handle.certinfo(true)?;
    handle.ssl_verify_peer(true)?;
    handle.ssl_verify_host(true)?;

    handle.timeout(Duration::from_secs(20))?;
    handle.connect_timeout(Duration::from_secs(20))?;

    let transfer = handle.transfer();
    transfer.perform()?;

    Ok(())
}

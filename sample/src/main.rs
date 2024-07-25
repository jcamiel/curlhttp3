use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;
use curl::easy;
use curl::easy::{HttpVersion, IpResolve, SslOpt};

struct Client {
    /// The handle to libcurl binding
    handle: easy::Easy,
}

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


impl Client {
    pub fn new() -> Client {
        let handle = easy::Easy::new();
        Client { handle }
    }

    pub fn execute(&mut self) -> Result<(), Error>{
        self.handle.reset();
        self.configure()?;

        let mut response_body = Vec::<u8>::new();
        {
            let mut transfer = self.handle.transfer();
            transfer.write_function(|data| {
                response_body.extend(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        Ok(())
    }

    fn configure(&mut self) -> Result<(), Error>{
        self.handle.cookie_file("")?;
        self.handle.http_version(HttpVersion::V3)?;
        self.handle.ip_resolve(IpResolve::Any)?;
        self.handle.certinfo(true)?;
        self.handle.ssl_verify_host(true)?;
        self.handle.ssl_verify_peer(true)?;
        self.handle.path_as_is(false)?;
        self.handle.timeout(Duration::from_secs(20))?;
        self.handle.connect_timeout(Duration::from_secs(20))?;
        self.set_ssl_options(false)?;
        self.handle.url("https://google.com")?;
        self.handle.custom_request("HEAD")?;
        self.handle.nobody(true)?;
        Ok(())
    }

    fn set_ssl_options(&mut self, no_revoke: bool) -> Result<(), Error> {
        let mut ssl_opt = SslOpt::new();
        ssl_opt.no_revoke(no_revoke);
        self.handle.ssl_options(&ssl_opt)?;
        Ok(())
    }
}


fn main() -> Result<(), Error> {
    let mut client = Client::new();
    client.execute()?;
    Ok(())
}

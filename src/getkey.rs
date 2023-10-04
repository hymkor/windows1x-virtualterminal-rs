pub enum Error {
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
}

pub fn getkey() -> std::result::Result<String, Error> {
    use std::io::Read;

    let mut buffer: Vec<u8> = vec![0; 256];
    let mut stdin = std::io::stdin();

    let n = match stdin.read(&mut buffer) {
        Ok(n) => n,
        Err(err) => return Err(Error::Io(err)),
    };
    buffer.truncate(n);
    match String::from_utf8(buffer) {
        Ok(s) => return Ok(s),
        Err(err) => return Err(Error::FromUtf8(err)),
    }
}

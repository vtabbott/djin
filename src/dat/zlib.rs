use eyre::Result;
use flate2::read::DeflateDecoder;
use std::io::{Cursor, Read};
use std::path::Path;

pub(crate) fn decompress<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Cursor<Vec<u8>>> {
    let file = std::fs::read(path)?;
    let mut decoded = DeflateDecoder::new(file.as_slice());
    let mut data = Vec::with_capacity(decoded.total_out() as usize);
    decoded.read_to_end(&mut data)?;
    Ok(Cursor::new(data))
}

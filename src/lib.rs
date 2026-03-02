use std::io;

use io::BufWriter;
use io::Write;

use io::BufRead;
use io::Read;

use io::Cursor;

use png::Decoder;
use png::Info;
use png::Reader;

pub fn info2exif<'a>(i: &'a Info) -> Option<&'a [u8]> {
    i.exif_metadata.as_deref()
}

pub fn bytes2png2exif2writer<W>(png: &[u8], mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let rdr = Cursor::new(png);
    let dec: Decoder<_> = Decoder::new(rdr);
    let rdr: Reader<_> = dec.read_info()?;
    let i: &Info = rdr.info();
    let oexif: Option<&[u8]> = info2exif(i);
    match oexif {
        None => Ok(()),
        Some(exif) => {
            wtr.write_all(exif)?;
            wtr.flush()
        }
    }
}

pub fn rdr2limited2png2exif2wtr<R, W>(rdr: R, limit: u64, wtr: W) -> Result<(), io::Error>
where
    R: BufRead,
    W: Write,
{
    let mut taken = rdr.take(limit);
    let mut buf: Vec<u8> = vec![];
    taken.read_to_end(&mut buf)?;
    bytes2png2exif2writer(&buf, wtr)
}

pub fn stdin2limited2png2stdout(limit: u64) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    rdr2limited2png2exif2wtr(io::stdin().lock(), limit, BufWriter::new(&mut ol))?;
    ol.flush()
}

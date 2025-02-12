use std::io;

use std::io::Read;

use std::io::BufWriter;
use std::io::Write;

use std::path::Path;

use std::fs::File;

use apache_avro::Reader;

use apache_avro::Schema;
use apache_avro::Writer;

pub const SPLIT_COUNT_DEFAULT: u32 = 16384;

pub fn reader2values2fs<R, P, F>(
    rdr: R,
    dirname: P,
    index2basename: F,
    split_cnt: u32,
) -> Result<(), io::Error>
where
    R: Read,
    P: AsRef<Path>,
    F: Fn(u32, &mut String),
{
    let mut ar: Reader<_> = Reader::new(rdr).map_err(io::Error::other)?;

    let schema: Schema = ar.writer_schema().clone();

    let mut basename: String = String::new();
    let mut ix: u32 = 0;

    let mut no_next: bool = false;

    loop {
        index2basename(ix, &mut basename);
        let full = dirname.as_ref().join(&basename);
        let mut f: File = File::create(&full)?;
        let bw: BufWriter<_> = BufWriter::new(&mut f);
        let mut wtr: Writer<_> = Writer::new(&schema, bw);

        let mut wrote_some: bool = false;

        for _ in 0..split_cnt {
            match ar.next() {
                None => {
                    no_next = true;
                    break;
                }
                Some(Err(e)) => return Err(io::Error::other(e)),
                Some(Ok(v)) => {
                    wtr.append(v).map_err(io::Error::other)?;
                    wrote_some = true;
                }
            }
        }

        let mut bw: BufWriter<_> = wtr.into_inner().map_err(io::Error::other)?;
        bw.flush()?;

        drop(bw);
        f.flush()?;

        if !wrote_some {
            std::fs::remove_file(full)?;
        }

        if no_next {
            return Ok(());
        }

        ix += 1;
    }
}

pub fn index2basename_hex(index: u32, buf: &mut String) {
    let s: String = format!("{index:08x}");
    *buf = s;
    *buf += ".avro";
}

pub struct Config {
    pub dirname: String,
    pub split_count: u32,
}

impl Config {
    pub fn reader_to_values_to_fs<R, F>(
        &self,
        reader: R,
        index_to_basename: F,
    ) -> Result<(), io::Error>
    where
        R: Read,
        F: Fn(u32, &mut String),
    {
        let dname: &str = &self.dirname;
        reader2values2fs(reader, dname, index_to_basename, self.split_count)
    }

    pub fn stdin2values2fs<F>(&self, index2basename: F) -> Result<(), io::Error>
    where
        F: Fn(u32, &mut String),
    {
        let i = io::stdin();
        let il = i.lock();
        self.reader_to_values_to_fs(il, index2basename)
    }

    pub fn stdin2values2fs_default(&self) -> Result<(), io::Error> {
        self.stdin2values2fs(index2basename_hex)
    }
}

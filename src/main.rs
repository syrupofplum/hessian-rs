use std::io;

pub struct Error {

}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Serializer<W> {
    writer: W
}

impl <W> Serializer<W>
    where
        W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Serializer {
            writer
        }
    }

    pub fn serialize_i8(&self, value: i8) -> Result<()> {
        Ok(())
    }
}

pub struct Formatter {

}

fn main() {

}

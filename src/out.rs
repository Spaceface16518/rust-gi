use std::io::{self, stdout, Write};

pub enum Output<O: Write> {
    Stdout,
    Other(O),
}

pub fn write<O: Write>(
    output: &mut Output<O>,
    buf: &[u8],
) -> io::Result<usize> {
    match output {
        Output::Stdout => stdout().write(buf),
        Output::Other(handle) => handle.write(buf),
    }
}

use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_through: usize,
    ops: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(reader: R) -> ReadStats<R> {
        Self {
            reader,
            bytes_through: 0,
            ops: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.reader.read(buf);
        self.ops += 1;

        if result.is_ok() {
            self.bytes_through += result.as_ref().unwrap();
        }

        return result
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_through: usize,
    ops: usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> WriteStats<W> {
        Self {
            writer,
            bytes_through: 0,
            ops: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.writer.write(buf);
        self.ops += 1;

        if result.is_ok() {
            self.bytes_through += result.as_ref().unwrap();
        }

        return result
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

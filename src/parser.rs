use anyhow::{Context, Error, Result};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct DnsBytePacketBuffer {
    buf: [u8; 512],
    pos: usize,
    bytes_read: usize,
}

impl DnsBytePacketBuffer {
    pub fn load(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file: File = File::open(file_name).context("Unable to find the file")?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut buf: [u8; 512] = [0u8; 512];
        let bytes_read: usize = reader.read(&mut buf).context("unable to read from file")?;
        Ok(DnsBytePacketBuffer {
            buf,
            bytes_read,
            pos: 0,
        })
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
    // get byte and change position
    pub fn read(&mut self) -> Result<u8> {
        match self.pos < self.bytes_read {
            true => {
                let data: u8 = self.buf[self.pos];
                self.pos += 1;
                Ok(data)
            }
            false => Err(Error::msg("No more bytes to read")),
        }
    }

    // get u16 and update position
    pub fn read_u16(&mut self) -> Result<u16> {
        match self.pos + 1 < self.bytes_read {
            true => {
                let data: u16 = ((self.read()? as u16) << 8) | (self.read()? as u16);
                Ok(data)
            }
            false => Err(Error::msg("No more u16 to read")),
        }
    }
    // get byte at pos without changing position
    pub fn get(&self, index: usize) -> Result<u8> {
        match index < self.bytes_read {
            true => Ok(self.buf[index]),
            false => Err(Error::msg(format!("Can't read at index: {}", index))),
        }
    }
    // step usize position forward
    pub fn step(&mut self, steps: usize) -> Result<()> {
        match self.pos + steps <= self.bytes_read {
            true => {
                self.pos += steps;
                Ok(())
            }
            false => Err(Error::msg(format!("Can't step for: {}", steps))),
        }
    }
    // read 4 bytes
    pub fn read_u32(&mut self) -> Result<u32> {
        match self.pos + 3 < self.bytes_read {
            true => {
                let data: u32 = ((self.read()? as u32) << 24)
                    | ((self.read()? as u32) << 16)
                    | ((self.read()? as u32) << 8)
                    | (self.read()? as u32);
                Ok(data)
            }
            false => Err(Error::msg("No more u32 to read")),
        }
    }
    // get range of bytes without updating the pos
    pub fn get_range(&self, start: usize, end: usize) -> Result<&[u8]> {
        match start > 0 && start + end < self.bytes_read {
            true => Ok(&self.buf[start..start + end]),
            false => Err(Error::msg(format!("Can't get range: {} - {}", start, end))),
        }
    }
    // read the label
    pub fn read_label(&mut self) -> Option<String> {
        todo!()
    }
}

impl std::string::ToString for DnsBytePacketBuffer {
    fn to_string(&self) -> String {
        String::from_utf8(self.buf[..self.bytes_read].to_vec())
            .expect("unable to convert into string ")
    }
}

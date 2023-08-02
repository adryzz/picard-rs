use std::io::{Cursor, Seek, Read, Write, self};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};

pub struct ShaderBuffer {
    buf: Cursor<Vec<u8>>,
    little_endian: bool
}

impl ShaderBuffer {
    pub fn new() -> Self {
        Self {
            //TODO: start with_capacity
            buf: Cursor::new(Vec::new()),
            little_endian: true
        }
    }

    pub(crate) fn set_little_endian(&mut self, little: bool) {
        self.little_endian = little;
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64, io::Error> {
        if self.little_endian {
            self.buf.read_u64::<LittleEndian>()
        } else {
            self.buf.read_u64::<BigEndian>()
        }
    }

    pub(crate) fn write_u64(&mut self, value: u64) -> Result<(), io::Error> {
        if self.little_endian {
            self.buf.write_u64::<LittleEndian>(value)
        } else {
            self.buf.write_u64::<BigEndian>(value)
        }
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, io::Error> {
        if self.little_endian {
            self.buf.read_u32::<LittleEndian>()
        } else {
            self.buf.read_u32::<BigEndian>()
        }
    }

    pub(crate) fn write_u32(&mut self, value: u32) -> Result<(), io::Error> {
        if self.little_endian {
            self.buf.write_u32::<LittleEndian>(value)
        } else {
            self.buf.write_u32::<BigEndian>(value)
        }
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16, io::Error> {
        if self.little_endian {
            self.buf.read_u16::<LittleEndian>()
        } else {
            self.buf.read_u16::<BigEndian>()
        }
    }

    pub(crate) fn write_u16(&mut self, value: u16) -> Result<(), io::Error> {
        if self.little_endian {
            self.buf.write_u16::<LittleEndian>(value)
        } else {
            self.buf.write_u16::<BigEndian>(value)
        }
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, io::Error> {
        self.buf.read_u8()
    }

    pub(crate) fn write_u8(&mut self, value: u8) -> Result<(), io::Error> {
        self.buf.write_u8(value)
    }

    pub(crate) fn read_f32(&mut self) -> Result<f32, io::Error> {
        if self.little_endian {
            self.buf.read_f32::<LittleEndian>()
        } else {
            self.buf.read_f32::<BigEndian>()
        }
    }

    pub(crate) fn write_f32(&mut self, value: f32) -> Result<(), io::Error> {
        if self.little_endian {
            self.buf.write_f32::<LittleEndian>(value)
        } else {
            self.buf.write_f32::<BigEndian>(value)
        }
    }

    pub(crate) fn read_raw(&mut self, buffer: &mut [u8]) -> bool {
        self.buf.read_exact(buffer).is_ok()
    }

    pub(crate) fn write_raw(&mut self, buffer: &[u8]) -> bool {
        self.buf.write_all(buffer).is_ok()
    }

    pub(crate) fn seek(&mut self, pos: u64, mode: io::SeekFrom) -> Result<u64, io::Error> {
        self.buf.seek(mode)
    }

    pub(crate) fn tell(&mut self) -> u64 {
        self.buf.position()
    }

    /// Returns the buffer as a Vec, consuming it
    pub fn into_vec(self) -> Vec<u8> {
        self.buf.into_inner()
    }
}
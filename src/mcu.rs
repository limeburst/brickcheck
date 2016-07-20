use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::io;

use bit_vec::BitVec;

pub struct K20 {
    /// Backdoor Comparison Key.
    pub backdoor_key: [u8; 8],
    /// Program flash protection bytes.
    pub fprot: [u8; 4],
    /// Data flash protection byte.
    pub fdprot: u8,
    /// EEPROM protection byte.
    pub feprot: u8,
    /// Flash nonvolatile option byte.
    pub fopt: u8,
    /// Flash security byte.
    pub fsec: u8,
}

impl K20 {
    pub fn from_file(f: &mut File) -> Result<K20, io::Error> {
        try!(f.seek(SeekFrom::Start(0x400)));
        let mut backdoor_key = [0; 8];
        let mut fprot = [0; 4];
        let mut fdprot = [0; 1];
        let mut feprot = [0; 1];
        let mut fopt = [0; 1];
        let mut fsec = [0; 1];
        try!(f.read_exact(&mut backdoor_key));
        try!(f.read_exact(&mut fprot));
        try!(f.read_exact(&mut fdprot));
        try!(f.read_exact(&mut feprot));
        try!(f.read_exact(&mut fopt));
        try!(f.read_exact(&mut fsec));
        Ok(K20 {
            backdoor_key: backdoor_key,
            fprot: fprot,
            fdprot: fdprot[0],
            feprot: feprot[0],
            fopt: fopt[0],
            fsec: fsec[0],
        })
    }
    pub fn meen(&self) -> bool {
        let bv = BitVec::from_bytes(&[self.fsec]);
        !(bv[5] && !bv[4])
    }
    pub fn sec(&self) -> bool {
        let bv = BitVec::from_bytes(&[self.fsec]);
        !(bv[1] && !bv[0])
    }
}

use binread::{BinReaderExt, BinRead};
use std::borrow::Cow;
use std::mem::size_of;
use crate::mem::csgo::bsp::lib::BspResult;
use crate::mem::csgo::bsp::data::{VisData, Entities};
use binread::io::Cursor;
use std::io::Read;

pub struct LumpReader<R> {
    inner: R,
    length: usize,
}

impl<'a> LumpReader<Cursor<Cow<'a, [u8]>>> {
    pub fn new(data: Cow<'a, [u8]>) -> Self {
        let length = data.len();
        let reader = Cursor::new(data);
        LumpReader {
            inner: reader,
            length,
        }
    }
}

impl<R: BinReaderExt + Read> LumpReader<R> {
    pub fn read_entities(&mut self) -> BspResult<Entities> {
        let mut entities = String::with_capacity(self.length);
        self.inner.read_to_string(&mut entities)?;
        Ok(Entities { entities })
    }

    pub fn read_vec<F, T>(&mut self, mut f: F) -> BspResult<Vec<T>> where F: FnMut(&mut LumpReader<R>) -> BspResult<T>, {
        let num_entries = self.length / size_of::<T>();
        let mut entries = Vec::with_capacity(num_entries);

        for _ in 0..num_entries {
            entries.push(f(self)?);
        }

        Ok(entries)
    }

    pub fn read<T: BinRead>(&mut self) -> BspResult<T> {
        Ok(self.inner.read_le()?)
    }

    pub fn read_visdata(&mut self) -> BspResult<VisData> {
        if (self.length as usize) < std::mem::size_of::<u32>() * 2 {
            return Ok(VisData::default());
        }

        let cluster_count = self.inner.read_le()?;
        let mut pvs_offsets = Vec::with_capacity(cluster_count as usize);
        let mut pas_offsets = Vec::with_capacity(cluster_count as usize);

        for _ in 0..cluster_count {
            pvs_offsets.push(self.inner.read_le()?);
            pas_offsets.push(self.inner.read_le()?);
        }

        let mut data = Vec::new();
        self.inner.read_to_end(&mut data)?;

        Ok(VisData {
            cluster_count,
            pvs_offsets,
            pas_offsets,
            data,
        })
    }
}
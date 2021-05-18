#![allow(clippy::double_parens)]
use nom::le_u32;
use nom::le_i32;

use crate::mem::Module;
use crate::mem::csgo::RecvTable;

#[derive(Debug, Clone, PartialEq)]
pub struct RecvProp {
    pub name: String,
    pub offset: i32,
    pub table: Option<RecvTable>,
}

#[derive(Debug)]
pub struct RecvPropIterator<'a> {
    base: usize,
    current: usize,
    max: usize,
    module: &'a Module,
}

impl RecvProp {
    // offset_name, offset_table, value
    #[rustfmt::skip]
    named!(
        parse_raw<(usize, usize, i32)>,
        do_parse!(
            offset_name  : le_u32 >>
            take!(0x24)           >>
            offset_table : le_u32 >>
            value        : le_i32 >>
            ((
                offset_name as usize,
                offset_table as usize,
                value,
            ))
        )
    );

    fn parse(base: usize, module: &Module) -> Option<RecvProp> {
        trace!("Starting to parse RecvProp at {:#x}", base);
        let data = module.get_slice(base, 0x30, false)?;
        let (_, (offset_name, offset_table, value)) = RecvProp::parse_raw(&data).ok()?;

        let name = crate::helpers::parse_string(module.get(offset_name, false)?)
            .ok()?
            .1
            .to_string();
        trace!(
            "Found RecvProp '{}' at {:#x}, value {:#x} childtable {:#X}",
            name,
            base,
            value,
            offset_table
        );

        let table = match offset_table {
            0 => None,
            _ => RecvTable::parse(offset_table, module),
        };

        Some(Self {
            name,
            offset: value,
            table,
        })
    }

    pub fn get_offset(&self, name: &str) -> Option<i32> {
        if self.name == name {
            return Some(self.offset);
        }

        match self.table {
            Some(ref table) => match table.get_offset(name) {
                Some(o) => Some(o + self.offset),
                _ => None,
            },
            _ => None,
        }
    }
}

impl<'a> RecvPropIterator<'a> {
    pub fn new(base: usize, max: usize, module: &'a Module) -> Self {
        Self {
            base,
            current: 0,
            max,
            module,
        }
    }
}

impl<'a> Iterator for RecvPropIterator<'a> {
    type Item = RecvProp;

    fn next(&mut self) -> Option<RecvProp> {
        if self.current >= self.max {
            return None;
        }

        let prop = RecvProp::parse(self.base + self.current * 0x3C, self.module)?;
        self.current += 1;

        Some(prop)
    }
}

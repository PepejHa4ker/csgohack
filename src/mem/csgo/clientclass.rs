#![allow(clippy::double_parens)]
use nom::*;


use crate::mem::Module;
use crate::mem::csgo::RecvTable;
use crate::util::helpers::parse_string;


#[derive(Debug, PartialEq)]
pub struct ClientClass {
    pub id: i32,
    pub name: String,
    pub table: RecvTable,
}

#[derive(Debug)]
pub struct ClientClassIterator<'a> {
    next_offset: usize,
    module: &'a Module,
}

impl ClientClass {
    // offset_name, offset_table, offset_next, id
    #[rustfmt::skip]
    named!(
        parse_raw<(usize, usize, usize, i32)>,
        do_parse!(
            take!(8)              >>
            offset_name  : le_u32 >>
            offset_table : le_u32 >>
            offset_next  : le_u32 >>
            id           : le_i32 >>
            ((
                offset_name as usize,
                offset_table as usize,
                offset_next as usize,
                id,
            ))
        )
    );


    fn parse(base: usize, module: &Module) -> Option<(ClientClass, usize)> {
        debug!("Starting to parse ClientClass at {:#x}", base);
        let data = module.get_slice(base, 0x18, false)?;
        let (_, (offset_name, offset_table, offset_next, id)) =
            ClientClass::parse_raw(&data).ok()?;

        let name = parse_string(module.get(offset_name, false)?)
            .ok()?
            .1
            .to_string();
        debug!("Found ClientClass '{}' at {:#x}", name, base);

        let cc = ClientClass {
            id,
            name,
            table: RecvTable::parse(offset_table, module)?,
        };

        Some((cc, offset_next))
    }
}

impl<'a> ClientClassIterator<'a> {
    pub fn new(next_offset: usize, module: &'a Module) -> Self {
        Self {
            next_offset,
            module,
        }
    }
}

impl<'a> Iterator for ClientClassIterator<'a> {
    type Item = ClientClass;

    fn next(&mut self) -> Option<ClientClass> {
        if self.next_offset == 0 {
            return None;
        }

        let (cc, next) = ClientClass::parse(self.next_offset, self.module)?;

        self.next_offset = next;
        Some(cc)
    }
}
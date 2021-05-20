#![allow(clippy::double_parens)]
use nom::le_u32;

use crate::mem::csgo::prop::{RecvProp, RecvPropIterator};
use crate::mem::Module;
use crate::util::helpers::parse_string;

#[derive(Debug, Clone, PartialEq)]
pub struct RecvTable {
    pub name: String,
    pub props: Vec<RecvProp>,
}

impl RecvTable {
    // offset_name, offset_props, num_props
    #[rustfmt::skip]
    named!(
        parse_raw<(usize, usize, usize)>,
        do_parse!(
            offset_props: le_u32 >>
            num_props: le_u32    >>
            take!(4)             >>
            offset_name: le_u32  >>
            ((
                offset_name as usize,
                offset_props as usize,
                num_props as usize,
            ))
        )
    );

    pub fn parse(base: usize, module: &Module) -> Option<Self> {
        trace!("Starting to parse RecvTable at {:#x}", base);
        if base == 0 {
            return None;
        }

        let data = module.get_slice(base, 0x10, false)?;
        let (_, (offset_name, offset_props, num_props)) = Self::parse_raw(&data).ok()?;

        let name = parse_string(module.get(offset_name, false)?)
            .ok()?
            .1
            .to_string();
        trace!("Found RecvTable '{}' at {:#x}", name, base);

        Some(Self {
            name,
            props: RecvPropIterator::new(offset_props, num_props, module).collect::<Vec<_>>(),
        })
    }

    pub fn get_offset(&self, name: &str) -> Option<i32> {
        for prop in &self.props {
            if let Some(o) = prop.get_offset(name) {
                return Some(o);
            }
        }
        None
    }
}

use crate::mem::{Process, Module};
use crate::mem::csgo::{ClientClassIterator, RecvTable};
use nom::lib::std::collections::BTreeMap;
use crate::Map;

#[derive(Debug, Clone, PartialEq)]
pub struct NetvarManager {
    tables: BTreeMap<String, RecvTable>,
}

impl NetvarManager {
    pub fn new(first: usize, client_module: &Module) -> Option<Self> {
        debug!("First ClientClass at {:#X}", first);
        let classes = ClientClassIterator::new(first + client_module.base, &client_module);
        let tables = classes
            .map(|c| (c.table.name.clone(), c.table))
            .collect::<Map<_>>();
        debug!("Added {} parent RecvTables!", tables.len());
        Some(NetvarManager { tables })
    }

    pub fn get_offset(&self, table_name: &str, netvar_name: &str) -> Option<i32> {
        self.tables.get(table_name)?.get_offset(netvar_name)
    }
}
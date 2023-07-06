use std::collections::HashMap;

pub type Symbol<'s> = &'s str;

pub type Address = i16;

const SYMBOL_BASE_ADDR: i16 = 0x0010;

pub struct SymbolTable<'source> {
    table: HashMap<Symbol<'source>, Address>,
    // counter for symbol
    symbol_offset: i16,
}

impl<'s> SymbolTable<'s> {
    #[must_use]
    pub fn new() -> Self {
        #[rustfmt::skip]
        let table = HashMap::from([
            ("SP",     0x0000),
            ("LCL",    0x0001),
            ("ARG",    0x0002),
            ("THIS",   0x0003),
            ("THAT",   0x0004),
            ("R0",     0x0000),
            ("R1",     0x0001),
            ("R2",     0x0002),
            ("R3",     0x0003),
            ("R4",     0x0004),
            ("R5",     0x0005),
            ("R6",     0x0006),
            ("R7",     0x0007),
            ("R8",     0x0008),
            ("R9",     0x0009),
            ("R10",    0x000a),
            ("R11",    0x000b),
            ("R12",    0x000c),
            ("R13",    0x000d),
            ("R14",    0x000e),
            ("R15",    0x000f),
            ("SCREEN", 0x4000),
            ("KBD",    0x6000),
        ]);

        SymbolTable {
            table,
            symbol_offset: 0,
        }
    }

    pub fn contains(&self, sym: Symbol) -> bool {
        self.table.contains_key(sym)
    }

    pub fn register_symbol(&mut self, sym: Symbol<'s>) {
        debug_assert!(!self.table.contains_key(&sym));
        self.table
            .insert(sym, SYMBOL_BASE_ADDR + self.symbol_offset);
        self.symbol_offset += 1;
    }

    pub fn register_label(&mut self, sym: Symbol<'s>, addr: Address) {
        debug_assert!(!self.table.contains_key(&sym));
        self.table.insert(sym, addr);
    }

    pub fn address(&self, sym: Symbol<'s>) -> Option<&Address> {
        self.table.get(sym)
    }
}

impl<'s> Default for SymbolTable<'s> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_symbol_assign_addr() {
        let mut t = SymbolTable::new();
        let s1 = "s";
        t.register_symbol(s1);
        assert!(t.contains(s1));
        assert_eq!(*t.address(s1).unwrap(), 0x0010);

        let s2 = "t";
        t.register_symbol(s2);
        assert!(t.contains(s1));
        assert!(t.contains(s2));
        assert_eq!(*t.address(s2).unwrap(), 0x0011);
    }
}

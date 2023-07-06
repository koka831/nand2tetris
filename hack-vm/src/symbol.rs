//! Manage Symbol/Labels in VM sources.
use rustc_hash::FxHashMap;

pub type Symbol<'s> = &'s str;

/// Stores symbol (label) names and their occurrence counts.
/// These counts are used to generate unique names for each instruction,
/// such as `JEQ`, when multiple jump instructions are present within VM program.
#[derive(Default)]
pub(crate) struct SymbolTable<'source> {
    table: FxHashMap<Symbol<'source>, u16>,
}

impl<'s> SymbolTable<'s> {
    pub fn new() -> Self {
        let table = FxHashMap::default();
        SymbolTable { table }
    }

    /// Create unique label from `sym`
    pub fn ret_addr(&mut self, sym: Symbol<'s>) -> String {
        let index = *self.table.entry(sym).and_modify(|e| *e += 1).or_insert(0);
        format!("{sym}{index}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table_new_label() {
        let mut t = SymbolTable::new();

        let l0 = &t.ret_addr("label");
        assert_eq!(l0, "label0");

        let k0 = &t.ret_addr("other_label");
        assert_eq!(k0, "other_label0");

        let l1 = &t.ret_addr("label");
        assert_eq!(l1, "label1");
    }
}

use symbol_tables::symbol_table::SymbolTable;

#[test]
fn test_insertion() {
    let mut sym_table: SymbolTable<usize, &str> = SymbolTable::new();
    
    sym_table.insert(0, "symbol")
        .insert(1, "rust")
        .insert(2, "world")
        .insert(0, "hello");

    for sym_item in sym_table.contents {
        println!("{}. {}", sym_item.key, sym_item.value)
    }
}

#[test]
fn test_get_entry() {
    let mut sym_table: SymbolTable<usize, &str> = SymbolTable::new();

    sym_table.insert(0, "hello")
        .insert(1, "rust")
        .insert(2, "world");

    let second_entry = sym_table.get(&2)
        .expect("Should return the second entry");

    println!("{}", second_entry.value);
}

#[test]
fn test_from() {
    let mut sym_table: SymbolTable<usize, &str> = SymbolTable::from(&[
        (0, "hello"),
        (1, "rust"),
        (2, "world")
    ]);

    sym_table.insert(0, "hi");

    for sym_item in sym_table.contents {
        println!("{}. {}", sym_item.key, sym_item.value)
    }
}
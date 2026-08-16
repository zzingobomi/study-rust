pub mod library;

fn function_1() {
    let shelf = crate::library::bookshelf::BookShelf::new();
}

fn function_2() {
    use library::bookshelf;
    let shelf = bookshelf::BookShelf::new();
}

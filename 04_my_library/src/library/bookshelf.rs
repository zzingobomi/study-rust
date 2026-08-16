use super::book::Book;

pub struct BookShelf {
    books: Vec<Book>,
}
impl BookShelf {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        todo!("Implement `Bookshelf::search_books`");
    }

    pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }

    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_books`");
    }
}

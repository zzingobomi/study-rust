use super::book::Book;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

pub struct BookShelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}
impl BookShelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher: matcher,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, BookShelf};

    #[test]
    fn test_bookshelf() {
        let mut shelf = BookShelf::new();
        let book1 = Book::new("ChatGPT! AI로 배우는 Rust!", "홍길동");
        let book2 = Book::new("Python 프로그래밍 입문", "최영희");
        shelf.add_book(book1);
        shelf.add_book(book2);
        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}

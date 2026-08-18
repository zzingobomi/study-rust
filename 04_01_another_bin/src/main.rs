use my_library::library::{book::Book, bookshelf::BookShelf};

fn main() {
    let mut shelf = BookShelf::new();
    let book1 = Book::new("ChatGPT! AI로 배우는 Rust!", "홍길동");
    let book2 = Book::new("Python 프로그래밍 입문", "최영희");
    shelf.add_book(book1);
    shelf.add_book(book2);

    let found_books = shelf.search_books("chatgpt");
    println!("{:?}", found_books);
}

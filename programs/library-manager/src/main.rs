const SYSTEM_NAME: &str = "LibSys-v1";

#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u32,
    borrower: String
}

// new(id: u32, title: &str, author: &str, year: u16) -> Self

impl Book {
    fn new(id: u32, title: &str, author: &str, year: u32) -> Self {
        Book {
            id,
            title: title.to_string(),
            author: author.to_string(),
            year,
            borrower: String::new()
        }
    }

    fn checkout(&mut self, name: &str) {
        println!("\nAttempting to checkout to {name}...");

        if self.borrower.is_empty() {
            println!("{SYSTEM_NAME}: Success. Checked out to {name}");
            self.borrower = name.to_string()
        } else {
            println!("{SYSTEM_NAME}: Sorry {}, this book is already checked out to {}", name, self.borrower);
        };
    }

    fn return_book(&mut self) {
        println!("\n{SYSTEM_NAME}: Book '{}' Returned by {}", self.title, self.borrower);
        self.borrower.clear();
    }

    fn display(&self) {
        let availability_message: &str = if self.borrower.is_empty() {
            "Available"
        } else {
            &format!("Checked out to {}", self.borrower)
        };

        // ID: 1 | 'The Rust Book' by Steve Klabnik (2018) - Available
        println!("ID: {} | '{}' by {} ({}) - {}", self.id, self.title, self.author, self.year, availability_message);
    }
}

fn display_booklist(list: &[Book]) {
    for book in list {
        book.display();
    }
}


fn main() {
    println!("\n--- Library Catalog ---");
    let mut books = vec![
        Book::new(1, "Things Fall Apart", "Chinua Achebe", 1958),
        Book::new(2, "Deep Learning Book", "Ian GoodFellow et al.", 2006),
        Book::new(3, "The science behind portals", "Michael Aheebwa", 2027),
    ];

    println!("\nBooklist:");
    display_booklist(&books);

    books[0].checkout("Alice");
    books[0].checkout("John");

    println!("\nUpdated Book list:");
    display_booklist(&books);

    books[0].return_book();

    println!("\nUpdated Book list:");
    display_booklist(&books);

    books[0].checkout("John");

    println!("\n{:#?}", books[1]);
}

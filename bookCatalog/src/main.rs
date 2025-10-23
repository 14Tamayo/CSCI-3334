use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book 
{
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) 
{
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    
    //Create new file using filename parameter as name
    let mut file = File::create(filename).expect("Failed to create file.");

    //Iterate through the vector
    for book in books
    {
        //Write to the file the contents of the vector
        writeln!(file, "{}, {}, {}", book.title, book.author, book.year).expect("Failed to write to file.");
    }
}

fn load_books(filename: &str) -> Vec<Book> 
{
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let mut books = vec![];

    let file = File::open(filename).expect("Failed to open file.");
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        let line = line.expect("Failed to read line.");

        //Split line by commas
        let parts: Vec<&str> = line.split(',').collect();

        //Check that we got exactly 3 fields: title, author, year
        if parts.len() == 3
        {
            let title = parts[0].trim().to_string();
            let author = parts[1].trim().to_string();
            let year: u16 = parts[2].trim().parse().expect("Failed to parse year.");

            //Add book contents into vector
            books.push(Book {title, author, year});
        }
    }

    books
}

fn main() 
{
    let books = vec![
        Book { title: "Halo: The fall of Reach".to_string(), author: "Eric Nylund".to_string(), year: 2001 },
        Book { title: "A game of thrones".to_string(), author: "George R. R. Martin".to_string(), year: 1996 },
        Book { title: "The Hobbit".to_string(), author: "John Ronald Reuel Tolkien".to_string(), year: 1937 },
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books 
    {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
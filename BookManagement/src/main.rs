/* You are tasked with implementing a library management system using Rust. 
Your goal is to design a program that can handle books and magazines. 
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input 
and displays its information. The function should output 
the item's ID, title, publication year, and type (book or magazine). 
*/ 

enum ItemType{
    Book, 
    Magazine
}

struct Item {
    id: i32,
    title: String,
    year: i32,
    item_type: ItemType,
}

fn display_item_info(item: &Item){
    println!("ID: {}", item.id);
    println!("Title: {}", item.title);
    println!("Year: {}", item.year);
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
    }
}

fn main() {
    // Create a new book
    let book = Item {
        id: 1,
        title: String::from("The Alchemist"),
        year: 1988,
        item_type: ItemType::Book,
    };
    
    // Create a new magazine
    let magazine = Item {
        id: 2,
        title: String::from("The Economist"),
        year: 1843,
        item_type: ItemType::Magazine,
    };

    // Display the book's information
    display_item_info(&book);

    // Display the magazine's information
    display_item_info(&magazine);
}

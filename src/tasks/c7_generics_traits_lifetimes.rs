// This chapter is dedicated to the generics, traits and lifetimes.

// GENERICS
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a generic struct `Pair<T>` that holds two values of the same type.
// Add a method `max(&self) -> &T` that returns the larger value.

// IMPLEMENT HERE:
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PartialOrd> Pair<T> {
    pub fn max(&self) -> &T {
        if self.x >= self.y { &self.x } else { &self.y }
    }
}

// TRAITS AND TRAIT BOUNDS
// ================================================================================================

// ----- 2 --------------------------------------
// Define a trait `Area` with a method `area(&self) -> f64` which calculates an area of the figure.
// Implement it for a `Rectangle` struct with fields `width` and `height`.

// IMPLEMENT HERE:
pub trait Area {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// ----- 3 --------------------------------------
// Define a trait `Summarize` with method `summary(&self) -> String`.
// Implement it for two structs:
// - `Article { title, author, content }`
// - `Tweet { username, content }`
//
// Then, write a generic function `notify<T: Summarize>(item: &T) -> String` that returns a
// formatted notification string using a `summary` method.

// IMPLEMENT HERE:

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String,
    #[allow(dead_code)]
    pub content: String,
}

impl Article {
    pub fn new(title: String, author: String, content: String) -> Self {
        Self { title, author, content }
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Tweet {
    pub fn new(username: String, content: String) -> Self {
        Self { username, content }
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) -> String {
    format!("Breaking news: {}", item.summarize())
}

// LIFETIMES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function `longest_string(first: &str, second: &str) -> &str` that returns the longer of
// two string slices. Add the lifetimes where needed.

// IMPLEMENT HERE:
pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ----- 5 --------------------------------------
// Define a struct `Book` with fields:
// - title: &str
// - content: &str
//
// Implement a method `longest_word(&self) -> Option<&str>` that returns the longest word in the
// book’s content. Return `None` if the content is empty.
//
// Add the lifetimes where needed.

// IMPLEMENT HERE:
pub struct Book<'a> {
    #[allow(dead_code)]
    title: &'a str,
    content: &'a str,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self { title, content }
    }

    pub fn longest_word(&self) -> Option<&'a str> {
        self.content.split_whitespace().max_by_key(|word| word.len())
    }
}

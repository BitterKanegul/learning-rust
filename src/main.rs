// Learn me some good Rust
// Gotta figure out how to get into *hygenic macros*
//
use hashbrown::HashMap;
use hashbrown::HashSet;
use rand::Rng;
use std::cmp::Ordering;
use std::hash::Hash;
use std::io;

struct MyDisjointSet<T> {
    rootmap: HashMap<T, T>,
    sizemap: HashMap<T, i64>,
}
impl<T> MyDisjointSet<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        MyDisjointSet {
            rootmap: HashMap::new(),
            sizemap: HashMap::new(),
        }
    }
    fn getitem(&self, x: &T) -> Option<&T> {
        parent = self.rootmap.get(x).unwrap();
        while parent != self.rootmap.get(parent) {
            let grandparent = self.rootmap.get(parent);
            self.rootmap.insert(parent, self.rootmap.get(grandparent));
            parent = self.rootmap.get(parent);
        }
    }
    //Here is the conundrum, do I copy or do I clone...
    // if i were to be working directly with the hashes would be fun.
    // Can we have weak references here?
    fn add(&mut self, x: &T) {
        if !self.rootmap.contains_key(x) {
            self.rootmap.insert(x.clone(), x.clone());
            self.sizemap.insert(x.clone(), 1);
        }
    }
    //Need to figure out how to make the globals work here, borrow checker
    fn merge(&mut self, x: &T, y: &T) {
        let parent_x = self.rootmap.get(x).unwrap();
        let parent_y = self.rootmap.get(y).unwrap();
        let size_x = self.sizemap.get(x).unwrap();
        let size_y = self.sizemap.get(y).unwrap();
        if size_x > size_y {
            self.rootmap.insert(parent_y.clone(), parent_x.clone());
            self.sizemap.insert(parent_x.clone(), size_x + size_y);
        }
    }
    fn connected(&self, x: &T, y: &T) -> Option<bool> {}
    fn subset(&self, x: &T) -> Option<HashSet<T>> {}
    fn subsets(&self) -> Option<HashMap<T, T>> {}
}
fn large<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
struct Group<T, U> {
    x: T,
    y: U,
    size: usize,
}
impl<T, U> Group<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
    println!("Guess the number:");
    let secret_number = rand::thread_rng().gen_range(1..=50);
    // println!("The secret number is {secret_number}");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("rip");
        println!("you guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}

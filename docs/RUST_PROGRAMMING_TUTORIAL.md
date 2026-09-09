# Rust Programming Language Guide

Table of contents

[1. What is Rust?](#1-what-is-rust)
[2. Basic setup of Rust](#2-the-basic-makeup-of-a-rust-program)

---

## 1. What is Rust?

Rust is a systems programming language originally created at Mozilla and now developed through the Rust project.

It’s commonly used for:

- Operating systems and low-level software
- Backend services
- Networking
- Databases
- Game engines
- Command-line tools
- Embedded systems
- Cryptography
- WebAssembly
- Android/iOS native libraries
- Performance-critical applications

A very simple Rust program:

```rust
fn main() {
println!("Hello, world!");
}
```

This looks somewhat like C/C++ but Rust has a stronger emphasis on memory safety.

---

## 2. The basic makeup of a Rust program

A Rust programme is generally made from:

Programme
│
├── Functions
│
├── Variables
│
├── Data types
│
├── Structs
│
├── Enums
│
├── Traits
│
├── Modules
│
├── Ownership & borrowing
│
├── Error handling
│
└── Crates / packages

---

## 3. Variables

Rust variables are immutable by default (i.e. they are constants...cannot be changed/updated).

```rust
fn main() {
let name = "John";
println!("{}", name);
}
```

You can’t do:

```rust
let age = 20;
age = 21;
```

because age is immutable.

If you want it to change:

```rust
let mut age = 20;
age = 21;
```

The `mut` means mutable.

According to Rust’s philosophy, things should be immutable unless explicitly stated otherwise that they can change.

---

## 4. Basic data types

Rust has familiar primitive types.

### Integers

```rust
let age: i32 = 25;
let population: u64 = 8_000_000_000;
```

You have several integer sizes:

```text
i8 i16 i32 i64 i128
u8 u16 u32 u64 u128
```

`i` = signed integer.

`u` = unsigned integer.

For example:

`i32` - can represent negative and positive numbers.

`u32` - can only represent non-negative numbers.

---

### Floating point

```rust
let price: f64 = 19.99;
```

Rust has:

```text
f32
f64
```

---

### Boolean

```rust
let logged_in: bool = true;
```

---

### Character

```rust
let letter: char = 'A';
```

Notice that Rust uses:

```text
'A'
```

for a character but:

```text
"A"
```

for a string.

---

## 5. Strings

Rust has several ways of representing text, but are &str and String common ways

```rust
let name: &str = "Alice";
```

This is a string slice.

A growable string:

```rust
let mut name = String::from("Alice");
name.push_str(" Smith");
println!("{}", name);
```

Result:

```bash
Alice Smith
```

This distinction becomes important when learning ownership.

---

## 6. Functions

Functions are declared with fn.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Then:

```rust
fn main() {
    let result = add(5, 3);
    println!("{}", result);
}
```

Output:

```bash
8
```

Notice this:

```bash
a + b
```

doesn’t have a semicolon.

In Rust, the last expression in a function can automatically become the return value.

You could alternatively write:

```rust
fn add(a: i32, b: i32) -> i32 {
return a + b;
}
```

Both work.

---

## 7. if statements

Pretty familiar:

```rust
let age = 20;

if age >= 18 {
    println!("Adult");
} else {
    println!("Minor");
}
```

Rust doesn’t require parentheses around the condition.

So, don’t write:

```rust
if (age >= 18)
```

although parentheses can technically be used in expressions.

---

## 8. Loops

Rust has several types of loops.

### Infinite loop

```rust
loop {
    println!("Running...");
}
```

You can break:

```rust
loop {
    println!("Running...");
    break;
}
```

### while

```rust
let mut count = 0;

while count < 5 {
    println!("{}", count);
    count += 1;
}
```

### for

```rust
for number in 1..5 {
    println!("{}", number);
}
```

Outputs:

```bash
1
2
3
4
```

---

## 9. Structs

This is where Rust starts becoming more interesting.

A struct lets you create your own data type.

```rust
struct User {
    name: String,
    age: u32,
}
```

You can create one:

```rust
let user = User {
    name: String::from("Alice"),
    age: 25,
};
```

And access its properties:

```rust
println!("{}", user.name);
println!("{}", user.age);
```

Conceptually:

```text
User
├── name
└── age
```

This is similar to a class in other languages, although Rust’s model is different.

---

## 10. Methods

You can attach functions to structs using impl.

struct User {
name: String,
age: u32,
}
impl User {
fn greet(&self) {
println!("Hello, I'm {}", self.name);
}
}

Then:

let user = User {
name: String::from("Alice"),
age: 25,
};
user.greet();

The &self is important.

It essentially means:

“I want to borrow a reference to this User without taking ownership of it.”

That’s leading us toward one of Rust’s biggest concepts.

---

## 11. Ownership — the thing that makes Rust Rust

This is probably the most important concept to understand.

In languages like Java, Python, C#, etc., memory management is largely handled for you.

C and C++ give you much more direct control, but it’s easy to make mistakes.

Rust takes a different approach.

Every value has an owner.

For example:

let name = String::from("Alice");

Conceptually:

name
│
▼
┌───────────────┐
│ "Alice" │
└───────────────┘

When name goes out of scope, Rust automatically cleans up the memory.

You don’t normally need:

free(name);

and you don’t have a garbage collector constantly running like Java/C#.

Rust uses its ownership system and compile-time analysis to determine when memory can safely be released.

---

## 12. Moving ownership

Here’s a classic Rust example:

let a = String::from("Hello");
let b = a;

After this:

a ──X
b ──► "Hello"

a is no longer usable.

This:

println!("{}", a);

will cause a compiler error.

Why?

Because ownership of the String moved from a to b.

This prevents multiple pieces of code from incorrectly believing they own the same piece of memory.

---

## 13. Borrowing

Instead of transferring ownership, you can borrow a value.

fn print_name(name: &String) {
println!("{}", name);
}
fn main() {
let name = String::from("Alice");
print_name(&name);
println!("{}", name);
}

Here:

&name

means:

“Let this function temporarily borrow name.”

The original owner remains name.

Think:

             ┌──────────────┐

name ───────►│ "Alice" │
└──────────────┘
▲
│
temporary
borrow

---

## 14. Mutable borrowing

You can also borrow something mutably.

fn add_name(name: &mut String) {
name.push_str(" Smith");
}
fn main() {
let mut name = String::from("Alice");
add_name(&mut name);
println!("{}", name);
}

Output:

Alice Smith

Notice:

let mut name

and:

&mut name

Both are necessary.

Rust is extremely strict about who can modify data and when.

---

## 15. Why all this matters

Rust’s compiler checks these rules before your program runs.

For example, Rust won’t let you accidentally have conflicting access such as:

Someone is reading this data +
Someone else is modifying it +
Memory is being freed

These kinds of mistakes are responsible for many C/C++ bugs.

Rust tries to catch them during compilation.

That’s why Rust programmers sometimes joke:

“The compiler is your strictest code reviewer.”

---

## 16. References

You’ll frequently see:

&value

and:

&mut value

Think of them as:

&value
↓
"I want to look at this without owning it."
&mut value
↓
"I want temporary permission to modify this."

---

## 17. Arrays and vectors

An array has a fixed size:

let numbers = [1, 2, 3, 4, 5];

A Vec is a dynamically sized collection:

let mut numbers = Vec::new();
numbers.push(10);
numbers.push(20);
numbers.push(30);

Now:

numbers
│
▼
[10, 20, 30]

You can loop through it:

for number in &numbers {
println!("{}", number);
}

---

## 18. Option

One of Rust’s really useful features is Option.

Instead of using null everywhere, Rust can represent:

Some(value)

or:

None

Example:

fn find_user(id: u32) -> Option<String> {
if id == 1 {
Some(String::from("Alice"))
} else {
None
}
}

Then:

let user = find_user(1);
match user {
Some(name) => println!("User: {}", name),
None => println!("User not found"),
}

This makes the possibility of “there isn’t a value” explicit.

---

## 19. Result

Rust also has Result for operations that can fail.

For example:

fn divide(a: f64, b: f64) -> Result<f64, String> {
if b == 0.0 {
Err(String::from("Cannot divide by zero"))
} else {
Ok(a / b)
}
}

Then:

match divide(10.0, 2.0) {
Ok(value) => println!("Result: {}", value),
Err(error) => println!("Error: {}", error),
}

So instead of silently returning null or throwing an unexpected exception, the function communicates:

Result
├── Ok(value)
└── Err(error)

---

## 20. Enums

Enums allow a value to represent different possibilities.

enum Direction {
North,
South,
East,
West,
}

Then:

let direction = Direction::North;

Enums can also contain data:

enum Message {
Quit,
Text(String),
Number(i32),
}

So you could have:

Message::Quit

or:

Message::Text(String::from("Hello"))

This is extremely powerful in Rust.

---

## 21. Pattern matching

Rust’s match is closely related to enums.

let message = Message::Text(String::from("Hello"));
match message {
Message::Quit => {
println!("Quit");
}
Message::Text(text) => {
println!("Text: {}", text);
}
Message::Number(number) => {
println!("Number: {}", number);
}
}

Rust makes you handle the possible cases carefully.

---

## 22. Traits

Traits are roughly comparable to interfaces in languages like Java/C#.

For example:

trait Animal {
fn speak(&self);
}

Then:

struct Dog;
impl Animal for Dog {
fn speak(&self) {
println!("Woof!");
}
}

Now Dog implements Animal.

Conceptually:

Animal
│
└── speak()
Dog
│
└── implements Animal

Traits are a major part of Rust’s approach to abstraction.

---

## 23. Generics

Rust supports generic programming.

For example:

fn print_value<T>(value: T) {
println!("Something!");
}

More realistically, you might constrain the type using a trait:

fn largest<T: PartialOrd>(a: T, b: T) -> T {
if a > b {
a
} else {
b
}
}

Now, the function can work with multiple comparable types.

---

## 24. Modules

Large Rust applications are divided into modules.

You might have:

```text
my_app/
│
├── Cargo.toml
│
└── src/
├── main.rs
├── user.rs
├── database.rs
└── network.rs
```

Your `main.rs` might contain:

```rust
mod user;
mod database;
mod network;
```

This lets you organise large applications.

---

## 25. Crates

A crate is essentially a Rust compilation unit/package.

There are two broad types:

```text
Binary crate
↓
Executable program
Library crate
↓
Reusable library
```

For example, an application ca use third-party crates for:

```text
HTTP
JSON
databases
cryptography
GUI
logging
async networking
serialisation
```

Rust has a huge ecosystem of these.

---

## 26. Cargo

One of the nicest things about Rust is Cargo which is the package manager and build system of Rust.

You can typically create a project with:

```bash
cargo new my_app
```

You get:

```text
my_app/
├── Cargo.toml
└── src/
└── main.rs
```

Run it:

```bash
cargo run
```

Build it:

```bash
cargo build
```

Build an optimised release:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

---

## 27. Cargo.toml

This is the project’s configuration file.

For example:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = "1"
```

The important part is:

[dependencies]

That’s where you declare external Rust libraries.

---

## 28. A small complete Rust application

Putting some of this together:

```rust
struct User {
    name: String,
    age: u32,
}
impl User {

    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    let user = User {
    name: String::from("Alice"),
    age: 25,
};

user.greet();

if user.is_adult() {
    println!("Adult");
} else {
    println!("Minor");
}
}
```

Output:

```bash
Hello, my name is Alice
Adult
```

The structure is:

```text
main()
│
└── User
│
├── name
├── age
│
├── greet()
│
└── is_adult()
```

---

## 29. How Rust compares to Kotlin

Since you were asking about Android/Rust earlier, this comparison may be particularly useful.

```kotlin

data class User(
    val name: String,
    val age: Int
)
fun greet(user: User) {
    println("Hello ${user.name}")
}
```

```rust
struct User {
    name: String,
    age: u32,
}

fn greet(user: &User) {
    println!("Hello {}", user.name);
}
```

They’re conceptually similar, but Rust exposes more of what’s happening with memory and ownership.

Kotlin has:

```text
Garbage collector
↓
automatically manages memory

Rust generally has:

Ownership +
Borrowing +
Lifetimes
↓
compiler determines memory safety
```

There isn’t a conventional garbage collector managing Rust application’s objects.

---

30. Rust mental model/overview

If you’re coming from Kotlin, Java, Python, JavaScript, etc., a recommended learning order of Rust would be in this order:

1. Variables
   ↓
2. Types
   ↓
3. Functions
   ↓
4. Structs
   ↓
5. Enums
   ↓
6. Collections
   ↓
7. Ownership
   ↓
8. Borrowing & references
   ↓
9. Option / Result
   ↓
10. Traits
    ↓
11. Generics
    ↓
12. Lifetimes
    ↓
13. Modules & crates
    ↓
14. Async Rust
    ↓
15. Concurrency

Ownership, borrowing and lifetimes are the part that makes Rust feel strange initially. Once those click, the rest of the language becomes considerably easier.

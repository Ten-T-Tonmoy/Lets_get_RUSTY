---

# 🦀 Rust Ownership – Full Detail

### 1️⃣ The Core Rules of Ownership

Rust enforces memory safety without a garbage collector using **three simple rules**:

1. **Each value in Rust has a single owner** (a variable that controls it).
2. **At any time, there can be:**

   * *One* mutable reference, **OR**
   * Any number of immutable references.
     (a.k.a. *borrowing rules*)
3. **When the owner goes out of scope, the value is dropped** (freed).

That’s it. Everything else (borrowing, lifetimes, moves, copies) is built on top of these.

---

### 2️⃣ Ownership in Action

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1;  // ownership moves to s2

    println!("{}", s1); // ❌ error! s1 no longer owns the data
}
```

- Unlike `i32`, `String` is **heap-allocated**. Rust doesn’t copy by default because that’d be expensive.
- Instead, ownership _moves_.

---

### 3️⃣ Copy Types vs Move Types

Some types are **Copy** (live on the stack, cheap to duplicate):

```rust
fn main() {
    let x = 10;
    let y = x;  // copies
    println!("x = {}, y = {}", x, y); // ✅ works
}
```

But heap-allocated stuff (like `String`, `Vec`, `HashMap`) is **Move**:

```rust
let s1 = String::from("hi");
let s2 = s1; // move
// println!("{}", s1); // ❌ invalid, ownership moved
```

---

### 4️⃣ Borrowing (& and \&mut)

If you want to _use_ data without taking ownership → **borrow** it.

- Immutable borrow:

```rust
fn print_len(s: &String) {
    println!("len = {}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_len(&s);   // borrow, ownership not moved
    println!("{}", s); // ✅ still usable
}
```

- Mutable borrow (only one at a time!):

```rust
fn change(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // hello world
}
```

⚠️ You cannot have mutable + immutable refs alive at the same time.
That’s Rust preventing data races _at compile time_.

---

### 5️⃣ Slices (special borrows)

A slice is a view into data, doesn’t own it:

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // "hello"
let world = &s[6..];   // "world"
```

They are also subject to borrowing rules.

---

### 6️⃣ Lifetimes (👑 the scary word)

Lifetimes are how Rust checks **that references don’t outlive the data they point to**.

Example of invalid borrow:

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;   // ❌ x is dropped at end of scope
    }
    println!("{}", r);
}
```

Rust uses lifetimes to prevent dangling references at compile time.
Most lifetimes are inferred automatically; you only write them for complex functions.

---

### 7️⃣ Dropping (freeing memory)

When an owner goes out of scope, Rust automatically calls `drop()`.

```rust
fn main() {
    {
        let s = String::from("bye");
    } // s goes out of scope here, memory freed automatically
}
```

No manual `free()` needed, no GC pause → deterministic memory cleanup.

---

# 🔑 Summary (mental model)

- **Ownership = who cleans up the memory.**
- **Borrowing = temporary access without taking ownership.**
- **Copy vs Move = does assignment duplicate or transfer ownership.**
- **Lifetimes = make sure references are always valid.**

This system gives you:
✅ Memory safety
✅ No null/dangling pointers
✅ No GC overhead
✅ Compiler yells at you before runtime bugs happen

---

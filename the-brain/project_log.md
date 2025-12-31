# Gathering Rust Project Log

> **Purpose:** Tracking my Rust learning journey following a structured roadmap (Phase 0 → Phase 3). This is my "memory" for Claude across chats—shows what I've done, what I'm working on, and what I've learned. Update as I go!

**Current Phase:** Phase 1 - Core Language Basics  
**Week:** 1  
**Focus:** Getting comfortable with Rust fundamentals

---

## ✅ Completed

### Environment Setup
- [x] Installed Rust (rustup)
- [x] Set up editor (VS Code + rust-analyzer recommended)
- [x] Installed essential tools (clippy, rustfmt, cargo-watch)
- [x] Verified `rustc --version` and `cargo --version`

### Week 1: Fundamentals (Rust Book Ch 1-4)
- [x] Read Chapters 1-4 (Variables, data types, functions, control flow)

---

## 🔄 In Progress

### Week 1: Practice Projects
**Topics:** Variables, data types, functions, control flow

#### Practice Projects
- [ ] **Hello Adventure CLI** - Basic I/O, String vs &str
  - _Status:_
  - _What I learned:_
  
- [ ] **Temperature Converter** - Parsing, error handling
  - _Status:_
  - _What I learned:_
  
- [ ] **Simple Calculator** - Pattern matching, division by zero
  - _Status:_
  - _What I learned:_
  
- [ ] **FizzBuzz** - Loops and conditionals
  - _Status:_
  - _What I learned:_

---

## 📋 Coming Up Next

### Week 1 (Days 4-7): Ownership & Borrowing (Ch 4-5)
- [ ] Read ownership chapters
- [ ] Build string manipulator
- [ ] Build in-memory todo list
- [ ] Build file word counter

### Week 2: Enums, Structs, Collections (Ch 6-8)
- [ ] Command-line argument parser
- [ ] Simple JSON parser (by hand!)
- [ ] Inventory management system

### Week 3: Error Handling, Generics, Traits (Ch 9-11)
- [ ] Build a library crate (split previous project)
- [ ] Write comprehensive tests
- [ ] Create reusable utilities library

### Week 4: CLI Program & Iterators (Ch 12-13)
- [ ] Build minigrep clone (Rust Book walkthrough)
- [ ] Learn iterators and closures
- [ ] Explore crates.io (clap, serde, anyhow)

### Week 5-6: First Real Project
**Pick one:**
- [ ] CSV/JSON Data Pipeline
- [ ] Simple HTTP Parser

### Week 7-8: Second Project
**Pick one:**
- [ ] Terminal text editor
- [ ] Log analyzer
- [ ] Markdown renderer

### Week 9+: Bigger Projects
- [ ] Personal website backend
- [ ] Game engine/framework
- [ ] Something that scratches an itch

---

## 💡 Key Learnings & Gotchas

_Note discoveries, "aha!" moments, and things that tripped you up here:_

- **Shadowing scopes** (12/14): You can shadow variables and change them in inner scopes without affecting the outer scope. The shadowed variable in the inner scope is a completely separate binding that disappears when the scope ends. Different from mutation!

- **Slices are "fat pointers"** (12/19): Slices (`&str`, `&[T]`) are pointers to memory + a length. This is why they're safe—they track both where they start AND where they stop. Super useful for referencing portions of data without copying or worrying about the data going out of scope while you still have the reference.

---

## 🎯 Current Questions/Blockers

_Stuff you're stuck on or want to explore:_

- 

---

## 🎨 Code Snippets (That Didn't Suck)

_Stuff that actually worked or felt clever:_

```rust
// Shadowing example - inner scope doesn't affect outer
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x = 12
    }
    println!("The value of x is: {x}"); // x = 6
}
```

```rust
// Slice example - fat pointer in action
fn main() {
    let s = String::from("Hello, World!");
    let hello: &str = &s[0..5];
    let world: &str = &s[7..11];
    println!("{} {}", hello, world);
}
```

---

## 📝 Session Notes

_Only for interesting/eventful sessions:_

### 12/12-12/19 - First week with Rust
Read Ch 1-4, took notes on shadowing and slices. Both concepts clicked pretty quickly coming from C#.

---

**Last Updated:** December 22, 2024  
**Next Goal:** Complete first practice project (Hello World CLI)
#### cargo basics

- basically `cargo new proj_name` => makes the repo
- it initialized a git repo tho
- `cargo build` => builds it
- `cargo run` => runs it
- `cargo check` => checks it
- `cargo add` => adds dependencies to .toml
-

#### dir check

```
rust_project/
├── Cargo.toml
├── Cargo.lock
├── tests/
|    └── integration_test.rs
└──src/
    ├── main.rs     # binary crate entry point
    ├── lib.rs      # optional, shared library crate root
    ├── utils.rs    # a module file
    └── models/
        ├── mod.rs  # module declaration for "models"
        └── user.rs # submodule of "models"
```

#### So basically:

- Cargo.toml =
  - dependencies + metadata (JS → package.json)
- src/main.rs =
  - program start (JS → index.js)
- src/lib.rs =
  - if you’re writing reusable library code
- tests/ =
  - for integration tests

### rust modularity

_Like JS’s import, Rust uses use:_

    ```

        use crate::utils; // bring utils in scope
        use crate::models::user::User; // bring User struct

        fn main() {
        utils::greet();
        let user = User::new("Alice".to_string());
        println!("User: {}", user.name);
        }

    ```

### Why :: and not .?

- Because in Rust:
  - . is for method calls on values (like object.method() or "hello".len()).
  - :: is for navigating modules, types, and associated functions (like namespaces in C++).

#### 🔑 Key ideas

- Every file is a module if you mod it.

  - Every folder with mod.rs is a namespace.
  - crate = current project root (like a package).
  - pub makes stuff public (otherwise it’s private by default).
  - use = import into scope (like import in JS/TS).

- `models/mod.rs`

  ```
  pub mod user;   // link user.rs
  pub mod book;   // link book.rs
  ```

- `main.rs`

  ```
  mod models; // bring the folder in
  fn main() {
    models::user::greet_user();   // ✅ works
    models::book::print_book();   // ✅ works
  }
  ```

- _mod vs use_

  ```
  mod models;
  use models::user::greet_user;
  use models::book::print_book;

  fn main() {
      greet_user();
      print_book();
  }
  ```

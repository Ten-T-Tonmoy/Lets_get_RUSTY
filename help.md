### Rust crate tree

- intellisense only works on modules part of that !

### Rust uses `:: `for paths

In Rust, `::` is the path separator, used to navigate namespaces, modules, enums, and types.

## Rust Enum variants

- 1️⃣ Unit variant
  - Syntax: no data
  - Example: `Ordering::Less`
  - Notes: Just a name, like a constant. No extra data.
- 2️⃣ Tuple variant
  - Syntax: holds unnamed values
  - Example: `Ok(42)`
  - Notes: Acts like a tuple inside the variant. Can extract with patterns.
- 3️⃣ Struct variant
  - Syntax: holds named fields
  - Example: `Message::Move { x: 5, y: 10 }`
  - Notes: Acts like a struct inside the variant. Can extract fields by name.

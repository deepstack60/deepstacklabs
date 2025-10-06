# Rust Memory Model — Solana Context

- Ownership = linear logic enforced by compiler.
- References split into:
  - `&T` → shared, immutable.
  - `&mut T` → exclusive, mutable.
- Solana runtime mimics this aliasing model:
  - Writable accounts == `&mut`
  - Readonly accounts == `&`
- Dropping accounts = end-of-scope cleanup, like Rust's `Drop`.


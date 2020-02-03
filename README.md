# npfi

Non-Primitive Fixed Integer types: `u<K>` and `i<K>` for all `K ∈ [1, 128]` (excluding primitive types).

## Related Work:

This library is largely inspired by [the `uX` crate](https://github.com/kjetilkjeka/uX), with a few differences:

- `npfi` relies on code generation to reduce large tables of type relationships; for more concise code.
- `npfi` provides `BitWidth` and `BitSplice` traits for all types, including primitives for ergonomic bit operations with type refinement.

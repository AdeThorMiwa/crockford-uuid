# `crockford-uuid`

Here's an example of a crockford-uuid:

```text
aacy7965prs7631zgtk6100gzagmvv7x2
```

A crockford-uuid Uuid is a unique 160-bit value, stored as 20 bytes identifier,
and a fixed number checksum character derived from the identifier for value's
integrity check.

The uniqueness property is not strictly guaranteed, however for all
practical purposes, it can be assumed that an unintentional collision would
be extremely unlikely as the 160 bit key space is about the same size as
the number of bacterial cells in existence on the planet. :)

## Getting started

Add the following to your `Cargo.toml`:

```toml
crockford-uuid = "0.0.1"
```

When you want a identifier, you can generate one:

```rust
use crockford-uuid::Uuid;

let id = Uuid::new();
```

If you have a crockford-uuid value, you can use its string literal, BigUint and Bytes values inline:

```rust
use uuid::Uuid;

const from_string_lit: Uuid = "aacy7965prs7631zgtk6100gzagmvv7x2".try_into().unwrap();
const from_big_uint: Uuid = BigUint::parse_bytes(
                            b"471569087780948647371060810118848519319753452797",
                            10
                        ).unwrap().try_into().unwrap();
```

You can also convert crockford-uuid to be used as a string literal, BigUint, `Vec<u8>` (Bytes)

```rust
const to_str = Uuid::to_string();
const to_big_uint: BigUint = Uuid::new().into();
const to_vec: Vec<u8> = Uuid::new().into();
```

For more details on using `crockford-uuid`, [see the library documentation](https://docs.rs/crockford-uuid).

## References

https://learning.oreilly.com/library/view/api-design-patterns/9781617295850/OEBPS/Text/06.htm#:-:text=6.3.3
https://www.crockford.com/base32.html

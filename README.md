# rustSolana

# 🚀 Learning Rust 🦀

Welcome to my Rust learning journey!  
This repository includes code snippets, notes, and hands-on experiments to understand Rust — from the basics to advanced concepts.

---

## 📦 Project Initialization

To start a new Rust project, use the following command:

```bash
cargo init

cargo run
Compiling rustSolana v0.1.0 (/Users/janmesh23/Desktop/allCode/learningStuff/learnRust/rustSolana)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
Running `target/debug/rustSolana`

🧠 Integer Types in Rust (i8, i16, etc.)

Rust gives you precise control over memory and data size with fixed-size integer types. These are divided into:

✅ Signed Integers
| Type    | Size       | Range                              |
| ------- | ---------- | ---------------------------------- |
| `i8`    | 8 bits     | −128 to 127                        |
| `i16`   | 16 bits    | −32,768 to 32,767                  |
| `i32`   | 32 bits    | −2,147,483,648 to 2,147,483,647    |
| `i64`   | 64 bits    | −9 quintillion to +9 quintillion   |
| `i128`  | 128 bits   | Insanely large! 😄                 |
| `isize` | arch-based | Depends on 32-bit or 64-bit system |

✅ Unsigned Integers
Type	Size	Range
u8	8 bits	0 to 255
u16	16 bits	0 to 65,535
u32	32 bits	0 to 4,294,967,295
u64	64 bits	0 to 18 quintillion
u128	128 bits	Also huge 🚀
usize	arch-based	Depends on system architecture
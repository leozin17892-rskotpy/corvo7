![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.1.0-black)

# Corvo7 🐦

Welcome to this project!
Corvo7 is a small compiled language made for fun, speed, and safety.


---

## What is *Corvo7* ❓

It's a *compiled language* I made for fun.

The compiler generates C code (mainly using Clang).

The compiler itself is written in Rust along with the *supporting tools*.

**Corvo7** aims to be **fast**, **simple**, and **safe**, with strong typing and minimal overhead.



---

## Syntax 🧩

Its syntax is designed to be **simple** and **concise**.

Let's start with variable declarations.


#### Variables

**Corvo7** Has 2 types of *variables*:

- ```const``` → Immutable by definition

- ```var``` → Mutable by definition


Example without type annotation:
```typescript
// mutable variable
var variable1 = 10;
// immutable variable
const variable2 = 28;
```
Example with type annotation (recommended):
```typescript
var int variable1 = 10;
const int variable2 = 28;
```

> 💡 Tip: If you don’t annotate a type, Corvo7 will infer it automatically. You can also explicitly define types like 19u for u8.




---

### Functions

Functions are declared with fun:

```typescript
fun int add(int a, int b) {
    return a + b;
}
```

---

### Loops

While loops are simple and safe:

```typescript
var int i = 0;
var int sum = 0;

while(i < 10){
    sum += i;
    i += 1;
}
print(sum);
```

> Corvo7 is super fast for loops and numeric operations, even faster than Python for similar code.




---

### Advantages

- Compiled & fast → executes almost instantly

- Strong typing → reduces runtime errors

- No manual memory management → Rust handles everything in the compiler

- Simple syntax → easy to read and write



---

### Future Features

Classes with single inheritance

Static & class methods

More standard library features

--- 

## 🚀 Requirements

Before installing it, you need to have:

- [Rust](https://www.rust-lang.org/tools/install) installed
- [Git](https://git-scm.com/) installed

---

## 📥 installation 

1️⃣ *Clone* the *repository*:

```bash
git clone https://github.com/leozin17892-rskotpy/corvo7.git
cd corvo7
```

2️⃣ *Compile* the compiler:
```bash
cargo build --release
```

3️⃣ Test if it's working:
```bash
./target/release/corvo7 --version
```
> Windows: use ```corvo7.exe``` instead of ```./corvo7```
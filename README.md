![Build](https://img.shields.io/badge/build-passing-brightgreen)
![Version](https://img.shields.io/badge/version-0.2.0-black)
![Language](https://img.shields.io/badge/language-Rust-orange)
![Target](https://img.shields.io/badge/target-C-blue)

# Corvo7 🐦‍⬛

Corvo7 is a small compiled language made for fun, speed, and safety.  
The compiler is written in **Rust** and generates **C code**, compiled via Clang.

---

## What is Corvo7? ❓

A **compiled, strongly-typed language** built from scratch.  
Corvo7 aims to be **fast**, **simple**, and **safe** — with minimal syntax overhead and no manual memory management.

---

## Syntax 🧩

### Variables

Corvo7 has two kinds of variables:

- `const` → immutable by definition  
- `var` → mutable by definition

```typescript
// without type annotation (inferred)
var variable1 = 10;
const variable2 = 28;

// with type annotation (recommended)
var int variable1 = 10;
const int variable2 = 28;
```

> 💡 Type annotations are optional — Corvo7 infers types automatically. You can also use explicit literals like `19u` for `u8`.

---

### Functions

Declared with `fun`, return type comes first:

```typescript
fun int add(int a, int b) {
    return a + b;
}

fun int main() {
    return 0;
}
```

---

### Loops

#### While

```typescript
var int i = 0;
var int sum = 0;

while (i < 10) {
    sum += i;
    i += 1;
}
print(sum);
```

#### For (range-based)

Syntax: `for var in start..step..end`

```typescript
// count up: 0 to 10, step +1
for i in 0..1..10 {
    print(i);
}

// count down: 10 to 0, step -2
for b in 10..-2..0 {
    print(b);
}

// nested loops
for b in 10..-2..0 {
    for c in 10..-3..-1 {
        print(c);
    }
}
```

> The step can be negative — Corvo7 automatically adjusts the loop condition based on direction.

---

### Printing

```typescript
print(42);
print(my_var);
```

---

## Advantages ✅

- **Compiled & fast** — executes almost instantly, faster than interpreted languages for numeric operations
- **Strong typing** — reduces runtime errors
- **No manual memory management** — Rust handles everything inside the compiler
- **Simple syntax** — easy to read and write
- **Bidirectional for-loops** — range syntax with explicit step, positive or negative

---

## Future Features 🔭

- Classes with single inheritance
- Static & class methods
- Expanded standard library
- More type primitives

---

## Requirements 🚀

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/)
- [Clang](https://clang.llvm.org/) (for compiling the generated C code)

---

## Installation 📥

**1. Clone the repository:**
```bash
git clone https://github.com/leozin17892-rskotpy/corvo7.git
cd corvo7
```

**2. Compile the compiler:**
```bash
cargo build --release
```

**3. Test if it's working:**
```bash
./target/release/corvo7 --version
```

> Windows: use `corvo7.exe` instead of `./corvo7`

---

## Example Program

```typescript
fun int main() {
    for i in 0..1..10 {
        print(i);
    }
    return 0;
}
```

Compiles to clean, readable C — then to a native binary via Clang.

---

*Built with 🦀 Rust · Targets C · Made for fun*

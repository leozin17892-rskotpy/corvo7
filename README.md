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

**Corvo7** has 2 types of *variables*:

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

> Tip: If you don’t annotate a type, Corvo7 will infer it automatically. You can also explicitly define types like 19u for u8.




---

Functions

Functions are declared with fun:

```typescript
fun int add(int a, int b) {
    return a + b;
}
```

---

Loops

While loops are simple and safe:

```typescript
var int i = 0
var int sum = 0

while(:=i < 10){
    sum += i
    i += 1
}
print(sum)
```

> Corvo7 is super fast for loops and numeric operations, even faster than Python for similar code.




---

Advantages

Compiled & fast → executes almost instantly

Strong typing → reduces runtime errors

No manual memory management → Rust handles everything in the compiler

Simple syntax → easy to read and write



---

Future Features

Classes with single inheritance

Static & class methods

More standard library features
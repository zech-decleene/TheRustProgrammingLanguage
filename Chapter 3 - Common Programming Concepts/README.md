# Chapter 3 - Common Programming Concepts

## 3.1 Variables and Mutability
#### Why would we want to make our variables immutable?
 When a variable is immutable, once a value is bound to a name, you can't change that value.

Here you see we created `x` as a immutable variable.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
This results in the following error:
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

This demonstrates how the compiler can be used to help find errors. Compiler errors mean the program cannot safely execute and ultimately helps reduce mistakes.

Mutability can be a useful tool as well. You can make a variable mutable by adding the keyword `mut` before the variable name. In addition to helping reduce errors, the addition of `mut` also informs the reader of future intent to alter the variable.
\
\
We can create a mutable variable by adding `mut`.

 ```rust
 fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
 ```

As a result we get 
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

<br>

#### Variable vs. Constant: What's the Difference?
Like immutable variables, constants are values that are bound to names and are not allowed to change. 

The key difference between them are:
1. You aren't allowed to use `mut` with a constant. Constants are always immutable.
2. You declare a constant by using the keyword `const` instead of using `let` and the type of data *must* be annotated.
3. Constants can be declared in any scope, including the global scope.
4. Lastly, constants can only be set to a constant expression, not the result of a function or call or any other value that could only be computed at runtime.

Here is an example of a constant.
```rust 
const MAX_POINTS: u32 = 100_000;
```

Constants are valid for the entire time a program runs, within the scope they were declared in.

<br>

#### Shadowing
As we saw in **Chapter 2**, you can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable.

We say the first variable is being *shadowed* by the second variable.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```
Here we see that the program first binds a value of 5 to `x`
Then it *shadows* `x` by adding `1` to `x`, giving it a new value of `6`
The third `let` statement also *shadows* `x` by multiplying it's new value by `2`, resulting in the final print statement: `The value of x is: 12`

***
## 3.2 Data Types

#### What is a data type?

Every value in Rust is of a certain *Data Type*. This is to tell Rust what kind of data is being specified so it knows how to work with the data. 

Rust is a statically typed language, which means that it must know the types of all variables at compile time.

The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, we must add a *type annotation*

<br>

##### Scalar Types

A *scalar* type represents a single value. Rust has four primary scalar types: integers, floating-point numbers (floats), Booleans, and characters.

###### Integer Types
An *integer* is a number without a fractional component.

Here we use *type annotation* to declare a `32-bit unsigned integer`.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Each variant in the table below can be used to declare the type of an integer value. The variants can be signed or unsigned and have an *explicit* size.

| Length        | Signed    | Unsigned  |
| ------------- | :-------- | :-------- |
| ***8-bit***   | `i8`      | `u8`      |
| ***16-bit***  | `i16`     | `u16`     |
| ***32-bit***  | `i32`     | `u32`     |
| ***64-bit***  | `i64`     | `u64`     |
| ***128-bit*** | `i128`    | `u128`    |
| ***arch***    | `isize`   | `usize`   |

```rust
let signed_8bit:    i8;       let unsigned_8bit:    u8;
let signed_16bit:   i16;      let unsigned_16bit:   u16;
let signed_32bit:   i32;      let unsigned_32bit:   u32;
let signed_64bit:   i64;      let unsigned_64bit:   u64;
let signed_128bit:  i128;     let unsigned_128bit:  u128;
let signed_arch:    isize;    let unsigned_arch:    usize;

```
Signed numbers have a positive or negative value, meaning you can only have half as many possible values for a given number of bits. Signed integers are stored using [Two's Complement](https://en.wikipedia.org/wiki/Two%27s_complement). 

Each signed variant can store numbers from `-(2n - 1)` to `(2n - 1) - 1` inclusive, where `n` is the number of bits that variant uses.

Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on: **64 bits** if you’re on a ***64-bit** architecture* and **32 bits** if you’re on a ***32-bit** architecture*.

> **Integer Overflow**
>
> Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, *integer overflow* will occur. Rust has some interesting rules involving this behavior. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to *panic* at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error;
>
>When you’re compiling in release mode with the `--release` flag, Rust does *not* include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs *two’s complement wrapping*. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a `u8`, 256 becomes 0, 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error. If you want to wrap explicitly, you can use the standard library type [Wrapping](https://doc.rust-lang.org/std/num/struct.Wrapping.html).


<br>

###### Floating-Point Types
Rust has two primitive types for *floating-point numbers*. They are `f32` and `f64` which are *32-bit* and *64-bit* respectively. The default is `f64` because on modern CPUs it's roughly the same speed as `f32` but is capable of more precision.

Here is an example of a floating-point number

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The `f32` type is a single-precision float, and `f64` has double precision.

<br>


> ###### Numeric Operations
> Rust supports the basic mathematical operations you’d expect for all of the number types: <font color='purple'>addition</font>, <font color='purple'>subtraction</font>, <font color='purple'>multiplication</font>, <font color='purple'>division</font>, and <font color='purple'>remainder</font> (<font color='purple'>modulo</font>). The following code shows how you’d use each one in a let statement:
>
> ```rust
> fn main() {
>    // addition
>    let sum = 5 + 10;
>
>    // subtraction
>    let difference = 95.5 - 4.3;
>
>    // multiplication
>    let product = 4 * 30;
>
>    // division
>    let quotient = 56.7 / 32.2;
>
>    // remainder
>    let remainder = 43 % 5;
> }
> ```

<br>

###### Boolean Type
As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. For example:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

###### Character Type
Rust’s `char` type is the language’s most primitive alphabetic type, and the following code shows one way to use it. (Note that `char` literals are specified with single quotes, as opposed to string literals, which use double quotes.)

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. 

Accented letters; Chinese, Japanese, and Korean characters; emoji 😎; and zero-width spaces are all valid `char` values in Rust.

<br>

##### Compound Types
*Compound types* can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

###### Tuples
A tuple is a general way of grouping multiple values of varying types into a single compound type. Tuples have a fixed length: once declared, they cannot grow or shrink.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple, because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y); // The value of y is: 6.4
}
```

This program first creates a tuple and binds it to the variable `tup`. It then uses a pattern with `let` to take `tup` and turn it into three separate variables, `x`, `y`, and `z`. This is called *destructuring*, because it breaks the single tuple into three parts. Finally, the program prints the value of `y`, which is `6.4`.

In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;   // 500

    let six_point_four = x.1; // 6.4

    let one = x.2;            // 1
}
```

<br>

###### Arrays
Another way to have a collection of values is by using an *array*. Unlike a tuple, every value in an array must be of the same type. Arrays in Rust are different than arrays in some other languages because in Rust arrays have a fixed length.

In Rust, the values going into an array are written as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

You would write an array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```
Here, `i32` is the type of each element. After the semicolon, the number `5` indicates the array contains five elements.

###### Accessing Array Elements
An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];   // 1
    let second = a[1];  // 2
}
```

###### Invalid Array Element Access
What happens when you try to access an element that is past the end of the array?

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Running this code using `cargo run` produces the following result:
```
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/arrays`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:5:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The compilation didn’t produce any errors, but the program resulted in a *runtime error* and didn’t exit successfully.

This is the first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.

## 3.3 Functions

## 3.4 Comments

## 3.5 Control Flow
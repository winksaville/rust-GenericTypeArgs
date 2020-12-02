# Generic Type Arguments

The most "precise" way to specify type arguments to a Generic type
is to separate the Type from the Generic with `::` as show below
where `f1` is `1.0`, `2.0` and `3.0`. My expectation was that `4.0`
would work, but it does't!

See [This](https://users.rust-lang.org/t/why-cant-i-specify-type-parameters-directly-after-the-type/2365)
for more info. The synopsis is that the `::` is needed on the
right hand side of a let statement to disamigute the `<` used
for generic type arugments and the less than comparison operator.

```Rust
fn main() {
    struct OneField<T> {
        f1: T,
    }

    // OK
    let of: OneField::<f32> = OneField::<f32> {f1: 1.0};
    println!("of: {}", of.f1);

    // OK
    let of: OneField<f32> = OneField::<f32> {f1: 2.0};
    println!("of: {}", of.f1);

    // OK
    let of = OneField::<f32> {f1: 3.0};
    println!("of: {}", of.f1);

    // Error compile
    //let of = OneField<f32> {f1: 4.0};
    // comparison operators cannot be chained
    // use `::<...>` instead of `<...>` to specify type arguments
    // or use `(...)` if you meant to specify fn argumentsrustc
    // See: https://users.rust-lang.org/t/why-cant-i-specify-type-parameters-directly-after-the-type/2365

    // OK
    type OneF32 = OneField<f32>;
    let of = OneF32 {f1: 5.0};
    println!("of: {}", of.f1);
}
```
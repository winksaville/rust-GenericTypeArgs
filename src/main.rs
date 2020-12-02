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
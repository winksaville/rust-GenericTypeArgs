fn main() {
    struct OneField<T> {
        f1: T,
    }

    // OK
    let of: OneField::<f32> = OneField::<f32> { f1: 1.0 };
    println!("of: {}", of.f1);

    // OK
    let of: OneField<f32> = OneField::<f32> { f1: 2.0 };
    println!("of: {}", of.f1);

    // OK
    let of = OneField::<f32> { f1: 3.0 };
    println!("of: {}", of.f1);

    //let of = OneField<f32> { f1: 4.0 };
    //println!("of: {}", of.f1);
    // Compile Error
    //   See: https://users.rust-lang.org/t/why-cant-i-specify-type-parameters-directly-after-the-type/2365
    //
    //  $ cargo build
    //     Compiling GenericTypeArgs v0.1.0 (/home/wink/prgs/rust/projects/GenericTypeArgs)
    //  error: comparison operators cannot be chained
    //    --> src/main.rs:18:22
    //     |
    //  18 |     let of = OneField<f32> { f1: 4.0 };
    //     |                      ^   ^
    //     |
    //     = help: use `::<...>` instead of `<...>` to specify type arguments
    //     = help: or use `(...)` if you meant to specify fn arguments
    //  
    //  error: aborting due to previous error
    //  
    //  error: could not compile `GenericTypeArgs`.
    //  
    //  To learn more, run the command again with --verbose.

    // OK
    type OneF32 = OneField<f32>;
    let of = OneF32 { f1: 5.0 };
    println!("of: {}", of.f1);
}

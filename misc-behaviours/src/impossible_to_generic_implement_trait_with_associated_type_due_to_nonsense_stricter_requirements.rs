// As seen in issue https://github.com/rust-lang/rust/issues/116200

trait StoresItem {
    type Item: Clone;
    fn bar(&self) {}
}

// #region Implement on structs
struct MyBar<T> {
    t: T,
}
impl<T> StoresItem for MyBar<T>
where
    T: Clone,
{
    type Item = T;
}

struct MyFoo0 {}
struct MyFoo1 {}

// #endregion

mod try_use_implied_trait_bounds {

    use super::*;

    trait Foo {
        // trait StoresItem specifies that StoresItem::Item: Clone
        // so there's no need for Foo to specify that
        // and this is one of the few places where the compiler can infer that trait bound.
        // so the compiler doesn't complain here.
        type Bar<T>: StoresItem<Item = T>;
    }

    // But if we try and implement Foo, we can't because of the paradox in trait bounds.
    // Therefore, for consistency, without the support of implied trait bounds, the above should not compile.

    impl Foo for MyFoo0 {
        // What if we assume the compiler can again imply that T: Clone?
        //
        // Error: 'the trait bound `T: Clone` is not satisfied
        //         required for `MyBar<T>` to implement `StoresItem`
        //         required by a bound in `Foo::Bar`'
        type Bar<T> = MyBar<T>;
    }
    impl Foo for MyFoo1 {
        // Ok, what if we appease the compiler and specify that T: Clone so that Foo::Bar<T> is satisfied?
        //
        // Error: 'impl has stricter requirements than trait
        //         impl has extra requirement `T: Clone`'
        type Bar<T> = MyBar<T> where T: Clone;
    }
}

mod use_specified_trait_bounds {

    use super::*;

    trait Foo {
        // Let's specify that T: Clone within Foo, even though this is implied by StoresItem, and the compiler appears to know this as seen above.
        type Bar<T>: StoresItem<Item = T>
        where
            T: Clone;
    }

    impl Foo for MyFoo0 {
        // The compiler can't see here that MyBar<T> implies T: Clone within this context.
        // Error: 'the trait bound `T: Clone` is not satisfied...'
        type Bar<T> = MyBar<T>;
    }

    impl Foo for MyFoo1 {
        // This works
        type Bar<T> = MyBar<T> where T: Clone;
    }
}

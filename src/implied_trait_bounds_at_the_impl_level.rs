trait Bar {}

trait Foo<T>
where
    T: Bar,
{
    fn get_t(&self) -> T;
}

impl Bar for u32 {}

mod try_use_implied_trait_bounds {
    use super::*;
    struct MyFoo {}

    /// This won't work because T: Bar is implied but not specified
    // If T is an inaccessible public trait would implied trait bounds here break sealed traits here??
    // TAG: implied bounds would solve this
    impl<T> Foo<T> for MyFoo {
        fn get_t(&self) -> T {
            todo!()
        }
    }
}

mod specify_implied_trait_bounds {
    use super::*;
    struct MyFoo {}

    // This works because T: Bar is specified even though it's implied
    impl<T> Foo<T> for MyFoo
    where
        T: Bar,
    {
        fn get_t(&self) -> T {
            todo!()
        }
    }
}

/// Doesn't work because T: Bar is implied but not specified
fn takes_foo_implying_bounds<T>(foo: impl Foo<T>) {}

/// This works
fn takes_foo<T>(foo: impl Foo<T>)
where
    T: Bar,
{
}

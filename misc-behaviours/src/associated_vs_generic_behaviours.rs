trait Bar {
    fn bar(&self) {}
}

trait Foo {
    type T: Bar;
    fn get_t(&self) -> Self::T;
}

/// Doesn't work because T is specified as a generic and T: Bar is implied but not specified
// TAG: implied bounds would solve this
fn takes_foo_implying_bounds<T>(foo: impl Foo<T = T>) {
    foo.get_t().bar();
}

/// Works because T is not specified as a generic, so we trust the associated T of Foo must impl Bar
// QUESTION: Does this fall down somewhere else in a similar way to impossible_to_generic_implement_trait_with_associated_type_due_to_nonsense_stricter_requirements?
fn takes_foo_not_specifying_associated_type(foo: impl Foo) {
    foo.get_t().bar();
}

/// Works because T is specified as a generic and T: Bar is specified
fn takes_foo_specifying_associated_types_implying_bounds<T>(foo: impl Foo<T = T>)
where
    T: Bar,
{
    foo.get_t().bar();
}

mod where_associated_type_has_bound {
    use super::*;

    trait Baz {
        fn baz(&self) {}
    }

    fn takes_foo_where_associated_t_impl_baz<F: Foo>(foo: F)
    where
        F::T: Baz,
    {
        foo.get_t().baz();
    }
}

/// Concrete usage
impl Bar for u32 {}
struct MyFoo {}
impl Foo for MyFoo {
    type T = u32;
    fn get_t(&self) -> Self::T {
        0
    }
}

fn test() {
    takes_foo_not_specifying_associated_type(MyFoo {});
    takes_foo_specifying_associated_types_implying_bounds(MyFoo {})
}

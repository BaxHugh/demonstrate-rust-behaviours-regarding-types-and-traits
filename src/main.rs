mod associated_vs_generic_behaviours;
mod impl_has_stricter_bounds;
mod impl_trait_with_generic_on_type_without_generic;
mod implied_trait_bounds_at_the_impl_level;
mod impossible_to_generic_implement_trait_with_associated_type_due_to_nonsense_stricter_requirements;
mod mockall_generic_methods;
mod orphan_rule_with_generic_impl_problem_and_workaround;

fn main() {
    println!("Hello, world!");
}

mod _foo {
    use mockall::automock;

    #[automock]
    pub trait _Foo<T> {
        fn foo(&self, t: T) -> T;
    }
}

// can't be automocked
pub trait Foo {
    fn foo<T>(&self, t: T) -> T
    where
        Self: _foo::_Foo<T>,
    {
        _foo::_Foo::<T>::foo(self, t)
    }
}

impl<T> Foo for _foo::Mock_Foo<T> {}

// mock! {
//     pub Ob {}
//     impl Foo for Ob {
//         fn foo<T>(&self, t: T) -> T;
//     }
// }

// struct R<'a>(&'a i32);

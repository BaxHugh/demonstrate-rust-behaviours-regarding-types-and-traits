// /// This is a toy problem of what I was facing in a project.
// /// The root cause of the issue here is that HasFoo has a generic
mod doomed_situation_unless_using_phantom_data {
    trait Foo<T> {
        fn get_t(&self) -> &T;
    }

    trait HasFoo {
        type Foo<T>: Foo<T>;
        fn get_foo<T>(&self) -> &Self::Foo<T>;
    }

    mod try_use_implied_trait_bounds {
        use super::*;

        struct MyHasFoo<F> {
            foo: F,
        }

        impl<F> HasFoo for MyHasFoo<F> {
            // Can't do because F: Foo<T> is implied but not specified.
            type Foo<T> = F;
            fn get_foo<T>(&self) -> &Self::Foo<T> {
                todo!()
            }
        }
    }

    mod try_specify_trait_bounds {
        use super::*;

        mod where_clause_on_assoc_type_declaration {
            use super::*;
            struct MyHasFoo<F> {
                foo: F,
            }
            impl<F> HasFoo for MyHasFoo<F> {
                // Can't do because Impl has stricter requirements than trait
                type Foo<T> = F where F: Foo<T>;
                fn get_foo<T>(&self) -> &Self::Foo<T> {
                    todo!()
                }
            }
        }

        mod where_clause_on_impl {
            use super::*;
            struct MyHasFoo<F> {
                foo: F,
            }

            struct MyHasFooWithPhantomData<F, T> {
                foo: F,
                phantom: std::marker::PhantomData<T>,
            }

            // Can't do because
            // the type parameter `T` is not constrained by the impl trait, self type, or predicates unconstrained type parameter
            impl<F, T> HasFoo for MyHasFoo<F>
            where
                F: Foo<T>,
            {
                type Foo<_T> = F;
                fn get_foo<_T>(&self) -> &F {
                    todo!()
                }
            }

            // Why is the above not valid?
            // <MyHasFoo<F> as HasFoo>::Foo<T> is F, F could impl Foo<i32> and Foo<String> and the assoc type for HasFoo would still be the same F.
            // This is different for when F takes T as a generic parameter, because F<i32> and F<String> are different types.
            // but I don't understand why that's a problem.

            /// This works
            impl<F, T> HasFoo for MyHasFooWithPhantomData<F, T>
            where
                F: Foo<T>,
            {
                type Foo<_T> = F;
                fn get_foo<_T>(&self) -> &F {
                    todo!()
                }
            }
        }
    }
}

/// The issue with the above was that T was not important / relevant at the trait level, but was important at the method level.
/// So the thing to do is to only to specify the trait bounds on F: Foo<T> at the method it's required at both in the trait and impl.
/// This avoids the impl being more strict than the trait.
mod avoid_the_problem_with_a_different_foo_trait {

    trait Foo<T> {
        fn get_t(&self) -> &T;
    }

    trait HasFoo {
        type Foo;
        fn get_foo<T>(&self) -> &Self::Foo
        where
            Self::Foo: Foo<T>;
    }

    struct MyHasFoo<F> {
        foo: F,
    }

    impl<F> HasFoo for MyHasFoo<F> {
        type Foo = F;

        // Note: Interestingly, you don't need 'where F: Foo<T>' here.
        fn get_foo<T>(&self) -> &F {
            todo!()
        }
    }

    fn test<F, T>(has_foo: MyHasFoo<F>)
    where
        F: Foo<T>,
    {
        let a = has_foo.get_foo();
    }
}

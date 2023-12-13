/// Initial problem leading me down this ally.
/// This won't work with automock because the T needs to be T: 'static in the individual function
/// due to the inner workings of mock! / automock and it's use of Any.
/// ```
/// #[automock]
/// pub trait Foo {
///     fn foo<T>(&self, t: T) -> T;
/// }
/// ```
///
/// Basically what we want is an API that looks like `Foo` above, with `Foo::foo<T>()`,
/// but from the development side, actually looks like, `Foo::<T>::foo()`, because we want to implement
/// for only certain T, and maybe we only wan't some specific T to be supported.
/// This also allows you to automock Foo.

/// Backend for implementing Foo with T being a trait level (rather than method level) generic.
mod _foo {
    #[cfg(test)]
    use mockall::automock;

    /// Sealed development side trait.
    /// (Doesn't have to be sealed though)
    /// trait itself needs to be public as it'll leak into the public API.
    /// But can put it in a private module to make it sealed.
    #[cfg_attr(test, automock)]
    pub trait Foo<T> {
        fn foo(&self, t: T) -> T;
    }
}
#[cfg(test)]
pub use _foo::MockFoo;

/// Trait with T being a generic at the method level, this might be the intended public API.
/// automock is currently not possible here, as explained above.
pub trait Foo {
    fn foo<T>(&self, t: T) -> T
    where
        // Using a sealed trait like this may not be what you want.
        Self: _foo::Foo<T>,
    {
        _foo::Foo::<T>::foo(self, t)
    }
}

/// Note: we can't have a generic implementation of Foo for T where T: _foo::_Foo<U>
/// So we have to explicitly implement structs which impl _foo::Foo<T>.
#[cfg(test)]
impl<T> Foo for MockFoo<T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut mock = MockFoo::default();
        // Because the method names are the same in the backend and public trait.
        // It's as if we've just done automock on the public trait, we initially wanted.
        mock.expect_foo().returning(|t| t * 2);
        let _ = mock.foo(1);
    }
}

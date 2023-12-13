/// These represent the various backends we might want to use.
/// We want them to implement our crate's public trait.
/// We also want to implement our public trait for other composite types.
mod specific_clients {

    /// This is type is partially owned by us, but since it's generic over `Any` it's not fully owned.
    /// we can't guarantee that downstream traits will implement i.e. `BackendClientTrait<T>`, even though
    /// `BackendClientTrait<T>` is a private: NOTE: Currently the compiler doesn't know that's the case so thinks
    /// downstream crates could implement it.
    pub struct UnownedClientA<Any> {
        _marker: std::marker::PhantomData<Any>,
    }

    /// Same storey as `UnownedClientA` representing i.e. an alternative backend.
    pub struct UnownedClientB<Any> {
        _marker: std::marker::PhantomData<Any>,
    }
}

// /// This represents some wrapper around a potentially foreign U.
// struct NestedUnownedClient<U> {
//     nested: U,
// }

/// This could be a higher level type build from composition of many types of clients, i.e. file storage and database clients.
struct DualNestedUnownedClient<U, P> {
    nested: U,
    other: P,
}

mod untreated_problem {
    use super::*;
    use std::fmt::Debug;

    pub trait PubClientTrait<T> {
        fn get(&self) -> T {
            todo!()
        }
    }

    /// Simple and easy entry point that many compatible types might be able implement.
    /// Puts those types into a known interface which more complex functionality can be built on
    /// without the need for complex macros.
    trait BackendClientTrait<T> {
        fn backend_get(&self) -> T {
            todo!()
        }
    }

    impl<T, Client> PubClientTrait<T> for Client
    where
        Client: BackendClientTrait<T>,
        T: Debug,
    {
    }

    impl<T> BackendClientTrait<T> for specific_clients::UnownedClientA<String> {}
    impl<T> BackendClientTrait<T> for specific_clients::UnownedClientB<String> {}

    // error[E0119]: conflicting implementations of trait `PubClientTrait<_>` for type `DualNestedUnownedClient<_,_>`
    //    = note: downstream crates may implement trait `BackendClientTrait<_>` for type `DualNestedUnownedClient<_,_>`
    impl<T, U, P> PubClientTrait<T> for DualNestedUnownedClient<U, P> where U: PubClientTrait<T> {}
}

mod fixed_problem {
    use super::*;
    use std::fmt::Debug;

    pub trait PubClientTrait<T> {
        fn get(&self) -> T {
            todo!()
        }
    }

    /// Simple and easy entry point that many compatible types might be able implement.
    /// Puts those types into a known interface which more complex functionality can be built on
    /// without the need for complex macros.
    trait BackendClientTrait<T> {
        fn backend_get(&self) -> T {
            todo!()
        }
    }

    /// Coerce some backends from foreign crates to align with our interface.
    impl<T> BackendClientTrait<T> for specific_clients::UnownedClientA<String> {}
    impl<T> BackendClientTrait<T> for specific_clients::UnownedClientB<String> {}

    /// Entry point trait for implementing the public API on composite or wrapped client types.
    trait HasBackendClient<T> {
        type BackendType: BackendClientTrait<T>;
        fn get_backend_client(&self) -> &Self::BackendType;
    }

    /// Here's our generic implementation of the public trait using that HasBackendClient entrypoint.
    impl<Hb, T> PubClientTrait<T> for Hb
    where
        Hb: HasBackendClient<T>,
    {
        fn get(&self) -> T {
            let be = self.get_backend_client();
            be.backend_get()
        }
    }

    /// Implicitly implement the public trait for our nested type, via the generic implementation.
    impl<T, U> HasBackendClient<T> for DualNestedUnownedClient<U, specific_clients::UnownedClientA<T>>
    where
        U: BackendClientTrait<T>,
    {
        type BackendType = U;
        fn get_backend_client(&self) -> &Self::BackendType {
            &self.nested
        }
    }

    /// If we want to implement the `PubClientTrait` on the original type itself.
    /// we can't do it directly because since we don't own the type with a generic, we have an orphan-rule problem.
    // error[E0119]: conflicting implementations of trait `fixed_problem::HasBackendClient<_>` for type `DualNestedUnownedClient<_, specific_clients::UnownedClientA<_>>`
    // = note: downstream crates may implement trait `fixed_problem::BackendClientTrait<_>` for type `DualNestedUnownedClient<_, specific_clients::UnownedClientA<_>>`
    impl<T, U> HasBackendClient<T> for U
    where
        U: BackendClientTrait<T>,
    {
        type BackendType = U;
        fn get_backend_client(&self) -> &Self::BackendType {
            &self
        }
    }

    // We either just implement the trait on a wrapped type, or write out the implementation where we can use a wrapper which implements HasBackendClient and thus BackendClientTrait.
}

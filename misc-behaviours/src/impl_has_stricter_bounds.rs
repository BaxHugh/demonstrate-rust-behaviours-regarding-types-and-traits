trait Foo {
    fn foo<T>(&self) -> T;
}

struct MyFoo {}
impl Foo for MyFoo {
    fn foo<T: Clone>(&self) -> T {
        todo!()
    }
}

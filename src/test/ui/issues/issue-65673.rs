#![feature(trait_alias)]
trait Trait {}
trait WithType {
    type Ctx;
}
trait Alias<T> = where T: Trait;

impl<T> WithType for T {
    type Ctx = dyn Alias<T>;
//~^ ERROR the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
}
fn main() {}

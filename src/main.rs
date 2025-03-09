mod derive_traits;
mod dyn_dispatch;
mod operator_overloading;
mod drop_trait;
mod iterators;
mod impl_trait;
mod supertraits;
mod disambiguation;

fn main() {
    println!("Rust Traits Exercises");
    derive_traits::run();
    dyn_dispatch::run();
    operator_overloading::run();
    drop_trait::run();
    iterators::run();
    impl_trait::run();
    supertraits::run();
    disambiguation::run();
}

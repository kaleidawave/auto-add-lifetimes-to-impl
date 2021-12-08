#![allow(unused)]

trait X<T> {}

struct A;
struct B<'a>(&'a ());
struct C<'a, 'b> {
    a: &'a (),
    b: &'b (),
}

macro_rules! build_impl {
    ($name:ty, $arg:ty) => {
        #[auto_add_lifetimes_to_impl::auto_add_lifetimes_to_impl]
        impl X<$arg> for $name {}
    };
}

build_impl!(A, B<'a>);
build_impl!(B<'a>, A);
build_impl!(B<'a>, C<'l1, 'l2>);
build_impl!(C<'a, 'b>, B<'a>);

fn main() {}

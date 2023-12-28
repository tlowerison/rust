// check-pass

#![feature(arbitrary_self_types)]

use std::{
    marker::PhantomPinned,
    ops::Deref,
    pin::Pin,
};


#[derive(Default)]
struct Foo {
    _pinned: PhantomPinned,
    val: bool,
}

pub trait Swap {
    type This<'a>: Deref<Target = Self> where Self: 'a;

    fn swap1(self: Pin<&mut Self>) -> bool;
    fn swap2(self: Self::This<'_>) -> bool;
}

impl Swap for Foo {
    type This<'a> = Pin<Pin<Pin<&'a mut Self>>> where Self: 'a;

    fn swap1(self: Pin<&mut Self>) -> bool {
        true
    }
    fn swap2(self: Self::This<'_>) -> bool {
        true
    }
}

fn main() {
    let mut foo = Foo::default();
    let foo1 = unsafe { Pin::new_unchecked(&mut foo) };
    foo1.swap1();

    let foo2 = unsafe { Pin::new_unchecked(Pin::new_unchecked(Pin::new_unchecked(&mut foo))) };
    foo2.swap2();
}

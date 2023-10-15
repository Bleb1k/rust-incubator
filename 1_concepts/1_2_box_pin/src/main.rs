use std::{
    fmt,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
    time::Instant,
};

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Implementation must be meaningful, and
        // obviously call something requiring `&mut self`.
        // The point here is to practice dealing with
        // `Pin<&mut Self>` -> `&mut self` conversion
        // in different contexts, without introducing
        // any `Unpin` trait bounds.
    }
}

impl<T: SayHi> SayHi for Box<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: SayHi> SayHi for Rc<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: SayHi> SayHi for Vec<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for &[u8] {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}
/*
/// conflicting implementations of trait `SayHi` for type `Box<_>`
/// This is more general and should be preferred
impl<T: SayHi> SayHi for T {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}
*/
impl<T> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let _inner = self.get_mut();
    }
}

impl<T> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let _inner = self.get_mut();
    }
}

impl<T> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        // Vec<T> doesnt have Unpin, unlike all these other types
        let _inner = unsafe { self.get_unchecked_mut() };
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let _inner = self.get_mut();
    }
}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let _inner = self.get_mut();
    }
}
/*
/// conflicting implementations of trait `MutMeSomehow` for type `Box<_>`
/// This is more general, but unsafe
impl<T> MutMeSomehow for T {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let _inner = unsafe { self.get_unchecked_mut() };
    }
}
*/

#[derive(Debug)]
struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut: Clone + Copy> Future for MeasurableFuture<Fut> {
    type Output = Fut;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = Poll::Ready(self.inner_future);
        println!(
            "Took: {:?}",
            Instant::now().checked_duration_since(self.started_at.unwrap())
        );
        res
    }
}

fn main() {
    let _ = futures::executor::block_on(MeasurableFuture {
        inner_future: 1,
        started_at: Some(Instant::now()),
    });
}

// actually I can't answer most of the questions
// "Boxing" is placing stuff in heap and giving a reference to this stuff

// Pin is a wrapper around a reference to a value that "keeps value in place" - disallows "move" of a value

// Unpin is a trait that allows pinned value to be unpinned and therefore moved

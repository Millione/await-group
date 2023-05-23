//! Golang like [WaitGroup](https://pkg.go.dev/sync#WaitGroup) implementation.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! await-group = "0.1"
//! ```
//!
//! ## Example
//! ```rust
//! use await_group::AwaitGroup;
//!
//! #[tokio::main]
//! async fn main() {
//!     let wg = AwaitGroup::new();
//!     for _ in 0..10 {
//!         let w = wg.clone();
//!         tokio::spawn(async move {
//!             _ = w;
//!         });
//!     }
//!     wg.await;
//! }
//!
//! ```

extern crate alloc;

use core::{
    future::{Future, IntoFuture},
    pin::Pin,
    task::{Context, Poll},
};

use alloc::{sync::Arc, sync::Weak};

use atomic_waker::AtomicWaker;

#[derive(Clone, Default)]
pub struct AwaitGroup {
    inner: Arc<Inner>,
}

impl AwaitGroup {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                waker: AtomicWaker::new(),
            }),
        }
    }
}

impl IntoFuture for AwaitGroup {
    type Output = ();

    type IntoFuture = AwaitGroupFuture;

    fn into_future(self) -> Self::IntoFuture {
        AwaitGroupFuture {
            inner: Arc::downgrade(&self.inner),
        }
    }
}

pub struct AwaitGroupFuture {
    inner: Weak<Inner>,
}

impl Future for AwaitGroupFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.upgrade() {
            Some(inner) => {
                inner.waker.register(cx.waker());
                Poll::Pending
            }
            None => Poll::Ready(()),
        }
    }
}

#[derive(Default)]
struct Inner {
    waker: AtomicWaker,
}

impl Drop for Inner {
    fn drop(&mut self) {
        self.waker.wake();
    }
}

#[cfg(test)]
mod test {
    use crate::AwaitGroup;

    #[tokio::test]
    async fn smoke() {
        let wg = AwaitGroup::new();
        for _ in 0..10 {
            let w = wg.clone();
            tokio::spawn(async move {
                _ = w;
            });
        }
        wg.await;
    }
}

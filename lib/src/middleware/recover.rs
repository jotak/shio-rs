//! Defines the recover middleware, that permit catch `panic!`
//! and return an internal error

use std::sync::Arc;

use handler::BoxHandler;
use context::Context;
use response::BoxFutureResponse;

use std::panic::AssertUnwindSafe;
use futures::{future, lazy, Future};
use handler::default_catch;
use super::Middleware;

/// Middleware that catches `panic!`, returning an error 500 to the user.
pub struct Recover;

impl Middleware for Recover {
    fn call(&self, next: BoxHandler) -> BoxHandler {
        let next = Arc::new(next);
        Box::new(move |ctx: Context| -> BoxFutureResponse {
            let next = next.clone();
            Box::new(
                AssertUnwindSafe(lazy(move || next.call(ctx)))
                    .catch_unwind()
                    .then(move |result| -> BoxFutureResponse {
                        Box::new(match result {
                            Err(err) => future::ok(default_catch(err)),
                            Ok(result) => future::result(result),
                        })
                    }),
            )
        })
    }
}
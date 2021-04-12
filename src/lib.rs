#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::{Infallible, TryInto};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use napi::{CallContext, Error as NapiError, JsNumber, JsObject, Result as JsResult, Status};

#[cfg(all(
  unix,
  not(target_env = "musl"),
  not(target_arch = "aarch64"),
  not(target_arch = "arm"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> JsResult<()> {
  exports.create_named_method("createApp", create_app)?;
  Ok(())
}

#[js_function(1)]
fn create_app(ctx: CallContext) -> JsResult<JsObject> {
  let port: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let start = async move {
    let addr = ([127, 0, 0, 1], port as _).into();
    let make_svc = make_service_fn(|_conn| {
      // This is the `Service` that will handle the connection.
      // `service_fn` is a helper to convert a function that
      // returns a Response into a `Service`.
      async { Ok::<_, Infallible>(service_fn(hello)) }
    });
    let server = Server::bind(&addr).serve(make_svc);
    server
      .await
      .map_err(|e| NapiError::new(Status::GenericFailure, format!("{}", e)))?;

    Ok(())
  };
  ctx
    .env
    .execute_tokio_future(start, |env, _| env.get_undefined())
}

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
  Ok(Response::new(Body::from("Hello!")))
}

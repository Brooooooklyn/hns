#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::{Infallible, TryInto};

use http::request::Parts;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use napi::{
  self,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
  CallContext, Error as NapiError, JsFunction, JsNumber, JsObject, Result as JsResult, Status,
};

#[cfg(all(
  unix,
  not(target_env = "musl"),
  not(target_arch = "aarch64"),
  not(target_arch = "arm"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64", not(debug_assertions)))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> JsResult<()> {
  exports.create_named_method("createApp", create_app)?;
  Ok(())
}

#[js_function(3)]
fn create_app(ctx: CallContext) -> JsResult<JsObject> {
  let port: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let ready_callback = ctx.get::<JsFunction>(1)?;
  let on_req_callback = ctx.get::<JsFunction>(2)?;
  let ready_tsfn_callback = ctx.env.create_threadsafe_function(
    &ready_callback,
    1,
    |cx: napi::threadsafe_function::ThreadSafeCallContext<Option<u32>>| {
      cx.env.get_boolean(true).map(|v| vec![v])
    },
  )?;
  let req_tsfn_callback = ctx.env.create_threadsafe_function(
    &on_req_callback,
    1,
    |cx: napi::threadsafe_function::ThreadSafeCallContext<Parts>| {
      let parts = cx.value;
      let version = format!("{:?}", &parts.version);
      let method = parts.method.as_str();
      let uri = format!("{}", &parts.uri);
      let headers = format!("{:?}", &parts.headers);
      Ok(vec![
        cx.env.create_string(&version)?,
        cx.env.create_string(method)?,
        cx.env.create_string(&uri)?,
        cx.env.create_string(&headers)?,
      ])
    },
  )?;
  let tsfn_for_err = ready_tsfn_callback.clone();
  let start = async move {
    let addr = ([127, 0, 0, 1], port as _).into();
    let make_svc = make_service_fn(move |_conn| {
      let req_tsfn_callback = req_tsfn_callback.clone();
      async {
        Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
          let req_tsfn_callback = req_tsfn_callback.clone();
          on_req(req, req_tsfn_callback)
        }))
      }
    });
    let server = Server::bind(&addr).serve(make_svc);

    ready_tsfn_callback.call(
      Ok(None),
      napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
    );
    server.await.map_err(move |e| {
      let err = NapiError::new(Status::GenericFailure, format!("{}", e));
      tsfn_for_err.call(
        Err(err),
        napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
      );
      NapiError::new(Status::GenericFailure, format!("{}", e))
    })?;

    Ok(())
  };
  ctx
    .env
    .execute_tokio_future(start, |env, _| env.get_undefined())
}

#[inline(always)]
async fn on_req(
  req: Request<Body>,
  callback: ThreadsafeFunction<Parts>,
) -> Result<Response<Body>, Infallible> {
  let (parts, _body) = req.into_parts();
  callback.call(Ok(parts), ThreadsafeFunctionCallMode::NonBlocking);
  Ok(Response::new(Body::from("Hello!")))
}

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::{Infallible, TryInto};

use hyper::body::HttpBody;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use napi::{
  self,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
  CallContext, Error as NapiError, JsFunction, JsNumber, JsObject, Result as JsResult, Status,
};

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
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
    |cx: napi::threadsafe_function::ThreadSafeCallContext<Request<Body>>| {
      let (parts, body) = cx.value.into_parts();
      let version = format!("{:?}", &parts.version);
      let method = parts.method.as_str();
      let uri = format!("{}", &parts.uri);
      let headers = format!("{:?}", &parts.headers);
      let body_size_hint = body.size_hint().upper().map(|s| s as i64);
      let body = cx.env.create_external(body, body_size_hint)?;
      Ok(vec![
        cx.env.create_string(&version)?.into_unknown(),
        cx.env.create_string(method)?.into_unknown(),
        cx.env.create_string(&uri)?.into_unknown(),
        cx.env.create_string(&headers)?.into_unknown(),
        body.into_unknown(),
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
  callback: ThreadsafeFunction<Request<Body>>,
) -> Result<Response<Body>, Infallible> {
  callback.call(Ok(req), ThreadsafeFunctionCallMode::NonBlocking);

  Ok(Response::new(Body::from("Hello!")))
}

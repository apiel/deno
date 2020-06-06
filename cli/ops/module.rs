// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::dispatch_json::{JsonOp, Value, Deserialize};
use crate::op_error::OpError;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::ZeroCopyBuf;
use std::path::{Path};

pub fn init(i: &mut CoreIsolate, s: &State) {
  i.register_op(
    "op_clear_cache_import",
    s.stateful_json_op(op_clear_cache_import),
  );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClearCacheImportArgs {
  path: String,
}

fn op_clear_cache_import(
  state: &State,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, OpError> {
  let state = state.borrow_mut();
  let referrer = state.main_module.to_string();

  let args: ClearCacheImportArgs = serde_json::from_value(args)?;
  let path = Path::new(&args.path).to_path_buf();

  println!("Hello module {:?} {:?}", path, referrer);
  // assert_eq!(zero_copy.len(), 1);

  // if let Some(ref mut seeded_rng) = state.borrow_mut().seeded_rng {
  //   seeded_rng.fill(&mut *zero_copy[0]);
  // } else {
  //   let mut rng = thread_rng();
  //   rng.fill(&mut *zero_copy[0]);
  // }

  Ok(JsonOp::Sync(json!({})))
}

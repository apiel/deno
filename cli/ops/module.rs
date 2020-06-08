// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::dispatch_json::{JsonOp, Value, Deserialize};
use crate::op_error::OpError;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::ZeroCopyBuf;
use deno_core::ModuleSpecifier;
use crate::permissions::Permissions;

pub fn init(i: &mut CoreIsolate, s: &State) {
  // i.remove_module();
  i.register_op(
    "op_clear_cache_import",
    s.stateful_json_op(op_clear_cache_import),
  );
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClearCacheImportArgs {
  specifier: String,
}

fn op_clear_cache_import(
  state: &State,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, OpError> {
  let state = state.borrow_mut();
  let referrer = state.main_module.to_string();

  let args: ClearCacheImportArgs = serde_json::from_value(args)?;
  let specifier = args.specifier.clone();


  let module_specifier =
    ModuleSpecifier::resolve_import(&specifier, &referrer)?;

  state.global_state.file_fetcher.remove_cached_source_file(&module_specifier);

  // state.global_state.modules.unregister("abc");

  // let out = state.global_state
  //                .file_fetcher
  //                .fetch_cached_source_file(&module_specifier, Permissions::allow_all());
  //               //  .expect("Cached source file doesn't exist");

  // if out.is_none() {
  //   println!("No Cache");

  // } else {
  //   let yo = out.unwrap();
  //   println!("Cache path {:?} {:?}", yo.filename, String::from_utf8(yo.source_code).unwrap());

  // }

  Ok(JsonOp::Sync(json!({})))
}

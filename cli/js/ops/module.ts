// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
import { sendSync } from "./dispatch_json.ts";

export function clearCacheImport(specifier: string) {
  return sendSync("op_clear_cache_import", { specifier });
}

// Copyright 2018-2025 the Deno authors. MIT license.

use std::env;
use std::path::PathBuf;

use deno_core::snapshot::*;
use deno_runtime::*;
mod release;
mod ts;

pub use ts::create_compiler_snapshot;

pub fn create_cli_snapshot(snapshot_path: PathBuf) {
  use deno_runtime::ops::bootstrap::SnapshotOptions;

  let snapshot_options = SnapshotOptions {
    ts_version: ts::version(),
    v8_version: deno_core::v8::VERSION_STRING,
    target: std::env::var("TARGET").unwrap(),
  };

  deno_runtime::snapshot::create_runtime_snapshot(
    snapshot_path,
    snapshot_options,
    vec![],
  );
}

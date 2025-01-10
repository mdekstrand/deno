use std::env;
use std::path::PathBuf;

use deno_snapshot::*;

fn main() {
  let args: Vec<_> = env::args_os().collect();
  if args.len() != 3 {
    eprintln!("Usage: deno-snapshot CLI_SRC_DIR OUT_DIR");
    panic!("invalid command line parameters");
  }

  let src = PathBuf::from(&args[1]);
  let out = PathBuf::from(&args[2]);

  let compiler_snapshot_path = out.join("COMPILER_SNAPSHOT.bin");
  create_compiler_snapshot(compiler_snapshot_path, &src);

  let cli_snapshot_path = out.join("CLI_SNAPSHOT.bin");
  create_cli_snapshot(cli_snapshot_path);
}

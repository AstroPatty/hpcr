# hpcr

Rust CLI for running containerized HPC jobs across multiple facilities and container runtimes. Invoked as a per-rank process by MPI launchers (`mpiexec`, `srun`); replaces itself with the container process via `exec()`.

## Build & test

```bash
cargo build
cargo test
cargo build --release
```

No special environment required to build or test — facility configs are embedded at compile time.

## Architecture

The codebase is a single Cargo package with a lib crate and two binaries (`hpcr`, `hpcr-setup` was folded into `hpcr setup`).

```
src/
  lib.rs                    # declares all modules
  main.rs                   # thin entry point; dispatches to command layer
  error.rs                  # HpcrError enum (thiserror)
  cli/
    mod.rs                  # Cli + Commands enum; --dry-run lives here
    common.rs               # CommonArgs: --mpi, --bind, --env
    run.rs                  # RunArgs (trailing_var_arg for passthrough)
    exec.rs                 # ExecArgs (trailing_var_arg for command + args)
    setup.rs                # SetupArgs (--facility override)
  config/
    mod.rs                  # re-exports
    facility.rs             # FacilityConfig, FacilityEnvVar, EnvOp; BUNDLED registry
    local.rs                # LocalConfig; reads ~/.config/hpcr/local.toml
  runtime/
    mod.rs                  # BindMount, EnvVar, RunSpec, ExecSpec, Runtime enum, ContainerRuntime trait
    apptainer.rs            # --bind src:dst  --env KEY=VAL
    podman.rs               # -v src:dst  -e KEY=VAL; shared helpers used by podman_hpc
    podman_hpc.rs           # delegates to podman helpers with "podman-hpc" binary name
  command/
    mod.rs                  # parse_bind/parse_env, resolve_env, expand_facility, build_*_command
    conflict.rs             # check_bind_conflicts (by dst), check_env_conflicts (by key)
    setup.rs                # run_setup: hostname detection, interactive prompt, writes local.toml
facilities/
  perlmutter.toml           # NERSC — podman-hpc
  frontier.toml             # ORNL — apptainer
  polaris.toml              # ALCF — apptainer
```

## Key design decisions

**Facility configs are compiled into the binary** via `include_str!` in `src/config/facility.rs`. Adding a new facility means adding a TOML file under `facilities/` and an entry in the `BUNDLED` array — then recompiling.

**The command pipeline** (`command/mod.rs`):
1. Parse user `--bind`/`--env` flags into `BindMount`/`EnvVar`
2. `expand_facility`: clone always-on binds/envs, extend with `mpi_*` if `--mpi`; run `resolve_env` on each `FacilityEnvVar`
3. Conflict check (user binds vs facility binds by dst; user envs vs facility envs by key)
4. Merge (facility first, user after) into `RunSpec`/`ExecSpec`
5. Call `ContainerRuntime::build_*_command` → `std::process::Command`
6. `cmd.exec()` — replaces the hpcr process; critical for MPI signal propagation

**`resolve_env`** (`command/mod.rs:resolve_env`) handles `FacilityEnvVar.op`:
- `set` (default): use value as-is
- `prepend`: read host env var, prepend configured value with separator
- `append`: read host env var, append configured value with separator

This is needed for `LD_LIBRARY_PATH` on Polaris, where the container's existing paths must be preserved. `resolve_env` reads `std::env::var(key)` at hpcr invocation time (i.e., after job script module loading).

**`hpcr setup`** is dispatched in `main.rs` _before_ loading local config, since it creates that config. The `Commands::Setup` arm is matched first; everything else proceeds with `load_local_config()` → `load_facility()`.

**`--dry-run`** (`cli/mod.rs`, `main.rs:print_command`) prints the generated runtime command with shell quoting and line continuations instead of exec'ing. Useful for debugging facility configs.

**Passthrough args** (`RunArgs.args`, `ExecArgs.args`) use `trailing_var_arg = true, allow_hyphen_values = true`. For `run`, they go before the image in the apptainer/podman invocation (runtime flags like `--nv`). For `exec`, the full slice is treated as the command and its arguments, placed after the image.

**`podman-hpc`** shares flag translation with `podman` via `pub(crate)` free functions `build_run_args_for`/`build_exec_args_for` in `runtime/podman.rs`, parameterized by binary name.

## Adding a new facility

1. Create `facilities/<name>.toml` following the schema in `SPEC.md`
2. Add `("<name>", include_str!("../../facilities/<name>.toml"))` to `BUNDLED` in `src/config/facility.rs`
3. Add a hostname pattern to `PATTERNS` in `src/command/setup.rs`
4. Add a parse test in `src/config/facility.rs`
5. `cargo test`

## Facility config schema

```toml
[facility]
name = "example"
runtime = "apptainer"          # apptainer | podman | podman-hpc

[[binds]]                      # always injected
src = "/host/path"
dst = "/container/path"

[[envs]]                       # always injected
key = "SOME_VAR"
value = "some_value"
# op = "set"                   # default; also "prepend" or "append"
# separator = ":"              # default separator for prepend/append

[[mpi_binds]]                  # injected only with --mpi
src = "/opt/mpi"
dst = "/opt/mpi"

[[mpi_envs]]                   # injected only with --mpi
key = "LD_LIBRARY_PATH"
value = "/opt/mpi/lib"
op = "append"
```

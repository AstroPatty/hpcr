# hpcr

`hpcr` is a CLI for running containerized jobs on HPC systems with full MPI support. It wraps container runtimes like Apptainer and Podman, automatically injecting the bind mounts and environment variables your facility requires — so your job scripts stay portable across systems.

## How it works

You invoke `hpcr` as the per-rank executable inside your MPI launcher. It translates a simple, uniform interface into the correct runtime invocation for your facility, then replaces itself with the container process via `exec()`.

```bash
mpiexec -n 32 hpcr run --mpi my_container.sif
```

## Installation

```bash
curl -fsSL https://raw.githubusercontent.com/AstroPatty/hpcr/main/install.sh | sh
```

This downloads the latest release binary, installs it to `~/.local/bin`, and runs `hpcr setup` automatically. If `~/.local/bin` is not on your `PATH`, add it to your shell profile:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

To install a specific version, set `HPCR_VERSION`:

```bash
HPCR_VERSION=v0.1.0 curl -fsSL https://raw.githubusercontent.com/AstroPatty/hpcr/main/install.sh | sh
```

To install to a different directory, set `INSTALL_DIR`:

```bash
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/AstroPatty/hpcr/main/install.sh | sh
```

### Build from source

```bash
cargo build --release
cp target/release/hpcr ~/.local/bin/
```

## Setup

Run setup once after installation. It detects your facility from the hostname and writes `~/.config/hpcr/local.toml`:

```bash
hpcr setup
```

If detection succeeds, you'll be asked to confirm:

```
Detected facility: perlmutter
Use this facility? [Y/n]
```

If detection fails, you'll be prompted to choose from the supported facilities. You can also skip the prompt entirely by passing `--facility` directly:

```bash
hpcr setup --facility frontier
```

### Supported facilities

| Facility | System | Runtime |
|----------|--------|---------|
| `perlmutter` | NERSC Perlmutter | `podman-hpc` |
| `frontier` | ORNL Frontier | `apptainer` |

## Commands

### `hpcr run`

Runs a container using its built-in entrypoint. Equivalent to `apptainer run` or `podman run`.

```
hpcr run [OPTIONS] IMAGE [ARGS]...
```

### `hpcr exec`

Runs a container with a specific command. Equivalent to `apptainer exec`.

```
hpcr exec [OPTIONS] IMAGE COMMAND [ARGS]...
```

### Common options

| Flag | Description |
|------|-------------|
| `--mpi` | Inject facility MPI bind mounts and environment variables |
| `--bind SRC:DST` | Add a bind mount (repeatable) |
| `--env KEY=VALUE` | Set an environment variable (repeatable) |

Any arguments after `--` are passed through verbatim to the underlying runtime.

## Usage examples

**Run a container with MPI support:**
```bash
mpiexec -n 32 hpcr run --mpi my_container.sif
```

**Add a user bind mount:**
```bash
mpiexec -n 32 hpcr run --mpi --bind $(pwd):/output my_container.sif
```

**Run a specific command in the container:**
```bash
srun -n 8 hpcr exec --mpi --bind /scratch:/data my_container.sif python train.py
```

**Pass runtime flags through to the underlying runtime:**
```bash
mpiexec -n 4 hpcr run --mpi my_container.sif -- --nv
```

**Set an environment variable:**
```bash
srun -n 16 hpcr exec --mpi --env BATCH_SIZE=128 my_container.sif python train.py
```

## Conflict detection

If a `--bind` destination or `--env` key you provide conflicts with one required by the facility config, `hpcr` exits immediately with an error before touching the runtime:

```
hpcr: bind conflict: --bind dst '/pscratch' is reserved by facility 'perlmutter'
```

## Facility configs

Facility configurations are bundled into the binary at compile time. Each config specifies the runtime and the bind mounts and environment variables to inject — unconditionally, or only when `--mpi` is passed.

Adding a new facility requires adding a TOML file to `facilities/` and recompiling. User-editable configs are not supported.

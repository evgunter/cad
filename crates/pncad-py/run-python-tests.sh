#!/usr/bin/env bash
# Build the extension module and run the Python suite against it.
#
# WHY THIS EXISTS instead of just `maturin develop`: this box has a
# CPython 3.12 but NO pip and no ensurepip, so nothing can be
# installed into it and no virtualenv can be populated. maturin is
# therefore staged as a standalone binary (see the PR body) — and even
# without maturin at all, the module is a plain cdylib that cargo can
# build and Python can import once it is named `pncad.so`. That
# fallback is what this script does, so the suite is runnable in the
# most degraded environment we actually have.
#
# Usage: crates/pncad-py/run-python-tests.sh [python-binary]
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
python=${1:-python3}
stage=${PNCAD_STAGE:-$root/target/python-stage}

# The heavy row goes through the machine-wide build slot like every
# other cargo invocation in this repo.
"$root/local-scripts/with-build-slot.sh" -- \
    cargo build -p pncad-py --features extension-module

lib=$root/target/debug/libpncad_py.so
if [[ ! -f $lib ]]; then
    echo "no cdylib at $lib" >&2
    exit 1
fi

mkdir -p "$stage"
# CPython imports an extension module by FILE name, which must equal
# the `#[pymodule]` name; the crate is `pncad-py`, the module `pncad`.
cp "$lib" "$stage/pncad.so"
cp "$root/crates/pncad-py/pncad.pyi" "$stage/pncad.pyi"

echo "staged $stage/pncad.so"
PYTHONPATH=$stage "$python" -m unittest discover \
    --start-directory "$root/crates/pncad-py/tests" \
    --top-level-directory "$root/crates/pncad-py/tests" \
    --verbose

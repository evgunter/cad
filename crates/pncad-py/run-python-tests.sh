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
# A ZERO-TEST RUN IS NOT A PASS. `unittest discover` over a directory
# whose modules do not match its pattern prints `Ran 0 tests ... OK` and
# exits 0 — a renamed tests/ directory, or a `--start-directory` that
# stops resolving, leaves this row green having executed nothing. So the
# count is read back and required to be non-zero, and echoed, so "the
# python suite ran N tests" comes off the run rather than being assumed.
#
# ONE OF THREE COPIES, stated rather than hidden: the other two are
# ci.yml's `python suite` job and nightly.yml's ungated re-take. No one
# place all three call exists — the hosted jobs cannot call THIS script,
# which builds through `local-scripts/with-build-slot.sh` (a tree every
# hosted job deletes at checkout) and stages a cdylib rather than
# installing a wheel. The lift is filed at
# work/issues/python-suite-zero-test-guard-three-copies.md.
#
# `Ran N tests` goes to STDERR, so the redirect is load-bearing, and this
# script's `pipefail` is what keeps python's exit status from being
# swallowed by `tee`.
log=$(mktemp)
PYTHONPATH=$stage "$python" -m unittest discover \
    --start-directory "$root/crates/pncad-py/tests" \
    --top-level-directory "$root/crates/pncad-py/tests" \
    --verbose 2>&1 | tee "$log"
ran=$(sed -n 's/^Ran \([0-9]\{1,\}\) test.*/\1/p' "$log" | tail -1)
rm -f "$log"
echo "python tests run: ${ran:-0}"
if [ "${ran:-0}" -le 0 ]; then
    echo "ERROR: the python suite discovered no tests — a green run that executed nothing" >&2
    exit 1
fi

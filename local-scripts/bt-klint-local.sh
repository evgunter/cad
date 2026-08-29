#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Mirrors the k-lint CI job's command set exactly. Written after that job
# failed TWICE on things local `cargo check` cannot see: in-lib
# `#[cfg(test)]` modules, and `dead_code` under `-D warnings` in the
# excluded demo workspaces. `cargo check` is not `cargo clippy
# --all-targets -- -D warnings`, and -p scoping is not --workspace.
set -u
cd "$(dirname "$0")/.."
fail=0
run() { echo "--- $* "; if "$@" >/tmp/bt-klint.log 2>&1; then echo "    OK"; else echo "    FAILED:"; grep -E "^(error|warning)" -A 4 /tmp/bt-klint.log | head -20; fail=1; fi; }

( cd demos/tour   && cargo fmt --check ) && echo "tour fmt OK"   || { echo "tour fmt FAILED"; fail=1; }
( cd demos/wild   && cargo fmt --check ) && echo "wild fmt OK"   || { echo "wild fmt FAILED"; fail=1; }
( cd tools/k-lint && cargo fmt --check ) && echo "k-lint fmt OK" || { echo "k-lint fmt FAILED"; fail=1; }

echo "=== demos/tour clippy, DEFAULT features (the row that failed) ==="
( cd demos/tour && cargo clippy --all-targets -- -D warnings ) >/tmp/bt-klint.log 2>&1 \
  && echo "    OK" || { echo "    FAILED:"; grep -E "^error" -A 5 /tmp/bt-klint.log | head -20; fail=1; }

echo "=== demos/tour clippy, --features probe ==="
( cd demos/tour && cargo clippy --all-targets --features probe -- -D warnings ) >/tmp/bt-klint.log 2>&1 \
  && echo "    OK" || { echo "    FAILED:"; grep -E "^error" -A 5 /tmp/bt-klint.log | head -20; fail=1; }

echo "=== demos/wild clippy ==="
( cd demos/wild && cargo clippy --all-targets -- -D warnings ) >/tmp/bt-klint.log 2>&1 \
  && echo "    OK" || { echo "    FAILED:"; grep -E "^error" -A 5 /tmp/bt-klint.log | head -20; fail=1; }

echo "=== tools/k-lint clippy + test ==="
( cd tools/k-lint && cargo clippy --all-targets -- -D warnings && cargo test ) >/tmp/bt-klint.log 2>&1 \
  && echo "    OK" || { echo "    FAILED:"; tail -12 /tmp/bt-klint.log; fail=1; }

echo
echo "OVERALL: $([ $fail -eq 0 ] && echo PASS || echo FAIL)"

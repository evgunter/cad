#!/usr/bin/env bash
# no-ambient-env.sh — no ambient environment in the kernel. ONE home;
# ci.yml's "no ambient environment in the kernel" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# NO AMBIENT ENVIRONMENT IN THE KERNEL. A runtime env read inside
# crates/*/src is a back channel: it changes shipped behaviour
# with no rebuild, no flag, and no call site to review. This gate
# exists because one had been live — `mesh::probe_stats::armed()`
# answered true for `NURBS_PROBE`, which switched on a 91-sample
# resampling of every emitted triangle (measured 7.9 s -> 19.8 s
# on the tour's release binary, same binary, same arguments) AND
# put an `assert!` in the tessellation path, so an environment
# variable converted `tessellate`'s typed error contract into a
# panic.
#
# The allowlist is the two RATIFIED knobs, both read once and
# documented where they are read:
#  - geom-core tolerance.rs — CAD_TOLERANCE_EPS, the eps matrix
#    this whole workflow is built on (a OnceLock, read at init);
#  - test-utils fuzz.rs — CAD_FUZZ_SEED / CAD_FUZZ_EFFORT, and
#    test-utils is a dev-only leaf no shipped build can reach.
#
# `env!` is deliberately NOT matched: it is compile-time, baked
# into the binary, and cannot be an ambient channel.
#
# Telemetry is the recurring offender, so it has a rule of its
# own: gate it behind a feature at the module boundary from the
# first commit, and arm it by an explicit call — never by the
# environment. Worked examples: `mesh::budget`, `mesh::probe_stats`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  local hits
  hits=$(grep -rPn '\benv::vars?(_os)?\s*\(' crates/*/src --include='*.rs' \
    | grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | cut -d: -f1 | sort -u \
    | grep -vE '^crates/geom-core/src/tolerance\.rs$' \
    | grep -vE '^crates/test-utils/src/fuzz\.rs$' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a kernel crate reads the environment at runtime — that is a back channel into shipped code, changing behaviour with no rebuild and no call site to review (NURBS_PROBE was exactly this). Arm it by an explicit call and gate it behind a feature, or ratify this file into the allowlist."
    exit 1
  fi
  gate_ok "no kernel crate reads the environment at runtime"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn armed() -> bool { std::env::var("PLANTED_PROBE").is_ok() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_parse_args "$@"
gate_main "a kernel crate reads the environment at runtime" plant

#!/usr/bin/env bash
# no-extra-real-bounds.sh — no extra bounds on `Real`. ONE home;
# ci.yml's "no extra bounds on Real" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# Evaluation-code discipline tripwire (see the `real.rs` module docs):
# a type parameter written `T: Real + PartialOrd` (or any other extra
# bound) is the escape hatch that lets generic evaluation code smuggle
# raw comparisons past the trait — it compiles at f64 and only dies at
# the interval instantiation. Fail if any `Real +` bound combination
# appears in crate sources. grep exits 1 (no match) on a clean tree —
# that is the PASS case; a match exits 0 and we fail loudly here. When
# a legitimate `Real +` combination first appears, refine this into an
# allowlist as a design decision — do not silently delete the check.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  if grep -rnE '\bReal\s*\+' crates/*/src; then
    gate_error "found 'Real +' bound(s) above — evaluation-code discipline forbids extra bounds on scalar type parameters"
    exit 1
  fi
  gate_ok "no extra bounds on Real"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real + PartialOrd>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

gate_parse_args "$@"
gate_main "found 'Real +' bound(s) above" plant

#!/usr/bin/env bash
# evalscalar-allowlist.sh — the Bounds gate, over the compound bound's
# NAME. ONE home; ci.yml's "EvalScalar allowlist (the Bounds gate, over
# the name)" step and local-scripts/ci-local.sh's discipline row both
# call this file.
#
# The SAME gate, over the compound bound's NAME (ASM-2A review
# MINOR-4). `editor_core::EvalScalar` is the evaluation-service
# bound — `Decide + ContentBits + Bounds + Send + Sync +
# PropsQuadLane` — declared once at the seam the rule above
# already ratifies, so `eval/parts.rs` names the requirement
# instead of restating `+ Bounds`. That is a naming, and it must
# stay one: the trait is `pub` (the integration suites need it),
# so without this step any file in any crate could acquire a
# compound Bounds bound invisibly to the grep above. Allowlist =
# exactly the seam real.rs already names, so the two steps agree
# about where the seam is; growing it needs the same ratification
# the Bounds allowlist does. Both bound positions are caught
# (`T: EvalScalar` and `+ EvalScalar`), comments stripped as above.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  local hits
  hits=$(grep -rnE '(:|\+)\s*(editor_core::)?EvalScalar\b' crates/*/src \
    | grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | cut -d: -f1 | sort -u \
    | grep -vE '^crates/editor-core/src/eval/(mod|parts)\.rs$' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "EvalScalar (a compound Bounds bound by another name) outside the evaluation-service seam — see geom-core/src/real.rs (Bounds scope rule) and editor-core/src/eval/mod.rs; ratify before allowlisting"
    exit 1
  fi
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: editor_core::EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

gate_parse_args "$@"
gate_main "EvalScalar (a compound Bounds bound by another name)" plant

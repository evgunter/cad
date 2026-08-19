#!/usr/bin/env bash
# kernel-serde-free.sh — the kernel crates take NO serde dependency.
# ONE home; ci.yml's "kernel crates are serde-free" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# THE RULE (DESIGN.md's serde row, the F3/G1 layering rule): geometry
# and topology are the kernel; persistence is the document layer's job.
# A kernel crate that gained serde derives would let a file rebuild an
# arena field-by-field, bypassing the doors whose checks are the only
# reason a loaded body can be trusted — and it would freeze kernel
# struct layout into the persisted format, so a refactor below would
# become a schema break above.
#
# WHY A MANIFEST SCAN. serde reaches a crate only through its manifest:
# derives without the dependency do not compile. Scanning manifests is
# therefore complete for the thing being forbidden, which reading
# sources for `#[derive(Serialize)]` would not be (a re-export, an alias
# or a `with` module would slip past a source grep).
#
# WHAT THIS DOES NOT PROVE: that no kernel TYPE is persisted. Types
# defined in the kernel are persisted every day — the document layer
# describes their bytes itself, with `#[serde(with)]` modules that live
# above the boundary (`editor-core`'s `boolean_op_wire`,
# `declare_pairs_wire`). That is the sanctioned shape and this gate is
# blind to it by design: it polices the DEPENDENCY EDGE, nothing else.
#
# THE ALLOWLIST is the two crates above the boundary: `editor-core`,
# which owns persistence, and `pncad-py`, whose bindings cross values
# into python. A third name here is a layering decision — ratify it in
# DESIGN.md, do not add it in passing.
#
# `profile` is additionally sealed from inside (`profile/tests/seal.rs`)
# because a second argument rests on its absence there; this gate is the
# workspace-wide half.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# A GATE THAT SCANNED NOTHING IS NOT A PASS — the manifest analogue of
# `gate_require_crate_sources`, since this gate's subject is manifests.
require_crate_manifests() {
  GATE_SCAN_FILES=$(ls crates/*/Cargo.toml 2>/dev/null | wc -l)
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no crates/*/Cargo.toml under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
}

gate() {
  require_crate_manifests
  local hits
  hits=$(grep -lE '^[[:space:]]*serde(_[a-z]+)?[[:space:]]*=' crates/*/Cargo.toml \
    | grep -vE '^crates/(editor-core|pncad-py)/Cargo\.toml$' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a kernel crate names serde in its manifest — persistence is the document layer's job (DESIGN.md's serde row). Describe the bytes ABOVE the boundary with a \`#[serde(with)]\` module in editor-core, or ratify the new layering in DESIGN.md."
    exit 1
  fi
  gate_ok "no kernel crate depends on serde"
}

# The clean fixture is a manifest tree, not a source tree.
gate_plant_clean() {
  mkdir -p "$1/crates/clean"
  printf '[package]\nname = "clean"\n' > "$1/crates/clean/Cargo.toml"
}

plant() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[dependencies]\nserde = { workspace = true }\n' \
    > "$1/crates/planted/Cargo.toml"
}

gate_parse_args "$@"
gate_main "a kernel crate names serde in its manifest" plant

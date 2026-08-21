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
# WHY A TOML PARSE, NOT A GREP. serde reaches a crate only through a
# dependency entry, but a dependency entry has at least four spellings:
#
#   serde = { workspace = true }          # inline table
#   serde.workspace = true                # dotted key — already the
#                                         # house style for `edition`,
#                                         # `license`, `rust-version`
#   [dependencies.serde]                  # sub-table
#   ser = { package = "serde", ... }      # renamed
#
# The first version of this gate matched the first spelling with a
# regex and reported OK on the other three. A parser reads the same
# document the build reads, so the spelling stops mattering: the check
# is on the resolved PACKAGE NAME of every entry in every dependency
# table, `target.*` included.
#
# KNOWN GAPS, left open deliberately so a bypass hunt starts here rather
# than from scratch:
#  - the matcher is `serde` or a `serde_`-prefixed name, so a HYPHENATED
#    serde-family crate under a `package =` rename (`serde-value`,
#    `erased-serde`) passes. Not widened, because none of them can supply
#    `#[derive(Serialize)]` without serde itself — the rule this gate
#    exists to protect stays unbreakable.
#  - only `crates/*/Cargo.toml` is read, so a manifest nested deeper is
#    unscanned. Consistent rather than silently zero: the scan-target
#    guard counts at the same depth, so a workspace that moved its
#    crates fails the guard instead of reporting a hollow OK.
#
# WHAT THIS DOES NOT PROVE: that no kernel TYPE is persisted. Types
# defined in the kernel are persisted every day — the document layer
# describes their bytes itself, in `#[serde(with)]` modules that live
# above the boundary (`editor-core`'s `persist::kernel_wire`). That is
# the sanctioned shape and this gate is blind to it by design: it
# polices the DEPENDENCY EDGE, nothing else. Nor does it see a
# transitive edge — a kernel crate could reach serde through a
# non-kernel dependency — which no workspace crate does today and
# which `cargo tree` would be the tool for.
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
  GATE_SCAN_NOUN="crate manifest"
  # `find` on a missing `crates/` exits non-zero, and under `pipefail`
  # that killed the gate AT THIS ASSIGNMENT — the diagnosis below was
  # unreachable, and nothing could see it because no self-test fixture
  # ever handed the gate a tree without one. S157, found by the empty-
  # tree case `lib.sh` now plants for every gate.
  if [ ! -d crates ]; then
    gate_error "$(gate_name): no crates/ under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=$(find crates -mindepth 2 -maxdepth 2 -name Cargo.toml | wc -l)
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no crates/*/Cargo.toml under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
}

gate() {
  require_crate_manifests
  local hits
  # A parse failure or a missing `tomllib` must FAIL, never pass quiet:
  # a gate that cannot read its subject has not cleared it. The `if`
  # is what makes that a DIAGNOSIS: the bare assignment died here under
  # errexit, so a CI reader got the reader's own sentence with no gate
  # name and no `::error::` annotation around it (S157).
  if ! hits=$(python3 - <<'PY'
import glob, sys

try:
    import tomllib
except ImportError:  # pragma: no cover - guarded, not silenced
    sys.exit("kernel-serde-free: python3 has no tomllib (needs 3.11+) — "
             "the gate cannot read its subject, which is not a pass")

ALLOWED = {"crates/editor-core", "crates/pncad-py"}
TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


def entries(table):
    """(name, resolved package name) for one dependency table."""
    for name, spec in table.items():
        pkg = spec.get("package", name) if isinstance(spec, dict) else name
        yield name, pkg


def dep_tables(doc):
    for key in TABLES:
        if isinstance(doc.get(key), dict):
            yield doc[key]
    # [target.'cfg(...)'.dependencies] and friends.
    for cfg in (doc.get("target") or {}).values():
        if isinstance(cfg, dict):
            for key in TABLES:
                if isinstance(cfg.get(key), dict):
                    yield cfg[key]


bad = []
for path in sorted(glob.glob("crates/*/Cargo.toml")):
    crate = path.rsplit("/", 1)[0]
    if crate in ALLOWED:
        continue
    try:
        with open(path, "rb") as fh:
            doc = tomllib.load(fh)
    except Exception as exc:
        sys.exit(f"kernel-serde-free: {path} does not parse ({exc}) — "
                 "the gate cannot read its subject, which is not a pass")
    for table in dep_tables(doc):
        for name, pkg in entries(table):
            if pkg == "serde" or pkg.startswith("serde_"):
                shown = name if name == pkg else f"{name} (package = \"{pkg}\")"
                bad.append(f"{path}: {shown}")

print("\n".join(sorted(set(bad))), end="")
PY
  ); then
    gate_error "$(gate_name): the manifest reader failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a kernel crate depends on serde — persistence is the document layer's job (DESIGN.md's serde row). Describe the bytes ABOVE the boundary with a \`#[serde(with)]\` module in editor-core's \`persist::kernel_wire\`, or ratify the new layering in DESIGN.md."
    exit 1
  fi
  gate_ok "no kernel crate depends on serde"
}

# The subject is manifests, not sources, so both fixtures are manifests.
gate_plant_clean() {
  mkdir -p "$1/crates/clean"
  printf '[package]\nname = "clean"\n\n[dependencies]\ngeom-core = { path = "../geom-core" }\n' \
    > "$1/crates/clean/Cargo.toml"
}

# THE THREE SPELLINGS, planted separately. The first version of this
# gate self-tested only `plant_inline` — the one spelling its regex was
# written for — so it could never have discovered that the other two
# walked past it. A self-test that plants only what the matcher was
# built for proves nothing about the matcher.
plant_inline() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[dependencies]\nserde = { workspace = true }\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_dotted() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[dependencies]\nserde.workspace = true\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_renamed() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[dependencies]\nser = { package = "serde", workspace = true }\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_subtable() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[dependencies.serde_json]\nworkspace = true\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_dev() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[dev-dependencies]\nserde.workspace = true\n' \
    > "$1/crates/planted/Cargo.toml"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the manifest reader failed"
  gate_selftest_case "a kernel crate depends on serde" plant_inline
  gate_selftest_case "a kernel crate depends on serde" plant_dotted
  gate_selftest_case "a kernel crate depends on serde" plant_renamed
  gate_selftest_case "a kernel crate depends on serde" plant_subtable
  gate_selftest_case "a kernel crate depends on serde" plant_dev
  printf '%s selftest OK: passes a clean fixture, fires on all five planted spellings, and diagnoses a reader that fails rather than dying at the assignment that captured it\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

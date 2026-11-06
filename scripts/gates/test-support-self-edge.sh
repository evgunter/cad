#!/usr/bin/env bash
# test-support-self-edge.sh — a crate that declares a `test-support`
# feature turns it on for its OWN tests, through a dev-dependency on
# itself (`<crate> = { path = ".", features = ["test-support"] }`).
#
# WHY. Items behind `#[cfg(any(test, feature = "test-support"))]` are
# visible to the crate's unit tests through `test` alone, but its
# integration tests are another crate and see them only when the
# feature is on. Under `--workspace` cargo unifies the feature in from
# any sibling whose dev-dependency names it, so the suite compiles and
# the workspace gate is green; under `-p <crate>` nothing turns it on,
# and the suite does not compile. Every per-crate invocation in the
# repo — `scripts/k_probe_sweep.sh`, nightly's probe listing,
# a lane's `cargo test -p` — is the second kind. geom-brep's suites
# stopped compiling under `-p geom-brep --features probe` the day a
# fixture door went behind the feature, and only the nightly's probe
# row could see it.
#
# THE MANIFEST IS THE ONE HOME of what a crate's tests need: no script
# spells a crate's test features beside it.
#
# WHAT THIS DOES NOT SEE: a test-only feature spelled other than
# `test-support` (the `*-testing` features are switched on by their
# consumers' edges, not the declarer's), and a self edge in a
# `[target.*.dev-dependencies]` table, which counts only on that target
# and is reported as missing.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

require_crate_manifests() {
  GATE_SCAN_NOUN="crate manifest"
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
  if ! hits=$(python3 - <<'PY'
import glob, sys

try:
    import tomllib
except ImportError:  # pragma: no cover - guarded, not silenced
    sys.exit("test-support-self-edge: python3 has no tomllib (needs 3.11+) — "
             "the gate cannot read its subject, which is not a pass")

bad = []
for path in sorted(glob.glob("crates/*/Cargo.toml")):
    try:
        with open(path, "rb") as fh:
            doc = tomllib.load(fh)
    except Exception as exc:
        sys.exit(f"test-support-self-edge: {path} does not parse ({exc}) — "
                 "the gate cannot read its subject, which is not a pass")
    if "test-support" not in (doc.get("features") or {}):
        continue
    name = (doc.get("package") or {}).get("name")
    if not name:
        sys.exit(f"test-support-self-edge: {path} declares no package name — "
                 "the gate cannot tell which edge is the crate's own")
    self_features = None
    for dep, spec in (doc.get("dev-dependencies") or {}).items():
        if not isinstance(spec, dict) or spec.get("path") != ".":
            continue
        if spec.get("package", dep) == name:
            self_features = spec.get("features") or []
    if self_features is None:
        bad.append(f"{path}: declares `test-support` and has no dev-dependency on itself")
    elif "test-support" not in self_features:
        bad.append(f"{path}: its dev-dependency on itself does not enable `test-support`")

print("\n".join(bad), end="")
PY
  ); then
    gate_error "$(gate_name): the manifest reader failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a crate's own tests cannot see its test-support items under \`cargo test -p\`: add \`<crate> = { path = \".\", features = [\"test-support\"] }\` to its [dev-dependencies]"
    exit 1
  fi
  gate_ok "every crate declaring test-support enables it for its own tests"
}

gate_plant_clean() {
  mkdir -p "$1/crates/clean"
  printf '[package]\nname = "clean"\n\n[features]\ntest-support = []\n\n[dev-dependencies]\nclean = { path = ".", features = ["test-support"] }\n' \
    > "$1/crates/clean/Cargo.toml"
}

plant_no_self_edge() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[features]\ntest-support = []\n\n[dev-dependencies]\nclean = { path = "../clean", features = ["test-support"] }\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_self_edge_without_the_feature() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[features]\ntest-support = []\nprobe = []\n\n[dev-dependencies]\nplanted = { path = ".", features = ["probe"] }\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_no_test_support() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[features]\nprobe = []\n' \
    > "$1/crates/planted/Cargo.toml"
}

plant_renamed_self_edge() {
  mkdir -p "$1/crates/planted"
  printf '[package]\nname = "planted"\n\n[features]\ntest-support = []\n\n[dev-dependencies.me]\npackage = "planted"\npath = "."\nfeatures = ["test-support"]\n' \
    > "$1/crates/planted/Cargo.toml"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "has no dev-dependency on itself" plant_no_self_edge
  gate_selftest_case "does not enable \`test-support\`" plant_self_edge_without_the_feature
  gate_selftest_passes "a crate with no test-support feature" plant_no_test_support
  gate_selftest_passes "a self edge renamed through \`package =\`" plant_renamed_self_edge
  printf '%s selftest OK: passes a crate whose self edge enables test-support (plainly or renamed) and one with no such feature; fires on a crate with no self edge and on a self edge without the feature\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

#!/usr/bin/env bash
# test-features-dev-only.sh — a test-only cargo feature is reachable
# ONLY through a dev-dependency edge.
#
# WHAT THIS PROVES: across every `Cargo.toml` in the workspace, no
# `[dependencies]`, `[build-dependencies]` or `[target.*.dependencies]`
# entry enables a test-only feature, and no ordinary feature forwards to
# one (`prod = ["topo/sweep-testing"]`). Dev-dependency edges are exempt
# — they are the sanctioned door — and so is a test-only feature
# forwarding to another test-only feature.
#
# WHY IT EXISTS. Two crates carry test-only features whose whole
# safety argument is one sentence of manifest prose: "dev-dependencies
# are the only place it is on." `topo`'s `sweep-testing` opens
# FAILURE-INJECTION doors (`PlantedDegradation`, `sweep_traces`, the pad
# override); `topo`/`sweep`'s `test-support` opens test fixtures. Prose
# is not a guard, and this one was FALSE when this gate was written:
# `crates/sweep/Cargo.toml` had `topo = { path = "../topo", features =
# ["sweep-testing"] }` under `[dependencies]`, three lines below a
# comment claiming the opposite. Because cargo unifies features across a
# build graph, that single edge turned the injectors on for every crate
# that depends on `sweep` — measured: a downstream crate compiled
# `--release` naming `topo::PlantedDegradation` and
# `topo::sweep_traces::<f64>`, and `crates/pncad/Cargo.toml` depends on
# both. The violation sat in the tree undetected with ten gates running,
# which is the argument for an eleventh.
#
# WHAT COUNTS AS TEST-ONLY: a feature named `test-support`, or one
# whose name starts with `test-` or ends with `-testing`. That is a
# NAMING CONVENTION, not a semantic test — see KNOWN GAPS.
#
# WHY TOML PARSING AND NOT GREP. `features = [...]` says nothing about
# which table it sits in, and the tables that matter
# (`[target.'cfg(unix)'.dependencies]`) nest arbitrarily deep. A grep
# would have to reconstruct the section context it just threw away;
# `tomllib` already knows it. stdlib-only python3, like
# `check-interval-cfg-additive.py`, so this rides the cheap-tripwire job
# with no new dependency.
#
# KNOWN GAPS — named rather than papered over:
#
#   1. NAMING, NOT SEMANTICS. A test-only feature called `debug-hooks`
#      or `instrumentation` is invisible here. The gate enforces the
#      convention the repo already uses; it cannot discover a feature's
#      intent. A new test feature that does not match the pattern buys
#      no protection — name it `*-testing` or `test-*`.
#   2. DEFAULT FEATURES. A dependency whose OWN `default` feature list
#      pulls in a test feature is reachable without naming it here.
#      Nothing in the workspace does this (no crate has a `default`
#      list at all), and catching it means resolving the whole feature
#      graph, which is `cargo tree -e features` territory, not a
#      tripwire's.
#   3. TRANSITIVE ENABLE THROUGH A NON-TEST NAME. `a = ["b/x"]` where
#      `b`'s `x = ["c/sweep-testing"]` is caught at `b`'s manifest (the
#      forwarding rule), so the chain is covered link by link — but
#      only while every link lives in this workspace. A path or registry
#      dependency outside `crates/` is not scanned.
#   4. IT IS A MANIFEST CHECK. It says a door is not wired open; it
#      does not say the door is closed. Whether `sweep-testing`'s items
#      are actually `#[cfg(feature = ...)]`-gated is the compiler's
#      business, and the downstream-probe rows in the PR that added
#      this gate are the evidence for it.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# Scan-target guard: a gate that found no manifests is not a pass.
gate_require_manifests() {
  local manifests=(crates/*/Cargo.toml)
  if [ ! -f "${manifests[0]}" ]; then
    gate_error "$(gate_name): no crates/*/Cargo.toml under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=${#manifests[@]}
}

gate() {
  gate_require_manifests

  local out rc=0
  # python3 emits one line per violation; a parse failure exits non-zero
  # and is reported as a gate failure rather than swallowed as "clean".
  if ! out=$(python3 - <<'PY' 2>&1
import glob, sys, tomllib

def is_test_feature(name):
    return name == "test-support" or name.startswith("test-") or name.endswith("-testing")

# Every dependency table except [dev-dependencies], at any nesting depth
# ([target.'cfg(...)'.dependencies] included).
def dep_tables(node, path=()):
    if not isinstance(node, dict):
        return
    for key, value in node.items():
        if not isinstance(value, dict):
            continue
        if key in ("dependencies", "build-dependencies"):
            yield ".".join(path + (key,)), value
        elif key == "dev-dependencies":
            continue          # the sanctioned door
        elif key == "target" or path == ("target",) or path[:1] == ("target",):
            yield from dep_tables(value, path + (key,))

violations = []
manifests = sorted(glob.glob("crates/*/Cargo.toml"))
if glob.glob("Cargo.toml"):
    manifests.append("Cargo.toml")

for manifest in manifests:
    try:
        with open(manifest, "rb") as handle:
            doc = tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        print("PARSE\t%s\t%s" % (manifest, exc))
        sys.exit(1)

    for table, deps in dep_tables(doc):
        for dep, spec in deps.items():
            if not isinstance(spec, dict):
                continue
            for feature in spec.get("features", []):
                if is_test_feature(feature):
                    violations.append(
                        "DEP\t%s\t[%s] %s\t%s" % (manifest, table, dep, feature))

    # `prod = ["topo/sweep-testing"]` reaches the same door by a second
    # route: an ordinary feature that turns a test feature on.
    for feature, entries in (doc.get("features") or {}).items():
        if is_test_feature(feature):
            continue
        for entry in entries:
            if "/" in entry and is_test_feature(entry.split("/", 1)[1]):
                violations.append(
                    "FWD\t%s\t%s\t%s" % (manifest, feature, entry))

for line in violations:
    print(line)
sys.exit(2 if violations else 0)
PY
  ); then
    if [ -z "$out" ]; then
      gate_error "$(gate_name): the manifest scan failed to run under $PWD"
      exit 1
    fi
    while IFS=$'\t' read -r kind manifest where what; do
      [ -n "$kind" ] || continue
      case "$kind" in
        PARSE)
          gate_error "$(gate_name): cannot parse $manifest — $where"
          ;;
        DEP)
          gate_error "$manifest enables the test-only feature '$what' on $where, which is NOT a dev-dependency — cargo unifies features across the build graph, so this turns '$what' on for every production dependent of this crate; move the featured edge to [dev-dependencies] (the plain dependency stays)"
          ;;
        FWD)
          gate_error "$manifest forwards the ordinary feature '$where' to the test-only feature '$what' — anything that enables '$where' in a production build reaches a test-only door; gate the forwarding feature as test-only, or drop the entry"
          ;;
      esac
      rc=1
    done <<<"$out"
    [ "$rc" -eq 0 ] && rc=1
    exit "$rc"
  fi

  gate_ok "no [dependencies]/[build-dependencies] entry enables a test-only feature (test-support, test-*, *-testing) and no ordinary feature forwards to one — dev-dependencies are the only door (manifests scanned, not builds; see KNOWN GAPS)"
}

# The subject is manifests, not `crates/*/src`, so the clean fixture is
# a miniature workspace. It plants the CORRECT shape deliberately: the
# same crate depended on twice, plain under [dependencies] and featured
# under [dev-dependencies], plus a self dev-dependency — the pattern
# `editor-core` and `topo` use. A gate that fired on that would be
# unusable.
gate_plant_clean() {
  mkdir -p "$1/crates/lib" "$1/crates/app"
  cat > "$1/crates/lib/Cargo.toml" <<'EOF'
[package]
name = "lib"

[features]
sweep-testing = []
test-support = []
interval = ["geom/interval"]

[dependencies]
geom = { path = "../geom" }

[dev-dependencies]
lib = { path = ".", features = ["sweep-testing", "test-support"] }
EOF
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[dependencies]
lib = { path = "../lib" }

[dev-dependencies]
lib = { path = "../lib", features = ["sweep-testing"] }
EOF
}

# THE LIVE VIOLATION this gate was written for: a featured edge in
# [dependencies], carrying the exact feature whose manifest comment
# three lines above claims dev-dependencies are its only home.
plant_prod_dependency() {
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[dependencies]
lib = { path = "../lib", features = ["sweep-testing"] }
EOF
}

# The same leak wearing the other name.
plant_test_support_in_prod() {
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[dependencies]
lib = { path = "../lib", features = ["test-support"] }
EOF
}

# Nesting is why this gate parses TOML: the table that matters is three
# levels down and a grep for `features = [` cannot see which one it is.
plant_target_dependency() {
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[target.'cfg(unix)'.dependencies]
lib = { path = "../lib", features = ["sweep-testing"] }
EOF
}

# A build script reaches production too.
plant_build_dependency() {
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[build-dependencies]
lib = { path = "../lib", features = ["test-support"] }
EOF
}

# The second route to the same door: an ordinary feature that turns a
# test-only one on. Nothing is in [dependencies]' feature list here.
plant_feature_forward() {
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[features]
fancy = ["lib/sweep-testing"]

[dependencies]
lib = { path = "../lib" }
EOF
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "enables the test-only feature 'sweep-testing'" plant_prod_dependency
  gate_selftest_case "enables the test-only feature 'test-support'" plant_test_support_in_prod
  gate_selftest_case "[target.cfg(unix).dependencies] lib" plant_target_dependency
  gate_selftest_case "[build-dependencies] lib" plant_build_dependency
  gate_selftest_case "forwards the ordinary feature 'fancy'" plant_feature_forward
  printf '%s selftest OK: passes a fixture that uses the correct dev-dependency shape; fires on a featured [dependencies] edge (both feature names), a [target.*.dependencies] one, a [build-dependencies] one, and an ordinary feature forwarding to a test-only one\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

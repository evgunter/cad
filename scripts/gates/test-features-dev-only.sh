#!/usr/bin/env bash
# test-features-dev-only.sh — no MANIFEST wires a test-only cargo
# feature onto a non-dev dependency edge.
#
# WHAT THIS DOES NOT COVER: a build COMMAND may enable any feature it
# likes (`--all-features`, `--features topo/test-support`). This gate
# constrains the manifests, not the invocations — the repo's own rustdoc
# gate runs `cargo doc --workspace --all-features`, which is precisely
# why both `test_support` facades carry `#[doc(hidden)]`. An earlier
# version of this header said a test-only feature was reachable "ONLY
# through a dev-dependency edge", which is an absolute a manifest parser
# cannot support and which this repo's own CI falsifies. (This is a
# different axis from KNOWN GAPS #4, which is about whether the items
# behind the feature are cfg-gated at all.)
#
# ONE home; ci.yml's "test-only features are dev-dependency-only" step
# and local-scripts/ci-local.sh's discipline row both call this file.
#
# WHAT THIS PROVES: across EVERY `Cargo.toml` in the repository — the
# kernel workspace, its root, and the excluded roots `demos/`, `tools/`
# and `interval-transcendentals/` — no non-dev dependency edge enables a
# test-only feature, and no ordinary feature forwards to one. Dev-
# dependency edges are exempt; they are the sanctioned door.
#
# WHY IT EXISTS. Two crates carry test-only features whose whole safety
# argument was one sentence of manifest prose: "dev-dependencies are the
# only place it is on." `topo`'s `sweep-testing` opens FAILURE-INJECTION
# doors (`PlantedDegradation`, `sweep_traces`, the pad override);
# `topo`/`sweep`'s `test-support` opens test fixtures. Prose is not a
# guard, and this one was FALSE when this gate was written:
# `crates/sweep/Cargo.toml` had `topo = { path = "../topo", features =
# ["sweep-testing"] }` under `[dependencies]`, three lines below a
# comment claiming the opposite. Because cargo unifies features across a
# build graph, that single edge turned the injectors on for every crate
# depending on `sweep` — measured: a downstream crate compiled
# `--release` naming `topo::PlantedDegradation` and
# `topo::sweep_traces::<f64>`, and `crates/pncad/Cargo.toml` depends on
# both. It sat there undetected while every other gate in this directory
# ran.
#
# WHAT COUNTS AS TEST-ONLY: a feature named `test-support`, or one whose
# name ends with `-testing`. The two spellings the repo actually uses,
# and a NAMING CONVENTION rather than a semantic test — see KNOWN GAPS.
#
# WHY TOML PARSING AND NOT GREP, for the reason kernel-serde-free.sh
# gives about spellings and one more of its own. A dependency entry has
# at least three spellings (`lib = { features = [...] }`,
# `lib.features = [...]`, `[dependencies.lib]`) that a parser folds into
# one shape; and `features = [...]` says nothing about WHICH table it
# sits in, while the tables that matter nest arbitrarily deep
# (`[target.'cfg(unix)'.dependencies]`, `[workspace.dependencies]`). A
# grep would have to reconstruct the section context it just discarded.
# stdlib-only python3, like check-interval-cfg-additive.py.
#
# THE ROUTES. This gate's self-test is derived from an enumeration of
# every way a feature can be switched on for a non-dev edge, NOT from
# the spellings the matcher happens to handle. That distinction is the
# point: the first version of this gate self-tested the two spellings
# its author had thought of and passed both of the following, which are
# the live leak's own semantics wearing different clothes.
#
#   R1  inline non-dev dependency ......... [dependencies] featured edge
#   R2  workspace inheritance ............. [workspace.dependencies]
#                                           featured + `workspace = true`
#   R3  platform-conditional .............. [target.*.dependencies]
#   R4  build script ...................... [build-dependencies]
#   R5  cross-crate forward ............... prod = ["dep/test-support"]
#   R6  SAME-crate forward ................ prod = ["test-support"]
#   R7  forward chain, two deep ........... a = ["b"], b = ["test-support"]
#   R8  weak / optional spellings ......... prod = ["dep?/test-support"]
#
# R2 and R6 are the two that defeated the first version. R2 was a
# traversal bug that READ as covered: the walker recursed only under the
# key `target`, so `[workspace.dependencies]` — which this repo has, at
# `Cargo.toml`, with members writing `workspace = true` — was never
# visited even though the root manifest was in the scan list. R6 was a
# false claim in the matcher AND in the pass message: the forward check
# required a `/`, so a forward to a test feature OF THE SAME CRATE was
# invisible while any consumer naming the forwarding feature reached it.
#
# WHY [workspace.dependencies] IS TREATED AS PRODUCTION. Inheritance is
# per-member: a featured entry there is inherited by every member that
# writes `workspace = true`, in whichever table that member chooses, and
# the declaration site cannot say "dev only". A member that wants the
# feature for tests adds it on its own `[dev-dependencies]` edge, which
# composes with the inherited base. So a test feature in
# `[workspace.dependencies]` is always a finding — deliberate
# strictness, not a gap.
#
# KNOWN GAPS — named rather than papered over:
#
#   1. NAMING, NOT SEMANTICS. A test-only feature called `debug-hooks`
#      or `instrumentation` is invisible here. The gate enforces the
#      convention the repo already uses; it cannot discover a feature's
#      intent. A new test feature that does not match the pattern buys
#      no protection — name it `test-support` or `*-testing`. (A
#      mis-named one is still visible at review, which is why this is
#      the gap the repo accepts.)
#
#      DELIBERATELY NARROWED: the pattern was once `test-*` as well.
#      That collides with `test-utils`, a real member of this
#      workspace — `fuzz = ["test-utils"]` is the valid slashless
#      spelling for activating an optional dependency, so a blanket
#      `test-*` turns correct code into a red. Two exact spellings are
#      also a more honest statement of the convention than a wildcard
#      that no feature in the repo actually uses.
#   2. DEFAULT FEATURES. A dependency whose OWN `default` list pulls in
#      a test feature is reachable without naming it here — though a
#      `default` that forwards to a test feature IS caught, by R5/R6, at
#      the manifest that declares it. Resolving the full feature graph
#      is `cargo tree -e features` territory, not a tripwire's.
#   3. OUT-OF-REPO LINKS. Every manifest IN this repository is scanned,
#      including the three excluded roots, so a chain is covered link by
#      link across all 23. A registry dependency's own manifest is not
#      in the tree and is not read.
#   4. IT IS A MANIFEST CHECK. It says a door is not wired open; it does
#      not say the door is closed. Whether `sweep-testing`'s items are
#      actually `#[cfg(feature = ...)]`-gated is the compiler's
#      business, and the downstream-probe rows in the PR that added this
#      gate are the evidence for it.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  local out
  # A parse failure, a missing tomllib, or a tree with no crate
  # manifests must FAIL LOUD, never pass quiet and never fail silent.
  # The `if` is what makes it a DIAGNOSIS rather than a death: python
  # writes its reason to stderr, but a bare assignment died here under
  # errexit, so a CI reader got that sentence with no gate name and no
  # `::error::` annotation around it (S157).
  if ! out=$(python3 - <<'PY'
import os, sys

try:
    import tomllib
except ImportError:  # pragma: no cover - guarded, not silenced
    sys.exit("test-features-dev-only: python3 has no tomllib (needs 3.11+) — "
             "the gate cannot read its subject, which is not a pass")


def is_test_feature(name):
    # The two sanctioned spellings, and only those. A blanket `test-*`
    # collided with `test-utils`, a real workspace member: `fuzz =
    # ["test-utils"]` is the valid slashless spelling for activating an
    # optional dependency, and reporting it as a forward to a test-only
    # FEATURE would be a false red on correct code. See KNOWN GAPS #1.
    return name == "test-support" or name.endswith("-testing")


def manifests():
    """Every Cargo.toml in the repo, target/ and .git/ pruned."""
    found = []
    for root, dirs, files in os.walk("."):
        dirs[:] = sorted(d for d in dirs if d not in ("target", ".git"))
        if "Cargo.toml" in files:
            found.append(os.path.normpath(os.path.join(root, "Cargo.toml")))
    return sorted(found)


def dep_tables(node, path=()):
    """Every non-dev dependency table cargo actually reads.

    Descent is by the places cargo puts dependency tables, NOT by key
    name anywhere in the document:

      root        -> `target` and `workspace`
      target      -> any cfg spec (`target.'cfg(unix)'.dependencies`)

    v1 recursed only under `target`, which silently dropped
    `[workspace.dependencies]` — route R2. v2 fixed that by recursing
    unconditionally, which bought R2 at the price of firing on any table
    that merely happens to be spelled `dependencies`, such as
    `[package.metadata.deb.dependencies]` — a tool's own config, not a
    cargo edge, and a red with advice that makes no sense there.
    `dev-dependencies` is skipped by key name before any descent, so it
    is never followed at any depth: it is the sanctioned door.
    """
    if not isinstance(node, dict):
        return
    for key, value in sorted(node.items()):
        if not isinstance(value, dict):
            continue
        if key == "dev-dependencies":
            continue
        if key in ("dependencies", "build-dependencies"):
            yield ".".join(path + (key,)), value
        elif path == () and key in ("target", "workspace"):
            yield from dep_tables(value, path + (key,))
        elif path == ("target",):
            yield from dep_tables(value, path + (key,))


def forwarded_feature(entry):
    """The feature an entry of a [features] list turns on, or None.

    Cargo's spellings: `feat` (same crate), `dep/feat`, `dep?/feat`
    (weak), and `dep:name` (activates an optional dependency, enabling
    no feature). Requiring a `/` — the first version's rule — made the
    same-crate form invisible: route R6.
    """
    if entry.startswith("dep:"):
        return None
    if "/" in entry:
        return entry.split("/", 1)[1]
    return entry


violations = []
paths = manifests()
crate_manifests = [p for p in paths if p.startswith("crates" + os.sep)]
if not crate_manifests:
    sys.exit("test-features-dev-only: no crates/*/Cargo.toml under %s — "
             "the gate scanned nothing, which is not a pass" % os.getcwd())

for manifest in paths:
    try:
        with open(manifest, "rb") as handle:
            doc = tomllib.load(handle)
    except Exception as exc:
        sys.exit("test-features-dev-only: %s does not parse (%s) — "
                 "the gate cannot read its subject, which is not a pass"
                 % (manifest, exc))

    for table, deps in dep_tables(doc):
        if not isinstance(deps, dict):
            continue
        for dep, spec in deps.items():
            if not isinstance(spec, dict):
                continue
            features = spec.get("features")
            if not isinstance(features, list):
                continue
            for feature in features:
                if isinstance(feature, str) and is_test_feature(feature):
                    violations.append(
                        "DEP\t%s\t[%s] %s\t%s" % (manifest, table, dep, feature))

    for feature, entries in (doc.get("features") or {}).items():
        if is_test_feature(feature) or not isinstance(entries, list):
            continue
        for entry in entries:
            if not isinstance(entry, str):
                continue
            target = forwarded_feature(entry)
            if target is not None and is_test_feature(target):
                violations.append(
                    "FWD\t%s\t%s\t%s" % (manifest, feature, entry))

print("\n".join(violations))
print("COUNT\t%d" % len(paths))
PY
  ); then
    gate_error "$(gate_name): the manifest reader failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi

  local rc=0 count=0 kind manifest where what
  while IFS=$'\t' read -r kind manifest where what; do
    case "$kind" in
      COUNT) count=$manifest ;;
      DEP)
        gate_error "$manifest enables the test-only feature '$what' on $where, which is NOT a dev-dependency — cargo unifies features across the build graph, so this turns '$what' on for every production dependent; move the featured edge to [dev-dependencies] (the plain edge stays)"
        rc=1 ;;
      FWD)
        gate_error "$manifest forwards the ordinary feature '$where' to a test-only feature (entry '$what') — anything enabling '$where' in a production build reaches a test-only door; make the forwarding feature test-only, or drop the entry"
        rc=1 ;;
      "") ;;
      *)
        gate_error "$(gate_name): unrecognised scan output '$kind' — the gate could not read its own result, which is not a pass"
        rc=1 ;;
    esac
  done <<<"$out"

  if [ "$count" -eq 0 ]; then
    gate_error "$(gate_name): the manifest scan reported no count — the gate cannot show what it looked at, which is not a pass"
    exit 1
  fi
  [ "$rc" -eq 0 ] || exit 1

  GATE_SCAN_NOUN="manifest"
  GATE_SCAN_FILES=$count
  gate_ok "no manifest wires a test-only feature (test-support, *-testing) onto a non-dev dependency edge, and no ordinary feature forwards to one — manifests parsed, NOT build commands, which may enable any feature (see the header and KNOWN GAPS)"
}

# The subject is manifests, so the clean fixture is a miniature repo.
# The NEGATIVE CONTROL is doing real work here — every shape in it is
# correct code that a careless matcher reports as a violation, and two
# of them are shapes this gate HAS reported:
#
#   - the sanctioned split: the same crate plain under [dependencies]
#     and featured under [dev-dependencies], plus a self
#     dev-dependency, plus an unfeatured [workspace.dependencies] entry
#     inherited with `workspace = true` (what `editor-core` and `topo`
#     do);
#   - `[package.metadata.deb.dependencies]` carrying a test feature — a
#     packaging tool's own config, not a cargo edge. v2 fired on it;
#   - `fuzz = ["test-utils"]`, the valid slashless spelling for
#     activating an optional dependency that happens to be named like a
#     test feature. The `test-*` pattern fired on it.
gate_plant_clean() {
  mkdir -p "$1/crates/lib" "$1/crates/app"
  cat > "$1/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/lib", "crates/app"]

[workspace.dependencies]
lib = { path = "crates/lib" }
EOF
  cat > "$1/crates/lib/Cargo.toml" <<'EOF'
[package]
name = "lib"

[features]
sweep-testing = []
test-support = []
interval = ["geom/interval"]
default = ["interval"]
fuzz = ["test-utils"]

[dependencies]
geom = { path = "../geom" }
test-utils = { path = "../test-utils", optional = true }

[dev-dependencies]
lib = { path = ".", features = ["sweep-testing", "test-support"] }
EOF
  cat > "$1/crates/app/Cargo.toml" <<'EOF'
[package]
name = "app"

[package.metadata.deb.dependencies]
lib = { features = ["sweep-testing"] }

[dependencies]
lib = { workspace = true }

[dev-dependencies]
lib = { workspace = true, features = ["sweep-testing"] }
EOF
}

app_manifest() { cat > "$1/crates/app/Cargo.toml"; }

# R1 — THE LIVE VIOLATION this gate was written for: a featured edge in
# [dependencies], carrying the exact feature whose manifest comment
# three lines above claimed dev-dependencies were its only home.
plant_r1_inline() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[dependencies]
lib = { path = "../lib", features = ["sweep-testing"] }
EOF
}

# R2 — inherited, not written inline. The featured edge is declared once
# at the root and every member picks it up with `workspace = true`. The
# first version of this gate never traversed here.
plant_r2_workspace_inherit() {
  cat > "$1/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/lib", "crates/app"]

[workspace.dependencies]
lib = { path = "crates/lib", features = ["sweep-testing"] }
EOF
}

# R3 — the table is three levels down; a grep for `features = [` cannot
# see which one it is.
plant_r3_target() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[target.'cfg(unix)'.dependencies]
lib = { path = "../lib", features = ["sweep-testing"] }
EOF
}

# R4 — a build script reaches production too.
plant_r4_build() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[build-dependencies]
lib = { path = "../lib", features = ["test-support"] }
EOF
}

# R5 — no dependency table names the feature at all; an ordinary feature
# forwards to another crate's test-only one.
plant_r5_cross_forward() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[features]
fancy = ["lib/sweep-testing"]

[dependencies]
lib = { path = "../lib" }
EOF
}

# R6 — the same forward WITHOUT a slash: a test-only feature of the same
# crate. Invisible to the first version, and its pass message claimed
# otherwise.
plant_r6_same_crate_forward() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[features]
test-support = []
interval = ["test-support"]

[dependencies]
lib = { path = "../lib" }
EOF
}

# R7 — two links deep. Caught at the second link, which is what makes
# "covered link by link" a claim rather than a hope.
plant_r7_chain() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[features]
outer = ["middle"]
middle = ["lib/test-support"]

[dependencies]
lib = { path = "../lib" }
EOF
}

# R8 — the weak-dependency spelling `dep?/feat`.
plant_r8_weak_forward() {
  app_manifest "$1" <<'EOF'
[package]
name = "app"

[features]
fancy = ["lib?/sweep-testing"]

[dependencies]
lib = { path = "../lib", optional = true }
EOF
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "the manifest reader failed"
  # THE TWO GUARDS ON THE GATE'S OWN RESULT, and neither can be reached
  # by planting a manifest: the reader below emits `DEP`, `FWD` and
  # `COUNT` and nothing else, and it refuses a tree with no
  # `crates/*/Cargo.toml` before it can reach a zero count. So the
  # fixture is a reader that SUCCEEDS and says something this gate
  # cannot read -- which is exactly what these two guards claim to
  # catch, and what a rewrite of that heredoc would produce.
  gate_selftest_with_broken_tool python3 "unrecognised scan output" \
    "printf 'NOPE\\tone\\ttwo\\tthree\\nCOUNT\\t2\\n'"
  gate_selftest_with_broken_tool python3 "the manifest scan reported no count" \
    "exit 0"
  gate_selftest_case "[dependencies] lib" plant_r1_inline
  gate_selftest_case "[workspace.dependencies] lib" plant_r2_workspace_inherit
  gate_selftest_case "[target.cfg(unix).dependencies] lib" plant_r3_target
  gate_selftest_case "[build-dependencies] lib" plant_r4_build
  gate_selftest_case "forwards the ordinary feature 'fancy'" plant_r5_cross_forward
  gate_selftest_case "forwards the ordinary feature 'interval'" plant_r6_same_crate_forward
  gate_selftest_case "forwards the ordinary feature 'middle'" plant_r7_chain
  gate_selftest_case "entry 'lib?/sweep-testing'" plant_r8_weak_forward
  printf '%s selftest OK: passes a clean fixture carrying the sanctioned dev-dependency and workspace-inheritance shapes, a [package.metadata.*.dependencies] table, and an optional-dependency activation named like a test feature; fires on all eight routes (R1 inline, R2 workspace inheritance, R3 target.*, R4 build-dependencies, R5 cross-crate forward, R6 same-crate forward, R7 two-deep chain, R8 weak forward), and diagnoses a reader that fails rather than dying at the assignment that captured it, a reader that SUCCEEDS with an unreadable record, and one that succeeds without reporting a count\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

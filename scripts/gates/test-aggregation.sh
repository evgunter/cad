#!/usr/bin/env bash
# test-aggregation.sh — one integration-test binary per crate.
#
# THE INVARIANT: every workspace member declares AT MOST ONE `[[test]]`
# target. Suites live as `tests/*.rs` files aggregated into a single
# `tests/all.rs` via `#[path]`, with `autotests = false` in the member's
# Cargo.toml.
#
# WHY IT IS A GATE AND NOT A CONVENTION: each extra test target is a
# whole codegen+link of the ~40-rlib graph, re-monomorphizing the
# kernel's generic `T: Real` code into its own compilation unit. The
# bazel-verdict measurement put 494 of the 514 s of the CI build job
# (96%) in exactly those per-binary actions, ~1.9 s each. #179 collapsed
# 249 targets to 12 on that evidence — and `step-import` was MISSED,
# silently keeping 26 targets, two thirds of every test target the
# workspace then had, until the 2026-08-10 build-contention investigation
# found it. A convention that is checked by nobody decays exactly that
# way, so it is checked here. Those two figures are the record of that
# event and not claims about today; what this gate asserts is the
# INVARIANT, and it prints the current total rather than pinning one.
#
# The complementary half lives in each crate's `tests/all.rs`:
# `every_suite_file_is_aggregated` fails when a member HAS opted in but
# a new `tests/*.rs` file was not `#[path]`-declared. That test cannot
# fire for a member that never opted in at all — which is the hole this
# script closes. Both halves are needed; neither subsumes the other.
#
# WHY IT LIVES UNDER `scripts/gates/` rather than beside its siblings in
# `scripts/`: `scripts/gates/` is not a naming convention, it is the set
# of checks under `lib.sh`'s two-mode contract — pointable at a fixture
# root, with a `--selftest` that must pass a clean one and fire on a
# planted one — and it is the set both halves of CI run without a
# hand-written list (`gate-roster.sh` for the hosted half, the loop over
# this directory for the local one). This check meets that contract: its
# subject is `cargo metadata`, so its fixture root is a miniature
# dependency-free workspace rather than a source tree, which resolves
# offline in milliseconds. Living here is what buys it a `--selftest`
# that has been shown to fire, a roster gate that fails if `ci.yml` stops
# wiring it, and a local half that reaches it through the directory loop
# rather than through a row hand-synced against this one.
#
# Adding a suite: put the file in `tests/` and add its `#[path]` line to
# that crate's `tests/all.rs`. Never add a second `[[test]]`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN="workspace member"

gate() {
  local meta detail rc err

  # A GATE THAT SCANNED NOTHING IS NOT A PASS. `--root` makes an empty
  # or non-cargo tree reachable, and `cargo metadata` in one reports no
  # members at all, which reads exactly like a clean workspace.
  if [ ! -f Cargo.toml ]; then
    gate_error "$(gate_name): no Cargo.toml under $PWD — the gate has no workspace to read, which is not a pass"
    exit 1
  fi
  # STDERR STAYS OUT OF `meta`. Folding it in corrupts the SUCCESS path,
  # not just the failure one: cargo's own progress and rustup's toolchain
  # chatter go to stderr, and a JSON reader handed them fails on a
  # perfectly good workspace. That is what a first hosted run of this
  # gate did, on a runner that had to fetch the pinned toolchain.
  err=$(mktemp)
  if ! meta=$(cargo metadata --no-deps --format-version 1 2>"$err"); then
    gate_error "$(gate_name): cargo metadata failed under $PWD, so the target counts are unknown; that is not a pass"
    cat "$err" >&2
    rm -f "$err"
    exit 1
  fi
  rm -f "$err"

  # Exit 3 is the VIOLATION, and nothing else is: any other non-zero is
  # the reader itself failing, which must not be reported as a clean
  # workspace nor as a violated one.
  detail=$(printf '%s' "$meta" | python3 -c '
import json, sys

meta = json.load(sys.stdin)
packages = meta["packages"]
offenders = []
for pkg in packages:
    tests = [t for t in pkg["targets"] if "test" in t["kind"]]
    if len(tests) > 1:
        offenders.append((pkg["name"], sorted(t["name"] for t in tests)))

if offenders:
    for name, targets in sorted(offenders):
        print(f"  {name}: {len(targets)} test targets: {targets}")
    print(
        "\nEach extra test target is a full codegen+link of the whole rlib graph "
        "(~96% of the build job, measured). Set `autotests = false` in the crate\x27s "
        "Cargo.toml, aggregate every tests/*.rs into tests/all.rs via #[path], and "
        "declare one [[test]] named \"all\" — see crates/topo/tests/all.rs for the "
        "pattern, including the every_suite_file_is_aggregated guard to copy."
    )
    sys.exit(3)

targets = sum(1 for p in packages for t in p["targets"] if "test" in t["kind"])
print(len(packages), targets)
') && rc=0 || rc=$?
  if [ "$rc" -eq 3 ]; then
    gate_error "$(gate_name): test-aggregation discipline VIOLATED — one [[test]] target per crate"
    printf '%s\n' "$detail" >&2
    exit 1
  fi
  if [ "$rc" -ne 0 ]; then
    gate_error "$(gate_name): could not read the target counts out of cargo metadata under $PWD; that is not a pass"
    printf '%s\n' "$detail" >&2
    exit 1
  fi

  if [ "${detail%% *}" -eq 0 ]; then
    gate_error "$(gate_name): the workspace under $PWD has no members — the gate counted nothing, which is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=${detail%% *}
  gate_ok "at most one [[test]] target per member (${detail##* } test targets in all)"
}

# The fixture is a miniature CARGO WORKSPACE, not a source tree: this
# gate's subject is `cargo metadata`. It carries no dependencies, so the
# resolve is offline and instant, and it copies the repo's pinned
# toolchain so the fixture pass and the real pass use one compiler.
gate_plant_clean() {
  local t=$1
  printf '[workspace]\nmembers = ["crates/*"]\nresolver = "2"\n' > "$t/Cargo.toml"
  if [ -f "$GATE_REPO_ROOT/rust-toolchain.toml" ]; then
    cp "$GATE_REPO_ROOT/rust-toolchain.toml" "$t/rust-toolchain.toml"
  fi
  mkdir -p "$t/crates/aggregated/src" "$t/crates/aggregated/tests"
  printf '[package]\nname = "aggregated"\nversion = "0.0.0"\nedition = "2021"\nautotests = false\n\n[[test]]\nname = "all"\npath = "tests/all.rs"\n' \
    > "$t/crates/aggregated/Cargo.toml"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$t/crates/aggregated/src/lib.rs"
  printf '#[path = "one.rs"]\nmod one;\n' > "$t/crates/aggregated/tests/all.rs"
  printf '#[test]\nfn t() {}\n' > "$t/crates/aggregated/tests/one.rs"
}

# A member that never opted in: cargo autodiscovers one [[test]] per
# file, which is the shape #179 left in `step-import` for months.
plant_second_test_target() {
  local t=$1
  mkdir -p "$t/crates/loose/src" "$t/crates/loose/tests"
  printf '[package]\nname = "loose"\nversion = "0.0.0"\nedition = "2021"\n' \
    > "$t/crates/loose/Cargo.toml"
  printf 'pub fn f() {}\n' > "$t/crates/loose/src/lib.rs"
  printf '#[test]\nfn a() {}\n' > "$t/crates/loose/tests/a.rs"
  printf '#[test]\nfn b() {}\n' > "$t/crates/loose/tests/b.rs"
}

# The scan-target guard, proved rather than asserted: a root with no
# manifest at all must fail rather than report a clean workspace.
plant_no_manifest() { rm -f "$1/Cargo.toml"; }

# CARGO'S STDERR IS NOT ITS OUTPUT, and the clean fixture cannot show
# that on a machine where cargo happens to be quiet — a shim ahead of the
# real one on PATH makes it deterministic. The chatter modelled here is
# rustup fetching the pinned toolchain, which is exactly what a runner
# does on a cold job and what a local run never sees.
selftest_stderr_noise() {
  local tmp bin out
  tmp=$(mktemp -d)
  bin=$(mktemp -d)
  gate_plant_clean "$tmp"
  {
    printf '#!/usr/bin/env bash\n'
    printf 'echo "info: downloading component" >&2\n'
    printf 'exec %s "$@"\n' "$(command -v cargo)"
  } > "$bin/cargo"
  chmod +x "$bin/cargo"
  if ! out=$(PATH="$bin:$PATH" "$0" --root "$tmp" 2>&1); then
    rm -rf "$tmp" "$bin"
    printf 'SELFTEST FAILED: the gate FAILED on a clean fixture when cargo wrote to stderr\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp" "$bin"
}

# TWO GUARDS WHOSE SUBJECT IS CARGO'S ANSWER, NOT A WORKSPACE. Exit 3
# means "a member declares two [[test]] targets"; anything else non-zero
# means the reader itself failed, and the whole point of separating them
# is that the second must not be reported as the first. Neither reply is
# constructible from a manifest — cargo will not emit unparseable JSON,
# and it refuses a workspace with no members outright — so a shim ahead
# of cargo on PATH is the only way to show these two fire at all.
selftest_cargo_reply() {
  local want=$1 reply=$2 tmp bin out
  tmp=$(mktemp -d)
  bin=$(mktemp -d)
  gate_plant_clean "$tmp"
  {
    printf '#!/usr/bin/env bash\n'
    printf 'if [ "${1:-}" = metadata ]; then printf %s "%s"; exit 0; fi\n' '%s' "$reply"
    printf 'exec %s "$@"\n' "$(command -v cargo)"
  } > "$bin/cargo"
  chmod +x "$bin/cargo"
  if out=$(PATH="$bin:$PATH" "$0" --root "$tmp" 2>&1); then
    rm -rf "$tmp" "$bin"
    printf 'SELFTEST FAILED: the gate PASSED on a cargo reply it cannot use (%s)\n%s\n' "$reply" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp" "$bin"
  gate_selftest_assert_diagnosed "$reply" "$out"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED: expected %s, got:\n%s\n' "$want" "$out" >&2; exit 1 ;;
  esac
}

gate_selftest() {
  gate_selftest_clean
  selftest_stderr_noise
  gate_selftest_case 'discipline VIOLATED' plant_second_test_target
  gate_selftest_case 'no Cargo.toml under' plant_no_manifest
  selftest_cargo_reply 'could not read the target counts' 'not json at all'
  selftest_cargo_reply 'has no members' '{\"packages\": []}'
  # A cargo that REPLIES badly and a cargo that FAILS are two cases; the
  # harness above covers only the first, and the second is the one that
  # dies at the assignment with the reader's diagnosis to write.
  gate_selftest_without_tool cargo 'cargo metadata failed under'
  printf '%s selftest OK: passes a workspace whose members each declare one [[test]], and one whose cargo writes to stderr; fires on a member with two test targets, a root with no manifest, a cargo reply it cannot parse, a reply naming no members, and a cargo that cannot run at all\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

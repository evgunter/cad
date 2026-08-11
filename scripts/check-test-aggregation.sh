#!/usr/bin/env bash
# check-test-aggregation.sh — one integration-test binary per crate.
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
# silently keeping 26 targets, two thirds of every remaining test target
# in the workspace, until the 2026-08-10 build-contention investigation
# found it. A convention that is checked by nobody decays exactly that
# way, so it is checked here.
#
# The complementary half lives in each crate's `tests/all.rs`:
# `every_suite_file_is_aggregated` fails when a member HAS opted in but
# a new `tests/*.rs` file was not `#[path]`-declared. That test cannot
# fire for a member that never opted in at all — which is the hole this
# script closes. Both halves are needed; neither subsumes the other.
#
# Adding a suite: put the file in `tests/` and add its `#[path]` line to
# that crate's `tests/all.rs`. Never add a second `[[test]]`.
set -euo pipefail

cd "$(dirname "$0")/.."

cargo metadata --no-deps --format-version 1 | python3 -c '
import json, sys

meta = json.load(sys.stdin)
offenders = []
for pkg in meta["packages"]:
    tests = [t for t in pkg["targets"] if "test" in t["kind"]]
    if len(tests) > 1:
        offenders.append((pkg["name"], sorted(t["name"] for t in tests)))

if offenders:
    print("test-aggregation discipline VIOLATED — one [[test]] target per crate:\n")
    for name, targets in sorted(offenders):
        print(f"  {name}: {len(targets)} test targets: {targets}")
    print(
        "\nEach extra test target is a full codegen+link of the whole rlib graph "
        "(~96% of the build job, measured). Set `autotests = false` in the crate\x27s "
        "Cargo.toml, aggregate every tests/*.rs into tests/all.rs via #[path], and "
        "declare one [[test]] named \"all\" — see crates/topo/tests/all.rs for the "
        "pattern, including the every_suite_file_is_aggregated guard to copy."
    )
    sys.exit(1)

n = sum(1 for p in meta["packages"] for t in p["targets"] if "test" in t["kind"])
members = len(meta["packages"])
print(f"test-aggregation discipline OK: {members} members, {n} test targets")
'

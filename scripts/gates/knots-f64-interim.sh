#!/usr/bin/env bash
# knots-f64-interim.sh — C6's interim `f64` knot vector: its callers,
# counted by crate, may only go down.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (C6 in crates/geom-brep/README.md, and the INLANE
# orchestrator's ruling on its staging, 2026-10-18). A carrier's knots
# are held in its scalar, and at an enclosure scalar a knot may be wider
# than a point. Its structure exposes no position. Where every knot is a
# point, the `f64` vector read off them is still reachable, through four
# named interim accessors that refuse loud on a wide carrier:
#
#   NurbsCurve{2,3}::knots_f64_interim
#   NurbsSurface::knots_u_f64_interim / knots_v_f64_interim
#   Knots::structure_f64_interim
#   (and geom_core::spline::Span::knots_f64_interim)
#
# Every caller is a reader that has not yet moved to the held knots or
# the combinatorial structure. C6's slices S2–S5 move them; S5 deletes
# the accessors and this gate with them. So the count is a RATCHET: the
# baseline file below holds each crate's count, a crate above its
# baseline fails (a new reader past the funnel), and a crate below it
# fails too, asking for the baseline to be lowered — otherwise the slack
# would let a reader come back unseen.
#
# WHAT IS COUNTED: every call `.NAME(` of the four names in every `.rs`
# under `crates/*/` and `demos/*/` (tests included: a test that reads
# the f64 vector is a reader too), less the accessors' own `fn`
# declarations. A path call (`Type::knots_f64_interim(x)`) is counted by
# the same `knots_f64_interim(` text.
#
# KNOWN GAP: a call spelled through a macro argument or a closure
# `|c| c.knots_f64_interim()` is still the text and is counted; a call
# through a function pointer `Curve::knots_f64_interim` with no `(` is
# not. None exists.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

BASELINE=scripts/gates/knots-f64-interim.counts

gate() {
  gate_require_file "$BASELINE"
  gate_require_crate_sources
  local report
  if ! report=$(python3 - "$BASELINE" <<'PY'
import glob, re, sys

baseline_path = sys.argv[1]
NAMES = r"(?:knots_f64_interim|knots_u_f64_interim|knots_v_f64_interim|structure_f64_interim)"
CALL = re.compile(r"\b" + NAMES + r"\s*\(")
DECL = re.compile(r"\bfn\s+" + NAMES + r"\b")

counts = {}
for path in sorted(glob.glob("crates/*/**/*.rs", recursive=True)
                   + glob.glob("demos/*/**/*.rs", recursive=True)):
    crate = "/".join(path.split("/")[:2])
    with open(path, encoding="utf-8") as fh:
        text = fh.read()
    n = len(CALL.findall(text)) - len(DECL.findall(text))
    if n:
        counts[crate] = counts.get(crate, 0) + n

base = {}
with open(baseline_path, encoding="utf-8") as fh:
    for line in fh:
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        crate, n = line.split()
        base[crate] = int(n)

rose, fell = [], []
for crate in sorted(set(counts) | set(base)):
    now, was = counts.get(crate, 0), base.get(crate, 0)
    if now > was:
        rose.append(f"{crate}: {was} -> {now}")
    elif now < was:
        fell.append(f"{crate}: {was} -> {now}")
total = sum(counts.values())
for r in rose:
    print("ROSE " + r)
for f in fell:
    print("FELL " + f)
print(f"TOTAL {total}")
PY
  ); then
    gate_error "$(gate_name): the counter failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if grep -q '^ROSE ' <<<"$report"; then
    grep '^ROSE ' <<<"$report"
    gate_error "C6 interim f64 knot vector callers rose — a new reader takes a carrier's f64 knots past the funnel. Read the held knots (\`knot_values\`) or the combinatorial structure (\`structure()\`), not the interim accessor"
    exit 1
  fi
  if grep -q '^FELL ' <<<"$report"; then
    grep '^FELL ' <<<"$report"
    gate_error "C6 interim f64 knot vector callers fell — lower $BASELINE to the new counts, so the ratchet holds them there"
    exit 1
  fi
  gate_ok "C6 interim f64 knot vector callers at their baseline ($(sed -n 's/^TOTAL //p' <<<"$report") in all)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# crate count\n' > "$1/$BASELINE"
}

plant_new_caller() {
  printf 'pub fn read(c: &C) -> usize { c.knots_f64_interim().knots().len() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_lowered_unrecorded() {
  printf 'crates/clean 2\n' >> "$1/$BASELINE"
  printf 'pub fn read(c: &C) -> usize { c.knots_u_f64_interim().len() }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

plant_at_baseline() {
  printf 'crates/clean 1\n' >> "$1/$BASELINE"
  printf 'pub fn read(k: &K) -> usize { k.structure_f64_interim().len() }\n' \
    >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn knots_f64_interim(&self) -> &V { &self.v }\n' \
    >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "callers rose" plant_new_caller
  gate_selftest_case "callers fell" plant_lowered_unrecorded
  gate_selftest_passes "a caller at its baseline beside the accessor's declaration" plant_at_baseline
  printf '%s selftest OK: passes a clean fixture and one at its baseline, fires on a rise and on an unrecorded fall\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

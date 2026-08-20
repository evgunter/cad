#!/usr/bin/env bash
# bounds-allowlist.sh — the compound `Bounds` bound allowlist
# (ratified 2026-07-29). ONE home; ci.yml's "Bounds compound-bound
# allowlist (ratified 2026-07-29)" step and local-scripts/ci-local.sh's
# discipline row both call this file.
#
# Compound Bounds bounds (`T: Decide + Bounds`) are legal ONLY in
# the seams the 2026-07-29 amendment ratified (geom-core
# `real.rs`, Bounds scope rule): the C10 boolean-sweep driver
# lane in topo, the evaluation-service seam in editor-core,
# profile's pre-amendment fillet gate, and topo/props.rs — the
# M5 PR 11 certified-quadrature plumbing (Evan's lane-split
# ruling at that PR: certification is the CERTIFYING lanes'
# business, statically split from the dual lane through
# `PropsQuadLane`), and — since M5 PR 12 — the fillet-battery
# seam in sweep. That last one is an ORCHESTRATOR ruling
# (2026-08-03) applying the PR 11 precedent, flagged for
# retroactive Evan review per the self-merge convention: the
# battery's margins are certified metric quantities (sup-κ
# curvature hulls, blend setback bounds) reported as `f64`
# payloads, i.e. enclosure consumers of exactly the quadrature's
# class.
#
# STANDING OBLIGATION (D1 ruling, 2026-08-19): the fillet seam is
# the one allowlisted seam with NO refusing lane behind it, and
# the guard that made that acceptable — `Bounds` having no `Dual`
# impl — has lapsed. The argument, the reachability check and what
# is owed live in ONE home: geom-core/src/real.rs, the M5 PR 12
# entry of the `Bounds` scope rule. Not restated here; keep this a
# pointer.
# geom-brep/src/{ssi.rs,ssi/certify.rs,pcurve_cache.rs} is the
# M6-2 SSI generic-T lift: the rung-3 certificate simultaneously
# DECIDES (its `ssi_*` funnel margins) and reads brackets into the
# C9 ring (its hull and tube limbs ARE ring enclosures), so
# `Decide + Bounds` is its honest signature — the same class as
# the quadrature seam, and unlike the fillet seam its refusing
# side is NOT empty: `PcurveFittedLane` splits f64/Probe/Interval
# (certified) from `Dual` (typed refusal — since the D1 ruling a
# dual DOES carry a bracket, the value channel's; what it may not
# do is certify, which is `CertifiedEnclosure`'s job to refuse).
# `ssi/enclose.rs` is deliberately absent, and still decides nothing —
# it holds both BRACKET doors (stored endpoints, plus the fallible
# certified bracket that refuses below `Decoration::Def`) and no
# `Decide`. That pair is spelled `geom_core::CertifiedBounds`, so the
# file takes a SOLE bound and is outside this check's class rather than
# carved out of it. `Decide + CertifiedBounds` is a compound bound again
# and DOES fire here, which is right: that is a parameter that decides
# AND brackets. The matcher below sees that spelling in both operand
# orders and the self-test plants both.
# geom-brep/src/edge_nurbs.rs is M7-8's plane × NURBS edge lane,
# the narrowest possible extension of that same seam: it DELEGATES
# to the already-listed `certify_rung3` door with a declared
# carrier instead of a marched one, inheriting the door's
# signature rather than widening anything. Its split is written in
# the same shape — `EdgeNurbsLane` splits f64/Probe/Interval
# (certified) from `Dual` (typed refusal) — and it is what keeps
# `Bounds` out of `topo`'s signatures.
# topo/src/chart_region.rs is M9-2 PR-1's chart-region overlap
# predicate (spec item 1; the PR 11 class, retroactive Evan
# review per the self-merge convention): it simultaneously
# DECIDES (its chart_region_* funnel margins) and reads
# exact-f64 STRUCTURE through the bracket — the C6 planar-trim
# inventory gate (a Harmonic trig channel is straight only when
# its bracket is a point at exactly 0.0, the props.rs
# rectangle-trim read) and the bit-identical-region fast path —
# so `Decide + Bounds` is its honest signature. The
# PropsQuadLane-shape static lane that PR-1 owed its first
# Decide-generic consumer (M9-2 PR-2's census arms) EXISTS:
# `topo::chart_region::ChartRegionLane`, with a refusing `Dual`
# impl. (This paragraph used to say "no dual lane exists to split
# today (Bounds has no Dual impl, so the predicate is
# uninstantiable at duals)". Both clauses are false: the lane
# exists, and since the D1 ruling of 2026-08-19 `Bounds` IS
# implemented for `Dual`.)
#
# SCOPE OF THE GUARD, since it is easy to overstate: the lane is
# the whole guard on the CENSUS path, and only there. Alone among
# the four lanes' doors this one's bound carries no
# CertifiedEnclosure, and `chart_region_overlap` is `pub` and
# re-exported from topo/src/lib.rs — so an EXTERNAL caller is
# bounded by `Decide + Bounds`, which a dual satisfies, and the
# lane is never consulted. See the real.rs rule entry.
# Sole-bound `T: Bounds`
# (certification/driver code) is untouched by this check. A NEW
# file writing a compound Bounds bound fails here until it is
# ratified into the real.rs rule AND this allowlist.
# profile/src/path/arc_fillet.rs is the LIB-G2 PATHS arc-carrier
# fillet boundary (ruling LB3, 2026-08-08): the algebra forbids
# authoring a fillet's corner, so it DERIVES 0/1/2 corners from the
# two carriers and the S8 choice is over (corner, candidate) pairs —
# it decides (the carrier-meet and angular advance/reach gates) and
# reads the selection channel in one function, which is
# `Decide + Bounds` honestly, carrying sugar.rs's ratified
# justification verbatim: a representation-level choice between
# already-classified constructions, never a re-decision of geometry.
# The LITERAL spelling is confined to this one file; the BOUND is not,
# and the difference is KNOWN GAP 3 below. fillet_select.rs is NOT listed
# (sole-bound `T: Bounds`).
# `geom_core::CertifiedBounds`'s own two DEFINITION lines — the trait
# declaration and its blanket impl, in geom-core/src/real.rs — are
# skipped by name. A name's definition is not a use of it, and the pair
# has to be written literally exactly once for the alias to exist. This
# skips two lines, not the file: any other compound bound in real.rs
# still fires, and the self-test PROVES it does — a third planted case
# writes real.rs carrying both skipped definition lines and an ordinary
# compound signature below them, and requires the gate to fire. Widening
# the skip to a file-wide entry makes that case fail.
# WHAT THE MATCHER MATCHES, and why it is shaped by NAME rather than by a
# list of names. It fires on any identifier ending in `Bounds` sitting on
# either side of a `+`, through any path prefix: `Decide + Bounds`,
# `Bounds + Decide`, `Decide + CertifiedBounds`,
# `geom_core::CertifiedBounds + Decide`. Order-insensitivity was #676's
# fix for one half-blindness; the ALIAS half is the same defect one name
# later, because an enumerating matcher is blind to the next alias the day
# it is written -- which is how `CertifiedBounds` stayed invisible while
# the paragraph above asserted it fired. `\w*Bounds` takes the reverse
# trade knowingly: a future trait whose name ends in `Bounds` but which is
# NOT a bracket door will fire and have to be ratified here. That is the
# conservative direction, and the answer to it is a ratification line,
# never a narrowing of this regex back to a list of known names.
# A SOLE bound -- `T: CertifiedBounds`, `T: Bounds` -- is outside the
# class and does NOT fire; the self-test plants that as a negative case,
# since a matcher that fired on it would red every certification file in
# geom-brep and geom.
# KNOWN GAP 1: the match is line-based, so a bound broken across lines —
# `T: Bounds` ending one line and `+ Foo` beginning the next — is
# invisible to it. Stated rather than left to be discovered; closing it
# needs a parser, not a grep.
#
# KNOWN GAP 2, and the ONE sanctioned use of it (D1, 2026-08-19):
# an EQUIVALENT bound spelled through a supertrait obligation is
# invisible too. `geom-core/src/dual.rs` writes
#
#     impl<T> Bounds for Dual<T> where Self: Real, T: Bounds
#
# and, because `impl<T: KinkJacobian> Real for Dual<T>`, that is
# semantically `impl<T: Bounds + KinkJacobian> Bounds for Dual<T>` — a
# compound bound in a file this allowlist does not name. Written in the
# equivalent form it FIRES (planted below, so the evasion is a pinned
# fact rather than a claim). On the RULE's own words the impl is fine:
# `KinkJacobian` is neither an evaluation nor a decision parameter, so
# the pairing this gate exists to catch — a parameter that DECIDES and
# has also been handed bracket extraction — is not what is written
# there. So this is the sanctioned spelling of that one impl, declared
# here rather than left to read as "satisfied by construction": it is
# satisfied by the RULE, and it evades the GREP. A second use of the
# supertrait spelling to dodge this gate is a violation; ratify it here
# first, exactly as a file entry would be.
#
# KNOWN GAP 3: a compound bound given a NAME is invisible at its USE
# sites. `profile/src/path/arc_fillet.rs:593` declares
#
#     pub trait ArcCarrierScalar: Decide + Bounds {}
#
# The DECLARATION fires (it writes `Decide + Bounds` literally) and is
# ratified by this file's arc_fillet.rs entry. Every USE of the name is a
# `Decide + Bounds` bound that this matcher cannot see, and there are ~49
# of them across profile/src/path/{family,program}.rs, with the name
# re-exported from profile/src/path.rs, profile/src/lib.rs and
# pncad/src/profile.rs -- so the pair is `pub` from the top-level crate.
# This is KNOWN GAP 2's family (a bound reached through a supertrait
# obligation), and closing it by grep would mean redding those two files
# and allowlisting them, which is the confession this gate exists to avoid
# rather than a disposition. Recorded as S124 of
# docs/SMELL-SCAN-2026-08.md and owned by the row that decides the alias's
# final bound (S87's `ArcCarrierScalar` conversion), because whether these
# sites are legal depends on what the alias is bound to -- not on whether
# a regex can spell it.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  local hits
  hits=$(grep -rnE '(\+\s*(\w+::)*\w*Bounds\b)|(\b(\w+::)*\w*Bounds\s*\+)' crates/*/src \
    | grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | grep -vE ':[0-9]+:(pub )?trait CertifiedBounds:' \
    | grep -vE ':[0-9]+:impl<T: Bounds \+ CertifiedEnclosure> CertifiedBounds for T ' \
    | cut -d: -f1 | sort -u \
    | grep -vE '^crates/topo/src/boolean/(boxes|mod|ops|reduce|rest)\.rs$' \
    | grep -vE '^crates/topo/src/separation\.rs$' \
    | grep -vE '^crates/topo/src/props\.rs$' \
    | grep -vE '^crates/topo/src/chart_region\.rs$' \
    | grep -vE '^crates/editor-core/src/eval/(mod|wire)\.rs$' \
    | grep -vE '^crates/profile/src/path/arc_fillet\.rs$' \
    | grep -vE '^crates/geom-brep/src/(pcurve_cache|ssi|ssi/certify|edge_nurbs)\.rs$' \
    | grep -vE '^crates/sweep/src/fillet/(battery|build|surgery)\.rs$' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "compound Bounds bound outside the ratified seams above — see geom-core/src/real.rs (Bounds scope rule); ratify before allowlisting"
    exit 1
  fi
  gate_ok "no compound Bounds bound outside the ratified seams"
}

# The two operand orders are SEPARATE cases, planted one at a time: a
# single fixture carrying both would still fire if only one spelling
# matched, which is exactly the blindness being guarded against.
plant_decide_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + Bounds>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_bounds_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Bounds + Decide>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The definition skip is NARROW, and this is what holds it narrow. The
# fixture is real.rs itself, carrying BOTH the alias's two skipped
# definition lines AND an ordinary compound bound in a signature below
# them: the gate must still fire. Without this case the header's claim
# that the skip costs only two lines is prose, and a future widening of
# those `grep -v` filters — someone loosening them for a reformat — would
# blind the gate to the very file that defines the rule with nothing
# going red.
# KNOWN GAP 2's positive half: the equivalent spelling of dual.rs's
# `where Self: Real` + sole `T: Bounds` impl. The gate MUST fire on it,
# which is what makes the header's "the supertrait spelling evades the
# grep" a measured fact rather than an assertion — this case goes red the
# day someone widens the filters enough to stop catching the written form.
plant_dual_equivalent_spelling() {
  mkdir -p "$1/crates/planted/src"
  printf 'impl<T: Bounds + KinkJacobian> Bounds for Dual<T> {}\n' > "$1/crates/planted/src/lib.rs"
}

# The ALIAS cases, and the reason they are planted one spelling at a time
# for the same reason the two operand orders are: the gate was blind to
# BOTH `Decide + CertifiedBounds` orders while its own header asserted it
# fired on them, and a fixture carrying both would still pass with only
# one spelling matched.
plant_certified_decide_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + CertifiedBounds>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_certified_bounds_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: geom_core::CertifiedBounds + Decide>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The property that stops this being S56/S59 a third time: the matcher is
# shaped by the NAME, so an alias that does not exist yet is already
# covered. This case goes red the day someone narrows `\w*Bounds` back to
# an enumeration of the names in the tree today.
plant_unknown_alias() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + RingBounds>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The NEAR MISS, and the case that keeps the widening honest. A SOLE
# bracket bound is outside this gate's class by construction; a matcher
# that fired on it would red geom-brep/src/ssi/enclose.rs, geom/src/net.rs
# and both geom nurbs files, and the cheap way green would be to allowlist
# them -- which is how a gate stops guarding the case it was written for.
plant_sole_bracket_bounds() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn a<T: CertifiedBounds>(_t: T) {}\n'
    printf 'pub fn b<T: geom_core::CertifiedBounds>(_t: T) {}\n'
    printf 'pub fn c<T: Bounds>(_t: T) {}\n'
    printf 'pub struct S<T: CertifiedBounds, P: ControlPoint<T>>(T, P);\n'
  } > "$1/crates/planted/src/lib.rs"
}

# gate_selftest_case's negative twin: the fixture must PASS. lib.sh has no
# such helper -- its only passing fixture is the empty clean tree, which
# proves nothing about a spelling that must not fire. Local until the
# shared file can take it.
selftest_passes() {
  local what=$1; shift
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  "$@" "$tmp"
  if ! out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate FIRED on %s, which is not a violation\n%s\n' "$what" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

plant_real_rs_signature() {
  mkdir -p "$1/crates/geom-core/src"
  {
    printf 'pub trait CertifiedBounds: Bounds + CertifiedEnclosure {}\n'
    printf 'impl<T: Bounds + CertifiedEnclosure> CertifiedBounds for T {}\n'
    printf 'pub fn planted<T: Decide + Bounds>(_t: T) {}\n'
  } > "$1/crates/geom-core/src/real.rs"
}

gate_selftest() {
  local want="compound Bounds bound outside the ratified seams"
  gate_selftest_clean
  gate_selftest_case "$want" plant_decide_first
  gate_selftest_case "$want" plant_bounds_first
  gate_selftest_case "$want" plant_certified_decide_first
  gate_selftest_case "$want" plant_certified_bounds_first
  gate_selftest_case "$want" plant_unknown_alias
  gate_selftest_case "$want" plant_real_rs_signature
  gate_selftest_case "$want" plant_dual_equivalent_spelling
  selftest_passes "a sole bracket bound" plant_sole_bracket_bounds
  printf '%s selftest OK: passes a clean fixture and a sole bracket bound, fires on both operand orders of `Decide + Bounds` and of `Decide + CertifiedBounds`, fires on an alias name not in the tree today, fires on a compound bound in real.rs beside the skipped definition lines, and fires on the equivalent spelling of dual.rs Bounds impl (KNOWN GAP 2)\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main "compound Bounds bound outside the ratified seams" plant_decide_first

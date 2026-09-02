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
# `crates/verbs/src/run.rs` is on the list for a different reason from
# every seam above: it DECIDES NOTHING and READS NO BRACKET. It is the
# verb vocabulary's dispatch site (one file, one run door per declared
# arity), and its bound is the delegated
# kernel doors' own (the blend pair's and, since the boolean's
# migration, `topo::boolean_op_with`'s — the same three-term bound on
# each), satisfied so the call type-checks. SEAT-4's entry in the
# scope rule carries the necessity argument (the weakest bound that
# works, with the tighter one shown breaking its dual-instantiated
# caller); this is a pointer, not a restatement.
#
# The fillet seam is the one allowlisted seam with NO refusing lane
# behind it; the written reason it needs none is the DELEGATION RULE
# (DUAL-DESIGN DL5) recorded in ONE home: geom-core/src/real.rs, the
# M5 PR 12 entry of the `Bounds` scope rule. Not restated here; keep
# this a pointer.
#
# `Enclosure` is grepped exactly as `Bounds` (DUAL-DESIGN DL4, the
# issue-701 hole): the blanket `impl<T: Bounds> Enclosure for T`
# makes every `Dual` an `Enclosure`, so an `…Enclosure`-named term in
# a compound bound is the same class of decide-and-bracket parameter
# and shares this file's allowlist. KNOW THE SCOPE CONSEQUENCE: the
# allowlist is per-FILE, so every file ratified for its `Bounds`
# compounds is thereby exempt for `Enclosure` compounds too — a new
# `Decide + Enclosure` inside an allowlisted file rides that file's
# ratification and never fires here. `CertifiedBounds`'s definition
# lines stay skipped as exact text, as below.
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
# so a compound bound is its honest signature. The bound is
# `Decide + CertifiedBounds`, still compound and still firing here
# correctly.
#
# The door's own bound now excludes `Dual`, and `ChartRegionLane`'s
# refusing impl is still not redundant with that. WHY both are needed
# has ONE home: geom-core/src/real.rs, the M9-2 entry of the `Bounds`
# scope rule. Not restated here; keep this a pointer.
# editor-core/src/checks.rs is the advisory-check registry, the
# SECOND production caller of topo::separation (ratified by Evan
# 2026-08-29). Its bound is `Decide + CertifiedBounds` — TIGHTENED,
# per the M9-2 entry's discriminator ("nothing generic calls this
# door"), which is what the real.rs rule actually prescribes here;
# the `separation` entry's "passes keep their lanes" does not
# apply, its caller being a mixed pass beneath evaluate<T> and
# run_checks being beneath nothing.
#
# THE RULING ALSO SAYS WHAT THIS GATE IS FOR, and it binds every
# future row here: the gate avoids the dangerous pattern WHEN NOT
# NECESSARY, so a necessary one is fine. What a candidate owes is
# therefore a demonstration of necessity — that the bound cannot be
# avoided — not a resemblance to a seam already listed.
#
# NECESSITY IS A FILTER, NOT A LICENCE. A candidate that needs the
# bracket in order to DECIDE something outside the trilean is
# refused rather than weighed, however necessary: brackets never
# decide, every topology-determining branch stays a Decide call
# site, and boxes only ever prune. That is the thing this grep
# exists to catch, and it is checked FIRST — a necessity argument
# for a deciding read is an argument for a different design.
#
# AND A NECESSITY ARGUMENT MUST NAME THE WEAKEST BOUND THAT WORKS,
# showing the next tighter one FAILING. This row's first draft did
# not, argued for `Decide + Bounds`, and was refuted by a reviewer
# compiling `Decide + CertifiedBounds` — which works, because
# nothing generic calls run_checks. The row now carries the tighter
# bound. An argument that a bound SUFFICES is not the argument this
# rule asks for.
#
# That ordering, the two negative results that carried this row,
# and what a future row owes instead of citing them, have ONE home:
# geom-core/src/real.rs, the 2026-08-29 entry. Pointer only.
#
# A NEW file writing a compound Bounds bound fails here until it
# is ratified into the real.rs rule AND this allowlist.
# profile/src/path/arc_fillet.rs is the LIB-G2 PATHS arc-carrier
# fillet boundary (ruling LB3, 2026-08-08): the algebra forbids
# authoring a fillet's corner, so it DERIVES 0/1/2 corners from the
# two carriers and the S8 choice is over (corner, candidate) pairs —
# it decides (the carrier-meet and angular advance/reach gates) and
# reads the selection channel in one function, which is
# `Decide + Bounds` honestly, carrying verbatim the ratified
# justification written at profile/src/fillet_select.rs:17,68 and
# arc_fillet.rs:24: a representation-level choice between
# already-classified constructions, never a re-decision of geometry.
# The LITERAL spelling is confined to this one file; the BOUND is not,
# and the difference is KNOWN GAP 3 below. fillet_select.rs is NOT listed
# (sole-bound `T: Bounds`).
# `geom_core::CertifiedBounds`'s two DEFINITION lines are skipped as EXACT
# TEXT, not by name: a definition is not a use, but a skip keyed on the
# name would exempt `trait CertifiedBounds: Decide + Bounds +
# CertifiedEnclosure` — the one edit that makes every sole-bound site in
# the tree a decide-and-bracket parameter. Two planted cases hold it: a
# real.rs carrying both skipped lines AND an ordinary compound signature
# must fire, and a real.rs whose alias has been GIVEN `Decide` must fire.
# The second is caught by `gate_definition_skip_subject` rather than by the
# skip pattern, and the overlap is worth knowing rather than hiding: with
# the subject check in place, reverting this skip to a name anchor no
# longer reds the self-test, because the subject check refuses the same
# edit one step earlier. The skip stays exact text because it is the more
# precise statement of what is exempted; the guarantee is the check's.
# WHAT THE MATCHER MATCHES, shaped by NAME rather than by a list of names.
# Three alternatives: an identifier ending in `Bounds` or `Enclosure`
# after a `+` (path prefix allowed); one before a `+` (no prefix group and
# no `\b` -- `\w*`
# already spans a path segment and `\b` adds nothing; both verified dead by
# a tree-wide hit-set diff, and a dead regex element is removed rather than
# kept as untested reassurance); and a SINGLE-LINE trait DECLARATION whose
# supertrait or `where` list names a `…Bounds`/`…Enclosure` identifier,
# which is the
# only one that catches an alias spelled without a `+`. Each is planted
# separately below.
#
# READ THIS BESIDE THE THIRD ALTERNATIVE, NOT FORTY LINES LOWER: it is a
# PARTIAL catch, not a defence. `rustfmt --edition 2021` rewrites the
# spelling it catches into a multi-line `where` block that this matcher
# CANNOT see, so the silent form is the formatter-stable one. Seeing this
# alternative fire tells you nothing about the neighbouring form. KNOWN
# GAP 4 below has the counterexample and the reason it stays open.
#
# An enumerating matcher is blind to the next
# alias the day it is written — which is how `CertifiedBounds` stayed
# invisible while this header asserted it fired. The trade is knowing: any
# `…Bounds` IDENTIFIER fires, not only a trait (`TangentSpanBounds`,
# `FaceCutBounds`, `FaceBounds` exist and would), and a false positive is
# answered by a ratification line, never by narrowing back to a list. A
# SOLE bound does NOT fire, planted as a negative case: a matcher that
# fired on it would red every certification file in geom-brep and geom.
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
# sites. arc_fillet.rs declares `trait ArcCarrierScalar: Decide + Bounds`;
# the DECLARATION fires and is ratified by the entry above, while the ~49
# USES in profile/src/path/{family,program}.rs are not visible to any
# grep, and the name is re-exported as far as pncad. Closing it by grep
# means redding those files and allowlisting them, which is a confession
# rather than a disposition. S124 / D68 — NOT discharged by changing what
# the alias is bound to.
#
# KNOWN GAP 4, and it is OPEN WITH NO MITIGATION -- read this before
# trusting the alternative above. An alias NOT named `…Bounds` is invisible
# at its USES (`Decide + Bracket` says nothing about brackets), so the only
# possible defence is catching the DECLARATION. The third alternative
# catches the single-line spellings -- `trait Bracket: CertifiedBounds`,
# `trait Bracket: CertifiedEnclosure where Self: Bounds` -- and that is
# worth having, but it DOES NOT CLOSE THE GAP:
#
#     pub trait Bracket: CertifiedEnclosure
#     where
#         Self: Bounds,
#     {
#     }
#
# is silent, and it is the form `rustfmt --edition 2021` CONVERGES ON from
# the single-line spelling above. A hole a formatter produces out of the
# caught form is not a corner case; it is the resting state. No line-based
# matcher closes it, and widening alternative three to drop its `:`
# requirement false-positives on a trait generic over a SOLE bracket bound
# (`trait ArrivalSpec<T: CertifiedBounds>`), which is outside this gate's
# class -- so the answer is not a bigger regex.
#
# This header previously claimed a mitigation here ("the declaration writes
# the pair literally and therefore fires"). THAT WAS FALSE, and a false
# mitigation is worse than a disclosed hole because it tells the next
# author the door is shut. Retracted rather than narrowed.
#
# The real subject is bigger than aliases and is recorded as S158 / D102:
# this matcher anchors on `+`, and `+` is not how Rust expresses a compound
# bound, only one of the ways. `where T: Decide, T: Bounds` and
# `<T: Decide>(…) where T: Bounds` are plain compound bounds with no alias
# in sight and are silent too -- no live instance in an unratified file
# today, so a hole rather than a violation. Closing that is a redesign of
# what this gate matches, not a patch to this regex.
#
# KNOWN GAP 5: the comment strip is leading-`//` only, so a trailing or
# block comment or a string literal carrying the spelling fires. S63's
# false-positive class, F3/lane F-g's to close with a shared stripper;
# `\w*Bounds` grows it proportionally.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# THE SKIP'S SUBJECT, proved before the scan that depends on it. The two
# definition lines below are skipped as EXACT TEXT, which is deliberate --
# a skip keyed on the name would exempt the alias being GIVEN `Decide` --
# but exact text is brittle in the other direction: a reformat, a rename or
# a retirement makes the skip stop matching, and the tempting repairs are
# both wrong (widen the skip back to a name; allowlist real.rs). So the
# gate proves its own assumption instead of discovering it as a confusing
# red on the file that defines the rule, and says which repair is meant.
gate_definition_skip_subject() {
  local f=crates/geom-core/src/real.rs
  [ -f "$f" ] || return 0
  local want_trait='pub trait CertifiedBounds: Bounds + CertifiedEnclosure {}'
  local want_impl='impl<T: Bounds + CertifiedEnclosure> CertifiedBounds for T {}'
  if ! grep -qxF "$want_trait" "$f" || ! grep -qxF "$want_impl" "$f"; then
    gate_error "$(gate_name): the CertifiedBounds definition lines this gate skips by exact text are no longer in $f verbatim. The alias may have been reformatted, renamed, retired -- or GIVEN a decision bound, which would make every sole \`T: CertifiedBounds\` in the tree a decide-and-bracket parameter. Re-derive the two skip patterns against what real.rs now says; do NOT widen them to a name and do NOT allowlist real.rs"
    exit 1
  fi
}

gate() {
  gate_require_crate_sources
  gate_definition_skip_subject
  local hits
  hits=$(gate_grep -rnE '(\+\s*(\w+::)*\w*(Bounds|Enclosure)\b)|(\w*(Bounds|Enclosure)\s*\+)|(\btrait\s+\w+\b[^;{]*:[^;{]*\w*(Bounds|Enclosure)\b)' crates/*/src \
    | gate_grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | gate_grep -vE ':[0-9]+:pub trait CertifiedBounds: Bounds \+ CertifiedEnclosure \{\}$' \
    | gate_grep -vE ':[0-9]+:impl<T: Bounds \+ CertifiedEnclosure> CertifiedBounds for T \{\}$' \
    | cut -d: -f1 | sort -u \
    | gate_grep -vE '^crates/topo/src/boolean/(boxes|mod|ops|reduce|rest)\.rs$' \
    | gate_grep -vE '^crates/topo/src/separation\.rs$' \
    | gate_grep -vE '^crates/topo/src/props\.rs$' \
    | gate_grep -vE '^crates/topo/src/chart_region\.rs$' \
    | gate_grep -vE '^crates/editor-core/src/eval/(mod|wire)\.rs$' \
    | gate_grep -vE '^crates/editor-core/src/checks\.rs$' \
    | gate_grep -vE '^crates/profile/src/path/arc_fillet\.rs$' \
    | gate_grep -vE '^crates/geom-brep/src/(pcurve_cache|ssi|ssi/certify|edge_nurbs)\.rs$' \
    | gate_grep -vE '^crates/sweep/src/blend/(battery|build|surgery)\.rs$' \
    | gate_grep -vE '^crates/verbs/src/run\.rs$')
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "compound Bounds/Enclosure bound outside the ratified seams above — see geom-core/src/real.rs (Bounds scope rule); ratify before allowlisting"
    exit 1
  fi
  gate_ok "no compound Bounds/Enclosure bound outside the ratified seams"
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

# The path prefix AFTER the `+` is its own case, and it is the one element
# of the matcher nothing else exercises: `geom/src/curves/nurbs.rs`
# already writes `impl<T: geom_core::CertifiedBounds>`, so the qualified
# spelling beside a `Decide` is realistic, and deleting `(\w+::)*` from the
# right-hand alternative leaves every other case green.
plant_certified_path_prefixed() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + geom_core::CertifiedBounds>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The three SINGLE-LINE alias declarations the third alternative catches: a
# `+` PAIR; a SOLE supertrait (`trait Bracket: CertifiedBounds`), which
# carries both bracket doors with no `+` anywhere on the line; and the
# `where Self:` spelling. These pin what that alternative DOES. They are
# NOT a mitigation for GAP 4 -- rustfmt rewrites the third into a
# multi-line `where` block that is silent, so the gap is open. The first
# draft of this gate claimed otherwise on the strength of the pair spelling
# alone, which was S59's own defect one turn later, minted by the fix that
# closes it.
plant_non_bounds_alias_declaration() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub trait Bracket: Bounds + CertifiedEnclosure {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_sole_supertrait_alias() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub trait Bracket: CertifiedBounds {}\n'
    printf 'pub fn k<T: Decide + Bracket>(_t: T) {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_where_self_alias() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub trait Bracket where Self: CertifiedBounds {}\n' > "$1/crates/planted/src/lib.rs"
}

# The property that stops this being S56/S59 a third time: the matcher is
# shaped by the NAME, so an alias that does not exist yet is already
# covered. This case goes red the day someone narrows `\w*Bounds` back to
# an enumeration of the names in the tree today.
plant_unknown_alias() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + RingBounds>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The `Enclosure` rows (DUAL-DESIGN DL4): a `T: Enclosure` bound outside
# the allowlist must fire, in both operand orders — planted one at a
# time for the same blindness reason as the `Bounds` pair — and the
# name-shaped matcher covers an `…Enclosure` alias that does not exist
# in the tree today, exactly as it covers `RingBounds`.
plant_enclosure_decide_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + Enclosure>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_enclosure_bounds_side_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: geom_core::Enclosure + Decide>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_unknown_enclosure_alias() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + RingEnclosure>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The NEAR MISS, and the case that keeps the widening honest. A SOLE
# bracket bound is outside this gate's class by construction; a matcher
# that fired on it would red geom-brep/src/ssi/enclose.rs, geom/src/net.rs
# and both geom nurbs files, and the cheap way green would be to allowlist
# them -- which is how a gate stops guarding the case it was written for.
# Bundled rather than planted one at a time, and the asymmetry is not an
# oversight: in the must-FIRE direction a bundle passes when one spelling
# matches, so it hides blindness; in the must-NOT-fire direction any one
# line firing fails the case, so a bundle is strictly stronger.
plant_sole_bracket_bounds() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn a<T: CertifiedBounds>(_t: T) {}\n'
    printf 'pub fn b<T: geom_core::CertifiedBounds>(_t: T) {}\n'
    printf 'pub fn c<T: Bounds>(_t: T) {}\n'
    printf 'pub struct S<T: CertifiedBounds, P: ControlPoint<T>>(T, P);\n'
  } > "$1/crates/planted/src/lib.rs"
}

# The definition skip is NARROW, and these two fixtures are what hold it
# narrow. The first is real.rs carrying BOTH skipped definition lines AND
# an ordinary compound signature below them: the gate must still fire, so
# the skip costs two lines rather than the file. The second is the edit
# the skip must NOT survive -- the alias GIVEN `Decide`, which would make
# every sole `T: CertifiedBounds` in the tree a decide-and-bracket
# parameter without a single call site changing. A skip keyed on the name
# passes it silently; the exact-text skip fires.
plant_real_rs_signature() {
  mkdir -p "$1/crates/geom-core/src"
  {
    printf 'pub trait CertifiedBounds: Bounds + CertifiedEnclosure {}\n'
    printf 'impl<T: Bounds + CertifiedEnclosure> CertifiedBounds for T {}\n'
    printf 'pub fn planted<T: Decide + Bounds>(_t: T) {}\n'
  } > "$1/crates/geom-core/src/real.rs"
}

plant_real_rs_alias_redefined() {
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub trait CertifiedBounds: Decide + Bounds + CertifiedEnclosure {}\n' \
    > "$1/crates/geom-core/src/real.rs"
}

gate_selftest() {
  local want="compound Bounds/Enclosure bound outside the ratified seams"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_decide_first
  gate_selftest_case "$want" plant_bounds_first
  gate_selftest_case "$want" plant_certified_decide_first
  gate_selftest_case "$want" plant_certified_bounds_first
  gate_selftest_case "$want" plant_certified_path_prefixed
  gate_selftest_case "$want" plant_unknown_alias
  gate_selftest_case "$want" plant_enclosure_decide_first
  gate_selftest_case "$want" plant_enclosure_bounds_side_first
  gate_selftest_case "$want" plant_unknown_enclosure_alias
  gate_selftest_case "$want" plant_non_bounds_alias_declaration
  gate_selftest_case "$want" plant_sole_supertrait_alias
  gate_selftest_case "$want" plant_where_self_alias
  gate_selftest_case "$want" plant_real_rs_signature
  gate_selftest_case "no longer in crates/geom-core/src/real.rs verbatim" plant_real_rs_alias_redefined
  gate_selftest_case "$want" plant_dual_equivalent_spelling
  gate_selftest_passes "a sole bracket bound" plant_sole_bracket_bounds
  printf '%s selftest OK: passes a clean fixture and a sole bracket bound; fires on both operand orders of Decide+Bounds, of Decide+CertifiedBounds and of Decide+Enclosure, on a path-qualified alias after the plus, on Bounds- and Enclosure-shaped alias names not in the tree today, on all three spellings of a non-Bounds-named alias DECLARATION (GAP 4 mitigation: pair, sole supertrait, where-clause), on a compound bound in real.rs beside the skipped definition lines, on real.rs redefining the alias to carry Decide (through the definition-skip subject check), and on the equivalent spelling of dual.rs Bounds impl (GAP 2); and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

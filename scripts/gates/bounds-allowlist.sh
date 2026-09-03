#!/usr/bin/env bash
# bounds-allowlist.sh — the compound `Bounds`/`Enclosure` bound gate.
# ONE home for the FILE LIST; ci.yml's "Bounds compound-bound allowlist
# (ratified 2026-07-29)" step and local-scripts/ci-local.sh's discipline
# row both call this file.
#
# THE REASONS ARE NOT HERE, AND ARE NOT RESTATED HERE. The scope rule,
# the "brackets never decide" clause that is weighed BEFORE any necessity
# argument, and the necessity standard — name the WEAKEST bound that
# works and show the next tighter one FAILING, an argument that a bound
# SUFFICES not being the argument the rule asks for — are stated at
# geom-core/src/real.rs's `Bounds` scope rule. The ratified seams are one
# entry per ratification in its `bounds_allowlist` module, and every
# filter in `gate()` below cites the entry that earned it by the RULING'S
# NAME — its label, its date, or both, whichever names the entry. MOST of
# the ratifications record no date at their own home; there the label is
# the whole name, and it is what a reader greps. A name is what survives
# a move that a line number would not. A new file writing a compound
# bound fails here until it is ratified into that rule AND cited into the
# list below.
#
# NOTHING CHECKS THAT CITATION, and the ledger says the same of itself
# (`real.rs`: "New entries are appended here, and nothing checks that").
# A filter naming an entry that does not exist, or naming the wrong one,
# reds nothing here or anywhere — and neither would a filter carrying no
# comment at all. The correspondence is kept by hand, exactly like the
# entries it points at. Said out loud for the reason `real.rs` says the
# matching thing about `verbs/run.rs`'s abstention: so nobody reads this
# instrument as stronger than it is.
#
# TWO of the eleven filters cite a ratification that is NOT in that
# ledger, and the two are homed differently rather than alike.
# `crates/verbs/src/run.rs`'s SEAT-4 is in `real.rs` as well — inside the
# `Bounds` trait's OWN doc, above the `bounds_allowlist` module that
# holds the rest. `profile/src/path/arc_fillet.rs`'s LIB-G2 LB3 is not in
# `real.rs` at all: it is in that file's own module docs, beside the
# `fillet_select` rule it rests on. The axis is WHICH HOME, not
# in-or-out of one file.
#
# WHAT THIS FILE IS THE ONE HOME FOR — three properties of the
# INSTRUMENT, which no ledger entry states:
#
# 1. THE LIST IS PER FILE, AND `Enclosure` RIDES IT. WHY `Enclosure` is
#    grepped exactly as `Bounds` is DUAL-DESIGN DL4's argument and is
#    stated at the blanket `impl<T: Bounds> Enclosure for T` in
#    `real.rs`; not restated here. What has no other home is the
#    RIDE-ALONG CONSEQUENCE of putting both names against this one list:
#    a file ratified for its `Bounds` compounds is thereby exempt for
#    `Enclosure` compounds too — a new `Decide + Enclosure` inside an
#    allowlisted file rides that file's ratification and never fires
#    here. Read that per-FILE granularity as an OPEN QUESTION and not a
#    settled property of the instrument: KNOWN GAP 6.
# 2. A CRATE THE RULE NAMES IS NOT A FILTER. The 2026-07-29 amendment
#    licenses `crates/bvh/` — C10 spatial-index driver code — to write
#    `T: Decide + Bounds`, and the crate has no filter below, so the
#    compound form fires there. That absence is deliberate: bvh writes
#    a bracket bound at exactly ONE site today and it is the SOLE form
#    (`bvh/src/aabb.rs`'s `from_points`) — the form this gate must NOT
#    fire on, watched instead by geom-core/tests/bounds_census.rs's
#    roster — a filter is owed by the first file that writes the
#    compound one, and until then the red is what puts the amendment's
#    driver-code scope in front of whoever writes it. A crate-wide
#    filter would exempt files that do not exist yet.
#
#    THAT RED WOULD BE FALSE, and the cost is recorded here rather than
#    discovered: the construct it lands on is one the 2026-07-29
#    amendment ALREADY ratified, so this gate's own message — *ratify
#    before allowlisting* — would be wrong about it. What is owed there
#    is a FILTER, not a ratification. `lib.sh`'s warning applies at full
#    strength (a false red "is a nudge toward the allowlist rather than
#    the fix"), and the disposition stands anyway, with its price paid
#    in one red on the first bvh file that writes the compound form.
# 3. THE DEFINITION SKIP OVERLAPS ITS SUBJECT CHECK, and the overlap is
#    worth knowing rather than hiding: with `gate_definition_skip_subject`
#    below in place, reverting the two skipped `CertifiedBounds`
#    definition lines to a name anchor no longer reds the self-test,
#    because the subject check refuses the same edit one step earlier.
#    The skip stays exact text because it is the more precise statement
#    of what is exempted; the guarantee is the check's. Why exact text
#    and which repair is meant is at that function.
#
# WHAT THE MATCHER IS RUN OVER is `lib.sh`'s shared CODE-ONLY view, never
# the raw file. Comment text and string-literal bodies are blanked before
# the regex sees a line, so a trailing `// … Decide + Bounds …`, a
# one-line `/* … */`, and the spelling inside a `&str` are all invisible
# to it — each planted below as a must-NOT-fire case, because
# `\w*Bounds` makes this gate's false-positive surface grow with every
# alias in the tree and an unearned red is a nudge toward the allowlist
# rather than toward the fix. What that reader in turn cannot see is
# stated at `gate_rust_code` and is not restated here.
#
# WHAT THE MATCHER MATCHES, shaped by NAME rather than by a list of
# names. Three alternatives: an identifier ending in `Bounds` or
# `Enclosure` after a `+` (path prefix allowed); one before a `+` (no
# prefix group and no `\b` — `\w*` already spans a path segment and `\b`
# adds nothing; both verified dead by a tree-wide hit-set diff, and a
# dead regex element is removed rather than kept as untested
# reassurance); and a SINGLE-LINE trait DECLARATION whose supertrait or
# `where` list names a `…Bounds`/`…Enclosure` identifier, which is the
# only one that catches an alias spelled without a `+`. Each is planted
# separately below.
#
# READ THAT THIRD ALTERNATIVE BESIDE KNOWN GAP 4 BELOW, NEVER ON ITS
# OWN: it is a PARTIAL catch, not a defence. `rustfmt --edition 2021` rewrites
# the spelling it catches into a multi-line `where` block this matcher
# CANNOT see, so the silent form is the formatter-stable one. Seeing this
# alternative fire tells you nothing about the neighbouring form.
#
# The NAME shape is what keeps the matcher from going blind on the next
# alias written: any `…Bounds` IDENTIFIER fires, not only a trait
# (`TangentSpanBounds`, `FaceCutBounds`, `FaceBounds` exist and would),
# and a false positive is answered by a ratification line, never by
# narrowing back to a list. A SOLE bound does NOT fire, planted as a
# negative case: a matcher that fired on it would red every certification
# file in geom-brep and geom.
#
# KNOWN GAP 1: the match is line-based, so a bound broken across lines —
# `T: Bounds` ending one line and `+ Foo` beginning the next — is
# invisible to it. Stated rather than left to be discovered; closing it
# needs a parser, not a grep.
#
# KNOWN GAP 2, and the ONE sanctioned use of it (D1, 2026-08-19):
# an EQUIVALENT bound spelled through a supertrait obligation is
# invisible too. `geom-core/src/dual.rs` writes, in the multi-line
# `where` spelling rustfmt converges on and rendered here on one line,
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
# the DECLARATION fires and is ratified by that file's filter below, while
# the ~50 uses of the name in profile/src/path/{family,program}.rs (grep
# it; the count moves with the files) are not visible to any grep, and the
# name is re-exported as far as pncad. Closing it by grep means redding
# those files and allowlisting them, which is a confession rather than a
# disposition. S124 / D68 — NOT discharged by changing what the alias is
# bound to.
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
# class -- so the answer is not a bigger regex. Nothing here is a
# mitigation and nothing may be written as one: a claimed mitigation is
# worse than a disclosed hole, because it tells the next author the door
# is shut.
#
# The real subject is bigger than aliases and is recorded as S158 / D102:
# this matcher anchors on `+`, and `+` is not how Rust expresses a compound
# bound, only one of the ways. `where T: Decide, T: Bounds` and
# `<T: Decide>(…) where T: Bounds` are plain compound bounds with no alias
# in sight and are silent too -- no live instance in an unratified file
# today, so a hole rather than a violation. Closing that is a redesign of
# what this gate matches, not a patch to this regex.
#
# KNOWN GAP 6, and it is the LIST's gap rather than the matcher's: every
# filter below is a PATH, while the ratification each one cites is
# per-seam and often per-function. So a second, unrelated compound bound
# added to an allowlisted file inherits the first entry's ratification
# silently, and item 1's `Enclosure` ride-along is that same granularity
# seen from the other side. That is S159 / D103's class — an OPEN row,
# whose evidence is this file ("Every entry in `bounds-allowlist.sh` is a
# path") — and nothing here answers it: what a per-file list should
# become, if anything, is that row's question and not this header's.
# `interval-square-allowlist.sh` carries the same disclosure, citing the
# same row, for the same reason.
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
  # The shared CODE-ONLY view, so comment text and literal bodies never
  # reach the matcher (see the header).
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" |
    gate_grep -E '(\+\s*(\w+::)*\w*(Bounds|Enclosure)\b)|(\w*(Bounds|Enclosure)\s*\+)|(\btrait\s+\w+\b[^;{]*:[^;{]*\w*(Bounds|Enclosure)\b)' |
    # `CertifiedBounds`'s two DEFINITION lines, by exact text (see above).
    gate_grep -vE ':[0-9]+:pub trait CertifiedBounds: Bounds \+ CertifiedEnclosure \{\}$' |
    gate_grep -vE ':[0-9]+:impl<T: Bounds \+ CertifiedEnclosure> CertifiedBounds for T \{\}$' |
    cut -d: -f1 | sort -u |
    # The FILE LIST, each filter naming the `bounds_allowlist` entry that
    # ratified it. A filter with no entry to name is not a filter yet.
    # 2026-07-29 (M5 PR 8), the driver amendment: the boolean-sweep and
    # evaluation-service seams, and `separation` under the same entry.
    gate_grep -vE '^crates/topo/src/boolean/(boxes|mod|ops|reduce|rest)\.rs$' |
    gate_grep -vE '^crates/topo/src/separation\.rs$' |
    gate_grep -vE '^crates/editor-core/src/eval/(mod|wire)\.rs$' |
    # M5 PR 11, the certified-quadrature plumbing.
    gate_grep -vE '^crates/topo/src/props\.rs$' |
    # M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery.
    gate_grep -vE '^crates/sweep/src/blend/(battery|build|surgery)\.rs$' |
    # M6-2, the SSI rung-3 certificate; edge_nurbs under M7-8.
    gate_grep -vE '^crates/geom-brep/src/(pcurve_cache|ssi|ssi/certify|edge_nurbs)\.rs$' |
    # M9-2 PR-1, the chart-region overlap predicate.
    gate_grep -vE '^crates/topo/src/chart_region\.rs$' |
    # 2026-08-29, the advisory-check registry.
    gate_grep -vE '^crates/editor-core/src/checks\.rs$' |
    # 2026-09-02, the certified at-rest validator and the shell verbs.
    gate_grep -vE '^crates/topo/src/(validate|shell)\.rs$' |
    # SEAT-4, in the `Bounds` trait's own doc rather than the
    # `bounds_allowlist` ledger: the verb dispatch site, which decides
    # nothing and reads no bracket.
    gate_grep -vE '^crates/verbs/src/run\.rs$' |
    # LIB-G2's LB3 (ruled 2026-08-08), homed in the file's own module docs.
    gate_grep -vE '^crates/profile/src/path/arc_fillet\.rs$')
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

# THE THREE FORMS THE CODE-ONLY VIEW BUYS, and they are must-NOT-fire
# cases because each one FIRED before the conversion: a trailing comment
# naming the spelling, a one-line block comment around it, and the
# spelling inside a string literal. On the tree as it stands the swap
# moves no hit, so these fixtures are the only place the change is
# observable — without them the reader could be reverted to the
# leading-`//` strip and every case here would stay green.
#
# BUNDLED, and the asymmetry is the same one `plant_sole_bracket_bounds`
# states: in the must-NOT-fire direction any one line firing fails the
# case, so a bundle is strictly stronger than three separate fixtures.
plant_spelling_in_comments_and_literals() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn a<T: Bounds>(_t: T) {} // never write Decide + Bounds here\n'
    printf '/* a Decide + Bounds pairing belongs in the ratified ledger */\n'
    printf 'pub const WHY: &str = "Decide + Bounds is ratified per file";\n'
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
  gate_selftest_passes "the spelling in a trailing comment, a block comment and a string literal" \
    plant_spelling_in_comments_and_literals
  printf '%s selftest OK: passes a clean fixture and a sole bracket bound; fires on both operand orders of Decide+Bounds, of Decide+CertifiedBounds and of Decide+Enclosure, on a path-qualified alias after the plus, on Bounds- and Enclosure-shaped alias names not in the tree today, on all three spellings of a non-Bounds-named alias DECLARATION (the PARTIAL catch of GAP 4, not a mitigation for it: pair, sole supertrait, where-clause), on a compound bound in real.rs beside the skipped definition lines, on real.rs redefining the alias to carry Decide (through the definition-skip subject check), and on the equivalent spelling of dual.rs Bounds impl (GAP 2); passes the spelling written into a trailing comment, a block comment and a string literal, which the leading-`//` strip this gate carried fired on; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

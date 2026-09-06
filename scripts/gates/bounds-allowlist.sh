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
# entry in the FILE LIST below cites the ratification that earned it by
# the RULING'S NAME — its label, its date, or both, whichever names the entry. MOST of
# the ratifications record no date at their own home; there the label is
# the whole name, and it is what a reader greps. A name is what survives
# a move that a line number would not. A new file writing a compound
# bound fails here until it is ratified into that rule AND cited into the
# list below.
#
# NOTHING CHECKS THAT CITATION, and the ledger says the same of itself
# (`real.rs`: "New entries are appended here, and nothing checks that").
# An entry naming a ratification that does not exist, or naming the
# wrong one, reds nothing here or anywhere. What IS checked is only that
# the entry HAS a ruling field (`gate_allowlist_wellformed`), which is a
# check on the shape and not on the citation: the correspondence itself
# is kept by hand, exactly like the ratifications it points at. Said out loud for the reason `real.rs` says the
# matching thing about `verbs/run.rs`'s abstention: so nobody reads this
# instrument as stronger than it is.
#
# TWO of the list's rulings cite a ratification that is NOT in that
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
#    here, up to the count its entry pins. The granularity is per FILE
#    and stays there: separating one ratified bound from another needs
#    each hit's ENCLOSING ITEM, which is a parser and not a grep, so
#    what a per-file entry can hold is a COUNT. That is the settled
#    cost and its residue, written at KNOWN GAP 6.
# 2. A CRATE THE RULE NAMES IS NOT A FILTER. The 2026-07-29 amendment
#    licenses `crates/bvh/` — C10 spatial-index driver code — to write
#    `T: Decide + Bounds`, and the crate has no entry below, so the
#    compound form fires there. That absence is deliberate: bvh writes
#    a bracket bound at exactly ONE site today and it is the SOLE form
#    (`bvh/src/aabb.rs`'s `from_points`) — the form this gate must NOT
#    fire on, watched instead by geom-core/tests/bounds_census.rs's
#    roster — an entry is owed by the first file that writes the
#    compound one, and until then the red is what puts the amendment's
#    driver-code scope in front of whoever writes it. A crate-wide
#    entry would exempt files that do not exist yet.
#
#    THAT RED WOULD BE FALSE, and the cost is recorded here rather than
#    discovered: the construct it lands on is one the 2026-07-29
#    amendment ALREADY ratified, so this gate's own message — *ratify
#    before allowlisting* — would be wrong about it. What is owed there
#    is an ENTRY, not a ratification. `lib.sh`'s warning applies at full
#    strength (a false red "is a nudge toward the allowlist rather than
#    the fix"), and the disposition stands anyway, with its price paid
#    in one red on the first bvh file that writes the compound form.
# 3. THE DEFINITION SKIP IS EXACT TEXT AT ONE PATH, and the two halves
#    of that are held by two different guards. The PATH is the skip's
#    own anchor: the two `CertifiedBounds` definition lines are exempt
#    in `real.rs` and nowhere else, so the same definition written into
#    any other crate is an ordinary scan hit and a moved `real.rs` reds
#    on arrival. The TEXT is `gate_definition_skip_subject`'s: it fires
#    when the file is still there and the lines are not.
#
#    THE SKIP AND THE CHECK OVERLAP, and the overlap is worth knowing
#    rather than hiding: with the check in place, reverting the skip to
#    a name anchor no longer reds the self-test, because the check
#    refuses the same edit one step earlier. The skip stays exact text
#    because it is the more precise statement of what is exempted; that
#    guarantee is the check's. Why exact text and which repair is meant
#    is at that function.
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
# IN ITS STATEMENT RECORD SHAPE, cut at `{`, `}` and `;` with whitespace
# collapsed, so a declaration or a signature arrives with its whole
# `where` clause as ONE record whatever the line breaks. That is what
# lets the multi-line block `rustfmt --edition 2021` converges on be read
# exactly as the single-line spelling it came from, and it is why the
# definition-skip patterns below carry no `{}`: a statement ends where
# the brace begins.
#
# WHAT THE MATCHER MATCHES, shaped by NAME rather than by a list of
# names and grouped by BOUND TARGET rather than by a punctuation mark. A
# record is a hit when either of two things holds in it:
#
#   A COMPOUND BOUND — one target (a type parameter, `Self`, or an
#   applied type such as `Sym<T>`) carrying an identifier that ends in
#   `Bounds` or `Enclosure` BESIDE at least one further bound term,
#   wherever in the record the two are written. `T: Decide + Bounds`,
#   `where T: Decide, T: Bounds` and `<T: Decide>(…) where T: Bounds`
#   are one obligation spelled three ways: the reader groups every
#   constraint written for a target in the record, so all three are the
#   same hit. `+` is a spelling and never was the rule; anchoring on it
#   is what left the other two silent (S158 / D102).
#
#   A TRAIT DECLARATION whose supertrait or `where` list names a
#   `…Bounds`/`…Enclosure` identifier AT ALL, sole or not. This is NOT a
#   case of the rule above and is not folded into it: what it catches is
#   a bracket door handed to a NAME, and `trait Bracket: CertifiedBounds`
#   hands over both doors with a single term. The uses of that name are
#   then invisible (KNOWN GAP 3), which is what the alias roster
#   registers.
#
# WHY NOT "A DECISION TRAIT BESIDE A BRACKET TRAIT", which is the rule
# the class is about: this gate has never known which traits decide and
# must not start keeping a list of them — the D1-sanctioned
# `impl<T: Bounds + KinkJacobian>` is planted below as must-FIRE and
# `KinkJacobian` is neither an evaluation nor a decision parameter. A
# SECOND bound term is exactly what the `+` spelling asserted, so a
# second bound term is what the grouping asks for.
#
# A SOLE bracket bound on a parameter does NOT fire, in a generic list
# or in a `where` clause, planted as a negative case both ways: a
# matcher that fired on it would red every certification file in
# geom-brep and geom.
#
# THE MATCHER IS A READER AND NOT A REGEX, because it has to skip a
# trait's own generic list, tell `Sym<T>`'s bound from `T`'s, and follow
# bracket depth to know which `,` ends a list — none of which an ERE can
# nest. What it reads is at `gate_bound_reader` below.
#
# The NAME shape is what keeps the matcher from going blind on the next
# alias written: any `…Bounds` IDENTIFIER fires, not only a trait
# (`TangentSpanBounds`, `FaceCutBounds`, `FaceBounds` exist and would),
# and a false positive is answered by a ratification line, never by
# narrowing back to a list. A SOLE bound does NOT fire, planted as a
# negative case: a matcher that fired on it would red every certification
# file in geom-brep and geom.
#
# THE GAP NUMBERS ARE STABLE IDS, not a sequence: a gap that closes
# keeps its number for whatever remains of it, and a retired one leaves
# a hole rather than renumbering its neighbours. There is no KNOWN GAP 5
# and there will not be one; nothing is missing from the register.
#
# KNOWN GAP 1, NARROWED TO THE RECORD BOUNDARY. A bound broken across
# LINES is read: a record is a statement, so `T: Bounds` ending one line
# and `+ Foo` beginning the next arrive together. What is still
# invisible is a bound assembled across two STATEMENTS, because nothing
# here carries a target from one record into the next —
#
#     impl<T: Decide> Carrier<T> {
#         fn read(&self) where T: Bounds { … }
#     }
#
# is two records, one term each, and neither is compound on its own.
# Closing that needs a scope, which is a parser and not a grep.
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
# KNOWN GAP 3, AND THE USES STAY OPEN: a compound bound given a NAME is
# invisible at its USE sites. arc_fillet.rs declares
# `trait ArcCarrierScalar: Decide + Bounds`; the DECLARATION fires and is
# ratified by that file's entry below, while the uses of the name in
# profile/src/path/{family,program}.rs (grep it; the count moves with the
# files) are not visible to any grep, and the name is re-exported as far
# as pncad. Every one of those uses is `T: ArcCarrierScalar` — CHARACTER
# FOR CHARACTER the sole bracket bound `plant_sole_bracket_bounds` below
# pins as must-NOT-fire — so no WIDENING of the matcher above reaches
# them: it would red geom-brep/src/ssi/enclose.rs, geom/src/net.rs and
# both geom nurbs files on the same construct.
#
# WHAT DOES REACH THEM, AND IT IS NOT A NAME-RESOLVING PASS. A matcher
# keyed on the ROSTER's own name list separates the two perfectly, with
# no resolution and no second pass — the roster is exactly the ratified
# list of names, so the fact it needs is already recorded here:
#
#     gate_grep -E "[A-Za-z0-9_][[:space:]]*:[[:space:]]*(ArcCarrierScalar)\b\
#                  |\+[[:space:]]*(ArcCarrierScalar)\b"
#
# It PASSES `plant_sole_bracket_bounds` and its output on this tree is
# family.rs and program.rs and nothing else. So THE ONLY CLOSE AVAILABLE
# IS THE ONE S63 FORBIDS — redding those two files and allowlisting
# them, the cry-wolf-then-allowlist outcome recorded at linalg/mat.rs —
# and NOT "no instrument of this kind can see the sites". Read the
# narrow claim, not the wide one: an earlier draft of this paragraph
# wrote the wide one while the roster three screens down was the
# counterexample to it.
#
# THE INVISIBILITY OF THE SITES TO *THIS* MATCHER IS MEASURED RATHER
# THAN ASSERTED. The fixture `plant_alias_uses_invisible` plants the
# declaration in its ratified home and the three use shapes family.rs
# actually writes in a file that is NOT ratified, and requires this gate
# to PASS. That case reds the day the matcher is keyed on the roster's
# names, which is the day this paragraph has to be rewritten.
#
# WHAT STANDS IN ITS PLACE is the ALIAS ROSTER below, which checks the
# PREMISE the gap rests on instead of the uses it cannot reach. Read it
# there. S124 / D68 — NOT discharged by changing what the alias is bound
# to.
#
# KNOWN GAP 4, NARROWED TO A SPELLING THIS READER CANNOT PARSE. An alias
# NOT named `…Bounds` is invisible at its USES (`Decide + Bracket` says
# nothing about brackets), so the only possible defence is catching the
# DECLARATION -- and the declaration is now read whatever its line
# breaks, including the multi-line `where` block `rustfmt --edition 2021`
# converges on, which is where the caught single-line spelling comes to
# rest. That is what a record being a STATEMENT buys, and it is planted
# in both directions: `plant_rustfmt_where_block` fires, and
# `plant_alias_declaration_rustfmt_block` PASSES, which is the census
# saying it can still see a rostered name after the formatter has been
# over it.
#
# WHAT STAYS OPEN is a declaration whose NAME is not in the text:
#
#     macro_rules! carrier {
#         ($n:ident) => { pub trait $n: Decide + Bounds {} };
#     }
#     carrier!(ArcCarrierScalar);
#
# There is no `trait <name>` token to read, so the declaration test finds
# nothing and an alias is minted with the census silent -- which is why
# that is the shape `plant_alias_declaration_gone_silent` plants. Closing
# it means expanding macros, which is a compiler and not a grep. Nothing
# here is a mitigation and nothing may be written as one: a claimed
# mitigation is worse than a disclosed hole, because it tells the next
# author the door is shut.
#
# KNOWN GAP 6, NARROWED TO WHAT A COUNT CANNOT SEE. Every entry below is
# a PATH, while the ratification each one cites is per-seam and often
# per-function, so an entry cannot say WHICH bounds its file is ratified
# for — only how many. Each entry therefore PINS ITS FILE'S COUNT OF
# MATCHER OCCURRENCES, and the gate re-derives that count on every run:
# a ratified file that gains or loses one reds with the ratification
# named, so a second bound no longer inherits the first one's argument
# in silence.
#
# OCCURRENCES AND NOT RECORDS, because a record is a STATEMENT and a
# statement holds more than one bound: `fn f<T: Decide + Bounds, U:
# Decide + CertifiedBounds>` is one record and two bounds, and a pin on
# records would take the second one for free — the very ride-along this
# pin exists to refuse, arriving inside a signature instead of on a new
# one. So the reading is the number of `…Bounds`/`…Enclosure` doors
# standing in a COMPOUND GROUP on a record, or ONE where the record is
# caught only as a trait declaration. It is the matcher's own reading
# counted by the matcher itself — `gate_bound_reader`'s `count` mode —
# and not a second matcher beside it, which is what a separate regex
# for the count would be.
#
# THE READING IS NOT "HOW MANY BOUNDS", and the difference is stated
# rather than left to surprise: `T: Decide + Bounds + CertifiedEnclosure`
# is ONE parameter's bound and TWO occurrences, so a three-term seam
# counts two. That is stable and re-derived, which is all a pin needs;
# what it is NOT is a budget on bounds.
#
# What the count cannot see is the SUBSTITUTION — one ratified bound
# replaced by an unrelated one at the same count — and item 1's
# `Enclosure` ride-along arriving in PLACE of a `Bounds` compound rather
# than beside it. Separating those needs a per-SYMBOL entry, which needs
# each hit's enclosing item, which is a parser and not a grep. The same
# class on the sibling list is `interval-square-allowlist.sh`'s KNOWN
# GAP 4, and it is not closed there.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

DEFINITION_HOME=crates/geom-core/src/real.rs
DEFINITION_TRAIT='pub trait CertifiedBounds: Bounds + CertifiedEnclosure {}'
DEFINITION_IMPL='impl<T: Bounds + CertifiedEnclosure> CertifiedBounds for T {}'
# The same two lines as a STATEMENT RECORD renders them -- `{`, `}` and
# `;` are where a statement is cut, so the trailing `{}` is not part of
# the record and one space follows the line number -- and BOTH halves
# anchored: the skip applies to this text in this file and nowhere else.
DEFINITION_HOME_RE='crates/geom-core/src/real\.rs'
DEFINITION_TRAIT_RE='pub trait CertifiedBounds: Bounds \+ CertifiedEnclosure'
DEFINITION_IMPL_RE='impl<T: Bounds \+ CertifiedEnclosure> CertifiedBounds for T'

# THE SKIP'S SUBJECT, proved before the scan that depends on it. The two
# definition lines below are skipped as EXACT TEXT, which is deliberate --
# a skip keyed on the name would exempt the alias being GIVEN `Decide` --
# but exact text is brittle in the other direction: a reformat, a rename or
# a retirement makes the skip stop matching, and the tempting repairs are
# both wrong (widen the skip back to a name; allowlist real.rs). So the
# gate proves its own assumption instead of discovering it as a confusing
# red on the file that defines the rule, and says which repair is meant.
#
# IT ABSTAINS WHEN THE FILE IS GONE, and that costs nothing now: the
# skip is anchored to the same path, so a moved `real.rs` carries the
# two lines to a path the skip does not match and the SCAN reds on them.
# Before the anchor this abstention was the silent half of the failure.
gate_definition_skip_subject() {
  local f=$DEFINITION_HOME
  [ -f "$f" ] || return 0
  if ! grep -qxF "$DEFINITION_TRAIT" "$f" || ! grep -qxF "$DEFINITION_IMPL" "$f"; then
    gate_error "$(gate_name): the CertifiedBounds definition lines this gate skips by exact text are no longer in $f verbatim. The alias may have been reformatted, renamed, retired -- or GIVEN a decision bound, which would make every sole \`T: CertifiedBounds\` in the tree a decide-and-bracket parameter. Re-derive the two skip patterns against what real.rs now says; do NOT widen them to a name and do NOT allowlist real.rs"
    exit 1
  fi
}

# `CertifiedBounds`'s two DEFINITION lines, dropped by EXACT TEXT (the
# reasoning is at `gate_definition_skip_subject` above). ONE HOME for the
# two patterns, because two readers need them now: the scan below and the
# alias census. A second copy would be a second thing to keep in step with
# real.rs, which is the drift the subject check exists to catch.
#
# THE PATTERNS ARE ANCHORED TO THEIR HOME, and that is what makes the
# file MOVING loud rather than silent. Unanchored, the two lines were
# exempt wherever under `crates/*/src` they were written -- so the same
# definition copied into any crate rode the skip, and a renamed `real.rs`
# carried it along while the subject check above abstained on the file it
# could no longer find. Anchored, both halves speak: the text at the
# wrong path is an ordinary scan hit, and the text gone from the right
# path is the subject check's red. Planted as
# `plant_definition_lines_elsewhere`.
gate_definition_skip() {
  gate_grep -vE "^$DEFINITION_HOME_RE:[0-9]+: $DEFINITION_TRAIT_RE\$" |
    gate_grep -vE "^$DEFINITION_HOME_RE:[0-9]+: $DEFINITION_IMPL_RE\$"
}

# THE FILE LIST, one entry per ratified file: PATH, the COUNT of
# matcher OCCURRENCES that file carries, and the RULING that ratified
# it. An entry with no ruling to name is not an entry yet, and
# `gate_allowlist_wellformed` refuses one before any scan runs, so that
# sentence has a check behind it rather than being a convention.
#
# THE COUNT IS A READING, NOT A BUDGET, and it is re-derived on every
# run and compared with the pin (KNOWN GAP 6, where what it counts and
# what it does not are stated). The entry is per FILE and the argument
# that earned it is per SEAM, so the number is the only part of "which
# bounds are ratified here" a grep can hold: it fires when a ratified
# file gains an occurrence, and when it loses one, because an entry
# claiming a seam the file no longer carries is as stale as one
# covering a seam nobody argued. Neither direction is answered by
# editing the number — the seam is re-argued at `real.rs`'s
# `bounds_allowlist` (or at whichever home the entry's ruling names),
# and the pin moves in the change that carries the argument.
BOUNDS_ALLOWLIST=(
  # 2026-07-29 (M5 PR 8), the driver amendment: the boolean-sweep and
  # evaluation-service seams, and `separation` under the same entry.
  'crates/topo/src/boolean/boxes.rs 4 2026-07-29 (M5 PR 8), the driver amendment'
  'crates/topo/src/boolean/mod.rs 5 2026-07-29 (M5 PR 8), the driver amendment'
  'crates/topo/src/boolean/ops.rs 11 2026-07-29 (M5 PR 8), the driver amendment'
  'crates/topo/src/boolean/reduce.rs 4 2026-07-29 (M5 PR 8), the driver amendment'
  'crates/topo/src/boolean/rest.rs 1 2026-07-29 (M5 PR 8), the driver amendment'
  # `separation.rs` is FOUR and the ledger entry enumerates THREE
  # (`Separation::of`, `Separation::certify`, `image`). The fourth,
  # `SolidSeparation::of`, is argued in that struct's own doc as
  # INHERITANCE from the sibling door — a dual-admission argument, not
  # the necessity standard the ledger asks for — so its ledger row is
  # owed and the entry says so rather than letting the count read as
  # ratified by a ruling that does not mention it.
  "crates/topo/src/separation.rs 4 2026-07-29 (M5 PR 8) for three of them; the fourth (SolidSeparation::of) rests on SolidSeparation's own doc and OWES a ledger row"
  'crates/editor-core/src/eval/mod.rs 7 2026-07-29 (M5 PR 8), the driver amendment'
  'crates/editor-core/src/eval/wire.rs 15 2026-07-29 (M5 PR 8), the driver amendment'
  # M5 PR 11, the certified-quadrature plumbing.
  'crates/topo/src/props.rs 14 M5 PR 11, the certified-quadrature plumbing'
  # M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery.
  'crates/sweep/src/blend/battery.rs 15 M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery'
  'crates/sweep/src/blend/build.rs 5 M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery'
  'crates/sweep/src/blend/surgery.rs 14 M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery'
  'crates/sweep/src/blend/open/planar.rs 3 M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery'
  'crates/sweep/src/blend/open/ruled.rs 3 M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery'
  # M6-2, the SSI rung-3 certificate.
  'crates/geom-brep/src/pcurve_cache.rs 7 M6-2, the SSI rung-3 certificate'
  'crates/geom-brep/src/ssi.rs 2 M6-2, the SSI rung-3 certificate'
  'crates/geom-brep/src/ssi/certify.rs 16 M6-2, the SSI rung-3 certificate'
  # M7-8, the declare-and-check edge lane.
  'crates/geom-brep/src/edge_nurbs.rs 6 M7-8, the declare-and-check edge lane'
  # M7-8's 2026-09-02 amendment, the lane's split as a BOUND: the two
  # DOORS that name the certified body `plane_nurbs_limbs`.
  'crates/geom-brep/src/certify.rs 1 M7-8 2026-09-02, the lane split as a BOUND'
  'crates/topo/src/euler.rs 1 M7-8 2026-09-02, the lane split as a BOUND'
  # M9-2 PR-1, the chart-region overlap predicate.
  'crates/topo/src/chart_region.rs 26 M9-2 PR-1, the chart-region overlap predicate'
  # 2026-08-29, the advisory-check registry.
  'crates/editor-core/src/checks.rs 3 2026-08-29, the advisory-check registry'
  # 2026-09-02, the certified at-rest validator and the shell verbs.
  'crates/topo/src/validate.rs 9 2026-09-02, the certified at-rest validator'
  'crates/topo/src/shell.rs 2 2026-09-02, the certified at-rest validator'
  # SEAT-4, in the `Bounds` trait's own doc rather than the
  # `bounds_allowlist` ledger: the verb dispatch site, which decides
  # nothing and reads no bracket. SEAT-9 is the second header.
  'crates/verbs/src/run.rs 2 SEAT-4 with SEAT-9, in the Bounds trait doc'
  # LIB-G2's LB3 (ruled 2026-08-08), homed in the file's own module docs.
  "crates/profile/src/path/arc_fillet.rs 3 LIB-G2's LB3 (ruled 2026-08-08), in the file's module docs"
)

# The paths alone, for the SCAN's exclusion filter.
gate_allowlist_paths() {
  local entry
  for entry in "${BOUNDS_ALLOWLIST[@]}"; do
    printf '%s\n' "${entry%% *}"
  done
}

# THE LIST IS READ BEFORE IT IS USED, and a malformed entry is refused
# rather than mis-parsed. Every field is load-bearing and every one of
# them fails QUIETLY when it is wrong: a non-numeric count makes `-eq`
# die mid-run, a missing label makes the label silently become the
# count, a duplicate path double-counts its file into the OK line, and
# a path with a space shifts every field one place right. So the shape
# is checked once, before any scan, and named.
#
# TAKES THE ENTRIES AS ARGUMENTS rather than reading the array, which
# is what makes it testable at all: the self-test harness runs the gate
# as a subprocess and cannot plant a list, so the malformed cases call
# this function directly with lists the array never holds.
gate_allowlist_wellformed() {
  local entry path rest pinned label bad=""
  local -A seen=()
  for entry in "$@"; do
    case "$entry" in
      *' '*) ;;
      *) bad+="  [$entry]: not \`PATH COUNT RULING\` -- no fields after the path"$'\n'
         continue ;;
    esac
    path=${entry%% *}; rest=${entry#* }; pinned=${rest%% *}; label=${rest#* }
    if [ "$label" = "$pinned" ] || [ -z "${label// /}" ]; then
      bad+="  [$entry]: no RULING after the count -- an entry with no ruling to name is not an entry yet"$'\n'
    fi
    case "$pinned" in
      ''|*[!0-9]*|0*)
        bad+="  [$entry]: the count is not a positive decimal integer"$'\n'
        continue ;;
    esac
    if [ -n "${seen["$path"]+set}" ]; then
      bad+="  [$entry]: $path already has an entry -- one file, one ratification"$'\n'
    fi
    seen["$path"]=1
  done
  if [ -n "$bad" ]; then
    printf 'MALFORMED ALLOWLIST ENTRY:\n'
    printf '%s' "$bad"
    gate_error "$(gate_name): the allowlist entry listed above under MALFORMED ALLOWLIST ENTRY is not a usable \`PATH COUNT RULING\` row, so this gate cannot say what it exempts or what it pins -- and every one of these shapes fails silently rather than loudly if it reaches the scan. Fix the entry; do not work around it downstream"
    exit 1
  fi
}

# THE PINS, CHECKED AGAINST THE TREE. The scan above answers "is this
# bound in a ratified file"; this answers "is it one of the bounds that
# file was ratified FOR", to the only resolution a per-file list has.
# It runs LAST, and after the census, because an edit that is both a
# new alias and a moved count is the census's to diagnose: a NAME's
# uses spread past the file it is declared in, and an extra bound does
# not.
#
# WHAT IS COUNTED IS OCCURRENCES, NOT RECORDS -- the
# `…Bounds`/`…Enclosure` doors standing in a compound group on a record,
# or ONE where the record is caught only as a trait declaration. A
# record is a STATEMENT, and a statement holds more than one bound;
# KNOWN GAP 6 carries the argument and what the reading is not. The
# reader does both readings, so the count is the matcher's own and
# cannot drift from it.
gate_allowlist_counts() {
  local records=$1 entry path path_re rest pinned label recs have dir bad=""
  BOUNDS_OCCURRENCE_TOTAL=0
  for entry in "${BOUNDS_ALLOWLIST[@]}"; do
    path=${entry%% *}; rest=${entry#* }; pinned=${rest%% *}; label=${rest#* }
    path_re=${path//./\\.}
    recs=$(printf '%s\n' "$records" | gate_grep -E "^$path_re:")
    if [ -z "$recs" ]; then
      have=0
    else
      have=$(printf '%s\n' "$recs" | gate_bound_reader count |
        awk '{ s += $1 } END { print s + 0 }')
    fi
    BOUNDS_OCCURRENCE_TOTAL=$((BOUNDS_OCCURRENCE_TOTAL + have))
    if [ "$have" -eq "$pinned" ]; then continue; fi
    if [ "$have" -gt "$pinned" ]; then
      dir="MORE than the entry argues for"
    else
      dir="FEWER than the entry argues for"
    fi
    bad+="  $path: the entry pins $pinned, this tree carries $have -- $dir"
    [ -f "$path" ] || bad+="; the file is not in this tree"
    bad+=$'\n'"    ratified as: $label"$'\n'
  done
  if [ -n "$bad" ]; then
    printf 'RATIFIED FILE, COMPOUND-BOUND COUNT MOVED:\n'
    printf '%s' "$bad"
    gate_error "$(gate_name): the compound-bound occurrence count listed above under RATIFIED FILE, COMPOUND-BOUND COUNT MOVED is not the count its allowlist entry pins. An entry is per FILE and the ratification it names is per SEAM, so the count is what stands in for WHICH bounds the file was ratified for: a file that GAINED an occurrence is writing a bound nobody argued -- on a new line or beside an existing one -- and one that LOST one has an entry claiming a seam it no longer carries. Neither is repaired by editing the number. Re-argue the seam against geom-core/src/real.rs's Bounds scope rule and the ratification named beside each line above, and move the pin in the change that carries the argument"
    exit 1
  fi
}

# THE ALIAS ROSTER, AND IT IS NOT AN ALLOWLIST — BUT IT IS NOT INERT
# EITHER. Read the two apart before adding to either. A FILE FILTER above
# exempts a file from the SCAN; a roster entry changes no scan hit and
# exempts no file from anything the matcher does. What it DOES do is make
# the CENSUS below green for the name it lists, and that is an exemption
# surface for the check it ships with: the cheap way to green a new alias
# is a new roster line, which is exactly the edit
# `plant_new_alias_in_ratified_file` plants. So an entry earns its place
# by naming the ratification that licensed the name — it registers a
# trait DECLARATION that hands a `…Bounds`/`…Enclosure` door to a NAME,
# and it says one thing: this name exists, every USE of it is a bound
# this matcher cannot read (KNOWN GAP 3), and here is what licensed it.
# The blind spot is the NAME, not the file, so the register of blind
# spots is keyed by name.
#
# IT DOES NOT MAKE THE USES VISIBLE and nothing here should be read as
# claiming it does — that is D68's answer, not a step toward one. What it
# checks is the PREMISE the gap rests on, and there are two halves:
#
#   THE POPULATION IS CLOSED. A second alias declared anywhere under
#   crates/*/src fires here, INCLUDING inside a file the list above
#   ratifies — where the scan is silent, and where the new name's uses
#   would otherwise spread `pub` through the tree with nothing anywhere
#   recording that they exist. That is ONE SLIVER of the ride-along
#   KNOWN GAP 6 describes: the DECLARATION and nothing else. What
#   catches the rest of a ratified file's ride-along is the COUNT its
#   entry pins, which speaks after this check for the reason given at
#   `gate_allowlist_counts`; what neither reads is a bound SWAPPED for
#   another at the same count.
#
#   THE MITIGATION IS PROVED RATHER THAN ASSUMED. GAP 3's only defence is
#   that the declaration is written in a spelling this matcher catches and
#   sits in a ratified file. Line breaks no longer take that defence away
#   — a record is a statement — so what does is a spelling with no
#   `trait <name>` token in it at all, the macro-pasted name of GAP 4.
#   Write `ArcCarrierScalar`'s declaration that way and the defence is
#   gone with the entry still in place, the uses still spreading and
#   nothing red anywhere. The census reds instead, and names what was
#   lost.
#
# AN ENTRY WHOSE FILE IS NOT IN THE TREE IS REPORTED, and it used to be
# skipped. The skip was argued as a disposition — an alias that MOVES is
# loud through the SCAN by the other route, and one that is RETIRED
# "leaves in the change that retires it" — but nothing checked that
# second half, so the roster could accumulate dead decoration while this
# header asserted it could not: an entry naming a path that never existed
# left the gate green, on the live tree and in the self-test. The real
# reason the skip was there is smaller and is worth writing down, because
# it is the shape this file exists to catch: `gate_plant_clean` never
# wrote `arc_fillet.rs`, so without the skip EVERY fixture red on the
# roster entry and the clean case failed first. The fixture was
# load-bearing for the argument, not the other way round. The clean
# fixture below now plants the ratified declaration, which is what makes
# the check runnable, and the retirement case is planted as
# `plant_roster_file_gone`.
#
# `CertifiedBounds` IS DELIBERATELY NOT HERE. Its two definition lines are
# dropped by `gate_definition_skip`, and `gate_definition_skip_subject`
# already watches the same property for it — by exact text, in its own
# file. A second watcher would be a second thing to keep in step.
BOUNDS_ALIAS_ROSTER=(
  # LIB-G2's LB3 (ruled 2026-08-08), homed in the file's own module docs —
  # the same ratification the `arc_fillet.rs` entry above cites. The uses
  # are in profile/src/path/{family,program}.rs; they are S124 / D68, and
  # they are unchecked.
  'crates/profile/src/path/arc_fillet.rs ArcCarrierScalar'
  # 2026-07-29 (M5 PR 8), the driver amendment — the same ratification
  # the `eval/mod.rs` entry above cites, and the declaration's own doc
  # names it. The name is confined to `eval/mod.rs` and `eval/parts.rs`
  # by that doc; the uses are `T: EvalScalar` and are unchecked, exactly
  # as `ArcCarrierScalar`'s are.
  'crates/editor-core/src/eval/mod.rs EvalScalar'
)

# THE READER — the whole matcher, the count, and the census's name list,
# in one function because they are three questions about one reading of a
# record. Splitting them is what let the trait-declaration half carry a
# generic-list skip the scan half did not.
#
# TWO TESTS OVER ONE RECORD, stated at the header: a TRAIT DECLARATION
# naming a bracket door, and a COMPOUND BOUND grouped by target. Neither
# subsumes the other — a sole supertrait is a declaration and not a
# compound bound, and `where T: Decide, T: Bounds` is a compound bound
# and no declaration — so a record fires if either holds and the count
# takes the compound reading where there is one.
#
# A `:` INSIDE THE TRAIT'S OWN GENERIC LIST IS NOT A SUPERTRAIT COLON.
# `pub trait ArrivalSpec<T: CertifiedBounds>` is a SOLE bracket bound on
# a type parameter — the construct `plant_sole_bracket_bounds` pins as
# must-NOT-fire, and outside this gate's class — so the balanced `<…>`
# after the name is skipped before the colon is looked for. That skip is
# why this is an awk reader and not one more alternative in the regex
# beside it: bracket nesting is not expressible in an ERE, and the
# depth-2 spelling (`trait Carrier<T: CertifiedBounds, P: ControlPoint<T>>`)
# is the one an approximation gets wrong.
#
# THE TEST AFTER THE SKIP IS THE REGEX IT REPLACED: from the name (or
# from the end of the generic list), a `:` reached without crossing `;`
# or `{`, then a `…Bounds`/`…Enclosure` identifier reached the same way.
#
# IT IS NOT THE SAME MATCHER EITHER SIDE OF THAT, and the two directions
# it moves in are here rather than in a claim of identity. `match()` takes
# the FIRST `trait` token on a record, so a second declaration written after
# a first on ONE record (`trait A<T: Foo> {} trait B: Bounds {}`) is no
# longer read as a declaration — though `trait B`'s own supertrait list
# is a separate STATEMENT and reaches the test on its own; and a `;` or
# `{` INSIDE the skipped generic list
# (`trait Arr<T: Array<[u8; 4]>>: Bounds {}`) no longer stops the search,
# so that one is read where the regex's `[^;{]*` could not cross it.
# Both have zero population in the tree, and both move toward the answer
# the rule wants — the first construct is two declarations, of which the
# second is caught by nothing either way; the second IS a supertrait
# bound. Neither is a subset claim: the reader is a different matcher,
# narrower where it counts and wider where the regex was accidentally
# blind.
#
# MODE `records` emits the matching FILE:LINE:TEXT record, which is what
# the scan wants; `names` emits `PATH NAME` for a DECLARATION, which is
# what the alias census wants; `count` emits the record's occurrence
# count, which is what each allowlist entry's pin is compared against.
# Two readers would drift, and the drift is silent in whichever half
# nothing plants — the census carried the generic-list skip while the
# scan did not, and a count keyed on its own regex would take a bound
# the matcher stopped calling one.
gate_bound_reader() {
  awk -v MODE="$1" '
    BEGIN { Q = sprintf("%c", 39) }
    # A BRACKET DOOR IS NAMED, NOT LISTED — the property the header
    # states: any identifier ending in `Bounds` or `Enclosure`, so an
    # alias nobody has written yet is already covered.
    function bracket(t) { return t ~ /(Bounds|Enclosure)$/ }
    # NOT TRAIT TERMS, AND A NEW EXPRESSION STARTS AFTER EACH. `impl`/
    # `dyn` introduce one, `mut`/`ref`/`const`/`static` qualify a type,
    # `for` opens an HRTB, `where` opens the clause list; counting any of
    # them as a second bound would make `x: impl Bounds` — a SOLE bracket
    # bound — read as compound, and letting one sit INSIDE a bound target
    # would make two clauses of one `where` list two different targets.
    function skipword(t) {
      return t == "impl" || t == "dyn" || t == "for" || t == "where" ||
             t == "as" || t == "mut" || t == "ref" || t == "move" ||
             t == "const" || t == "static" || t == "pub" || t == "crate" ||
             t == "super" || t == "self" || t == "fn" || t == "let" ||
             t == "type" || t == "struct" || t == "enum" || t == "trait" ||
             t == "union" || t == "unsafe" || t == "extern" || t == "in" ||
             t == "use" || t == "mod" || t == "return" || t == "match"
    }
    {
      line = $0
      path = line; sub(/:[0-9]+:.*$/, "", path)
      sub(/^[^:]*:[0-9]+:/, "", line)
      # --- the trait DECLARATION, which is not the grouping question ---
      isdecl = 0; name = ""
      if (match(line, /(^|[^A-Za-z0-9_])trait[ \t]+[A-Za-z_][A-Za-z0-9_]*/)) {
        name = substr(line, RSTART, RLENGTH); sub(/^.*trait[ \t]+/, "", name)
        rest = substr(line, RSTART + RLENGTH)
        sub(/^[ \t]*/, "", rest)
        ok = 1
        if (substr(rest, 1, 1) == "<") {
          d = 0; i = 1; n = length(rest)
          while (i <= n) {
            c = substr(rest, i, 1)
            # `->` IS AN ARROW, NOT A CLOSER. A parameter bounded by an
            # `Fn(..) -> ..` closes the list one bracket early otherwise,
            # and the tail it hands back carries the REAL parameters
            # colons — which is the false positive this reader exists to
            # refuse. Taken as two characters, which is the one thing a
            # depth counter can do that a nesting-blind ERE cannot.
            if (c == "-" && substr(rest, i + 1, 1) == ">") { i += 2; continue }
            if (c == "<") d++
            else if (c == ">") { d--; if (d == 0) break }
            i++
          }
          if (d != 0) ok = 0
          else rest = substr(rest, i + 1)
        }
        if (ok && rest ~ /^[^;{]*:[^;{]*[A-Za-z0-9_]*(Bounds|Enclosure)([^A-Za-z0-9_]|$)/)
          isdecl = 1
      }
      # --- the bound groups, keyed by TARGET ---
      # One walk over the record, tracking bracket depth so that a term
      # inside a type argument (`ControlPoint<T>`) is not a bound on the
      # target and the closer that ends the list is the one at ITS depth.
      # `key` is the TYPE EXPRESSION a `:` was reached from — a type
      # parameter, `Self`, the trait being declared, or an applied type
      # like `geom_core::Sym<T>` — and every list written for that same
      # expression in this record adds to the same group, which is what
      # makes `T: A, T: B` and a generic list plus a `where` clause one
      # bound. THE WHOLE EXPRESSION AND NOT ITS LAST IDENTIFIER: three
      # `where` clauses in this tree bound `Sym<T>` and `T` in one
      # record, and keying on the trailing identifier reads a decision
      # bound on the WRAPPER as a decision bound on the parameter the
      # next clause hands brackets to. `cs[d]` is where the expression
      # at this depth began; a closed `<…>` does not end it.
      split("", brk); split("", oth); split("", cs); split("", fresh)
      n = length(line); i = 1; d = 0; key = ""; kd = -1; inret = 0
      cs[0] = 1; fresh[0] = 1
      while (i <= n) {
        c = substr(line, i, 1)
        if (c == "-" && substr(line, i + 1, 1) == ">") {
          # An `Fn(..) -> R` return type is not a bound on the target;
          # the next `+` at the list depth ends it.
          if (key != "" && d == kd) inret = 1
          i += 2; continue
        }
        if (c == Q) {                    # a lifetime: the tick and its name
          i++
          while (i <= n && substr(line, i, 1) ~ /[A-Za-z0-9_]/) i++
          continue
        }
        if (c ~ /[A-Za-z_]/) {
          j = i
          while (j <= n && substr(line, j, 1) ~ /[A-Za-z0-9_]/) j++
          tok = substr(line, i, j - i)
          if (skipword(tok)) fresh[d] = 1
          else if (fresh[d] == 1) { cs[d] = i; fresh[d] = 0 }
          i = j
          # A PATH PREFIX IS NOT THE TRAIT. `geom_core::CertifiedBounds`
          # is one term and it is the LAST segment that names it — while
          # the expression it belongs to starts at the first segment.
          if (substr(line, i, 2) == "::") { i += 2; continue }
          if (key != "" && d == kd && inret == 0 && !skipword(tok)) {
            if (bracket(tok)) brk[key]++; else oth[key]++
          }
          continue
        }
        if (c == ":") {
          if (substr(line, i + 1, 1) == ":") { i += 2; continue }
          key = substr(line, cs[d], i - cs[d]); gsub(/[ \t]/, "", key)
          kd = d; inret = 0; fresh[d] = 1; i++; continue
        }
        if (c == "<" || c == "(" || c == "[") { d++; cs[d] = i + 1; fresh[d] = 1; i++; continue }
        if (c == ">" || c == ")" || c == "]") {
          d--
          if (d < 0) { d = 0; fresh[0] = 1 }
          if (key != "" && d < kd) key = ""
          # A CLOSED `<…>` ENDS THE EXPRESSION ONLY IF NOTHING IS BOUND
          # TO IT. `geom_core::Sym<T>:` continues through its own
          # brackets; the `>` of a generic PARAMETER list, or the `)` of
          # an argument list, hands back a position where the next name
          # is a new target.
          k = i + 1
          while (substr(line, k, 1) == " ") k++
          if (!(substr(line, k, 1) == ":" && substr(line, k + 1, 1) != ":"))
            fresh[d] = 1
          i++; continue
        }
        if (c == "," || c == "=") {
          if (key != "" && d == kd) key = ""
          fresh[d] = 1; i++; continue
        }
        if (c == ";" || c == "{" || c == "}") { key = ""; fresh[d] = 1; i++; continue }
        if (c == "+") { inret = 0; fresh[d] = 1; i++; continue }
        i++
      }
      # THE COUNT IS BRACKET DOORS INSIDE A COMPOUND GROUP, which is what
      # a `+`-adjacency count read before this reader existed, and what
      # KNOWN GAP 6 pins: `T: Decide + Bounds + CertifiedEnclosure` is
      # one parameter and TWO occurrences.
      cnt = 0
      for (k in brk)
        if (brk[k] > 0 && brk[k] + oth[k] >= 2) cnt += brk[k]
      if (cnt == 0 && isdecl == 0) next
      if (MODE == "names") { if (isdecl) print path " " name; next }
      if (MODE == "count") { print (cnt > 0 ? cnt : 1); next }
      print $0
    }
  '
}

# THE MATCHER, over `lib.sh`'s shared CODE-ONLY view on stdin: the two
# `+` alternatives as one regex, the trait declaration through the
# reader above, unioned — the shape `no-extra-real-bounds.sh` uses for
# its own two matchers.
#
# THE PREFILTER IS EXACT, not a heuristic: every alternative needs the
# token `Bounds` or `Enclosure` on the line, so the two matchers only
# ever see lines that carry one and neither reads the whole tree twice.
gate_matcher() {
  gate_grep -E 'Bounds|Enclosure' | gate_bound_reader records | gate_definition_skip
}

# The census. It runs AFTER the scan, and the order is the argument: a new
# alias in an UNRATIFIED file is an ordinary scan hit and `ratify before
# allowlisting` is the message that edit needs. This check exists for the
# edits the scan lets through — a declaration going quiet where it stands,
# and a name minted inside a file the list above already ratifies.
gate_alias_roster_census() {
  local seen entry path line extra="" missing="" gone="" msg=""
  local -A declared=() rostered=()
  seen=$(gate_rust_code --statements "${GATE_SOURCE_FILES[@]}" |
    gate_grep -E 'Bounds|Enclosure' |
    gate_definition_skip |
    gate_bound_reader names)
  while IFS= read -r line; do
    if [ -n "$line" ]; then declared["$line"]=1; fi
  done <<< "$seen"
  for entry in ${BOUNDS_ALIAS_ROSTER[@]+"${BOUNDS_ALIAS_ROSTER[@]}"}; do
    path=${entry%% *}
    if [ -f "$path" ]; then rostered["$entry"]=1; else gone+="  $entry"$'\n'; fi
  done
  for line in "${!declared[@]}"; do
    if [ -z "${rostered["$line"]+set}" ]; then extra+="  $line"$'\n'; fi
  done
  for line in "${!rostered[@]}"; do
    if [ -z "${declared["$line"]+set}" ]; then missing+="  $line"$'\n'; fi
  done
  if [ -n "$gone" ]; then
    printf 'ROSTERED, FILE NOT IN THE TREE:\n'
    printf '%s' "$gone" | sort
    msg="$(gate_name): the roster entry listed above under ROSTERED, FILE NOT IN THE TREE names a path this tree does not have. A roster entry is a register of a LIVE blind spot — a name whose uses this matcher cannot read — so an entry whose declaration is gone registers nothing and this gate cannot tell dead decoration from an alias that was retired without its entry. If the alias was RETIRED, delete the entry in that same change. If it MOVED, re-anchor the entry to the new path (the declaration at the new path is an ordinary scan hit and needs a file filter with its own ratification). Do not leave the entry standing"
  fi
  if [ -n "$missing" ]; then
    printf 'ROSTERED, NO DECLARATION FOUND:\n'
    printf '%s' "$missing" | sort
    msg="${msg:+$msg
}$(gate_name): this gate can no longer see the roster entry listed above under ROSTERED, NO DECLARATION FOUND — the file is still there and its declaration is not in a spelling this matcher catches. Minted through a macro, so there is no \`trait <name>\` token to read (KNOWN GAP 4), assembled across two statements (KNOWN GAP 1), or renamed. KNOWN GAP 3's ONLY defence is that the declaration fires and its file is ratified, so that defence is now gone while every use of the name goes on spreading. Spell the declaration back, or re-derive the roster and say in the entry what stands in its place; do NOT delete the entry to make this green"
  fi
  if [ -n "$extra" ]; then
    printf 'DECLARATION FOUND, NOT ROSTERED:\n'
    printf '%s' "$extra" | sort
    msg="${msg:+$msg
}$(gate_name): the trait declaration listed above under DECLARATION FOUND, NOT ROSTERED is not on this gate's alias roster. It hands a Bounds/Enclosure door to a NAME, and every USE of that name is a bound this matcher cannot read (KNOWN GAP 3) — including uses in files no entry above names. A file entry does not cover it: the entries exempt a file from the SCAN, and this is not a scan hit. Add a roster entry naming the ratification that licenses the name, or spell the bound out at the use sites where the matcher can see it"
  fi
  if [ -n "$msg" ]; then
    gate_error "$msg"
    exit 1
  fi
}

gate() {
  gate_allowlist_wellformed "${BOUNDS_ALLOWLIST[@]}"
  gate_require_crate_sources
  gate_definition_skip_subject
  local records hits
  # The shared CODE-ONLY view in its STATEMENT record shape, so comment
  # text and literal bodies never reach the matcher and a declaration
  # arrives with its whole `where` clause whatever the line breaks (see
  # the header). Read ONCE: the scan and the per-entry count are two
  # questions about one record set.
  records=$(gate_rust_code --statements "${GATE_SOURCE_FILES[@]}" | gate_matcher)
  hits=$(printf '%s\n' "$records" | gate_grep -v '^$' |
    cut -d: -f1 | sort -u |
    gate_grep -vxF -f <(gate_allowlist_paths))
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "compound Bounds/Enclosure bound outside the ratified seams above — see geom-core/src/real.rs (Bounds scope rule); ratify before allowlisting"
    exit 1
  fi
  gate_alias_roster_census
  gate_allowlist_counts "$records"
  gate_ok "no compound Bounds/Enclosure bound outside the ratified seams, and each ratified file carries the count its entry pins (${#BOUNDS_ALLOWLIST[@]} files, $BOUNDS_OCCURRENCE_TOTAL compound-bound occurrences)"
}

# THE CLEAN FIXTURE CARRIES EVERY LIST'S OWN SUBJECT, and it has to.
# The census reports a roster entry whose file is not in the tree, and
# the count check reports an allowlist entry whose file carries a
# different number of compound bounds than it pins — so a clean tree is
# one where every rostered declaration is present and spelled the way
# the matcher reads it, and every allowlisted file carries exactly the
# count its entry names. That is what "clean" means for this gate, both
# lists being part of what it checks; without it every fixture would
# red on the lists rather than on what it plants.
#
# THE FILES ARE SYNTHESISED FROM THE ENTRIES, never spelled a second
# time. A fixture that restated the counts would be a second place they
# live and would go stale against the first; derived, the clean tree is
# a satisfying tree BY CONSTRUCTION, and what proves the pin against
# the real seams is the gate's own pass over `crates/*/src`. A rostered
# declaration REPLACES one of its file's synthesised records rather
# than adding to it, because the declaration is itself a record.
gate_plant_clean() {
  local entry path name n i decl
  local -A rostered_decl=()
  mkdir -p "$1/crates/clean/src"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/clean/src/lib.rs"
  for entry in ${BOUNDS_ALIAS_ROSTER[@]+"${BOUNDS_ALIAS_ROSTER[@]}"}; do
    path=${entry%% *}; name=${entry##* }
    rostered_decl["$path"]="pub trait $name: Decide + Bounds {}"
  done
  for entry in "${BOUNDS_ALLOWLIST[@]}"; do
    path=${entry%% *}; n=${entry#* }; n=${n%% *}
    mkdir -p "$1/${path%/*}"
    : > "$1/$path"
    decl=${rostered_decl["$path"]:-}
    if [ -n "$decl" ]; then
      printf '%s\n' "$decl" >> "$1/$path"
      unset 'rostered_decl[$path]'
      n=$((n - 1))
    fi
    for ((i = 1; i <= n; i++)); do
      printf 'pub fn seam_%d<T: Decide + Bounds>(_t: T) {}\n' "$i" >> "$1/$path"
    done
  done
  for path in ${!rostered_decl[@]+"${!rostered_decl[@]}"}; do
    mkdir -p "$1/${path%/*}"
    printf '%s\n' "${rostered_decl[$path]}" > "$1/$path"
  done
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
# day someone widens the matcher enough to stop catching the written form.
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

# The path prefix on the SECOND door is its own case, and it is the one
# element of the reader nothing else exercises: `geom/src/curves/nurbs.rs`
# already writes `impl<T: geom_core::CertifiedBounds>`, so the qualified
# spelling beside a `Decide` is realistic, and a reader that took
# `geom_core` for the trait and `CertifiedBounds` for nothing leaves
# every other case green.
plant_certified_path_prefixed() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Decide + geom_core::CertifiedBounds>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The three alias DECLARATIONS the declaration test catches on one line: a
# `+` PAIR; a SOLE supertrait (`trait Bracket: CertifiedBounds`), which
# carries both bracket doors with no `+` anywhere and no second term, so
# no grouping reaches it; and the `where Self:` spelling. The multi-line
# form of the third is `plant_rustfmt_where_block` and it fires too, so
# these are no longer the boundary of what is read -- what GAP 4 keeps is
# a declaration with no `trait <name>` token at all. The first draft of
# this gate claimed a mitigation on the strength of the pair spelling
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

# THE SPELLINGS `+` IS NOT, one fixture each and must-FIRE. Each writes
# the same obligation as `T: Decide + Bounds` with no `+` between the two
# doors: the constraints repeated on one parameter, the same split
# between the generic list and the `where` clause, and the pair broken
# across lines. Planted one at a time because a bundle in the must-FIRE
# direction passes when ONE spelling matches, which is the blindness
# being guarded against.
plant_where_clause_pair() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T>(_t: T) where T: Decide, T: Bounds {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_generic_list_and_where() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn g<T: Decide>(_t: T) where T: Bounds {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_bound_split_across_lines() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn h<T>(_t: T)\n'
    printf 'where\n'
    printf '    T: Bounds\n'
    printf '        + Decide,\n'
    printf '{\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# THE FORM `rustfmt --edition 2021` CONVERGES ON from the single-line
# `where Self:` declaration above, and the reason a record is a STATEMENT
# rather than a line: this is the resting state of the caught spelling,
# so a matcher blind to it is blind to what the formatter leaves behind.
# THE SUPERTRAIT IS `Decide` AND THAT IS THE POINT: with a bracket-named
# supertrait on the first line the declaration is caught by that line
# alone, so the case would pass on a reader that never sees the `where`
# block and prove nothing about it.
plant_rustfmt_where_block() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub trait Bracket: Decide\n'
    printf 'where\n'
    printf '    Self: Bounds,\n'
    printf '{\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# A WRAPPER TYPE'S OWN COMPOUND BOUND, must-FIRE, and the twin of the
# near miss below: `Sym<T>` is the target here and it both decides and
# reads brackets, so the grouping that refuses the near miss must not
# refuse this.
plant_wrapper_type_compound() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'impl<T> PropsQuadLane for geom_core::Sym<T>\n'
    printf 'where\n'
    printf '    geom_core::Sym<T>: Decide + geom_core::Bounds,\n'
    printf '{\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
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
# The four TRAIT forms are the same bound on a trait's own type
# parameter, and they are here because the declaration test read that
# parameter's colon as the supertrait colon and fired. Each names one
# thing the skip has to get right: a bare list, a `<…>` nested inside it
# (the depth a regex approximation gets wrong), and an `Fn(..) -> ..`
# parameter in BOTH orders — the arrow's `>` closes the list early for a
# counter that does not know it, and only the order with the certified
# parameter AFTER the arrow shows it.
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
    printf 'pub trait ArrivalSpec<T: CertifiedBounds> {}\n'
    printf 'pub trait Carrier<T: CertifiedBounds, P: ControlPoint<T>> {}\n'
    printf 'pub trait Arrow<F: Fn(u8) -> u8, T: CertifiedBounds> {}\n'
    printf 'pub trait Arrow2<T: CertifiedBounds, F: Fn(u8) -> u8> {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# THE NEAR MISSES THE GROUPING ITSELF OWES, must-NOT-fire and bundled
# for the reason above: any one line firing fails the case. A SOLE
# bracket bound is outside the class wherever it is written, and the
# `where` clause is where the widening could most easily forget that;
# two DIFFERENT parameters bounded one each are two bounds and not one
# compound; and a wrapper type bounded beside its own parameter is the
# shape three ratified files write today — `Sym<T>` decides, `T` is
# handed brackets, and reading the trailing identifier of the first
# target instead of the whole expression collapses them into one
# parameter that does both.
plant_where_clause_near_misses() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn a<T>(_t: T) where T: CertifiedBounds {}\n'
    printf 'pub fn b<T, U>(_t: T, _u: U) where T: Decide, U: Bounds {}\n'
    printf 'impl<T> ChartRegionLane for geom_core::Sym<T>\n'
    printf 'where\n'
    printf '    geom_core::Sym<T>: Decide,\n'
    printf '    T: geom_core::CertifiedBounds,\n'
    printf '{\n'
    printf '}\n'
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

# THE SKIP IS ANCHORED, and this case is what holds it anchored: the two
# definition lines at a path that is not their home are not exempt, and
# an unanchored skip would exempt them. It is also the case that makes
# `real.rs` MOVING loud -- a moved file IS this fixture, and there the
# subject check abstains on the file it cannot find, so the skip is the
# only guard left.
plant_definition_lines_elsewhere() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '%s\n' "$DEFINITION_TRAIT"
    printf '%s\n' "$DEFINITION_IMPL"
  } > "$1/crates/planted/src/lib.rs"
}

# THE ANCHOR'S POSITIVE DIRECTION, and without it the self-test only ever
# proved the skip does not apply. `real.rs` carrying the two definition
# lines AND NOTHING ELSE must PASS: over-narrow the anchor -- a path that
# never matches, or a `_RE` twin whose escaping drifted from the text
# beside it -- and the two lines the rule DEFINES become a red on the
# file that defines it. Its sibling `plant_real_rs_signature` cannot say
# this, because the compound line it adds fires either way.
plant_definition_lines_at_home() {
  mkdir -p "$1/crates/geom-core/src"
  {
    printf '%s\n' "$DEFINITION_TRAIT"
    printf '%s\n' "$DEFINITION_IMPL"
  } > "$1/$DEFINITION_HOME"
}

# THE ALIAS-ROSTER CASES. Each plants ONE edit that the SCAN lets through,
# because an edit the scan already reds says nothing about this check.
#
# The declaration going quiet WHERE IT STANDS: `arc_fillet.rs` is on the
# file list, so the scan is silent either way, and the spelling below is
# one this reader cannot read — the trait's NAME is pasted by a macro, so
# there is no `trait <name>` token for the declaration test to find
# (KNOWN GAP 4). Line breaks are no longer such a spelling: a record is a
# statement, so the multi-line `where` block rustfmt converges on arrives
# whole. Without this check the edit removes KNOWN GAP 3's whole defence
# with nothing red anywhere.
plant_alias_declaration_gone_silent() {
  mkdir -p "$1/crates/profile/src/path"
  {
    printf 'macro_rules! carrier {\n'
    printf '    ($n:ident) => {\n'
    printf '        pub trait $n: Decide + Bounds {}\n'
    printf '    };\n'
    printf '}\n'
    printf 'carrier!(ArcCarrierScalar);\n'
  } > "$1/crates/profile/src/path/arc_fillet.rs"
}

# THE SAME DECLARATION IN THE FORM `rustfmt --edition 2021` CONVERGES ON,
# and it is a must-NOT-fire case now: the census SEES it, so KNOWN GAP
# 3's defence is intact and nothing is owed. It reds the day a record
# stops being a statement.
plant_alias_declaration_rustfmt_block() {
  mkdir -p "$1/crates/profile/src/path"
  {
    printf 'pub trait ArcCarrierScalar: Decide\n'
    printf 'where\n'
    printf '    Self: Bounds,\n'
    printf '{\n'
    printf '}\n'
    printf 'pub fn seam_1<T: Decide + Bounds>(_t: T) {}\n'
    printf 'pub fn seam_2<T: Decide + Bounds>(_t: T) {}\n'
  } > "$1/crates/profile/src/path/arc_fillet.rs"
}

# The RENAME, which is the same file with a different name in it: the
# roster entry goes missing AND an unrostered declaration appears, so this
# is the case that shows the two halves compose into one diagnosis. It is
# asserted on the `extra` half, which the fixture above cannot produce.
plant_alias_renamed() {
  mkdir -p "$1/crates/profile/src/path"
  printf 'pub trait ArcCarrier: Decide + Bounds {}\n' \
    > "$1/crates/profile/src/path/arc_fillet.rs"
}

# A SECOND alias minted inside a file the list already ratifies —
# `topo/src/props.rs`, ratified under M5 PR 11 for the certified-
# quadrature plumbing. The scan is silent on it (the file is filtered) and
# the new name's uses would then spread with nothing recording that they
# exist. One sliver of KNOWN GAP 6's ride-along, and only that sliver: the
# pinned count moves too, and this is the diagnosis that wins, for the
# reason at `gate_allowlist_counts`.
plant_new_alias_in_ratified_file() {
  printf 'pub trait TubeCarrier: Decide + Bounds {}\n' >> "$1/crates/topo/src/props.rs"
}

# THE PRICE OF THE ROSTER, planted rather than left to be discovered: an
# alias carrying only BRACKET doors is outside this gate's class, and it
# is still asked for an entry, because what the roster registers is a name
# whose uses this matcher cannot read — not a verdict that the name is
# illegal. The entry is where somebody says which of the two it is.
plant_bracket_only_alias_in_ratified_file() {
  printf 'pub trait TubeBracket: CertifiedBounds {}\n' >> "$1/crates/topo/src/props.rs"
}

# THE ALIAS MOVED, and the SCAN is what reports it: the declaration at a
# path no entry names is an ordinary hit, and the scan runs before the
# census, so this case never reaches the roster check at all. That is the
# reason a moved entry needs no census message of its own. This case reds
# the day an entry above stops naming an exact path.
plant_alias_declaration_moved() {
  mkdir -p "$1/crates/profile/src/path"
  rm -f "$1/crates/profile/src/path/arc_fillet.rs"
  printf 'pub trait ArcCarrierScalar: Decide + Bounds {}\n' \
    > "$1/crates/profile/src/path/arc_fillet2.rs"
}

# THE ALIAS RETIRED — the case the header used to assert was safe because
# "its entry leaves in the change that retires it", with nothing checking
# that it did. The declaration's file is gone and the entry stands; the
# scan has nothing to say (there is no declaration anywhere) and the
# census is the only reader left. Before this the same tree, and a roster
# entry naming a path that never existed at all, both left the gate
# GREEN.
plant_roster_file_gone() {
  rm -f "$1/crates/profile/src/path/arc_fillet.rs"
}

# KNOWN GAP 3 ITSELF, MEASURED. The declaration in its ratified home, and
# the three shapes `family.rs` actually writes the name in — a free
# function, a public trait generic over it, and an impl block — in a file
# NO entry names. The gate must PASS: `T: ArcCarrierScalar` is character
# for character the sole bracket bound `plant_sole_bracket_bounds` pins as
# must-NOT-fire, which is why no WIDENING of the matcher separates them
# and why the answer to D68 is not a bigger regex. A matcher keyed on the
# ROSTER's names does separate them — read the header's GAP 3 for what it
# costs, which is the close S63 forbids. This case goes RED the day such
# a matcher is built, and GAP 3 has to be rewritten in the same change.
plant_alias_uses_invisible() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn open_arc<T: ArcCarrierScalar>(_t: T) {}\n'
    printf 'pub trait ArrivalSpec<T: ArcCarrierScalar> {}\n'
    printf 'impl<T: ArcCarrierScalar> ArrivalSpec<T> for Center<T> {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# THE CENSUS'S OWN NEAR MISS, and it is D102's named construct: a trait
# generic over a SOLE bracket bound. In a RATIFIED file the scan is silent
# and the census is the only reader left, so this is where its balanced
# `<…>` skip is observable — delete that skip and this case reds, asking
# for a roster entry for a name that aliases nothing.
plant_trait_generic_sole_bracket_ratified() {
  printf 'pub trait ArrivalSpec<T: CertifiedBounds> {}\n' >> "$1/crates/topo/src/props.rs"
}

# THE PIN'S TWO DIRECTIONS, one fixture each, on a file the list
# ratifies and the clean fixture writes at its pinned count. The GAIN is
# what the pin exists for: a second compound bound riding a ratification
# argued for the first, which a path filter cannot separate from it. The
# LOSS reds for its own reason, not for symmetry — an entry outliving
# the seam it names goes on exempting a file nobody has argued for
# since, which is the cover the next bound written there would ride.
plant_ratified_file_gains_a_bound() {
  printf 'pub fn second_seam<T: Decide + Bounds>(_t: T) {}\n' \
    >> "$1/crates/topo/src/props.rs"
}

plant_ratified_file_loses_a_bound() {
  local f=$1/crates/topo/src/props.rs
  awk 'NR > 1' "$f" > "$f.trimmed"
  mv "$f.trimmed" "$f"
}

# A SECOND BOUND ON AN EXISTING RECORD'S LINE, which is the shape a
# pin on RECORDS takes for free: one line, one record, two bounds. It is
# the ride-along arriving inside a line instead of on a new one, and it
# is why the reading is occurrences (KNOWN GAP 6).
plant_second_bound_on_one_record() {
  local f=$1/crates/topo/src/props.rs
  printf 'pub fn seam_1<T: Decide + Bounds, U: Decide + CertifiedBounds>(_t: T, _u: U) {}\n' \
    > "$f.two"
  awk 'NR > 1' "$f" >> "$f.two"
  mv "$f.two" "$f"
}

# The same loss taken to the end: the ratified file is gone and its
# entry still stands, which the diagnosis has to say out loud or a
# reader repairs it by editing the number.
plant_ratified_file_gone() {
  rm -f "$1/crates/topo/src/props.rs"
}

# THE PIN IS A COUNT AND NOT A TEXT PIN, planted rather than left to be
# assumed: one ratified compound bound replaced by an unrelated one at
# the same count PASSES here. That is KNOWN GAP 6's residue measured —
# the substitution a per-file count cannot see — and this case reds the
# day an entry becomes per-symbol, which is the day that gap is
# rewritten.
plant_ratified_file_swaps_a_bound() {
  local f=$1/crates/topo/src/props.rs
  awk 'NR > 1' "$f" > "$f.swapped"
  printf 'pub fn swapped_seam<T: Decide + geom_core::CertifiedBounds>(_t: T) {}\n' \
    >> "$f.swapped"
  mv "$f.swapped" "$f"
}

# THE MALFORMED-LIST CASES, run IN-PROCESS, and the exception is the
# harness's shape rather than a relaxation: every other case here is a
# real subprocess over a planted TREE, and the list is not in the tree —
# `--root` cannot reach it. So the check takes its entries as arguments
# and these cases call it with lists the array never holds, which is the
# same function `gate()` runs first.
gate_selftest_malformed() {
  local want=$1; shift
  local out
  if out=$( (gate_allowlist_wellformed "$@") 2>&1 ); then
    printf 'SELFTEST FAILED: the allowlist well-formedness check PASSED a malformed list (%s)\n' "$*" >&2
    exit 1
  fi
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s): the check fired with an unexpected message:\n%s\n' "$*" "$out" >&2
       exit 1 ;;
  esac
}

gate_selftest() {
  local want="compound Bounds/Enclosure bound outside the ratified seams"
  # The LIST's own shape, before any fixture: a check that runs before
  # every scan is worth nothing unless a malformed list is shown to
  # reach it. A path carrying a SPACE lands on the count field, which is
  # what the diagnosis names.
  gate_selftest_malformed "no fields after the path" 'crates/planted/src/lib.rs'
  gate_selftest_malformed "no RULING after the count" 'crates/planted/src/lib.rs 3'
  gate_selftest_malformed "not a positive decimal integer" 'crates/planted/src/lib.rs nine A ruling'
  gate_selftest_malformed "not a positive decimal integer" 'crates/planted/src/lib.rs 09 A ruling'
  gate_selftest_malformed "not a positive decimal integer" 'crates/pl anted/src/lib.rs 3 A ruling'
  gate_selftest_malformed "already has an entry" \
    'crates/planted/src/lib.rs 3 A ruling' 'crates/planted/src/lib.rs 4 Another ruling'
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
  gate_selftest_case "$want" plant_where_clause_pair
  gate_selftest_case "$want" plant_generic_list_and_where
  gate_selftest_case "$want" plant_bound_split_across_lines
  gate_selftest_case "$want" plant_rustfmt_where_block
  gate_selftest_case "$want" plant_wrapper_type_compound
  gate_selftest_case "$want" plant_real_rs_signature
  gate_selftest_case "no longer in crates/geom-core/src/real.rs verbatim" plant_real_rs_alias_redefined
  gate_selftest_case "$want" plant_definition_lines_elsewhere
  gate_selftest_case "$want" plant_dual_equivalent_spelling
  gate_selftest_case "can no longer see the roster entry" plant_alias_declaration_gone_silent
  gate_selftest_case "is not on this gate's alias roster" plant_alias_renamed
  gate_selftest_case "is not on this gate's alias roster" plant_new_alias_in_ratified_file
  gate_selftest_case "is not on this gate's alias roster" plant_bracket_only_alias_in_ratified_file
  gate_selftest_case "$want" plant_alias_declaration_moved
  gate_selftest_case "FILE NOT IN THE TREE" plant_roster_file_gone
  gate_selftest_case "MORE than the entry argues for" plant_ratified_file_gains_a_bound
  gate_selftest_case "FEWER than the entry argues for" plant_ratified_file_loses_a_bound
  gate_selftest_case "MORE than the entry argues for" plant_second_bound_on_one_record
  gate_selftest_case "the file is not in this tree" plant_ratified_file_gone
  gate_selftest_passes "a sole bracket bound in its fn, path-qualified, struct and trait-generic forms" \
    plant_sole_bracket_bounds
  gate_selftest_passes "a sole bracket bound in a where clause, two parameters bounded one each, and a wrapper type bounded beside its own parameter" \
    plant_where_clause_near_misses
  gate_selftest_passes "a rostered declaration in the multi-line where block rustfmt converges on, which the census reads" \
    plant_alias_declaration_rustfmt_block
  gate_selftest_passes "the two CertifiedBounds definition lines in real.rs, which is their home" \
    plant_definition_lines_at_home
  gate_selftest_passes "the spelling in a trailing comment, a block comment and a string literal" \
    plant_spelling_in_comments_and_literals
  gate_selftest_passes "an alias DECLARATION in its ratified home plus its uses in a file that is not ratified (KNOWN GAP 3, measured)" \
    plant_alias_uses_invisible
  gate_selftest_passes "a trait generic over a sole bracket bound inside a ratified file, a sole bracket bound in a where clause, two parameters bounded one each, a wrapper type bounded beside its own parameter (the near miss the grouping owes, and the shape three ratified files write), a rostered declaration reformatted into the rustfmt where block, which the census still reads" \
    plant_trait_generic_sole_bracket_ratified
  gate_selftest_passes "one ratified compound bound replaced by another at the same count" \
    plant_ratified_file_swaps_a_bound
  printf '%s selftest OK: passes a clean fixture and a sole bracket bound as a fn, a path-qualified fn, a struct, and a trait generic over one -- bare, nested two deep, and beside an `Fn(..) -> ..` parameter in both orders -- and the two CertifiedBounds definition lines at home in real.rs, which is the anchored skip proved in its positive direction; fires on both operand orders of Decide+Bounds, of Decide+CertifiedBounds and of Decide+Enclosure, on the same obligation spelled with no plus at all -- repeated on one parameter in a where clause, split between the generic list and the where clause, and broken across lines -- on a wrapper type that decides and reads brackets in its own where clause, on a path-qualified alias after the plus, on Bounds- and Enclosure-shaped alias names not in the tree today, on all three one-line spellings of a non-Bounds-named alias DECLARATION (pair, sole supertrait, where-clause) and on the multi-line `where` block rustfmt converges on from the third, which no line-based reader sees, on a compound bound in real.rs beside the skipped definition lines, on real.rs redefining the alias to carry Decide (through the definition-skip subject check), on the two definition lines written at a path that is not their home, which is the moved real.rs the skip is anchored against, and on the equivalent spelling of dual.rs Bounds impl (GAP 2); fires, through the ALIAS ROSTER, on a rostered declaration going quiet where it stands (its name pasted by a macro, which is what GAP 4 keeps), on the same declaration renamed (both halves of one diagnosis), and on a new alias -- compound OR bracket-only -- minted inside a file the list already ratifies, where the scan is silent, and on a roster entry whose file is no longer in the tree, which is the retirement the roster claims to make loud; fires, through the PINNED OCCURRENCE COUNT each allowlist entry carries, on a ratified file that has gained a compound bound, which is the silent inheritance S159 names, on one that has gained a SECOND bound inside a signature that already held one, which is that inheritance arriving where a record count cannot see it, on one that has LOST an occurrence, and on one that is gone entirely with its entry still standing; refuses, before any scan runs, an allowlist entry with no fields after its path, one with no ruling after its count, one whose count is not a positive decimal integer -- spelled out, zero-padded, or shifted off the end by a path carrying a space -- and a second entry for a path that already has one; passes the spelling written into a trailing comment, a block comment and a string literal, which the leading-`//` strip this gate carried fired on, a trait generic over a sole bracket bound inside a ratified file, a sole bracket bound in a where clause, two parameters bounded one each, a wrapper type bounded beside its own parameter (the near miss the grouping owes, and the shape three ratified files write), a rostered declaration reformatted into the rustfmt where block, which the census still reads, one ratified compound bound swapped for another at the SAME occurrence count, which is what a per-file count cannot see (KNOWN GAP 6), and KNOWN GAP 3 itself -- the alias declaration in its ratified home beside its uses in a file that is not, which this gate cannot see and does not claim to; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main

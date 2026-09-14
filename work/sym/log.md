# SYM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/sym/plan.md`. A/B band 4700–4799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-13)

Opened at M10's exit sweep, on Ev's call in chat that day: M10's walk
(#1700) was ratified and its twenty-two open rows re-homed, and of
those **fourteen stand on one territory** — `geom_core::sym` and the
registered-identity door beside it — which `work/README.md` calls a
successor's opening slate rather than residue. Ev's word: "the 14 to a
successor program, the others to either another successor program or a
preexisting program." This is that program; the other eight went to
PROPS (4), BLEND (2), SHELL (1) and CENSUS (1). The sweep is
`docs/DOC-LEDGER.md` sweep 13 and M10's walk is recoverable at the SHA
it names.

Items re-homed into this directory at opening, by header edit and
`git mv` only (ids unchanged), all from `work/m10/`:

- `real-margin-dependency-widening`
- `plate-ceiling-is-now-the-scaffold-pushforward`
- `rule-d-reaches-the-unit-bulge-only`
- `interval-self-dot-straddles-before-rule-a`
- `param-box-certification-of-implicit-quantities`
- `declared-tangency-needs-the-registered-identity-door`
- `the-span-identity-is-not-a-theorem-of-the-floats`
- `the-witness-slack-is-eps-independent`
- `sym-registration-flattens-two-axes`
- `symbolic-tier-costs-95-percent-of-the-m10-3-drive`
- `derived-frame-placement-freezes-on-the-symbolic-lane`
- `symbolic-tier-census`
- `sym-rs-is-one-file-with-a-347-line-header`
- `registered-is-spelled-five-times-and-pinned-once`

**Territory, written on both sides.** `crates/geom-core/src/*` is
PROPS' glob; this program claims `sym.rs` and `sym/*` inside it and
PROPS' `keep_out` names this program in the commit that opens it. The
door's own file, `geom-core/src/real.rs`, stays PROPS' — SYM reaches
`register_equal` / `SymRegistration` / `WITNESS_REL` there by
announced seam, and the `Real` trait itself is SCALAR's subject.
`crates/geom-core/tests/m10_*` is claimed here and is also S-TCOST's
and S-TINT's by their `crates/*/tests/*` declaration; that overlap is
the `*/tests/*` family `work/README.md` describes and is recorded from
this side only, as every other program's is.

**Two rows this program's door waits on are NOT here.** The fillet's
declared tangency cannot be registered until the constructor hands the
joint classifier its centre, and the revolve carriers cannot state
their span identity until the builder is handed the far endpoint —
both are constructor changes in `crates/profile` and
`crates/sweep/src/revolve`, so both went to BLEND
(`fillet-tangency-is-not-the-constructors-node`,
`revolve-carriers-state-only-the-rim`). This program holds the
consumer of the first (`declared-tangency-needs-the-registered-identity-door`)
and it is open rather than parked: `blocked_on` would be true of it,
but the row is worth reading whole and the dependency is stated in its
body and in the plan's door lane.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Orchestration opens (2026-09-13)

Ev, in chat: "you're the new orchestrator for work/sym … if not, you
can just get to it." No questions blocked the start; what was decided
unilaterally is below, each with the alternative it beat.

**This box.** Single-orchestrator remote container (four cores, ~25 GB
writable at opening), GitHub through the MCP tools, no `gh`, no
away-channel monitor. The orchestrator branch is the session's
designated `claude/pensive-hamilton-12ar36` rather than
`sym/orchestrator` (PROPS' and FILLET's precedent); unit branches keep
the `sym/` prefix. Lanes are Agent worktrees under
`/home/user/lanes/<lane>` with private `CARGO_TARGET_DIR`s at
`/home/user/<lane>-target`, each copied from one warm dev build of
`geom-core` and `editor-core` with tests under `interval`
(`/home/user/sym-seed-target`, 3.5 GB); at most two heavy lanes at
once. Briefs at `/home/user/sym-briefs/` (lane-local; the spec is the
binding text). `[ev]` PRs, when any open, are subscribed so their
comments wake the session.

**The first wave, and why these two.** The slate's three H rows that a
unit could take today all start with a measurement the item itself
asks for before any change — the cost row ("the first work is the
profile rather than a fix"), the bulge row ("a measurement of WHAT
stands at `bulge = 2` before any rule is proposed"), the widening row
("the next thing to measure … ranked"). So the first wave is
measurement plus the hygiene that makes every later tier diff
reviewable, and the first DUAL unit is cut from what they return:

- **SYM-1** (`sym/1-profile`, `docs/SYM-1-SPEC.md`): the profile
  inside the normal form on the M10-3 slab — asks 1 and 2 of
  `symbolic-tier-costs-95-percent-of-the-m10-3-drive`, no fix. Outside
  the experiment (moves no decision): one review that re-runs the
  measurement, no ordinal. The alternative — cut the cost FIX now from
  the design's own hypothesis (degree growth from carried denominators)
  — is exactly how the degree-16 hypothesis was refuted, and the item
  says so.
- **SYM-2** (`sym/2-split`, `docs/SYM-2-SPEC.md`): the three moves
  `sym-rs-is-one-file-with-a-347-line-header` names (the coefficient
  tower to `sym/rational.rs`, the polynomial to `sym/form.rs`, the
  header distributed with them) plus the header's archaeology cut to
  invariants, each cut listed. Style review, no row. Runs concurrently
  with SYM-1 and lands FIRST; SYM-1 re-sites its feature-gated
  counters on the split tree. The alternative — serialise them — costs
  a lane-day for a merge that is a relocation if the moves stay pure.
- **SYM-3** (next free lane): what stands at `bulge = 2`, rendered as
  M10-10 rendered the plate's four (`rule-d-reaches-the-unit-bulge-only`'s
  first ask; R1's segment boss is in tree). Spec cut when a lane frees.

**Not first, and why.** `real-margin-dependency-widening`'s ranking is
a query over the K CSV plus the expression each margin came from, which
the DAG can answer (occurrences of a parameter in the node against in
the normal form) — a real instrument, and a bigger unit than the bulge
render; it follows SYM-3. The door rows wait: the fillet consumer on
BLEND's constructor change, the witness-slack and span-identity
decisions on "the unit that next touches the door", none of which is
this wave. `param-box-certification-of-implicit-quantities` is a design
conversation before it is a unit and opens with Ev when the ceiling
lane has said what bounds the plate after the widening class is
measured. `plate-ceiling-is-now-the-scaffold-pushforward`'s fix half is
PCURVE/D3's and is opened with TRIM, not here.

**Seams announced at dispatch**: SYM-1 → PROPS (`work/props/log.md`:
the `sym-profile` feature line in `crates/geom-core/Cargo.toml` and its
dev-dependency forward in `crates/editor-core/Cargo.toml`, the shape of
`identity-pass-testing`; `drive.rs`'s cost note corrected only if the
profile shows it wrong) and → S-TCOST and S-TINT (`work/tcost/log.md`,
`work/tint/log.md`: a new `#[ignore]`/feature-gated row file under
`crates/editor-core/tests/m10_*`, registered in `tests/all.rs`, costing
the gate nothing). SYM-2 touches only this program's files.

**A/B**: neither unit draws; no block is opened. Block SYM-B1 opens
with the first dual unit, whose pre-draw fields go in
`docs/MODEL-AB-LOG.md` before the draw. Lane commits carry no model
trailer regardless, so the convention is uniform when a blinded unit
follows.

## First wave delivered; both units in review (2026-09-14)

**An outage, recorded.** The account's usage credits ran out
~22:30Z on 2026-09-13: the SYM-1 lane died after opening its PR and
before reading its own CI or writing its report (its worktree was
clean and pushed, so nothing was lost); the SYM-2 lane finished before
the limit. The orchestrator's check-in fired into the same outage and
was read at 03:30Z when the limit reset (Ev, in chat).

**SYM-2 delivered** — PR #2532, head `a7396bebe`, hosted run
34787545013 green on the full matrix. `sym.rs` 4,478 → 3,530 lines,
header 491 → 447; `sym/rational.rs` 533 and `sym/form.rs` 433, each
with its own header; six archaeology cuts, each quoted in the PR body;
three header sentences left as "looks wrong, not touched" and two of
them filed (`sym-header-says-every-freeze-is-counted`,
`sym-header-dag-paragraph-disagrees-with-the-code`). The lane proposes
the ids and nodes → `sym/dag.rs` (~240 lines) as the next cut and
declines the session-and-walk split as a design question; the item
stays open on that proposal. A class note from the lane worth keeping:
its first `doc-gate.sh` run was piped through `tail` and read green
over five broken links — the piped-wrapper hazard
`memories/agent-lane-operations.md` already names, met by a gate
script this time.

**SYM-1 delivered, CI red on its own new code** — PR #2530, head
`ef33aa27c`. The findings (all in the item body's `## The profile
(SYM-1)`): on the slab term storage is 57 % of the tier's instructions
in release, the walk and DAG build 18 %, the ring 10 %, and NOTHING
freezes at any of the drive's 2,559 leaves — the slab's cost is
volume (10,604 plain forms per leaf, the same DAG interned afresh per
leaf) times a fixed cost per tiny form; on the plate storage is the
same 57 %, the ring 27 %, and 1,032 of 1,312 freezes are on DEGREE
inside the per-node A/B reduction (53 % of the plate's instructions)
— the derived-frame mechanism, visible there and absent on the slab.
The term budget is never the wall; `drive.rs`'s note is confirmed by
count. Four proposals stand as the next unit's inputs, the largest
being a small-vector polynomial (bounded by the 57 %) and a
drive-scoped plain memo (a session-model change, needs the design
conversation). Hosted run 34786894144: six shard-1 `test` jobs red on
two `pncad-py` prose-census rows — `sym/profile.rs`'s `impl Display
for SymProfile` renders struct payloads through `Debug` at three sites
and one undecided; `main` is green; the fix is the implementer's and
goes into the fix pass with the review's findings, the review
dispatched on the frozen red head with the red disclosed.

**Reviews dispatched** (2026-09-14 ~03:50Z), both outside the
experiment, one reviewer each (reviews stay Fable), briefs at
`/home/user/sym-briefs/review-{1,2}-brief.md`: SYM-2's claims are the
pure-move reads commit by commit, the cut list, the remaining header's
scope, the doc gate; SYM-1's are the callgrind order re-taken, the
four re-spelled sites behaviour-identical and the feature-off build
clean, the tables' arithmetic and the structural rows' counts digit for
digit, the memo proposal's premise, the coverage paragraph, and the
editor-core interval shard's wall on this run against `main`'s (the
dev-dependency forward compiles the hooks into every editor-core test
build, which the spec asked the PR to price and the PR body does not).
Fix passes follow on the implementers' lanes; SYM-2 lands first.

**Disk on this box**: two implementer targets grew to 8.5 and 7.7 GB
(incremental caches 5.3 and 4.6); both lanes finished, so the
incremental caches were deleted and reviewer targets seeded from the
remaining `deps`. Two heavy lanes at once stays the rule.
## SYM-2 merged (2026-09-14): the tier's file split

PR #2532, fix-pass head `b73b289a2`, hosted run 34806691533 green on
the full matrix; one style review outside the experiment (no row), its
delta round confirming every fix item by execution. `sym.rs` 4,478 →
3,540 lines (header 491 → 457); `sym/rational.rs` 532 (the coefficient
tower, its own argument for the bound) and `sym/form.rs` 417 (the
polynomial and the quotient form, its own limits); six archaeology cuts
from the header and two twin arguments collapsed to one home each; the
readings that argued for `COEFF_BITS` read as readings again. The
review's one MINOR was a cut that changed tense into a present claim
nothing pins; taken. **Filed, not fixed** (the unit edits no sentence's
technical content): `sym-header-says-every-freeze-is-counted`,
`sym-header-dag-paragraph-disagrees-with-the-code`,
`sym-header-claims-outrun-the-code` (three more header sentences the
code has outgrown — D9's "every insertion order" against the opaque
sequence; the two-clause `Zero` against `Registered`/`SignGated`; the
constructor census against `sweep::extrude`'s arc wall, whose route
through the wrapper the allowlist gate does not see) and
`sym-item-docs-carry-unit-archaeology` (the §4 sweep the header pass was
fenced out of). **Not landed**: the reviewer's cheap fourth cut — the
test module out, 1,045 lines — trips `register-equal-allowlist.sh`,
which exempts `sym.rs` whole as a definition home and so hides the
tier's thirteen test calls of the door; the lane reverted rather than
edit `guard`'s gate and filed
`work/guard/register-equal-allowlist-exempts-a-whole-file-and-hides-test-calls`
(the reviewer's view, recorded for `guard`: a `#[cfg(test)]`-aware read
rather than a `TEST_HOMES` list, since the same whole-file skip hides
new calls in the other four definition homes too). `dag.rs` deferred
with two conditions on the item. The item stays open on both cuts.
Lane commits carried no trailer; the orchestrator's state-sync rides
the PR last.


## SYM-1 merged (2026-09-14): the profile inside the normal form

PR #2530, landing head `9153d39be` (fix-pass head `55b8857f5` merged
with SYM-2's split, every hook re-sited verbatim and read by the
reviewer), hosted run 34808710532 green on the full matrix; one review
outside the experiment (no row), its delta confirming every fix item by
execution and its numbers reproduced on a second box to the digit.

**What the tier costs, measured** (release, instruction share of one
nominal replay; the record is the item's `## The profile (SYM-1)`):
on the M10-3 slab term storage — the allocator, the `BTreeMap` per
form, a heap `Vec` per monomial — is 57 %, the walk and the DAG build
19 %, the coefficient ring 10 % (`BigInt` 1.3 %), and NOTHING freezes
at any of the drive's 2,559 leaves: the slab's cost is volume (10,604
plain forms per leaf for 1,490 decisions, the same 12,208-node DAG
interned afresh per leaf) times a fixed cost per tiny form (mean 1.5
terms). On the plate storage is the same 57 %, the ring 27 %, and 1,032
of 1,312 freezes are on DEGREE inside the per-node A/B reduction (53 %
of the plate's instructions) — the derived-frame mechanism, visible
there and absent on the slab. The term budget is never the wall (max 90
terms against 4,096); `drive.rs`'s note is confirmed by count.

**The review's MAJOR, by execution, and what it changed.** The
`Decide` impl runs `discharge` inside a `debug_assert!` on every
DEFINITE margin, and the workspace keeps debug assertions on in
release, so the profile had charged the assertion's work to the tier:
the instrument now splits every walk by origin (decision / assertion /
report) and the record reads against the split — on the slab the
assertion is a tenth of the plain forms and 95 % of the early-walk
forms (43 of 162 walk-seconds over the drive), on the plate a third of
the freezes (488 of 1,312). The header sentence claiming the tier skips
definite margins was false in every profile this workspace builds; it
now says what holds. The assertion is on the proposal list as a
next-unit input with its numbers and the design question it raises (a
soundness cross-check that is a second walk).

**The four proposals** (inputs, not designs): a small-vector
polynomial (bounded by the 57 %); a drive-scoped plain memo (the plain
form is a function of the content hash — a session-model change, with
its three side effects named: atoms registered in `combine`, `frozen`
incremented inside the walk, the opaque sequence load-bearing across
leaves); a cheaper `Rat::from_parts` normalisation on the dyadic shape
and a cached degree; and the assertion. The cost-fix unit is cut from
these and is this program's first dual.

**Two things the reviews caught that the lane had not**: its own
freeze rows had asked DEFINITE margins, so their freezes were the
assertion's — the instrument's first catch, rewritten to numerically
zero margins; and the CI red on its own new code (a report type's
`Display` rendering struct payloads through `Debug`, which the pncad-py
prose census rejects; now a `render()`), which the lane did not see
because the outage killed it between opening the PR and reading the
run. `symbolic-tier-costs-95-percent-of-the-m10-3-drive` stays open on
ask 3 (the change), carried by the next unit. Lane commits carried no
trailer; the orchestrator's state-sync rides the PR last.

## Block SYM-B1 in preparation; the door's witness put to Ev (2026-09-14)

Both first-wave units merged (#2532 at `94b7eea75`, #2530 at
`2ccf071e1`). The program's first DUAL units are cut from SYM-1's
numbers and open block SYM-B1 when the third slot's fields exist:

- **SYM-4** (`docs/SYM-4-SPEC.md`, M / STRUCTURAL): the cost of a
  form — `Poly`'s `BTreeMap` to a sorted vector in the map's own
  order (every digest, atom key and decision unchanged, held by the
  pins and a new rendered-form digest row), the degree cached,
  `Rat::from_parts`'s gcd skipped on the dyadic shape. Slot 0.
- **SYM-5** (`docs/SYM-5-SPEC.md`, H / NUMERIC): the derived-frame
  freeze — DOCM's two red rows ported, the freeze profiled and its
  chain rendered, then a rule of the atom algebra (a unit-vector
  atom with `Σ U_i² = 1` and `U_i · sqrt(S) = a_i`, or the common
  factor cancelled before the square, or a degree-resetting `Sqrt`),
  dial-gated and shipped on only if affordable. Slot 1.
- **Slot 2** is cut from SYM-3's render when it returns (the bulge
  rule, if the diagnosis names one), or from the door's witness if
  Ev answers first. The draw waits on all three pre-draw fields.

**Deliberately not in the block**: the drive-scoped plain memo
(SYM-1's largest lever on the slab — the plain form is a function of
the content hash and its memo could outlive the leaf) is a
session-model change with receipt consequences; it goes to Ev as a
decision document once SYM-4 has said what an in-session form costs.
The `Decide` impl's assertion discharge stays; its cost is on the
item.

**`[ev]` PR #2552 opened** (branch `sym/ev-witness`, subscribed):
D1 — which route for the ε-independent witness slack (thread `Tol` to
the door, recommended; a `Tol` on the session, not; leave it,
deferred) and D2 — one refusal arm or two (one, with the receipt as
the loud channel and a fixture-scale zero-refusals row closing the
span-identity row). Both rows carry `needs_ev`. FILLET's precedent:
the answer arrives while the units run.

## SYM-3 delivered; block SYM-B1 drawn (2026-09-14)

**SYM-3 delivered** — PR #2558, head `24cdf27c5`, hosted run
34813607577 green on the full matrix; one review outside the
experiment dispatched on the frozen head. The diagnosis, per document
(the item's `## What stands (SYM-3)`): the boss's
`carrier_matches_mapped_source` residue (6 of 54) is a COEFFICIENT-RING
freeze — every trig atom folded, one `sqrt 5`, a rim component's
`Sub` at ~318 bits against `COEFF_BITS = 256` (at 512 bits it is
72/0/54/0 and the ceiling moves onto `line_span`); its
`carrier_on_surface_2` residue (27 of 90) is an `abs` over a
non-constant argument that A0 does not reach (`seg.rs`'s
`signed_radius.abs()`), NOT the item's `atan|b|`-vs-`atan b` guess —
`abs(2)` itself folds. **And opening either loses discharges**: with
`abs(X) = X` on a syntactically non-negative `X`, or `abs(X)² = X²`,
20 of 40 fold, 20 then freeze on the products the opened atom joins,
and ten door decisions are LOST (48 → 38 registered); both patches
together drop the parameter D-tab's ceiling `3.52e2 → 2.82e2 · ε`.
The mechanism: a frozen node is one opaque indeterminate keyed by its
content hash, so two frozen twins cancel and a partial opening breaks
the symmetry — filed as
`coefficient-ring-width-is-not-monotone-in-reach`, a class finding
about the freeze discipline, and the reason slot 2 of the block is
NOT a bulge rule: the ring's reach is a design conversation before it
is a unit. The parameter bulge's residual at the dyadic control is
identically zero given `abs(R)² = R²`; the sign enters only through
`arc_span = 4 · atan|b|` against the pushforward's `4 · atan b`; route
A (rule C over `abs`) touches 50 decisions, route B (the door's
already-decided turn) 8 — counted, not chosen. The literal D-tab's
ceiling is bounded by `dihedral_wedge` and `arc_diameter_clearance`
(the widening class's third site, added to that row). Corpus: 13 tour
stops author a non-unit bulge, the unit bulge is the circle's only —
the boss buys a family. Three nominal pins per document.

**Block SYM-B1 drawn** (record branch-side on `sym/b1-block` per the
#1095 shape; it reaches `main` when the block concludes): slot 0 =
SYM-4 (M / STRUCTURAL), slot 1 = SYM-5 (H / NUMERIC), slot 2 = SYM-6
(M / STRUCTURAL, conditional on #2552's D1 — closes short if Ev picks
(0)). SYM-4 dispatches now on the seed target; SYM-5 when a lane
frees; SYM-6 on Ev's answer. The SYM-6 spec is on the orchestrator
branch and reaches `main` with this entry.


## SYM-3 merged (2026-09-14): what stands at a bulge that is not 1

PR #2558, fix-pass head `da1f21e50`, hosted run 34819053293 green on
the full matrix; one review outside the experiment (no row), its
delta confirming the fix pass by execution. The record is the item's
`## What stands (SYM-3)`; the fixtures `m10_bulge_interval.rs` and the
committed renders; the split table now has one home
(`m10_8_harness::{split, split_at_the_nominal, assert_split}`, five
callers) and every pin asserts the WHOLE table, so a predicate that
appears reds naming it.

**What stands, per document.** The boss (literal `bulge = 2`):
`carrier_matches_mapped_source` 6 of 54 is a coefficient-ring freeze
(a rim component's `Sub` at ~318 bits against 256; every trig atom
folded); `carrier_on_surface_2` 27 of 90 is an `abs` over a
non-constant argument (`seg.rs`'s `signed_radius.abs()`) that A0 does
not reach; the rim identity `carrier_endpoint_start` stands on the
same two residues on every D-tab (24/0/8/4) — the review's MAJOR, in
none of the first cut's record. The parameter bulge's residual at the
dyadic control is identically zero given `abs(R)² = R²`; the sign
enters only through `arc_span = 4·atan|b|` against the pushforward's
`4·atan b`; route A (rule C over `abs`) touches 50 decisions, route B
(the door's already-decided turn) 8 — counted, not chosen. The
literal D-tab's ceiling is the widening class (third site).

**The finding that reshaped the block.** Opening the atom loses
discharges: `abs(X) = X` on a syntactically non-negative `X` (patch
C) folds 20 of 40 and freezes 20 on the products the opened atom
joins, and TEN door decisions are lost (48 → 38 registered); the
square substitution (patch A) with a 512-bit ring drops the parameter
D-tab's ceiling `3.52e2 → 2.82e2·ε` — the reviewer could not
reproduce that number, and the fix pass recorded all three patches
as diffs and re-measured by pair: the fall is patch A's own (a square
substituted as a form carries the box's width into a predicate the
atom had kept out of it), C + 512 is the shipped bracket bit for bit.
A frozen node is one opaque indeterminate keyed by its content hash,
so two frozen twins cancel and a partial opening breaks the symmetry
(`coefficient-ring-width-is-not-monotone-in-reach`). The ring at 512
costs 3.9× per probe on the boss and 9× on the parameter D-tab. So
the ring's reach is a design conversation — what should freeze, and
when widening helps — before it is a rule unit, and block SYM-B1's
third slot went to the door's witness instead. The corpus: 14 tour
stops author a non-unit bulge, 31 test files on the stated pattern —
the unit bulge is the circle's only, and the boss buys a family.

**Class findings from the review, recorded**: the per-predicate split
table had five spellings across the M10 evidence files (now one
home); the ceilings evidence row ran five more documents by default
once the dyadic controls joined the shared index (now `controls()`,
by name). Lane commits carried no trailer; the orchestrator's
state-sync rides the PR last. `rule-d-reaches-the-unit-bulge-only`
stays open on its first ask's residue, carried by the ring
conversation.


## SYM-5 PR-1 merged (2026-09-14): the derived-frame freeze, measured — and the item's real case found

PR #2568, fix-pass head `869fd55bf`, hosted run 34834096015 green on the
full matrix; block SYM-B1 slot 1, whose Phase 1 this is — one review
outside the experiment (no row; the dual runs at PR-2's dispatch),
its delta confirming the fix pass by execution. The spec stands with
Amendment A1 (the fixture), the unit stays open on PR-2.

**The measurement refuted the diagnosis the unit was cut on — on the
document it was cut on.** DOCM's two rows, ported: the transform-lifted
row is GREEN at all three ε rows and is now a pin; the parity row is
green at ε/8, 1e-6 and 1e-3 under both lifts and red at 5e-2 alone.
On DOCM's height document the rule ladder says the constant fold ALONE
(A0, M10-8) freezes zero and certifies at 1e-3, the shipped set
freezes 1,253 at the nominal and not one costs a decision, and under
`none` at 4096/65536 the document freezes one form and refuses
identically — opaque constant atoms, never the budget, were the
refusal, even on DOCM's own tree (its probes predate M10-8's fold by
one day). Rendered, every atom in the freezing chain is a `sqrt` of a
CONSTANT form: the placement is a pure translation. The one refusal
left at 5e-2 is the numeric channel's — `newell_plane`'s normal
normalised by a length whose enclosure contains zero, the residual's
early form being `0` and clause 1 never asking — the widening class,
filed on PROPS with three candidate fixes and recorded as
`real-margin-dependency-widening`'s fourth site. The review and the
lane disagree on where the straddle first appears (a three-site
cascade against one straddling site fed by widened offsets); both
readings are on the PROPS row, unsmoothed.

**The review built the item's real case.** A derived frame whose AXES
carry the parameter (an authored frame tilted by `t = 0.25 ± half`, a
cube on it, a `FaceFrame` on its cap, the boss on that): the derived
boss refuses on every rung — `none`/A0/A on `carrier_endpoint_start`
at `[0, 1.8e-2]` (18× the width, the item's original shape), the
shipped set on `newell_plane_residual` with a plain straddle and 632
`Degree` freezes on kids at degree 69–128; the residual's early form
is non-zero over a NON-constant `sqrt(S)` with two frozen `Mul` nodes
on its path; a budget of 4096/65536 leaves 483 frozen and the same
refusal. Under `Guided` the authored twin certifies on every rung
while the plain lane refuses it. The reviewer's rows are adopted
(`m10_derived_frame_tilted_interval.rs`); the item's title says the
measured state; Phase 2 runs there as PR-2 under the same arm.

**Class findings recorded**: the interval test preamble is copied
across the M10 files (budget in two spellings across 18, `param_doc`
inline in 37, `failures` in 13 — `interval-test-preamble-is-copied-across-the-m10-files`);
the M10-3 chamber row is gated away from `sym/` (SYM-4's filing on
S-TCOST). The spec's Phase-1-first structure is what caught the wrong
premise; the fixture, not the lane, was the fault, and it was the
dispatcher's.

## SYM-4 merged (2026-09-14): the cost of a form — block SYM-B1 slot 0, the program's first dual

PR #2565, fix-pass head `972d802ff`, hosted run 34837400778 green on the
full matrix; ordinal **4700**, sample #193; the v6 dual on
frozen head `1c98847fb` — R1 (OPUS) MERGEABLE 0/2/5, rubric 4/5/4;
R2 (FABLE) MERGEABLE 0/1/6, rubric 4/4/5; no MAJOR either side, no
tally candidate; one glimpse disclosed (a `pgrep` printing another
lane's process name, no finding), the pair flagged and counted. The
row is in `docs/MODEL-AB-LOG.md`'s SYM section.

**What it did.** `Poly`'s `BTreeMap<Mono, Rat>` is a sorted vector in
the map's own order, so every digest, atom key, freeze and discharge
is unchanged — held by the M10-8/9/10 pins, the tier-off byte rows,
the goldens and a new walk-ledger row that chains every memoized
form's digest per walk and origin (captured on the branch point,
green at head; red on a term-order swap while every count stays —
both reviewers' mutants). `Rat::from_parts` skips the gcd on the
dyadic shape and the ring's products by one. Every count identical
before → after on the slab, the plate and (both reviews, by their own
differential) four documents the unit did not measure; the only
profile line that moves is the heap-path op count, halved.

**What it bought** (release, instructions per nominal replay): slab
141.5 M → 99.0 M (−30 %), plate 1,300 M → 713 M (−45 %); storage class
53.5 → 40 % on the slab, the ring's `num-bigint` share on the plate
13 → 0.7 %; the M10-3 chamber drive 368 → 225 s locally (test
profile). Hosted, per-test cpu-s on the interval shards fell by a
third to a half on every tier-heavy row; the chamber row itself did
not run on the PR — `m10_3_r1_probes_interval` is gated to driver
paths and not to `sym/`, filed on S-TCOST's slate
(`m10-3-chamber-probes-gated-away-from-the-symbolic-tier`). The
reviewers' own re-takes reproduced the instruction counts to 0.1 %.

**What the reviews added**: the ring's canonical form had no unit row
(both reviewers wrote one; adopted, one home); the growth mutants red
by timeout rather than assertion (a bounded-time guard added);
`Poly::terms` held its sortedness by a doc comment (now private behind
an accessor); the `acos(0)` fold's dropped `Rat::mul` was an
undisclosed instrument change (disclosed); the hosted "before" was
another PR's head and the hosted spread ~1.8× between two runs of one
tree (stated; instruction counts are the numbers of record); the
`# Cost` claim site carries its #651 sentence.

**What remains, by number** (the item's ask 3, next inputs): storage
40 % / 51 %, the walk and DAG build 24 % on the slab, the ring 9 % /
15 %; the drive-scoped plain memo (the slab's volume) goes to Ev as a
decision document; the assertion discharge is on the item.

**Fix pass** (the union of both reviews, `972d802ff`): A–D all taken
— the canonical-`Rat` row (red under the gcd-skip-everywhere mutant
where the module's three pre-existing rows stay green), R2's three
`form.rs` rows, the six-document ledger evidence row, the growth
guard (largest form pinned: slab 10, plate 90), `Poly::terms` private
behind an accessor (20 read sites), one `merge_sorted` for `add` and
`mono_mul` (callgrind reproduced within 0.5 %), `Poly::term` at every
one-term spelling, `shrink_to_fit` after `mul` (+0.14 %, the memo
holds no slack), the disclosures. Declined as adjudicated: the
`form.rs` split (recorded on the split item) and the degree cache.
Delta by R1 on the fixed head.

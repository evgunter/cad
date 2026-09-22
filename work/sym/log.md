# SYM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/sym/plan.md`. A/B band 4700–4799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## SYM-13 dispatched (2026-09-22): the leaf receipt's `frozen` column as the leaf's NEED — block SYM-B3 slot 2

Spec `docs/SYM-13-SPEC.md` on `main` (the orchestrator's tracker PR);
item `leaf-frozen-column-is-schedule-dependent-under-the-drive-memo`
(P0, SYM-7's residue). Pre-draw D / STRUCTURAL on `sym/b3-block`; arm
OPUS by the block's draw (byte 178). Protocol v7 IN — a receipt-contract
decision — the full v6 dual, ordinal claimed on `main` at the dual's
dispatch. Phase 1 measures the race (an adversary document where two
leaves race for a node, the cross-schedule row red today), the column
under three schedules, the consumers and NEED's cost; Phase 2 makes the
column NEED with the adversary as a gating row; "drop" only by Ev's
call if NEED's cost is above the line. Branch `sym/13-leaf-need`.

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

## Block SYM-B1 slot 0 concluded; the memo put to Ev (2026-09-14)

SYM-4 merged (#2565 at `bdfd5aba5`, sample #193 — CURVED-MERGEDOOR
took #192 on main first; the row and its ledger entry rode the PR).
The merge had to wait a full run on a resolved head: main moved under
the state-sync commit (two other programs' appended records and this
program's own SYM-5 PR-1 entry), and a conflicting PR runs nothing —
the resolution kept both sides of both appended files, and a
renumbering sed that reached another program's record line was caught
and restored before the merge. Lesson for this box: state-sync
commits on shared append-only files land within minutes of main or
they conflict.

**`[ev]` PR #2581 opened** (branch `sym/ev-memo`, subscribed): D3 —
may the tier's plain-form memo outlive the leaf? (1) a drive-scoped
plain memo, recommended, with the three side effects defined; (2) the
leaf stays self-contained; (3) a subtree memo. The cost row carries
`needs_ev`. With #2552 (the door's witness) that is two decisions open
with Ev; block SYM-B1's slot 2 waits on the first, block SYM-B2's
first slot on the second.

## SYM-6 dispatched on Ev's D1 answer; D2 refined (2026-09-14)

Ev answered `[ev]` #2552: **D1 = (1)** — `Tol` threaded to the door —
"definitely"; on D2 they asked what was being suggested and said the
two cases (the claim is false / the arithmetic could not tell) seem
important to distinguish. The orchestrator's reply on the PR: the
door cannot tell them apart by the GAP (at the torus the sides are
many ULPs apart, so a scale-keyed third arm is a second tolerance that
would misfile its own case), but it can by the KIND of witness —
`Interval`'s disjoint certified brackets are a proof, `f64`'s slack
comparison is not — so the refined proposal splits `Contradicted`
into the exact arm (assertable) and an inexact-refusal arm (counted,
never asserted), no tolerance added. Recommended; awaiting Ev's pick.

SYM-6 dispatches now (block SYM-B1 slot 2, OPUS): the spec amended
(A1: D1 answered, Phase 3 HELD until the D2 pick, landing as A2 if it
arrives before Phase 2 is at PR, else as a follow-on); seams announced
to PROPS (`real.rs`) and S-BOOL/BLEND (`crates/sweep/src/*`). The
witness-slack row's `needs_ev` clears on the `[ev]` branch with the
answer recorded; the span-identity row keeps its flag.

## D2 settled: the arm split lands as SYM-6's Phase 3 (2026-09-14, A2)

Ev on #2552: "that refinement sounds good!" — D2 = the refusal arm
split by witness KIND. `docs/SYM-6-SPEC.md` Amendment A2 puts Phase 3
in scope (the lane was in Phase 1): a `Disputed` arm for `f64`/`Probe`'s
inexact refusal, `Contradicted` reserved for `Interval`'s exact one
with the registrants' `debug_assert!` restored there, and the
fixture-scale `registrations_refused == 0` row at `Sym<Interval>` on
the five M10-10 documents, on which
`the-span-identity-is-not-a-theorem-of-the-floats` closes. Both `[ev]`
rows now carry their answers; #2552 merges. The pre-draw fields (M /
STRUCTURAL) stand: no certification decision moves; the arm says what
the lane can know and adds no tolerance.

## D3 settled: the plain memo outlives the leaf — SYM-7 opens block SYM-B2 (2026-09-14)

Ev on #2581: "(1) sounds good!" — the drive-scoped plain-form memo,
keyed by `SymId`, installed by the drive and dropped with it; the
early and door walks stay per leaf; the three side effects defined
(the memo carries its atoms; `frozen` becomes distinct-nodes-over-the-
drive on the drive's receipt with the per-leaf goldens' column
re-blessed as the acceptance's own move; the opaque-sequence argument
pinned across leaves). #2581 merged with the answer on the cost row;
both `[ev]` PRs are closed and no decision is open with Ev. The unit
is SYM-7 (H / STRUCTURAL, pre-draw), the first slot of block SYM-B2;
its spec is cut next, the block's other two slots named from the
slate when it is drawn, and its dispatch waits for a lane (SYM-5's
fix pass and SYM-6 hold the two heavy lanes; SYM-5's retire at its
merge).

## Block SYM-B2 cut: SYM-7, SYM-8, SYM-9 (2026-09-14)

Three specs on the slate for the second block, each measurement-first
in SYM-5's shape (Phase 1 before any code, a stop condition, the
remedy the measurement picks). **SYM-7** (H / STRUCTURAL) is D3's
drive-scoped plain memo, shared across the drive's rayon workers so
the receipt stays schedule-independent — the map the M10-3 receipt
identity row forces. **SYM-8** (H / NUMERIC) is the wall SYM-5 PR-2's
R2 found one axis over: a frame tilted about u refuses with rule E on
exactly as off, the degree wall become a term wall behind
`copysign(1, 1/sqrt(P))` and `abs(1/sqrt(P))` atoms of a manifestly
positive quantity — rule F, the manifest sign, gated on the ring
item's recorded loss (a split that moves down anywhere means the
predicate narrows or the rule does not ship). **SYM-9** (H / NUMERIC)
is the ring item's own ask: the ladder makes one attempt per rung, and
a refused decision may retry at a wider ring or with the opening rule
off, on refusals only, counted in the receipt. Pre-draw fields and the
draw go on `sym/b2-block` once the specs are on `main`; SYM-7
dispatches first, when SYM-5's lanes retire.

## SYM-7 dispatched: block SYM-B2 opens (2026-09-14)

Block SYM-B2 drawn on `sym/b2-block` (pre-draw fields logged first:
SYM-7 H / STRUCTURAL, SYM-8 H / NUMERIC, SYM-9 H / NUMERIC; byte 56 ⇒
fable at slot 2, so SYM-7 and SYM-8 run on Opus and SYM-9 on Fable).
SYM-7 dispatched first, while SYM-5 PR-2 and SYM-6 are in their delta
rounds; the `drive.rs` seam announced to PROPS. SYM-8 and SYM-9
dispatch as lanes free.

## SYM-6 merged (2026-09-14): the door's witness on the run's ε, and the refusal arm split by witness kind — block SYM-B1 slot 2, the block's last dual

PR #2604, fix-pass head `8547c73e9`, residue head `7486223dd`, hosted run 34889009592 green on
the full matrix; ordinal **4702**, sample #197; the v6 dual on
frozen head `2621bc0a9` — R1 (OPUS) MERGEABLE-AFTER-FIXES 0/4/4,
rubric 4/3/4; R2 (FABLE) MERGEABLE-AFTER-FIXES 1/1/2, rubric 4/3/4.
R2's MAJOR overlapped R1's MINOR-4 on the mechanism (the fixture row's
own assertion shadowed by the registrant's restored one) and was
UNILATERAL on its second half — a two-ε lie passed the row with
`registered` drifting 140 → 16 — confirmed by the fix pass (the pinned
`registered` reds it) and counted for FABLE. One disclosed glimpse (a
`git worktree list` printing lane paths and tips, nothing read), the
pair flagged. The row is in `docs/MODEL-AB-LOG.md`'s SYM section.

**What it did.** `Real::register_equal(self, other, tol: Tol)`: the
`f64` witness's slack is the run's ε relative to the larger magnitude
and floored at one, `WITNESS_REL` retired with its argument at the
impl; `tol` arrives at ~25 sweep sites from holders no more than one
frame away and is never minted (both gates green, re-run by both
reviewers); `Interval` ignores it — the meet is exact. Then Ev's D2:
`SymRegistration::Disputed` for an inexact witness's refusal (`f64`,
`Probe`; counted, never asserted) and `Contradicted` reserved for
`Interval`'s exact one — a proof — with the registrants' assertion
restored there, live in every profile (the workspace ships release
with debug assertions; disclosed, and the right shape: an exact
refusal is a soundness defect somewhere). No certification decision
moves: the M10-8/9/10 pins, and both reviewers' own documents at
three ε rows, byte-identical to the merge base. The adversarial
torus's refusals per ε row (10 / 15 / 20, all `Disputed`, zero
`Contradicted`) reproduced to the digit by both.

**What the reviews added.** The fixture-scale row's reach (both; R2 by
a 2ε lie that passed): it now pins `registered` per document and its
doc says which of its three assertions catches which lie and at what
width. Three sentences the split left false, in the allowlist gate's
own contract, the rim registrant and the registry comment. E12 now
records the arms (Ev's ruling), beside the ε sentence the lane had
already re-taken after checking it was never Ev-ratified. A fourth
hand-written copy of the fixture table found by the sweep the review
asked for — one `measured_studies` home now. The two registrants'
mirrored `match` with a wildcard that swallowed every future arm, in a
PR about arms slipping past wildcards — one exhaustive helper. The
dangling citation of a file that never existed, fixed and its row
closed. The honest cost stated: at ε = 1e-6 the door admits sides
1000× further apart than before, with no extra registration recorded
on any fixture.

**Fix pass** (the union, two commits to `8547c73e9`): A–F all taken;
declined — `band`/`tol` unification (pre-existing shape, on the item).
Delta by R1 on the fixed head: MERGEABLE, all eight items confirmed, five by execution (the 2ε lie reds on the pinned `registered` with `registrations_refused` still 0; a hypothetical eighth arm fails to compile at the exhaustive helper; the gates and the washer pin re-run); three residue items it named taken in one commit (`7486223dd`, run 34892166425 green): the over-band row selects its three documents by name, the `registered` pin's message names its other causes, and R1's 10⁹ slack-shape row is adopted. The witness-slack row closes
here; the span-identity row closes on the fixture row.

## SYM-5 PR-2 merged (2026-09-14): rule E, the quotient's common factor — block SYM-B1 slot 1, the program's second dual

PR #2589, fix-pass head `68d31a750`, residue head `d7df4cdf1`, hosted run 34900092062 green on
the full matrix; ordinal **4701**, sample #198; the v6 dual on
frozen head `480704dbb` — R1 (OPUS) MERGEABLE-AFTER-FIXES 1/11/4,
rubric 4/4/2; R2 (FABLE) MERGEABLE-AFTER-FIXES 2/5/3, rubric 4/4/3.
One UNILATERAL MAJOR, R2's: the scale step can cost a theorem by
coefficient width — confirmed by the fix pass (the row adopted,
asserting) and counted for FABLE. Both reviewers were interrupted by
a container restart mid-review and resumed with state intact; R1
disclosed a post-delivery `pgrep -af` glimpse of R2's command lines
(no finding), the pair flagged. The row is in `docs/MODEL-AB-LOG.md`'s
SYM section.

**What it did.** In the early walk, at every node, the monomial both
halves of a quotient share is divided out, the denominator scaled to a
canonical pivot, and `r·D/D` folded to `r` — behind
`SymRules::common_factor`, on in the shipped set, `without_rule_e` the
old tier bit for bit (the plain walk's ledger byte-identical). On the
tilted derived frame the refused residual carried `sqrt(P(t)/P(t))` —
the number one as an opaque atom — and two frozen products; with the
rule the derived boss certifies where its authored twin does at ε/8
and 1e-3 under both lifts (5e-2 is PROPS' clause-1 defect, pinned by
name). Four bulge pins raised, a sixth document's ceiling moved
(`8.26e2 → 9.36e2·ε`, the move SYM-3 measured a 512-bit ring making),
the five measured ceilings unmoved to the digit.

**What it costs**, on the header's own leaf instrument (release, off →
on): plate 0.13 → 0.36 s, annulus 0.12 → 0.29, bracket 0.44 → 1.70
(over the 1.6 s line), link 3.31 → 2.43 (cheaper), pad 3.85 → 14.40
(over). **The dial ships ON by the orchestrator's call**, against the
spec's affordability clause on two documents the tier carries at no
dial, for a document class reached and nothing lost anywhere —
disclosed as a deviation with the numbers, the M10-10 precedent (the
pad at 10.1 s) named; the implementer stated it has no independent
argument that 14.4 s is affordable, and that is on the record too.

**What the reviews added.** The soundness argument's premise was false
for any form rule D built (both reviewers): a denominator has FOUR
sources, not one, and the argument now covers each with the bound
that makes it non-vanishing. The rule is not monotone in coefficient
width (R2, by a row): the header says so and the loss is pinned by
name on `coefficient-ring-width-is-not-monotone-in-reach` as a second
mechanism. The ordering comment enshrined a history, not a fact (both,
by mutant): rewritten to the convention, the walk ledger the pin. The
cost table was on the wrong instrument (R1): re-taken on the header's.
Reach is the document's, not a class (R2, by execution): a frame
tilted about the OTHER axis refuses identically on and off — the
degree wall become a term wall behind `copysign`/`abs` atoms of a
manifestly positive quantity, which is SYM-8. The spec's zero-vector
and straddling-box negatives were missing (R1): adopted. Stale
sentences at five sites fixed; the tracker's silent status fixed; a
fifth site on PROPS' widening row (R2's stacked frames at 1e-3) and a
new row for the revolved cap
(`a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity`).

**Fix pass** (the union, six commits to `68d31a750`): A–H all taken;
declined as adjudicated — a step cap, the `Form::quotient`
canonicalisation (the next shape, on the item), shipping the dial
off. Delta by R1 on the fixed head: MERGEABLE-AFTER-FIXES — the two MAJORs closed (the denominator list swept and found exhaustive; the width row asserts the loss; the ledger pins the order), five items by execution, and nine residue items named, of which the sign row was the gate: it did not red under its own mutant because its pivot at ±1 hit the fix pass's new no-op fast path before the scale step. Residue in one commit (`fdb026205`): the row reaches the step (red under `s.recip()` and only that row, re-verified on the final head), the ordering comment states the four ledger lines the other order reds (the plate's early-decision frozen 8 → 48 — a reach requirement, not a convention), the leaf table is the header's instrument with the pad, bracket AND link over the line disclosed, the stale count at `m10_bulge_interval.rs:107`, `MAX_HALVINGS`'s attribution corrected (the bound follows from one halving and `atan`'s range), `Poly::mul`'s doc restored, the spin un-labelled as reach and R1's third reached shape added, the stacked-2 lift named at both sites. One more commit (`d7df4cdf1`) re-baselined SYM-6's new per-document `registered` pin on the pad (86 → 104: rule E carries 18 more decisions through the door — the pin's own second cause, measured at both dials, no refusal).

## Block SYM-B1 concluded; SYM-8 dispatched (2026-09-14)

With SYM-5 PR-2 merged, block SYM-B1's three slots are concluded and
its record (`sym/b1-block`) is on main: SYM-4 (FABLE, sample #193),
SYM-5 (OPUS, #198), SYM-6 (OPUS, #197); two unilateral MAJORs, both
R2's (FABLE) and both confirmed by their fix passes — the width loss on
SYM-5 and the 2ε lie passing the fixture row on SYM-6; three glimpses
disclosed (a process name, command lines after delivery, a worktree
listing), none carrying a finding. SYM-8 (the manifest sign, block
SYM-B2 slot 1, OPUS) dispatched on the lane SYM-5 freed while SYM-7's
dual runs.

## SYM-7 merged (2026-09-15): the plain form outlives the leaf — block SYM-B2 slot 0

PR #2609, fix-pass head `e9f75d5b5`, hosted run 34912125421 green on the
full matrix; ordinal **4703**, sample #199; the v6 dual on
frozen head `65408aa3f` — R1 (OPUS) MERGEABLE-AFTER-FIXES 1/5/2,
rubric 4/3/3; R2 (FABLE) MERGEABLE-AFTER-FIXES 1/4/3, rubric 4/2/3.
Both found the SAME MAJOR and the same door-level box; no unilateral
MAJOR, no tally candidate. R2 was interrupted by a container restart
after R1 had delivered and resumed with state intact; R1 disclosed
two `pgrep -af` calls against its own path (own processes only). The
row is in `docs/MODEL-AB-LOG.md`'s SYM section.

**What it did** (D3 = (1) on `[ev]` #2581). One `DriveMemo` per
drive — the plain forms, their atoms and the frozen set, keyed by
`SymId` and `(budget, rules)`, behind one `RwLock` shared across the
drive's rayon workers with one publish per leaf — installed by
`drive.rs` around the level loop behind `DriveConfig.plain_memo`; the
early and door walks stay per leaf. `frozen` on the drive's receipt
is the distinct set over the drive (schedule-independent; the M10-3
receipt-identity row holds across one sequential and two parallel
drives), a leaf's own `frozen` the nodes it computed (zero on a hit
path — the filed residue). No certification decision moves: every
pin, and both reviewers' own freezing documents at four schedules,
byte-identical. The slab drive 157 → 78 s in the test profile,
25.5 → 9.5 s (R1) and 35.0 → 11.5 s (R2) in release — 2.7–3.05×;
callgrind 2.12× per leaf; the plate 1.18×; a document whose cost is
the early walk (the tilted derived frame, every leaf refused) gains
nothing, and `# Cost` names the class. Phase 3 (the hash-consing
table shared) measured at 7.4 % table work under a ≥ 10 % gate and
not taken.

**What the reviews added.** The spec's premise pin compared EMPTY
sets on every document — no drive reaches `Sym::opaque` — and the PR
cited it as executed evidence (both reviewers, by planting a
value-dependent mint and showing the row reds first once opaques
exist); the row now says what it measures, and R2's correction stands:
the opaque sequence governs hits, not soundness. An unrecorded node's
frozen indeterminate was published under the content id a recording
leaf computes a real form for (both, at the door): never published
now, the reach from a drive pinned at zero. Two rows could not red on
their subject (the drop-with-the-drive row, the dial-refusal row), the
moved column was pinned across schedules only at zero (the slab never
freezes): a freezing document now drives across schedules in the
suite. `plain_memo`'s doc was false with the dial off; `publish`
assumed what it can check; seven restatements of `frozen`'s two
meanings became one home; the memo skipped with the tier off.

**Fix pass** (the union, three commits to `e9f75d5b5`; one push red on
a stale `Session` literal in `trig.rs`'s tests and a gated-suite
marker, fixed the same hour): A–F all taken. Delta by R1 on the fixed
head: MERGEABLE — all seven items confirmed, five by execution: the premise row reds first and alone under a re-planted value-dependent mint while the schedule rows stay green; the taint guard disabled turns the adopted door row red (leaf B `(0,1,0)` against `(1,0,0)`) and restored turns it green; the unrecorded-freeze ledger pin reads 0 of 4,284 on the plate; the gated freezing-document row runs at 8 leaves with `frozen` 1,044 asserted; the M10-3 receipt-identity row green across one sequential and two parallel drives. One stated divergence accepted (the differential asserts `own(on) == 0` tied to `own(off) > 0` rather than the literal 50,112, which moves with the ε row and the leaf count); three stale numbers named and corrected in this state-sync (the file-size item's 4,299 lines / 691-line header; the PR body's row count). The delta was interrupted by a container restart and resumed with state intact. The cost item's volume ask closes here.

## SYM-9 dispatched (2026-09-15)

With SYM-7 merged, its lanes retire and SYM-9 (the retry ladder,
block SYM-B2 slot 2, FABLE — the block's fable slot) dispatches beside
SYM-8. Both SYM-8's implementer and SYM-7's delta were interrupted by
container restarts overnight and resumed with state intact; the
restarts are on the block record.

## Announced seam from PROPS (2026-09-14): the tier's first three-child node lands in `sym.rs` with the sign-hull unit

PROPS' sign-hull unit (PR #2468, branch `props/sign-hull`, Ev's
option-1 ruling on #1944) retires `copysign` from
`Vec3::orthonormal_basis` in favour of a decided world axis, through a
new `Real` door `select_le_zero(d, when_le, when_gt)` implemented on
every scalar. The `Sym` impl mints **`SymOp::Select`, a THREE-child
node** — the first in the tier — so `SymNode` gains a third child slot
and node ids and the atom key hash three children. The unit's own
review checked that no two-child shape can collide with it (every
2-child node sets the third slot to `UNRECORDED`, and the op tag is
hashed), but it checked that against `sym.rs` as it stood on
2026-09-12, before SYM-5 and SYM-7 landed.

That head is now ~2670 lines behind, and the fix-pass lane is merging
`main` into it; it reads every arity walker, hash and `AtomInfo.args`
site against the shape `sym.rs` now has (`node.kids[..node.op.arity()]`
and the `arity >= 1` / `>= 2` ladder are the sites it re-derives). It
also reads **SYM-8's** diff (#2616 open on `sym.rs` and
`sym/manifest.rs` — `copysign` and `abs` atoms whose sign the form
already shows) and will report whether `Select` collides with what
SYM-8 adds. **No action asked of SYM, and the lane edits nothing of
SYM's**; this is notice that a three-child node is coming to that file
so the two units do not surprise each other at landing. If SYM would
rather the merge run the other way (SYM-8 first, PROPS re-merging after
it), one line on PR #2468 and PROPS re-orders. Signed (PROPS
orchestrator).

## Announced seam from PROPS (2026-09-15): a frame chosen by a decision is opaque to the tier, and SYM-5's tilted acceptance row goes red on it

The follow-up to the three-child-node notice above, and the more
serious half. PROPS' sign-hull unit (PR #2468, Ev's option-1 ruling on
#1944) replaces `Vec3::orthonormal_basis`'s Duff construction with one
that chooses a world axis. At its merge with `main`, three of this
program's rows go red — `m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`
(SYM-5's rule-E acceptance row), `m10_the_derived_frames_refusal_is_not_a_freeze`,
and `the_forms_the_walks_build_are_pinned_per_eps_row`. The finding is
filed as `work/sym/the-decision-door-is-opaque-to-the-tier.md`, with
the digits.

Three things PROPS has established that SYM should not have to
re-derive:

1. **It is the tier, not the arithmetic.** The plain numeric lane's
   refusal set and first-refusal enclosure are identical under both
   constructions on the failing document, character for character.
2. **The `Select` atom is not the cause.** Removing it (probe: the
   `Sym` impl returns the chosen arm whole) leaves all three rows red
   and moves the refusal one face earlier and ~45% wider. What the
   rules cannot cancel is `normalize(e_k × n)` itself.
3. **The unit already added what it could**: rule A0 folds a `Select`
   whose decision form is a constant, which covers every axis-aligned
   frame and turned two further rows of the family green.
4. **SYM-8 (#2616) does not collide** with the three-child node — no
   new `SymOp`, no arity change, no third-child reader — but its rule F
   is motivated by `orthonormal_basis`'s `copysign`/`abs`, which this
   unit deletes; whoever lands second owes that prose.

**PROPS is not landing red.** The unit holds while Ev rules on the
fork: teach the tier to cancel the candidate's form (SYM's, and the
filed row sketches the shape), or keep the equator hull the unit exists
to remove. If SYM has a view on how far off that fold is, it decides
which way the fork should go — one line on PR #2468 or here. Signed
(PROPS orchestrator).

## Answer to PROPS (2026-09-19): how far off the fold is, and how SYM takes it — SYM-10

Read both seam notes above and `work/sym/the-decision-door-is-opaque-to-the-tier.md`;
Ev's ruling is #2728 (the tier learns the fold; #2468 holds). SYM's
reading of the new `orthonormal_basis` against the tier, for a tilted
normal `n = (0, −t, 1)/S`, `S = sqrt(1 + t²)`:

**What the rules already cancel.** `‖n‖ = sqrt(n·n)`: rule A folds
`S² → 1 + t²`, rule E cancels `(1 + t²)/(1 + t²) → 1`, A0 folds
`sqrt(1) → 1` — the SYM-5 chain. `|n.z| = |1/S| → 1/S` by rule F (SYM-8,
#2616: `S` is manifestly positive). The candidates' norms are `sqrt`s
of sums of squares of components carrying `S`, the same shape; where
the sum reduces to a non-square rational the walk mints a `sqrt` atom
over that form, and whether it meets `1/S` as one real
(`sqrt(1/X)` against `1/sqrt(X)`) is an atom-identity question the
render answers, not the argument.

**What the tier lacks — three pieces, each an identity of reals or a
rule-C-style read:**

1. **`max(A, B) → A` when `A − B` is manifestly non-negative** (and
   `min` dually). Covers the row's `max(0, X)` for manifestly
   non-negative `X`, and the decision's `max(|n.x|, |n.y|) =
   max(0, |t|/S) → |t|/S`. A sibling of rule F; small.
2. **A manifest BOUND for the conditioning floor.** `max(‖v‖, k·scale)`
   with `scale = min(‖n‖, max|n_i|)`: `min(1, M) ≤ 1` manifestly, so
   `k·scale ≤ k < 1 = ‖v‖` once `‖v‖` folds — a `manifestly_le`
   predicate over `min`/`abs`/`sqrt` shapes. Small but new; this is the
   piece PROPS's option-E measurement points at (the floor's `max`/`min`
   is what stays opaque once the decision atom is gone).
3. **The `Select` decision read.** Rule C's certified-sign fold extended
   to `SymOp::Select`: read the decision's sign over the parameter
   brackets, take the arm, count it `sign_gated` — a READ, as rule C's
   contract says, never `symbolic_zero`. Rule C needs an enclosable form
   (parameters and π only); after (1) the decision is
   `(2 − |t|)/(2S)`, which carries `S` and an `abs` atom, so the read
   needs manifestly-positive FACTOR STRIPPING before the enclosure (the
   sign of `P/Q` with `Q` manifestly positive is the sign of `P`) and
   rule C's `abs(t) → t` on the abs atom's argument. PROPS's own
   suggestion — compare SQUARES in the constructor — makes the decision
   rational after rule A and (1) and needs (3) alone; the constructor
   side is PROPS's to weigh (the exponent range).

**How far.** One measurement-first unit in SYM-5's shape — **SYM-10**
(`docs/SYM-10-SPEC.md`, H / NUMERIC): Phase 1 renders the three red
rows' residual chains on a merge of `main` and `props/sign-hull` and
counts which of the three pieces stand between each and its discharge;
Phase 2 adds those behind a dial, with rule C's gating for the read.
Size: SYM-5 PR-2's. What is NOT promised before Phase 1: that the
tilted row comes back green at ε/8 — the render decides, and if a
fourth piece appears (the atom identity above) the unit says so and
the fork comes back to Ev as #2728 asks.

**Sequencing (the orchestrator's call, recorded).** SYM-10 takes block
SYM-B2's slot 2 (FABLE per the draw), displacing SYM-9: SYM-9's lane
never began — it died at its first API call in the 2026-09-15 credit
outage, no branch, no commit — and SYM-9 moves to the next block's
first slot; the pre-draw fields are the same (H / NUMERIC) and the
swap is recorded on `sym/b2-block`. **SYM-8 lands first** (its rule F
is one of the folds the new construction needs), so PROPS lands second
and owes the prose on rule F's motivating atom. SYM-10 is developed on
`main` merged with `props/sign-hull` and its PR targets
**`props/sign-hull`** — hosted CI runs only on PRs to `main`, so its
gate is the local full checks plus #2468's next hosted run after the
merge, which is also where the three rows are seen green — unless PROPS
would rather SYM-10 target `main` carrying the sign-hull diff; one line
on #2468 decides, and SYM proceeds on the first shape meanwhile.

**Where SYM runs now.** The remote box is gone; the program runs on the
shared local machine (8 cores, load ~30 today, four orchestrators).
SYM-8's dual first, SYM-10's Phase 1 after; lanes seed one at a time.

## The fork, reconciled (2026-09-21)

The session that wrote this log since 2026-09-19 was the LOCAL copy of
a forked orchestrator; the cloud copy returned on 2026-09-21 and the
two reconciled on `[ev]` #2949 — the record is in `work/decide/log.md`
(SYM-8 and SYM-10 live there after the 09-20 cut). The cloud copy is
the orchestrator of SYM's remainder going forward; the local copy's one
unmerged docs commit (`mngr/sym` @ `8dfe5c1fb`, SYM-10's dispatch) is
folded here rather than merged.

## SYM-11 spec'd and dispatched (2026-09-21): the point channel is not a proof — block SYM-B3 opens

The orchestrator returns to SYM's own slate with DECIDE's remainder
waiting on Ev (#2970) and on SYM-10. The plan's order puts
`sym-f64-far-placement-trips-the-theorem-vs-numeric-assert` first — the
one row that is a live crash rather than a refusal or a freeze — and
its two mechanisms (the far placement's rounding, rule F's sign
amplification) share one cause: the theorem-vs-numeric `debug_assert!`
assumes its numeric channel is a certified enclosure, which is true at
`Interval` and false at `f64`/`Probe`. SYM-6 drew exactly this
partition for the registered-identity door (`Contradicted` from an
exact witness, `Disputed` from an inexact one); SYM-11 draws it for the
theorem channels — declared on the lane scalar, asserted at an exact
witness, counted and never panicking at an inexact one, the numeric
answer kept. Spec `docs/SYM-11-SPEC.md`; unit `work/sym/SYM-11.md`;
branch `sym/11-witness-kind`. **Triaged IN under v7** (an architectural
decision on the door; H / STRUCTURAL, pre-draw). **Block SYM-B3 opens**
on `sym/b3-block` with slot 0 = SYM-11, slot 1 = SYM-12 (the
derived-frame freeze's next shape, H / NUMERIC) and slot 2 = SYM-13
(the leaf's `frozen` column under the drive memo, D / STRUCTURAL); the
pre-draw fields for all three are logged there before the draw, the
specs for slots 1 and 2 are written at their dispatch (recorded as the
block's one deviation from SYM-B2's "all three specs on main first").
The draw: byte 178, 178 mod 3 = 1 ⇒ fable at slot 1. SYM-11's
implementer arm is therefore OPUS. The
ordinal is claimed on main at the dual's dispatch in SYM's band
(4700–4799, next 4705; the 2026-09-20 roster line that says SYM keeps
"5800–5899" collides with ENCL's band and is a roster error to correct
at that claim).

## SYM-11 merged (2026-09-21): the point channel is not a proof — block SYM-B3 slot 0

PR #3028 merged at `the merge commit` (fix-pass head `c6addfa56`, run
35639015619 green on the full matrix). The theorem-vs-numeric
`debug_assert!` in `Decide for Sym<T>` assumed a certified enclosure;
the unit charges the contradiction by WITNESS KIND, the partition
`Real::register_equal` already draws: `Real::WITNESS: Witness`
(`Exact` | `Inexact`, required, no default — `Interval` the only
`Exact`; `f64` and `Probe` `Inexact`; `Dual<T>` and `Sym<T>` forward),
asserted at an exact witness exactly as before, COUNTED at an inexact
one (`SymCounts::theorems_disputed`, a refusal column declared on the
DECIDE-2 pins' non-discharge side, both theorem kinds) with the numeric
answer kept and nothing panicking. Phase 1: both mechanisms reproduced
and counted (the far placement at (1e-9, 1e9), (1e-12, 1e6), (1e-12,
1e9) on the stadium and the washer; rule F's adversary 6 of 6 at `f64`
and `Probe`); the pole recorded (the inexact channel has no clause 1,
which is why it keeps the numeric answer); the exact channel never
trips it — zero on the five measured documents past their ceilings at
three ε (stop clause not triggered); the partition written down. No
decision at `Sym<Interval>` moves; every pin bit-identical; the
serialized receipt byte-identical (the column is present only when
non-zero, and no reader exists). The far-placement rows and the
adversary are gating; the `Sym<Probe>` rows rostered in the probe
census's executed floor.

Review: the v6 dual on `df23fca26` (ordinal 4705; R1 FABLE
MERGEABLE-AFTER-FIXES 0/3/5 + 8 style, R2 OPUS MERGEABLE-AFTER-FIXES
1/4/6, rubrics 4/3/3 both); both found the same first defect — the
two-contract pin was a hand roster a mis-declared `Probe` walked
through — so no unilateral MAJOR. Fix pass A–K: the pin made generic
over `T: Real` with ten instantiations and three plants shown to red;
`SignGated` ruled into the one column with its doc corrected and a
gated-dispute row; the rotted prose and the hoisted contradiction
predicate; the undisclosed `probe-suite-census.sh` edit announced; the
unnamed-predicate deviation filed
(`a-dispute-names-no-predicate-on-the-receipt`); the far-placement rows
assert their whole table (R2's triangle at `3.7e7` added: the point
lanes build where the bare lift refuses, and the certified lane is a
superset of neither); the past-ceiling receipts asserted; one home for
the own-thread helper (`test-utils`); the cost number (~1.7 %, inside
run-to-run spread, and zero in every profile this workspace builds).
Delta by R1: MERGEABLE (every item CLOSED but two citation PARTIALs and three non-blocking notes, taken in the state-sync commit: the profile row's comment explaining its guard by the wrong arm, the filed row cited on the item and the unit, the one remaining uncited copy of `E`). Row at ordinal 4705, sample #230. The box
restarted once during the fix pass (18:35Z); every commit was on disk
and pushed, the cost measurement had completed, and the agent was
resumed. Spec deleted with its ledger entry; item
`sym-f64-far-placement-trips-the-theorem-vs-numeric-assert` closed;
unit closed.

## SYM-12 spec'd and dispatched (2026-09-21): the derived-frame freeze's next shape — block SYM-B3 slot 1

With SYM-11 merged (#3028, sample #230) a lane is free and block
SYM-B3's slot 1 dispatches per its pre-draw fields (H / NUMERIC, FABLE
by the draw's byte 178). The unit takes the derived-frame item's own
"whoever takes the next unit on this row should render it first":
Phase 1 renders `tiltUV` (the fold never fires, or fires into a frozen
node, or fires where no decision is asked — one of three, recorded),
counts the five other `copysign` mint sites against the eight measured
documents, and hand-plants the manifest-NEGATIVE arm against the ring
item's acceptance; Phase 2 takes the arm only if no split or ceiling
moves down anywhere. The Newell wall after rule F is DECIDE-3's ground
and is read-only here. Spec `docs/SYM-12-SPEC.md`; unit
`work/sym/SYM-12.md`; branch `sym/12-negative-arm`. Triaged IN under v7
by the program's default for a unit that changes what the tier decides
on a document (the block record says so; re-asked here: IN).

## SYM-12 at its PR (2026-09-21): the negative arm earned and taken

Phase 1 first, all three tables committed before Phase 2. `tiltUV`
rendered: the fold never fires — that document's `n.z` is not
`1/sqrt(P)` in the DAG but a degree-20/22 quotient in the parameter's
offset carrying odd powers beside three `sqrt` atoms (one over a frozen
node), and a size-frozen `Sub` refuses it at both dials; not the
budget. The `copysign` census: no mint site other than the orthonormal
basis reaches a decision on any takeable measured document, rule F on
or shut. The negative arm, hand-planted: the tilt-`u` START cap and
`FlipZ` read the END cap's rule-F-on numbers to the digit at both
lifts, nothing else moves, the eight documents' splits and ceilings and
the walk ledger are bit-identical — the ring item's acceptance met
without a second payment of its class. Phase 2 taken:
`manifest::negative` as `positive` of the negated numerator, both arms
under `manifest_sign`, the rows and the ordering pin, the gating
document row. Sample and ordinal at the dual's dispatch.

## SYM-12 fix pass (2026-09-22): the union of both reviews

Both reviews MERGEABLE-AFTER-FIXES (R1 one MAJOR: the mint-site census
incomplete and presented as exhaustive; R2 one demonstrated MINOR: the
two spellings of a negative magnitude parted). Taken, A–W: `magnitude`
reads `fold_abs` first, R2's row the pin; the tree's ten `copysign`
sites registered by a source-census row with the empirical claim
gated on the five cheap documents and measured on the revolved cap
(no atom reaches a decision); `negative` tests before it allocates;
each document's own numbers (the start cap 108 out of `numeric`,
`FlipZ` 122); the census table's totals printed by the row (three
hand sums were off by one to four); the release leaf instrument run
(the arm inside the run-to-run spread on all six leaves; the bracket,
link and pad over the line since rule E); the two forced rows
reflected; the render width restored by `take_shape_report`; the pad's
pins stated exactly (bracket pinned, nominal split by no row, over-band
set evidence-only); the reach narrowed to the tilt-`u` family (R1's
`FlipV` folds and moves nothing; R2's `FlipX` reads the end cap); the
one-dial reason's false half dropped; the negatives row's poison and
`copysign` cases; the fold row's F-shut arm said in full; the helpers
one home; the DECIDE-3 seam filed
(`the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic`).
Frozen again for the delta.

## SYM-12 merged (2026-09-22): the derived-frame freeze's next shape — block SYM-B3 slot 1

PR #3046 (fix-pass head `47e5a0a64` green on the full matrix, run
35672457048; the state-sync commit on top). Rule F's NEGATIVE arm:
`manifest::negative` as `positive` of the negated numerator, both folds
under `manifest_sign`; the reach the tilt-`u` family, either cap, either
sign (the start cap 108 out of `numeric`, `FlipZ` 122, both to the end
cap's state by name and by count; `FlipV` folds and moves nothing);
`tiltUV` rendered — the fold never fires on a 21-over-23-term quotient
with odd powers of `t`, not the budget; the `copysign` census — no atom
reaches a decision on the seven takeable documents or the revolved cap,
ten mint sites registered by a source-census row; `magnitude`'s two
spellings meet; the leaf cost measured (cheaper on five of six leaves,
the link +4.8 %); the eight
documents' splits, ceilings and the walk ledger bit-identical.

Review: the dual on `c6cf72319` (R1 OPUS MERGEABLE-AFTER-FIXES 1/5/5 —
the mint-site census incomplete and presented as exhaustive; R2 FABLE
MERGEABLE-AFTER-FIXES 0/4/5 — the two spellings of a negative
magnitude parted, demonstrated). Fix pass A–W (the register row and
its reader-ledger line, the census gate, the leaf instrument run, the
reflected rows, the reach narrowed, the seam filed as
`the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic`).
Delta by R1: MERGEABLE with F1 (must-fix: the render width per pass),
F2, F3 — taken in the state-sync commit. Spec deleted with its ledger
entry; the unit closed. Next: SYM-13 (block SYM-B3 slot 2, Opus); the
seam row is the `props/sign-hull` merge's.


## SYM-14 spec'd and dispatched (2026-09-22): the chain demo Ev asked for

Ev, in chat: "a demo of error propagation like with the two holed
plate but it's a chain of 4ish elements joined with some error in the
angle at each join and so it will be visibly be more and more
dispersed going down the chain" — "if it is possible ... take it on as
a unit; if it isn't possible, mark it and its prerequisites down in the
appropriate track ... p1 specifically requested". Surveyed: possible
today on the advisory lane (the plate's own picture lane — `f64`
replays over `Angle`-dimensioned parameters with a `Distribution`,
`Node::Transform`, the built bodies read back, the tour's SVG); NOT
today on the certified lane (a widened rotation angle unmeasured; the
derived-frame walls; the drive's cost). So: **SYM-14** takes the demo
and measures the certified lane on the same document; the certified
picture's prerequisites are on
`a-widened-rotation-angle-is-unmeasured-on-the-certified-lane` (P1,
requested), which the unit answers or leaves with the walls named,
each filed at P1. Spec `docs/SYM-14-SPEC.md`; v7 OUT (a demo and a
measurement, no kernel change): OPUS implementer, one OPUS style
review, no draw, no ordinal, no row. Lane `/home/user/lanes/sym-14`,
branch `sym/14-chain-demo`, cut from the spec commit; the seams
announced to CIW (`demos/render-mc.sh`) and PROPS (the analysis lane,
read only) in the PR. Runs beside SYM-13's fix pass.


## SYM-14 implemented (2026-09-22): the chain disperses, and the certified lane reaches its tip

**Phase 1.** `demos/tour/src/chain.rs` authors the document once (four
12 mm bars, a joint pin at each end, a fixed target pin at the nominal
tip); `mcchain.rs` replays 512 draws and draws the fan;
`demos/renders-mc/chain-density.svg` is published and `render-mc.sh`
now takes a list of sheets. The placement door is `Node::Transform`
nested over the downstream sub-chain — joint `k`'s node BELOW joint
`k−1`'s, so "joint `j` moves links `j..4`" is a property of the graph
and not of four hand-written partial sums; the `Datum::Frame`
alternative was declined because a frame's axes are orthonormalised at
evaluation and on intervals `cos² + sin²` is not 1, which is a `sqrt`
and two divides per link on top of the study. Straight nominal, one
Normal law at σ = 0.01 rad at every joint. Measured lateral σ at the
four pins: 0.1209, 0.2760, 0.4731, 0.6954 mm — `1 : 2.28 : 3.91 : 5.75`
against the accumulation law's `1 : 2.24 : 3.74 : 5.48`. The plate's
sheet is byte-identical.

**Phase 2, and the row it answers.** A widened rotation angle is now
measured. The plain `Interval` lane does not carry one AT ALL, at any
link count: `transform_rigid_col0_unit` refuses at the first transform
because `cos² + sin²` is a bracket around 1. `Sym<Interval>` discharges
exactly that and CERTIFIES the one-link chain whole over the study
(0.16 s); at 2–4 links the wall MOVES, to a transversality margin
during a mapped edge's re-certification (`dihedral_wedge`, poisoned, at
2; `dihedral_arm` with `[0, 7.34e-3]` straddling the band at 3 and 4).
Costs are 0.16–0.73 s per leaf, not the derived-frame family's minutes
— the chain never touches a `FaceFrame`.

**The certified picture exists.** The widest box that certifies whole
is `1.000`, `0.370`, `0.185`, `0.111` of the study at 1–4 links — one
number in four spellings, since `3σ · f · Σ(k−j)`, the accumulated
swing at the tip, is `0.0333` rad at every one of them: the certified
lane carries about 1.9° of swing however many joints it is spread over.
At the four-link box (`CERTIFIABLE_FRACTION`) the drive certifies and
the tip assertion HOLDS on every certified leaf. So the enclosure per joint is drawn on the sheet
beside the cloud: `0.0400, 0.1199, 0.2398, 0.3996` mm across the chain,
growing `1 : 3 : 6 : 10` — the worst-case lever sum — against the
advisory σ's quadrature `1 : 2.24 : 3.74 : 5.48`. E11's trade in one
picture, which the plate's sheet could not draw (`7.81e-7` of its
study). `a-widened-rotation-angle-is-unmeasured-on-the-certified-lane`
CLOSES on that.

**Filed** (all P1, Ev's request carried):
`a-widened-rotation-angle-refuses-on-the-plain-interval-lane`,
`a-two-joint-chain-poisons-its-transversality-margin`,
`a-chain-of-three-joints-straddles-dihedral-arm`,
`the-drivers-symbolic-dials-have-no-name-on-the-facade`.

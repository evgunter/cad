# M10 log — the error-propagation MVP

Narrative record; the plan is `docs/M10-PLAN.md`, the design record
`docs/ERROR-DESIGN.md` (E1–E11, ratified #110). Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-08-29)

Opened on Ev's direction ("you'll be doing M10, error
propagation"), by a fresh orchestrator on a remote container. The
plan is a DRAFT design conversation — nothing dispatches until Ev
ratifies it; this entry records the operational facts that hold
either way.

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `m10/`** — unit branches
  `m10/<unit>-<slug>`, orchestrator branch `m10/orchestrator`
  (Ev authorized the prefix at opening; the harness-designated
  session branch `claude/m10-error-propagation-q3e7i8` is unused).
- **A/B ordinal band: M10 = 500–599**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in the same commit that
  opens this program, per that entry's rule. Implementer blocks are
  named `M10-B1, M10-B2, …` (the GUI precedent — `M10-<n>` are
  unit names).
- **This session runs in a remote container** (the GUI program's
  precedent, adapted for a smaller disk): no persistent
  `~/.local/share/cad-work`, no script monitors (PR watching via
  MCP subscriptions + scheduled self check-ins; away-channel
  etiquette followed by hand under the `(M10 orchestrator)` tag),
  GitHub through MCP rather than `gh`. Disk ~29 G free is the
  binding constraint: lanes are worktrees sharing one object
  store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent lane targets,
  review targets reclaimed at report time (Ev, at opening:
  subagents share fewer repo copies than the local-machine
  workflow assumes; the orchestrator checkout carries no target of
  its own). The build-slot mutex, per-lane target rule,
  CONFLICTING-means-silent-CI, and push-early rules bind unchanged.
  The clone arrived SHALLOW; unshallowed with a blob filter at
  opening (ancestry checks and merges misbehave on shallow
  history — a successor in this environment should check
  `git rev-parse --is-shallow-repository` before trusting either).

**Sweep at opening** (what the plan's slate is grounded in, beyond
ERROR-DESIGN itself): #687 (`ContentBits for Dual` — the memo-seed
design question, the one lock left on `evaluate::<Dual64>`), #701
(`Enclosure` ungated in the bounds allowlist), the D1 ruling and
its hedge collected in DESIGN.md's roadmap entry, #1055 (shell's
curved wall-clearance window, aimed at M10's certificate),
CONTACT-DESIGN C5 (the gap-measure contract), PARAM-LINT-SPEC PL6
(independence semantics; DISCIPLINES' unit, not ours), PERF-PLAN's
M10 rows, and the codebase survey recorded in the plan's substrate
section (headline finding: the W2 sketch solver was never built,
so E8's solver walls are vacuous in v1 — plan Q1).

## Ratification (2026-08-29)

Ev ruled all five plan questions in-conversation the same day:
Q1 solver OUT, Q2 mass-prop Measures banked, Q3 MC lane rides
M10-6, Q4 clean breaks, Q5 the #1055 arm in M10-5 as a STRETCH
("possibly ambitious" — the follow-up-unit valve is the answer to
that hedge). Rulings folded, STATUS flipped, #1142 merged. Two
cross-orchestrator registrations landed on the plan PR before
ratification and are folded in the plan text: VERBS's consumer
demand for the clearance certificate (fixtures in-tree), and
PCURVE's measured poison-vs-widen datum, whose CLASS M10-D owns
(PCURVE files the class issue; the instance's mechanism stays
with P-1b).

**Next dispatches**: M10-D (orchestrator-led design pass, its own
design-conversation PR) and the M10-1 spec + block M10-B1 draw run
concurrently — M10-1 does not depend on M10-D.

## Seam: the C6 profile pin blocks E4/E6 on profile parameters (2026-08-29)

Found drafting M10-D, verified at the sites (`eval/slots.rs:27-30`,
`eval/wire.rs:440-482`): profile programs resolve parameter
expressions at f64 and lanes consume the elaborated segments via
`embed`, so a Dual seed on a profile dimension propagates no
tangent and an interval profile parameter does not widen the leaf
replay — silent zeros/points exactly where E4/E6 need signal.
PROFILES-V2 recorded the asymmetry and reserved it for Ev's
eyes; M10 is where the same parameter feeds both slot kinds for
real. Plan amended: unit **M10-P** added (design pass first, its
design PR waits for Ev); M10-3/M10-4 carry the
magnitude-parameter dispatch valve. Amendment self-merged as a
faithful elaboration (the ratified exit shape is unreachable
without it); the design fork itself is reserved for M10-P's PR.

## M10-1 MERGED (2026-08-29)

The program's first unit is in: distributions in the document, PR
#1147 at sample #39 (ordinal 500, the band's first). The dual
review's headline is R2's unilateral silent MAJOR — the deep-tail
`1 − erf` cancellation on exactly the number E2 forbids dropping —
fixed with one shared exterior/CDF pair that `quantile_z` also
reads, so the analyzed box always holds the mass the tail column
complements. The carry-forward class closed structurally
(`SetDocParamValue`); both probe suites promoted as merge parents.
Process findings recorded in the row: R2's rubric is missing data
(orchestrator worktree-reclaim broke the resume — the rule is now
"reclaim a lane's TARGET freely once its report is in; remove the
WORKTREE only when the unit fully concludes"), and the fix pass ran
on a fresh same-arm lane for the same reason.

## M10-DI MERGED (2026-08-29)

The Dual contract is implemented: the e4 door is OPEN
(`evaluate::<Dual64>` builds the whole corpus, value channel
bit-identical per-node), the policy seam is typed
(`AtRestOutcome`), the Enclosure gate fires, and the delegation
rule is the ledger's standing criterion. Sample #40, ordinal 501.
Both review arms independently proved correctness by differential;
the findings were all guards and honesty, the sharpest being R1's
mutation testing (three certifying arms could be gutted green —
now each is pinned to its validation door). Issues 687 and 701
close with this merge. The E4 pairing hook (DL3's own sentence)
is a NAMED obligation on M10-4's spec.

## M10-2 DELIVERED (2026-08-29)

Measure and Assertion exist. `Node::Measure { expr, refs }` is E3's
one dimension-generic sink — a `MeasureExpr` over `Primitive` leaves
that index the node's frozen `StableName` list, with the F1 lattice
asked (not restated) at every constructor. `Node::Assertion
{ measure, bound, dir }` is E10's persisted half, report-only by
construction: no op in the vocabulary takes a verdict as an operand,
so a `Violated` assertion cannot reach any downstream outcome.

The v1 primitive table ships with its scope stated: vertex x vertex,
vertex x plane, parallel plane x plane and parallel cylinder x
cylinder for `distance`; plane x plane and line x line for `angle`;
C5's three carrier pairs for the signed `gap`, sign convention
binding. Every other pair refuses typed naming the pair class. Two
trileans are consumed, both EXISTING funnel predicates at their
existing margin shapes (`bool_plane_parallel`,
`carrier_cyl_axis_parallel`); one new margined compare is minted, the
assertion comparison, and it took ledger row F16 with the honest
argument that E3 forecloses its own repair.

Schema v17, populated goldens, prior-version refusal fixtures,
load-door re-checks for both node kinds. The number MOVED at the fix
pass's re-merge and that is worth recording, because it is the case
this repo's ledger keeps warning about: this unit claimed v16 by an
explicit by-eye read of main's constant and named LIB-G16's
`Node::Chamfer` as a live rival for the same number, with the rule
stated in advance as order of merge. LIB-G16 landed first
(`a0427344`) and kept 16, so this unit took 17 and repaired what the
rule says it owes — the ledger paragraph, the
`assert_eq!(SCHEMA_VERSION, ..)` rows, the golden filename, and the
`plate_param`, bench-corpus and `gallery_ring` fixtures. The
constant itself merged CLEAN (both sides wrote `= 16`), which is
exactly why the read is by eye and not by git. The binding census closed with a
SPLIT disposition, which is this unit's own judgement: the READING
door ships (`Value.measure` → `Measurement`, `Value.assertion` →
`Verdict`, both readable off a document authored elsewhere), and the
AUTHORING vocabulary is chartered as `B-MEASURES`. The friction the
R-series keeps finding is unreadable results, so that is the half
that shipped.

THREE deviations after the fix pass, all stated in the PR (the first
draft of this entry said "two" while the PR disclosed four — the
review caught the miscount, and one of the four is now retired rather
than restated):

1. Measure references are CONSUMING DAG edges. Nothing else can order
   a sink after the geometry it measures, so deleting a referenced
   node is a delete-door refusal rather than N5 stranding.
2. `gap` takes its pair as two carrier references in mating-role order
   rather than a `Declare` node id plus a pair index.
3. The frozen golden carries no PRIMITIVE leaf (its document must
   evaluate green and its only well-known reference is a whole body);
   the three primitive leaves are pinned by round trip in the v17
   schema suite instead.

**RETIRED — the "carrier as minted" deviation.** The first draft
resolved a reference at the node that MINTED its name and documented
the recourse as "measure the moved one by referencing the moving
node's own emission". Both reviewers found that independently, and the
recourse was factually impossible: `wire_transform` is
identity-preserving, hands the input's table through by `Arc::clone`
and mints no name, so there was no transform-minted name to reference
and a transformed wall measured its UNMOVED carrier — a box translated
100 m measured 5 where the placed answer is 95, reported as plain
`Ok`. Fixed at the root rather than documented: a reference is now a
`MeasureRef { at, name }` pair naming the node to READ AT, which is
what the interrogation doors have always taken. Silent wrong numbers
are not a deviation to state.

`min_clearance` does not exist here.

The fix pass also closed three review MAJORs: the measurement
sublanguage restated `Expr`'s arithmetic without its non-finite door,
so `13/0` came back a typed success and an assertion reported
`Holds { measured: inf }` (both evaluators now share
`expr::refuse_non_finite`); and the plane `gap` arm read the raw chart
normal, so its sign was a charting artifact — half the parallel pairs
over two disjoint slabs read C5 "interference" with 2 m of air between
them (the S10 sense bit is now folded in, as `carrier_eq`'s plane arm
already did).

## M10-P MERGED (2026-08-29)

The profile-parameter lift is in: sample #42, ordinal 502, block
M10-B1's last slot. The C6 asymmetry that would have silently
zeroed every profile-dimension sensitivity is closed — structure
selected once at f64 as the witness, geometry lane-live under
guided replay with every consumed decision re-verified, the f64
path bit-identical (both review arms re-derived the fence on the
true merge base; CI's ε-sampling then caught the fence's own
ε-dependence, which is the sampled matrix earning its keep twice
in one unit). The latent-generic first commit found the
period-fold widening class, now #1191 with a 15-site by-shape hit
list — offered to S-CERT on the work-streams PR, M10-3 its first
consumer. M10-3 and M10-4 are now dispatchable at FULL scope
(profile-driven parameters included) once their remaining
dependencies land: M10-3 needs nothing further; M10-4 needs M10-2's
Measure sink (in implementation).
## Orchestrator-side entries, merged at block M10-B1's conclusion (2026-08-29)

The adjudication and operational entries below were recorded on
`m10/orchestrator` as the units ran (branch-side, with the block
records) and merge here now that the block is concluded; each is
dated and sits chronologically BEFORE its unit's MERGED entry
above.

## M10-1 dual review adjudication (2026-08-29)

Both arms APPROVE-WITH-FIXES on frozen 0e9ef0b4. R2 found the
unit's one silent correctness defect (deep-tail cancellation —
`1 − erf` bit-zeros the tail from ~8.5σ; unilateral MAJOR by
execution, tally candidate at merge); the carry-forward class and
the unpinned GUI fix were bilateral at different severities. Fix
pass dispatched as the adjudicated union to a FRESH same-arm lane
(the original lane's worktree had been reclaimed at report time,
which broke resume — reclaim implementer worktrees only after the
fix pass concludes, or accept fresh-lane fix passes and record the
executor covariate, as here).

Class findings given homes at adjudication (the standing rule):

- **"Priced" vs "set-theoretically forced" mass are one type**
  today: `box_mass(Band, covering) = Ok(1)` is measure-free and
  correct, but an E10 unresolved-mass budget over Band-only params
  would read "fully priced" while no shape was ever stated. M10-6's
  report/budget spec must distinguish the two (R2 MINOR-1).
- **"No distribution ⇒ mass 1" lives in prose, not API** — every
  consumer special-cases `None` by hand. M10-3's driver spec should
  give the fixed-param case a typed spelling (R2 style finding).
- **A distribution is invisible in the GUI** (`ParamRow` carries
  none), so users can hold state they cannot see; the census
  records Python's gap, nothing records the GUI's. GUI follow-up
  slate (R1 NOTE).

## M10-DI dual review adjudication (2026-08-29)

Both arms confirm CORRECTNESS with independent byte-identical
merge-base differentials (three lanes each); every finding is
guarding, coverage, or text. R1 (4 MAJ, by mutation testing and
witness re-measurement): three of four certifying policy arms
gut-to-green; the Dual arm's `Ok(())` grant + the unenforced E4
pairing hook (a silent deviation — DL3's own sentence); DL3
witnesses named wrongly (the real ones are cut_cylinder and
loft_prism, 2/18); the DL5 selection clause contradicting the
projection::mid counterexample class named sixty lines above it.
R2 (0 MAJ, APPROVE): the assemble-at-Dual availability inversion
pinned (same class as R1's second MAJOR), the digest-depth
overclaim, the DL5 surplus remedy clause. Fix pass dispatched
IMPLEMENTER-INHERITED (the worktree-kept rule worked — resume
succeeded). Tally candidates recorded at the row for the blinded
coding with class annotations (test/doc-class per the LILYWELD
3b precedent is the likely coding; not this log's call).

Durable homes at adjudication: R1's free DL6-class datum (an
EXACTLY coincident flush pair refusing `margin: Invalid` at plain
f64 — the class's third member, and its first on the default lane)
posted to #1143 with the fixture named; the E4 pairing hook is a
named obligation for M10-4's spec; `real.rs`'s ~270-line Bounds
ledger accumulation (both arms flagged the shape) banks as a
docs-home candidate rather than riding any unit.

## M10-P dual review adjudication (2026-08-29)

Both arms APPROVE-WITH-FIXES on frozen 55b1fd13, both re-deriving
the bit-identity fence independently on the TRUE merge base
(5fed0960 — the PR's provenance cited a stale one) and at greater
depth than the unit's own instrument; R2 additionally covered
Probe, which CI never runs. Every machinery claim held: guided ≡
plain at f64 (adversarial families included), no structure
selection at the lane scalar (the other-pocket consumption receipt
is the elegant proof), canonicalization structurally pinned with a
live-control decide-count row, ladders fork-proof by construction.
The MAJORs are reach/receipt/disclosure: the typed `Structure`
vocabulary covers 1 of the record's 11 decision classes vs the
claim (bilateral); the periodic-reduction class receipt missed
seven same-spelling sites and four same-shape floor-folds in topo
(bilateral — the class gets its own issue from the fix pass); the
interval-box door's unreachability through `evaluate` was
undisclosed (adjudicated: it IS M10-3's first spec bullet —
disclosure fix, not machinery). Fix pass IMPLEMENTER-INHERITED.

Durable homes at adjudication: the floor-based period-fold class →
its own issue (fix pass files, full both-spellings hit list); for
M10-3's spec — the interval parameter door is where R2's M1
friction dissolves, `ProfileLaneReplay` deliberately dropping lane
scalar payloads (R1 friction 2) should be revisited when the
driver wants the lane pass's refusal payloads, and the guided
enclosure being node-dependent (extrude widens, loft stays f64 by
C6/D9) needs stating in the driver's leaf semantics.

## Operational: disk math on this container (2026-08-29)

Measured under pressure: ONE workspace-wide interval battery in
debug costs ~15-16G of test artifacts — two concurrent lanes only
fit if at most one runs a world battery. Standing brief lines from
here: lane batteries run `CARGO_INCREMENTAL=0`, and scope to
touched crates (`-p ...`) unless the unit's sweep demands the
world — the hosted gate covers the rest (local-battery-scope's
time argument, now applied to disk). Reclaim-at-report remains
the transient's fix.

## M10-3 MERGED (2026-08-29)

PR #1231 merged at e93c2be6, sample #49 (ordinal 504; the number
annotated in the row — LIB's corpus-die merged minutes prior with
no row at this writing, and merge order rules if its recorder also
drew #49). The E6 driver is live: the interval parameter door is
open through `evaluate`, leaves certify on exact VerdictVector
equality with no width anywhere, refused mass is priced per-reason
with the ADDITIVE tail, and the macroscopic limitation is measured
and pinned red-the-day-it-closes. The dual review's headline was
bilateral and identically diagnosed — the accounting composed
unconditional columns as conditional, under-reporting the E10
honesty gate by the whole tail, invisible because every shipped
fixture was bounded; the fix states the argument at the type with
the measured 0.27% consequence. R2's unilateral structural MAJOR
(flip naming was a second verdict-diff engine by positional zip —
the method resolve/vdiff rejects in its own doc) was fixed at
branch (a): naming routes through the built-once engine, whose
interface was never the obstacle. Both arms attacked the unit's
one silent-defect shape — an escalation misclassified definite
that CERTIFIES — and independently returned NOT FOUND. Issues
#1254 (escalation channel, filed with the verdict log's banked
redo) and #1255 (three verdict shapes) are the fix pass's durable
homes; deviations went 7→9, honestly. M10-4's spec unblocks (the
E4 pairing hook obligation and M10-2's sink are its inputs; the
interval door it needed is now on main).

## M10-2 MERGED (2026-08-30)

PR #1213 merged at 7c4b54b3, sample #50 (ordinal 503). Measures and
assertions are document data: `Node::Measure` with `MeasureRef
{ at, name }` — the fix pass's deepest cut, changing the reference
SHAPE so a measure reads the placed geometry at its `at` node the
way the interrogation doors always have — the nine-arm primitive
table with C5's sign convention in one function, `Node::Assertion`
report-only by construction, schema v17 (the v16 race lost to
LIB-G16 and repaid exactly per the rule this unit had stated in
advance). The dual review's three adjudicated MAJORs all landed at
the root: the shared non-finite door (R2's unilateral
`Holds { measured: inf }` — the unit's severity headline), the
transform/minted-carrier silent wrong number (bilateral), and the
sense-folded plane gap — where the fix pass returned the program's
first reviewer-asked-row PUSHBACK on correctness grounds: an
opposed mating pair's gap is correctly symmetric under role swap
(one clearance, not two signed ones); only aligned pairs negate.
Both regimes pinned. Both reviewer lanes were killed mid-review by
the same account-limit wave and resumed — symmetric, recorded in
the row. The conflict round (M10-3 and LIB's corpus-die landed
under the finished fix) re-blessed the fences over the union
roster with the strongest removal measurement yet — the roster
minus this unit's document IS main's, so main's own committed
constants came back as green assertions. The withdrawn k-lint
escalation thread from implementation routes to #1223 (the teapot
tess-budget baseline, filed same-day). M10-4 is now dispatchable:
its spec's inputs — this unit's sink, M10-3's door, the DL3
pairing hook — are all on main.

## Tracker migration (2026-09-03)

The plan and this log moved here from `docs/M10-PLAN.md` /
`docs/M10-LOG.md`. The program's slate now lives in this directory's
item files and in `work/STATUS.md` (generated); this log stays the
narrative. Items created at migration: M10-4 (spec), M10-5 (spec),
M10-6 (open).

## M10-4 and M10-5 redispatched fresh (2026-09-03)

Both units' first implementer lanes were dispatched 2026-08-30 and
died within the hour at the account session limit, pre-first-commit;
the orchestrator session was away four days and main moved 524
merges in the interval — rewriting exactly the eval/bvh/props files
the lanes' unpushed drafts touched. Per the death-recovery rule and
the G16A precedent, fresh same-slot lanes go out today from current
main, the drafts handed over as untrusted reference material. The
tracker migration's "not yet dispatched" on both items was written
without sight of the branch-side dispatch records; the records
annotate the interruption and the pairs stand. Both specs' grounding
was re-verified against the moved main before redispatch — every
cited symbol still exists; the specs bind as written, with their
plan/log pointers updated to this directory.

## Orchestrator-side entries, merged at block M10-B2's conclusion (2026-09-03)

## M10-2 dual review adjudication (2026-08-29)

Split verdict on frozen e0cc0b20: R1 MERGEABLE-with-one-MAJOR
(rubric 4/3/4), R2 REQUEST CHANGES (4/2/2, 3 MAJ). Both reviewer
lanes were killed mid-review by the same account-limit wave and
resumed — a symmetric interruption, recorded for the row. The
transform/minted-carrier finding is bilateral and the center: a
measure over a transform-descended ref silently reads the UNMOVED
carrier (0.5 where the placed geometry sits at 0.75; 5 where it
sits at 95) and deviation 3's documented recourse — "the moving
node's own emission" — does not exist (wire_transform Arc-clones
the table through and mints nothing). R2 showed the interrogation
layer already takes `(ev, node, name)`, so the fix direction is
read-at-the-referenced-node, typed refusal as the fallback, plus
the schedule deviation 3 owed (Q6). R2's unilateral M1 is the
severity headline: `eval_measure` restates `expr::eval`'s
arithmetic without its non-finite door, so `13/s` at `s = 0`
measures `inf` and the assertion over it reports `Holds` — a false
PASS from the node whose job is certifying intent. The plane-gap
sign was bilateral at different severities (R1 MINOR, R2 MAJOR
with the role-swap-does-not-negate table over disjoint slabs);
adjudicated MAJOR — the plane arm folds sense (the carrier_eq S10
discipline) so g means material separation and role swap negates.
Fix pass IMPLEMENTER-INHERITED, adjudicated union: the 3 MAJ, both
probe suites adopted (six of nine closed-form arms had no
red-capable oracle in the PR's own suite — all six verified CORRECT
under both reviewers' independent oracles), the misattributed
parallelism refusal, the 1 m arm-floor honesty (docs must name the
floor as the operative sub-metre lever; the lever redesign banks on
chart_region.rs:804's standing criticism), the unreachable digest
arms (a corpus doc carrying Measure+Assertion makes them live), the
weak Python rows, and the accumulated prose/doc sweep (the
key-format bump-rule tension disclosed; this log's own
deviation-count line corrected by the fix pass).

## M10-3 dual review adjudication (2026-08-29)

Both arms on frozen 54a77ad9: R1 NOT-MERGEABLE (1 MAJ, rubric
4/4/4), R2 REQUEST CHANGES (2 MAJ; rubric requested post-report —
the lane omitted the triple and was resumed for it, the M9-3
missing-data shape avoided because the worktree-kept rule held).
The accounting composition is bilateral and byte-identically
diagnosed from independent fixtures: the mass columns are
UNCONDITIONAL (a leaf prices P(offset ∈ leaf), so the leaves
already sum to 1 − tail) while `total()`/`unresolved()` compose
`t·(1−tail) + tail` as if conditional — a Normal axis at the
default ±3σ totals 0.99730729, and the E10 honesty gate
under-reports unresolved mass by the whole tail, the unsafe
direction. Invisible to the shipped suite because every fixture is
bounded (tail ≡ 0) — the premise-excludes-the-failing-mode shape —
and both arms shipped deliberately-RED counterexample rows. R2's
unilateral second MAJOR is structural: `drive.rs`'s flip naming is
a SECOND verdict-diff engine using positional zip, the method
`resolve/vdiff.rs` ("built once", for exactly the f64-vs-Interval
case) rejects as unsound in its own 17-line argument; certification
stays conservative (unequal ⇒ refuse, no false certificate) but
FlipCrossing evidence can name permutation artifacts and miss true
flips. Adjudicated: name flips through the built-once engine, or
carry the confronting paragraph plus an honest best-effort label —
the missing paragraph is the defect either way. The claim-8 attack
(a definiteness misclassification that CERTIFIES) came back NOT
FOUND from both arms independently — the unit's one silent-defect
shape stood.

Fix pass IMPLEMENTER-INHERITED: the two MAJ, the bilateral
max_leaves 2× overshoot, the macroscopic pin's budget mismatch and
the ε/4-vs-ε/8 threshold correction, the irreproducible 26-hit
sweep receipt (both arms count 23 unique; R2's second sweep adds
~15 predicates the pattern missed), the containment positive arm
row (both arms constructed it — code correct, coverage absent),
deviation 3's missing schedule (an issue gets filed), the k_stats
narrowing vs E6's "every" disclosed as a deviation, the stale
deviation 7, and both probe suites adopted (the four red-by-design
rows go green with the composition fix).

Durable homes at adjudication: deviation 3's escalation-channel
issue is filed by the fix pass; the extra widening-class hits fold
into the corrected sweep receipt (the class's home remains #1191);
the verdict-diff triple-spelling (vdiff / drive.rs / verdict_summary)
gets a consolidation issue from the fix pass unless the MAJ fix
itself unifies.

## M10-4 dual review adjudication (2026-09-03)

Both arms on frozen fc8de0ac: R1 REQUEST-CHANGES (1 MAJ, rubric
4/3/4), R2 NOT-MERGEABLE (2 MAJ, rubric 4/3/3). The headline is
bilateral and identically diagnosed with independent red probes:
the chamber verdict is never bound to the build — `ForeignVerdict`
compares axis NAMES, `ForeignBox` compares offsets, and nothing
ties a `ParamBoxVerdict` to the document it was driven over, so an
edited document (a value edit that keeps the verdict vector) or
another document's verdict marks every sensitivity
`ChamberCertified`. R1 named the fix's raw material (the drive
already records per-leaf value-channel `node_keys`, and
`worst_case`'s replay recomputes and discards them); R2 named the
trap (a `VerdictVectorKey` compare would NOT close it — a pure value
edit keeps the vector; it needs a content tie). R2's unilateral
second MAJOR is the one the spec's grounding warned about: a LOFT
section's dimension seeds to a silent finite ZERO (`section_of`
emits the f64 elaboration's loops), the true derivative being 1 —
"the profile gap TYPED, never silent zeros" — while the PR body says
no typed valve was needed; the sweep seam has the same shape. R1
had predicted the loft as the unexercised arm (T1) and flagged it
unverified; R2 executed it. The remaining findings converge on
honesty of the advisory columns (contribution extrapolates past the
chamber it is marked with; `NothingCertified` throws away the
sensitivities it computed and carries no accounting; the e2e's −2
comes from the measure expression, not the lift — only the width
slab and the reviewers' own arc-carrying rows pin the guided lift)
and on disclosure (deviation 3 unscheduled and dropping a
plan-named deliverable; deviation 2's rationale conflating keys
with the value channel; stringified refusals against D2's letter;
the RSS fixed-parameter door). Every claim the reviewers could
execute against held otherwise: zero impact bit-identical over 22
corpus documents at three scalars, seed hygiene under DL2 in six
threading orders, σ derivations to 1e-9 against quadrature,
`worst_case` tangent-free by type and right where the
linearization is wrong.

Fix pass IMPLEMENTER-INHERITED: the two MAJ (a content tie from
the verdict to the anchor; a typed per-entry refusal for seeds that
feed a C6/D9-pinned section, loft and sweep), the advisory-column
honesty set, the disclosure set, both probe suites adopted (four
red-by-design rows across the two go green with the MAJ fixes).

Durable homes at adjudication: contribution bounds via
`Dual<Interval>` enclosures (deviation 3) get a `work/m10` issue
item from the fix pass — the plan named them as this unit's and
M10-5's entry names only pruning; the subgradient-at-a-kink honesty
gap (a one-sided derivative reported with a smooth one's confidence,
conformant but unmarked — R1 T3, R2 note f) is stated in E4-facing
docs by the fix pass and banked as an M10-6 report-shape question.

## M10-4 MERGED (2026-09-03)

PR #1627 merged, sample #114 (ordinal 505) — block M10-B2's last
slot, so the block's records land on main with this entry. The seed
door is open and E4 is real: every sensitivity carries a chamber
certificate that is CONTENT-TIED to the build (the fix pass's
deepest cut — the drive's own per-leaf `node_keys` replayed and
compared before any mark is written, closing both the stale-edit and
the same-name-foreign-document cases both reviewers had demonstrated),
or `LocalOnly`; DL3's pairing hook is a typed two-half gate that
discharges the obligation M10-DI's adjudication named; the `Stackup`
gates on `worst_case` alone and now hands a real tolerance study its
`LocalOnly` sensitivities, coverage and receipt inside
`NothingCertified` instead of a data-free error. R2's unilateral MAJOR
gave the program its typed profile valve for real — a loft or sweep
section's parameter seed refuses `SeedPinnedSection` rather than
producing the silent zero the plan's C6 seam entry predicted a year
of specs ago. Honest limits, measured: certification widths remain
ε-scale (M10-3's ceiling), `worst_case` at ε-scale is mostly the
interval lane's dependency padding (2·half on the plate, 10·half on
the slab), `contribution` extrapolates past its chamber by the
box/leaf ratio and says so. Durable homes: contribution bounds via
`Dual<Interval>` — `work/m10/contribution-bounds-via-dual-interval.md`;
the subgradient-at-a-kink report mark banked for M10-6. Process: this
unit's first lane died at the session limit and was redispatched fresh
four days later against a main that had moved 524 merges — the fresh
lane delivered in ~60 minutes, the rows annotate it. The six merged M10
specs (M10-1, DI, P, 2, 3, 4) leave `docs/` with this merge per the
ledger's rule. M10-5 is in flight; M10-6's spec is next.

## M10-5 MERGED (2026-09-03)

PR #1638 merged, sample #115 (ordinal 506). E7 is real: over a
certified leaf the engine answers `Holds` / `Violated` with an
f64-verified witness / `Refused` typed and priced, with the receipt
identity riding the report — and after the fix pass the three
places the deliverable could have lied are closed at the root: a
fold over a drive that certified nothing refuses instead of passing,
refused leaves are priced under their own class instead of riding as
certified mass, and the BVH's admission threshold carries the
funnel's band so nothing inside the window the funnel would refuse
is silently held. R2's unilateral MAJOR — the strict violation arm
had no reachable path, so gross interpenetration came back
`Refused(Budget)` — was fixed by an exhibit arm that can only ADD
verified violations; R1's unilateral MAJOR — the sweep never stopped
at the first violation, so every `Violated` was a budget exhaustion
and the limit narrative was wrong — by early exit with the
unexamined frontier accounted `abandoned` and the cost curve
re-measured in three regimes. The program's first symmetric tally
pair. Two design-level residues are filed, not fixed: the window
superset (D3) needs the boundary in chart coordinates; a signed
penetration depth (D7) needs a margin that is not a norm. The 1055
stretch did not land — the valve is the layering question (where a
curved gate lives: above editor-core or a duplicate engine in topo),
filed with the cost figures. The accelerator ships behind the
`MonotoneOracle` seam with `NoTangents`; the `Dual<Interval>` oracle
is the item M10-4 filed. Process: the lane spent three extra CI
cycles on doc-truth fixes found after the first push and said so —
the doc sweep belongs before the first push, recorded here for the
next brief. The spec leaves `docs/` with this merge. M10-6 is next
and last.

## M10-6 dual review adjudication (2026-09-03)

Both arms on frozen bf67a734 (PR #1685), with hosted CI
budget-blocked account-wide, so every claim rests on the reviewers'
local runs of what the gate would have covered — both stated their
coverage row by row. R1 NOT-MERGEABLE (2 MAJ, rubric 3/3/2); R2
"mergeable once two MAJORs are answered" (2 MAJ, rubric 4/3/3). The
four MAJORs are four different findings. R1, by execution: the
`min_separation` enclosure is over carrier WINDOWS (M10-5's
disclosed superset) but `MinSeparation`'s docs sell it as
containment-true over the trimmed faces and "what makes an `AtMost`
assertion sound" — an L-cap parked over a notch brackets
`[0.1, 0.1]` where the true face separation is 0.269, so `AtLeast
0.2` certifies a FALSE `Violated` and `AtMost 0.15` a FALSE `Holds`
(the row-1 gate would red a true assertion). The asymmetry is
exact: windows ⊇ faces, so `lo` is a valid lower bound for the
faces and `hi` is not — the two arms that consume `hi` are the
unsound ones. R2 met the same shape (`lo = 0` on a non-convex body,
the assertion `Unevaluated`) and scoped it to M10-5's filed item;
R1 is right that its propagation into a VALUE and two assertion
directions is this unit's. R1's second, by execution: the tour's
stop-2 headline — "the requirement FAILS somewhere in the box: this
is the number that gates" — comes from a raw `worst_case.lo <
bound` at −4.2e-11, inside the coincidence band, while the
document's own `Assertion` over the identical enclosure reads
`Holds`; the demo preaches the funnel and decides on a float. R2,
by execution: `VerdictVector::certifying` drops `Assertion` rows
from the certification comparison, moving `witness_vector`,
`serialize()` and every leaf's key for EVERY assertion-carrying
document — defensible, load-bearing, nowhere in D1–D9, its
justification written four commits after the code (R1 had it as a
MINOR). R2's second: the row-3 demotion is per-invocation, so the
E6 re-open trigger (rule 1 — an in-band indeterminate) would print
among 15,768 rule-2/3 flags and not red; the rule-1 count, the
number E6 actually asked for, is reported nowhere (zero today — the
population piles up just outside the escalation band at a floor of
1.0083e-5 ≈ Kε, which is the runbook's recourse 2 and correct). The
MINORs converge: the cache seam's `report_key` blind to every dial
and consumed by nothing; row 1's first entry never driven and its
budgets all 0.0; `stackup` refusing a `min_clearance` measure whole
where `worst_case` is computable (E9's own rule inverted); the
Python read door answering a typed absence with "not a measure";
the ci-local k-lint gate's exit-code mismatch; the MC tail row
admitting 0.0; the ci.yml cost note attributing 33 s to a 0.34 s
cell; `mc`/`report` interval-gated so E11.1's pure-f64 lane is
unusable in the default build. Everything both arms could execute
held otherwise: D1's additive load door refusing an unknown variant
by name, D2's value-not-error absence judged better than the spec's
own self-contradictory sentence, D6's goldens at all three ε rows
with mutation and re-bless, D8's binary primitive the faithful
elaboration, the enclosure's soundness ON WINDOWS surviving every
attack, the MC stream re-derived bit-exact.

Fix pass IMPLEMENTER-INHERITED: the enclosure typed one-sided for
faces (the `hi`-consuming assertion arms refuse typed until window
tightening lands, docs telling the truth); the tour deciding by the
assertion's verdict, never a float, on a fixture whose failure is
definite; `certifying` disclosed, justified and pinned; rule 1
gated red with its count reported and a K-REPORT entry; the MINOR
set; both probe suites adopted (R1's three ignored counterexamples
go green as pins).

Tally candidates: R1's window-enclosure MAJ and R1's tour-headline
MAJ (both by execution, both unilateral — R2 scoped the first and
did not exercise the narration); R2's two are bilateral at split
severity with R1's MINOR 3 and MINOR 10. Durable homes: window
tightening stays M10-5's filed item (this unit's fix is the honest
typing until it lands); the PRNG's single-copy sentence in
`test_utils::fuzz` is the fix pass's to make true again.

## M10-6 MERGED (2026-09-03)

PR #1685 merged, sample #118 (ordinal 507). The program's last unit.
E10 and E11 are real: every derived report serializes exact-bits,
keys on its content and renders for a human, priced and forced mass
are two types, the three CI rows run on the sampled matrix and were
read executing by step conclusion (rows 1–2 in the interval shards,
row 3 on the k-lint axis with rule 1 gating and 65,992 advisory
flags over the driver population, the tour cell in its own step),
the MC lane estimates and never gates, and E3's last primitive
ships with the engine door both M10-5 reviewers asked for — a
certified bracket of the minimum separation the budget narrows and
cannot falsify. The review's central finding shaped the door: the
bracket is over carrier windows, not trimmed faces, so only its
lower end is a bound on the measure; that is now a type the
assertion consults, the two arms that would read the upper end
refuse typed until window tightening lands, and the two gating
directions survive. R1's second execution finding — the tour
deciding on a float against its own assertion — is closed by a
door that did not exist (`assertion_at`), with the measured fact
that a definite failure and a real RSS divergence cannot coexist at
this ε stated in the caption rather than faked. R2's key-move
finding stands as a disclosed, pinned deviation (D10, retracting the
body's "keys bit-identical"); its demotion finding makes rule 1 the
one thing no caller can demote. Two tally candidates, both R1's, both
by execution. Filed, not fixed: `MinClearanceRefusal`'s stringly
twin (D12); ERROR-DESIGN's unary `min_clearance` arity against the
shipped binary door (D8) goes to the exit walk as a doc revision for
Ev.

Process, for the next brief: (1) the lane died five times on API
529s during the fix pass and was resumed each time from its
worktree — never redispatched — at 60- then 30-minute backoffs;
nothing was lost, but the pass took 4h40m for ~1h of work. (2) The
orchestrator's re-gate of the frozen head (an empty trailer commit
pushed after the Actions budget outage) reset the unit branch's
local ref UNDER the lane's live worktree and orphaned its four
unpushed commits; they were recovered from the reflog and the
re-gate merged in. Rule: an orchestrator re-gating a branch a lane is
working on makes the commit in its own checkout and pushes by
ref, never `branch -f` on the shared local ref. (3) Two red CI cycles
on one trap: `tools/` and `demos/` are excluded from the root
workspace, so the root `cargo fmt` reaches neither; the pre-push
sweep must run CI's own commands in every excluded root. The spec
leaves `docs/` with this merge. What remains for M10 is the exit
walk.

## Seam — residue re-homed for the exit (2026-09-03)

Per `work/README.md` (residue is re-homed before the sweep) and the
2026-09 work-track proposal `docs/WORK-TRACKS-2026-09.md`, every open issue this program held
moved by header-preserving `git mv` to the track that will carry it —
ids unchanged, nothing copied:

- `certify-locally-valid-range-instead-of-sampling` → `work/docm/`
- `signed-penetration-depth` → `work/curved/`
- `contribution-bounds-via-dual-interval` → `work/props/`
- `k-stats-escalation-channel-and-redo` → `work/props/`
- `three-per-node-verdict-shapes` → `work/props/`
- `certified-lane-non-real-contract-audit` → `work/props/`
- `clearance-window-tightening-needs-chart-boundary` → `work/trim/`

The directory now holds only its units, its narrative and (for S-MATE)
the `MATE-EXIT` ruling, so the sweep at ratification leaves nothing
behind.

## Handoff note from SHELL (2026-09-04)

Ev ruled B on `[ev]` #1737: the body-level half of
`crates/editor-core/src/clearance.rs` (the cell subdivision over
`Body<Interval>`, `min_separation` and its types) moves into `topo`
behind `interval`, and editor-core's leaf/param-box half calls down.
SHELL cuts it as `work/shell/SHELL-3.md`, a joint unit; M10-7 (#1725)
does not touch the file, measured, so the move is sequenced after
#1725 merges and SHELL asks the M10 orchestrator to co-review. The
gate itself is SHELL-4.

## M10-7 MERGED (2026-09-04)

PR #1725 merged, sample #124 (ordinal 508). Certification is
parameter-aware for explicit geometry: a margin that is identically
zero in the parameters decides `Zero` at any box width, inside the
scalar, with no funnel site touched — the M10-3 limit rows are
positive pins now, the slab's certifying half-width moved from ε/8
to 0.488 on a 1.0 nominal, and the ±0.05 study certifies in one
leaf at the shipped depth. The reviews did their job in the
direction that matters. R2 found the unit had DISARMED the driver K
gate it was supposed to feed — the new outcome token was unknown to
k-lint, zero samples were linted, and the step stayed green through
a status-capture hole that had made the row incapable of failing
since M10-6 wrote it; the fix gives the outcome vocabulary one home,
makes the gate able to red, and the merge run's log shows the three
driver CSVs read with rule 1 at zero. R1 found the ceiling's stated
mechanism was wrong — ε-dependent, bounded by dependency widening in
NON-identity margins (the Newell normal poisoning), not by the flip
the row claimed — so E12's "plain intervals suffice for the real
margins" has a caveat, filed as the class after M10-8. Both found
that every opaque value shared one id and could cancel to a false
theorem through a public door. And both e2e walks agreed that on
curved geometry a real study still gets the pre-E12 answer: the
arc family the tier misses is wider than D4's rim endpoint, and
M10-8's scope is widened to it with R2's per-predicate numbers. The
program's second symmetric tally pair. Twenty-eight deviations, the
plate's miss among them stated in the tour's own caption. The spec
leaves `docs/` with this merge; M10-8 is next, and the exit walk
stays a draft until it lands.

## Announced seam from DOCM (2026-09-04)

PROFILE-LIFT-DESIGN PP6 (`crates/editor-core/README.md`) is amended on
Ev's ruling (in-chat, 2026-09-04): its "the sketch plane stays f64"
sentence now holds for an AUTHORED frame only; a DERIVED frame's
profile (`Datum::FaceFrame`, DOCM-REFERENCES-DESIGN DM1/DM1c) is placed
at the lane scalar through `frame_plane_lane` under every lift, and a
section on a derived frame refuses typed off f64. The build is DOCM-1
(`docm/1-face-frame`); it touches `eval/wire.rs`'s profile placement
under `ProfileLift::Pinned` and nothing in the analysis lane. Signed
(DOCM orchestrator).

## Filed from DOCM (2026-09-04): the derived frame on the symbolic lane

`derived-frame-placement-freezes-on-the-symbolic-lane` — DOCM-1's
derived sketch frame (DM1c, PP6 as amended) places a profile at the
lane scalar, and on `Sym<Interval>` the extrude above it refuses
certification under a widened upstream parameter while the authored
twin certifies: `geom_core::sym::form_in` budget-freezes the rational
forms that re-normalising the kernel's stored unit vectors produces
(degree 400–700). The kernel's symbolic lane, M10's territory; the
red pin is on `docm/1-review-r1`. Signed (DOCM orchestrator).

## Announced seam from PROPS (2026-09-05)

The verdict-recording lane (`work/props/plan.md`) edits `drive.rs` by
announced seam, as the PROPS program header records. First build: the
unit `three-per-node-verdict-shapes` on `props/verdict-shapes` moves
`VerdictVector`/`VerdictRow`/`VerdictVectorKey` out of `drive.rs` into
`resolve/vdiff.rs` beside `NodeVerdicts`, folds `ReplayOutcome` into
`RunStatus` (`Absent` kept distinct; existing key tag bytes unchanged),
and pins the strict-vs-population split. `classify`'s gate, `FlipEvidence`
and the symbolic dials are untouched; the lane reads M10-8's diff before
starting so its hunks sit off that branch's lines. Signed (PROPS
orchestrator).

## Announced seam from PROPS (2026-09-05): `dual.rs`, one impl signature

The Span sweep (`work/props/span-carries-its-knot-vector.md`, Ev's
ruling A; spec `docs/PROPS-SPAN-SPEC.md`) gives `SpanLocate::locate_spans`
a method-level lifetime, so the trait's `Dual<T>` impl in
`crates/geom-core/src/dual.rs` changes by signature only — no arithmetic,
no policy. Named here because `dual.rs` is M10's; the PR body lists the
edit. Signed (PROPS orchestrator).

## Announced seam from PROPS (2026-09-05): the k-stats bracket at M10's `classify`, the M10-7 tier-off row, one issue filed here

The k-stats unit (`work/props/k-stats-escalation-channel-and-redo.md`,
PR #1969, spec `docs/PROPS-KSTATS-SPEC.md`) lands three things on M10's
ground, all listed in the PR body: (1) `drive::classify_replay` reads the
node's escalation channel — after the box-independent terminal classes
(the fix pass's order: a definite box-independent refusal is terminal
whatever else the op recorded; an escalation decides among the
box-dependent outcomes), so the planted flip's in-band strips price
`SliverTerminal` instead of `Budget` and the M10-6 accounting goldens are
re-blessed (witness-vector keys byte-identical); (2) the M10-7 R2 row
`r2_the_tier_off_accounting_is_the_merge_bases_bytes` and its
`golden_r2/` copies — the merge base's bytes are no longer reproducible at
either dial, so the row is re-cut with a bless arm and renamed to what it
asserts now (tier-off = committed bytes, tier-on differs); the review read
the independent half of that differential as gone, and M10-7 being closed,
M10 may retire the row; (3) filed into `work/m10/`:
`coincidence-zone-priced-budget-at-the-floor` — a leaf wholly inside
(−ε, ε) on the extrude depth refuses `DegenerateExtrusion` definitely on
every sub-box yet bisects to the floor as `Budget` (2.5 % of the planted
flip, measured). Signed (PROPS orchestrator).

## M10-8 MERGED (2026-09-05)

PR #1828 merged, sample #144 (ordinal 509). The unit was written
measurement-first and its own headline was a negative result: the
arc family's atom algebra inert on every document, rule C
unbuildable within the discipline, the reserve filed and the tier
M10-7 byte for byte. The dual review showed the negative result was
an artefact of the implementation. R1, by execution: the freezes
the unit blamed were `sqrt` atoms over CONSTANT forms with
exact-rational roots, and a value-free fold of them (A0) moved the
bracket's whole-certifying ceiling 10.4× and an annulus 39×; the
runaway was the first commit's per-node variant and the downgrade
came from replacing the plain form rather than reducing beside it;
rule C's `dyn Any` objection was a corner, and the spec's candidate
shape — the plate's own ceiling — had never been built. R2, by
execution: the §1 table, the sole deliverable, was read off a broken
instrument (a stale thread-local mis-attributing each replay's first
decisions; a detector comparing one column), and a rounded-corner
pad refuses first on a constructor-DECLARED tangency — the consumer
the registered-identity door was said to lack. Landed: A0 alongside
the plain form on a hybrid `i128`/`BigInt` ring; rule C built in the
candidate shape through per-parameter brackets and the ring
interval, measured to zero folds at the shipped bound and dial-off
with its receipts live; the instrument fixed; the pins re-cut
ε-relative with the moved numbers; two M10-4 rows moved by the tier
(the stepped shaft's real study certifies in one leaf); the K
instrument's 1,054 unnamed corpus samples named (F18). The honest
miss stands: the plate's real study is UNMOVED at `7.81e2·ε` with
every rule on — the spec's acceptance is not met, and what bounds it
next is filed with the rendered residual
(`plate-rim-residual-needs-the-wide-coefficient-ring`). Two tally
candidates, one each way; the lane died twice on Fable limits and
was resumed in place both times. Sixteen deviations. The spec leaves
`docs/` with this merge; the exit walk stays a draft — whether a
further unit for the plate is owed goes to Ev.

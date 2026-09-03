# M10 log — the error-propagation MVP

Narrative record; the plan is `docs/M10-PLAN.md`, the design record
`docs/ERROR-DESIGN.md` (E1–E11, ratified #110). Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-08-29)

Opened on Evan's direction ("you'll be doing M10, error
propagation"), by a fresh orchestrator on a remote container. The
plan is a DRAFT design conversation — nothing dispatches until Evan
ratifies it; this entry records the operational facts that hold
either way.

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `m10/`** — unit branches
  `m10/<unit>-<slug>`, orchestrator branch `m10/orchestrator`
  (Evan authorized the prefix at opening; the harness-designated
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
  review targets reclaimed at report time (Evan, at opening:
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

Evan ruled all five plan questions in-conversation the same day:
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
PROFILES-V2 recorded the asymmetry and reserved it for Evan's
eyes; M10 is where the same parameter feeds both slot kinds for
real. Plan amended: unit **M10-P** added (design pass first, its
design PR waits for Evan); M10-3/M10-4 carry the
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

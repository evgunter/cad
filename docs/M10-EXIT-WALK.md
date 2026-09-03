# M10 — exit walk

STATUS: **PROPOSED FOR RATIFICATION** (2026-09-03; an `[ev]` PR —
a program is closed when its exit walk is ratified, and this walk is
to be M10's done-state of record). Every claim below is answered
from main: M10-D (#1146), M10-DI (#1154), M10-1 (#1147), M10-P
(#1174), M10-2 (#1213), M10-3 (#1231), M10-4 (#1627), M10-5 (#1638)
and M10-6 (#1685) are all merged. One item in this walk is a DESIGN
question for Ev rather than a report (honesty row 11: the
`min_clearance` arity in ERROR-DESIGN); the rest is record.

The plan's criterion rows are quoted VERBATIM from
`work/m10/plan.md`'s "Exit shape (proposed)" and answered one at a
time. Honesty rows follow — the things a reader would be misled by if
they were left out.

## Criterion rows

**1. "Distributions, Measures and Assertions persist and
round-trip"** — MET. `Distribution::{Band, Uniform, Normal,
TruncatedNormal}` on continuous parameters (M10-1, schema v15, one
`check()` at the edit AND load doors); `Node::Measure` with
`MeasureRef { at, name }` and `Node::Assertion` report-only by
construction with the three-state verdict `Holds / Violated /
Unevaluated` (M10-2, schema v17 — the unit claimed v16 naming its
rival and the resolution rule in advance, lost the merge race to
LIB-G16 and repaid every fixture by that rule). Populated goldens
round-trip bit-exact at both steps; prior versions refuse typed with
the recourse. The `min_clearance` primitive (E3's last) shipped in
M10-6 as `MeasurePrimitive::MinClearance` over TWO selections, with
NO schema step: BOOL-13 had removed the persisted schema version
between the plan and the unit ("No schema version, on purpose"), so
wire growth is additive and an unknown variant refuses typed with
the regenerate recourse — the plan's "v18" has no field to bump
(M10-6 deviation D1). Its populated golden is the every-form wire
fixture. At every point scalar the measure carries a typed ABSENCE
as a value, so an assertion over it reads E10's third state,
`Unevaluated`, in an ordinary build; at `Interval` its value is the
engine's bracket (row 7's row 1 gates on it).

**2. "`drive` certifies, refuses and prices honestly with coverage
summing to 1 and chamber containment reported when it holds"** —
MET, after the fix pass that made it true: leaves certify on EXACT
verdict-vector equality with no width anywhere, refused mass is
priced per reason, and the tail is ADDITIVE — both reviewers found,
from independent fixtures, that the first version composed
unconditional mass columns as conditional and under-reported the E10
honesty gate by the whole tail, invisible because every shipped
fixture was bounded. Containment is pinned on both arms. Flip naming
routes through the tree's one verdict-diff engine
(`resolve::vdiff`), not a second one.

**3. "the e4 door is open and every sensitivity is chamber-certified
or `local_only`"** — MET. The door is a compiler fact
(`e4_dual_door.rs`, M10-DI) and a runtime one (a full corpus build at
`Dual64`, value channel bit-identical). Every sensitivity carries
`Chamber::{ChamberCertified, LocalOnly}` as a field of the derivative
arm — no third state — and the certificate is CONTENT-TIED to the
build: the leaf about to be cited is replayed with the drive's own
options and its recorded per-node keys compared before any mark is
written (both M10-4 reviewers demonstrated a stale or foreign verdict
certifying an edited document before that tie existed). A parameter
that feeds a loft or sweep section refuses `SeedPinnedSection` rather
than producing the silent zero the C6 seam entry predicted.

**4. "stackups gate on certified worst-case only"** — MET.
`worst_case` is the hull of `Interval` evaluations over certified
leaves, tangent-free by type; `contribution` and `rss` are advisory,
labeled, and forfeit under E9; a Band contributor refuses the RSS
whole naming every Band; a study that certifies nothing hands back
`NothingCertified` carrying its `LocalOnly` sensitivities, coverage
and receipt rather than a data-free error.

**5. "the trichotomy answers over box × domain with f64-verified
violation witnesses"** — MET, with three disclosed reaches (honesty
rows 3–5). `Holds / Violated / Refused` over a certified leaf, every
cell pair classified at exactly two funnel sites (ledger row F17), the
receipt identity `discharged + violated + refused + abandoned ==
splits + candidates` riding the report, the BVH's admission threshold
carrying the funnel's band, and a fold over a drive that certified
nothing REFUSING. Both M10-5 reviewers found the first fold passing
over zero leaves and the first prune deciding inside the band; the
fix pass closed both at the root.

**6. "the Dual question is ANSWERED and the
`Bounds`/`CertifiedEnclosure` cleanup landed (#687, #701 closed)"** —
MET. DL1–DL6 ratified (`docs/DUAL-DESIGN.md`, #1146): a dual is
tangent transport and never certifies (the D1 hedge CLOSED),
`ContentBits for Dual` feeds both channels so the memo cannot alias
passes, DL3's scalar-policy seam is a typed `AtRestOutcome`, and DL3's
pairing hook — unenforced at M10-DI, the adjudication's named
obligation — is a typed two-half gate since M10-4. #687 and #701
closed at M10-DI's merge.

**7. "the three E10 CI rows are live"** — MET, and read executing by
STEP conclusion on the final head's run (33784640811) and on the
frozen review head's re-gate (33767259261), never by job name.
Row 1 (assertion gating): every registered document's assertions
`Holds` over the certified leaves with refused + tail mass inside
that document's RECORDED budget, priced or forced stated as a type;
a completeness guard reds a document that grows an assertion without
a budget; the register holds the two-hole plate, a `min_clearance`
neck, a Band-carrying placement and a document whose drive actually
subdivides — in the row's own registry, not the goldened corpus (D7).
Row 2 (goldened accounting): M10-3's planted-flip and terminal-sliver
accounting bit-exact against three ε-KEYED goldens (the masses are
ε-invariant, their bits are four ulps apart — D6), re-blessable by a
documented env door, an unblessed ε red rather than green. Row 3
(the driver K population): on the k-lint axis, rule 1 GATES and
rules 2/3 are advisory by a flag no caller can widen — see criterion
9. The tour cell rides a fourth step (`demos tour suite`) because the
render lane that walks the tour passes no `--features interval`. The
rows ride the sampled matrix: rows 1–2 on `lane=interval`, row 3 on
`klint_row ∈ {dev-probe, all}`; the unit's heads pinned all three by
trailer.

**8. "the two-hole-plate cell ships in the tour"** — MET.
`demos/tour/src/tolerance.rs`, two stops through public doors, its
own test asserting the numbers the captions print. Stop 1, the real
study (±0.05 mm on the spacing, σ = 0.01 mm on the radii): 0 certified
/ 1024 refused, and the refusal carries the answer — the three
`LocalOnly` sensitivities, the coverage, the budget (refused 99.46%,
tail 0.54%, unresolved 100%) and the MC estimate labeled advisory —
with the ε-scale ceiling stated as this MVP's limit (honesty row 1).
Stop 2, the ε-scale box: one certified leaf, the certified worst case
beside the RSS figure, the tail on every line. The cell was the site
of one of the program's tally candidates (honesty row 13): as first
shipped it decided "the requirement FAILS" on a raw float while the
document's own `Assertion` read `Holds`; it now reads the verdict
off the assertion node through a door that did not exist
(`analysis::assertion_at`), and the captions carry the awkwardness
findings as findings.

**9. "k_stats carries driver rows (the K re-open trigger armed with
real data)"** — MET at M10-6, with an honesty row (row 6 below) on
how it got there. Hosted, on the k-lint axis: 257,025 driver samples
per ε row at 1e-6 / 1e-9 / 1e-12, rule 1 (an in-band indeterminate —
the trigger E6 names) = 0 at every row, against 65,992 rule-2/3
flags the population piles up just outside the escalation band (a
driver refines margins toward zero by construction; the runbook's
recourse 2, recorded as the M10 addendum in `docs/K-REPORT.md`).
Rule 1 is not demotable by any caller. The road there: M10-3 shipped the funnel row (`KProbe::CertifiedMidpoints`,
the sweep wiring, the `driver/` CSV beside the linted one) behind
`probe,interval`, off by default; the hosted K row is M10-6's. The
row was never executed hosted until another program's census repair
(#1268) exposed it, and four tracker items reported it red — three of
them naming ε as the cause. It was not: the fixtures are ε-relative
and certify the same leaves at every row; the panic ("nothing
certified, nothing to sample") was already unreachable on main after
#1343 raised the row's leaf budget, and the census half was #1268's
own fix. What remained was the SHAPE — a row that would panic rather
than report an empty population — and M10-6's hotfix (#1670) makes
the row a census line that reports `certified=0` honestly, asserts
the biconditional, and plants the empty case.

**10. "every unit merged on its own green hosted head"** — MET. Every
unit's final code head carried its own green run on an asked-for or
drawn point (the rows in `docs/MODEL-AB-LOG.md` name each run); the
docs-only state-syncs rode the unit PRs on top of those heads.

**11. "the walk convention applies at exit"** — this document.

## Honesty rows

**1. Certification widths are ε-scale, so the MVP's certified numbers
exist only over boxes a few ε wide today.** Measured by M10-3 and
re-measured by M10-4 and M10-5 on three predicate families: the
certification predicates are checked identities whose interval
enclosure widens with the box (`[0, c·w]`, c ≈ 2–4), so a leaf goes
definite only below ~ε/8 and a real ±0.1 mm study refuses all of its
mass as `Budget` — the stackup returns `NothingCertified`, the
clearance fold refuses over nothing. The verdicts stay honest; the
sentence "2.1% of the tolerance mass has no valid build" is not one
the MVP can say about a macroscopic box. The class's home is issue
1191; the fix is a geometry conversation (re-associating the identity
so the shared parameter cancels, or certifying against a mean-value
form), not this program's.

**2. The profile lift was ratified WITH a hedge** (Ev, on #1151: "not
totally sure about this one, but I think we can proceed"). Guided
replay selects structure once at f64 and re-verifies every consumed
decision at the lane scalar; the f64 path is bit-identical and pass 2
defaults OFF. What it bought: interval and Dual seeds propagate
through profile dimensions (M10-3's door, M10-4's ∂gap/∂r = −1 through
the lifted cylinder carrier). What it does not: a loft's or sweep's
section stays f64 by C6/D9 and a seed on it refuses typed.

**3. A clearance `Violated` is a claim about carrier WINDOWS**, which
are supersets of the trimmed faces: a slider in a U-channel reads
`Violated` at c = 0.3 with true clearance 0.5. `Holds` is sound for
the faces; tightening needs the face boundary in chart coordinates
(`work/m10/clearance-window-tightening-needs-chart-boundary.md`).
The superset reaches the MEASURE too: `min_separation`'s bracket
`[lo, window_hi]` is over windows, so `lo` bounds the face measure
from below and `window_hi` bounds it in neither direction — the
first version's docs sold it as a containment-true enclosure and an
L-cap over a notch certified a FALSE `Violated` and a FALSE `Holds`
(M10-6 R1, by execution). It is a type now (`Certified::{Enclosure,
LowerBoundOnly, Neither}`): the two assertion arms that read `lo`
gate, the two that would read `window_hi` refuse `Unevaluated {
WindowSuperset }` until tightening lands, and a `min_clearance`
under arithmetic refuses both.

**4. The self-intersection arm reports a COINCIDENCE, never a signed
penetration depth**: the margin is a norm minus zero, so gross
interpenetration is reported as "these surfaces touch" with a ~1e-16
witness distance — reachable at all only since the fix pass's exhibit
arm (`work/m10/signed-penetration-depth.md`).

**5. The witness is the closest pair the f64 rebuild FOUND** (a
station lattice), which attains the true closest approach on flat
pairs and is a near pair on curved ones; and every `Violated` is
order-dependent in which witness it finds first (D9-fixed, a property
of the schedule).

**6. The K funnel row was built and not executed hosted for four
days** (criterion 9): the reports that found it misnamed the cause
as ε; the defect that was real was a row that panicked on an empty
population instead of reporting it. Fixed at #1670, together with two
pre-existing ε-fragile M10-5 witness-floor assertions (an absolute
`1e-9` against an ε-relative box) the same pinned point exposed.

**7. Three plan-named capabilities ship as seams, not machinery.**
`Dual<Interval>` contribution bounds (M10-4 deviation 3,
`work/m10/contribution-bounds-via-dual-interval.md`); the
monotonicity accelerator's oracle (`MonotoneOracle` with `NoTangents`
— a lying oracle is indistinguishable from the truthful one on every
buildable fixture, pinned and disclosed); and the issue-1055 curved
wall-clearance arm, ruled a STRETCH at Q5 and NOT landed — the valve
is a layering question (a curved gate above editor-core, or a
duplicate engine inside topo), filed with the cost figures.

**8. The measure's parallelism lever floors at 1 metre**, so for any
sub-metre part the floor is the lever and the operands' separation
never enters; disclosed at the site, the redesign owed
(`chart_region.rs`'s standing criticism).

**9. Solver walls are vacuous** (ruling Q1: no W2 solver);
`Infeasible`/`Bifurcation` exist at the type, documented unreachable,
mechanically scanned unconstructed.

**10. The A/B record.** Nine blinded dual reviews at ordinals
500–507 (M10-D was a design pass); samples #39, #40, #43, #49, #50,
#114, #115, #118. Unilateral execution-class MAJOR candidates: M10-1
R2 (the deep-tail `1 − erf` cancellation), M10-2 R2 (`Holds {
measured: inf }`), M10-3 R2 (the second verdict-diff engine), M10-4
R2 (the loft's silent zero), M10-5 R1 (the sweep that never stopped)
and R2 (the unreachable violation arm) — the program's first
symmetric pair — and M10-6 R1 twice (the window-superset enclosure
sold as containment-true; the tour deciding on a float against its
own assertion); the coding is the blinded adjudication's. Two lanes
died at the account session limit before their first commit and were
redispatched fresh four days later against a main that had moved 524
merges; both delivered. M10-6's fix lane died five times on API 529s
and was resumed from its worktree each time, never redispatched. One
reviewer's report arrived without its rubric and the lane was resumed
for it (the M9-3 missing-data shape avoided because worktrees are
kept until a unit concludes). One sample number collided with a
concurrent recorder (M10-P, #42→#43 by main's merge order). Two
orchestrator-direct hotfixes: an adopted probe row asserting
unconditional door parity against DL3 (#1193), and two adopted rows
with absolute ε slack that were vacuous at 1e-12 and red at 1e-6
(#1651). One orchestrator incident, recorded in the log for the next
brief: re-gating a lane's frozen head by resetting the shared local
branch ref displaced the lane's unpushed commits under its live
worktree — recovered from the reflog, nothing lost, the rule written
down.

**11. ERROR-DESIGN's `min_clearance` is written unary and the
primitive shipped binary — a doc revision for Ev.**
`docs/ERROR-DESIGN.md:142` lists `min_clearance(sel)` among the F7
primitives and `:291` says "a named selection (a `min_clearance`
Measure + E10 assertion)"; the M10-6 spec said two selections, M10-5's
engine is a pair query (`clearance(a, b)`), and the door shipped as
`MinClearance { a, b }` over two M10-5 `Selection`s (a body against
itself is that selection's self-clearance). The design doc's arity is
wrong by one; the unit did not edit it because DESIGN-family
revisions are discussed with Ev first (M10-6 deviation D8). PROPOSED:
amend both lines to the pair form. Ratifying this walk ratifies the
amendment unless Ev says otherwise.

**12. `VerdictVector::certifying` moved the verdict-vector key of
EVERY assertion-carrying document** (M10-6 deviation D10, R2's
finding): a report node must not gate certification (the E10 v1
ruling), and without the filter a `min_clearance` assertion —
`Unevaluated` at every f64 witness by construction — would refuse
every leaf of every document carrying one. The move is intended and
pinned bit-exact on a `min_clearance` neck AND a plain-`Distance`
document; what was defective was the silence, and the unit's own
"keys bit-identical" claim was retracted for it.

**13. The MVP's reason-to-exist demo cannot show a definite certified
failure and a real RSS divergence at once at this ε** (M10-6
deviation D13, measured): a definite `Violated` needs the bound a
K·ε past the enclosure, which puts it where the 3σ figure fails too;
the divergence window is ~4.6e-11 against a 1e-8 escalation
threshold, 218× narrower, and no certifiable box closes the gap (the
drive certifies at a spread of ~ε and refuses everything at 4ε). Stop
2 prints the assertion's verdict and the window's size against the
threshold, and adds one bound a decade past it where the same
assertion reads a definite `Violated`, so a reader still sees the
gate gate. This is honesty row 1 seen from the demo's seat.

**14. Two seams the reporting layer left typed but unfinished.**
`MinClearanceRefusal` ferries `(class, String)` rather than the
engine's `ClearanceRefusal` it mirrors, and since the fix pass an
assertion arm dispatches on that string
(`work/m10/min-clearance-refusal-stringly-twin.md`, D12); and the
leaf histogram re-evaluates every certified leaf, duplicating replays
the stackup already did — disclosed at the site, a cost not a
soundness claim.

## Slate disposition

| item | state |
|---|---|
| M10-D | RATIFIED (#1146) — DUAL-DESIGN DL1–DL6; the D1 hedge closed |
| M10-DI | MERGED (#1154, sample #40) — the Dual contract in code; #687/#701 closed |
| M10-1 | MERGED (#1147, sample #39) — distributions, schema v15 |
| M10-P | MERGED (#1174, sample #43) — the profile-parameter lift, ratified with a hedge |
| M10-2 | MERGED (#1213, sample #50) — Measures and Assertions, schema v17 |
| M10-3 | MERGED (#1231, sample #49) — the E6 driver |
| M10-4 | MERGED (#1627, sample #114) — sensitivities and the stackup |
| M10-5 | MERGED (#1638, sample #115) — clearance and self-intersection; the 1055 arm not landed (valve filed) |
| M10-6 | MERGED (#1685, sample #118) — `min_clearance` + the `min_separation` door, the reporting layer, the three E10 rows, the MC lane, the histogram, the tour cell |

## Open, named, not this program's

Issue 1191 (the certification-width class), 1143 (the poison-vs-widen
contract, `work/m10/certified-lane-non-real-contract-audit.md`), 1055
(the curved wall-clearance consumer — VERBS + M10 design question),
1254 (the k_stats escalation channel and its banked redo — the
hosted row now feeds it real data), 1255
(three per-node verdict shapes), 1274 (the reader census in worktree
checkouts), 1183 (`viewer::bounds` sampling where the interval lane
could certify), the E10 `build()`-gating sub-question (report-only
stands; an `[ev]` ruling if re-opened), the subgradient-at-a-kink
report mark (banked at M10-4), the GUI's invisibility of a
distribution (M10-1 R1's note), the stringly `MinClearanceRefusal`
(row 14), and the ERROR-DESIGN arity amendment (row 11) — the one
item here that is Ev's to rule on.

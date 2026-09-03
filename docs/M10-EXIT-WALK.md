# M10 — exit walk

STATUS: **DRAFT** (M10-6, the program's last unit, is dispatched and
not merged; its rows below are marked PENDING and this walk is not
offered for ratification until they are answered from main). A
program is closed when its exit walk is ratified; this walk is to be
M10's done-state of record. Nothing here is a claim about work not
yet merged: M10-D (#1146), M10-DI (#1154), M10-1 (#1147), M10-P
(#1174), M10-2 (#1213), M10-3 (#1231), M10-4 (#1627) and M10-5
(#1638) are all on main.

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
the recourse. The `min_clearance` primitive (E3's last, schema v18)
is M10-6's — PENDING.

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

**7. "the three E10 CI rows are live"** — PENDING (M10-6).

**8. "the two-hole-plate cell ships in the tour"** — PENDING (M10-6).

**9. "k_stats carries driver rows (the K re-open trigger armed with
real data)"** — PARTIALLY MET, and this is an honesty row (row 6
below). M10-3 shipped the funnel row (`KProbe::CertifiedMidpoints`,
the sweep wiring, the `driver/` CSV beside the linted one) behind
`probe,interval`, off by default; the hosted K row is M10-6's. Under
it, main is RED on the k-lint axis at eps=1e-6 today: the driver's
K-probe dump row panics "nothing certified, nothing to sample" where
the ε-scaled fixture certifies no leaf, and the probe-suite census
cannot see the interval-gated suite — four tracker items, M10's
ground, assigned to M10-6's lane as a hotfix ahead of the unit.

**10. "every unit merged on its own green hosted head"** — MET. Every
unit's final head carried its own green run on an asked-for or drawn
point (the rows in `docs/MODEL-AB-LOG.md` name each run); the two
docs-only state-syncs rode the unit PRs.

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

**6. The K funnel row has open reds under it** (criterion 9): the
row was built and never executed hosted until another program's
census repair exposed it; four items, one defect, M10-6's hotfix.

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

**10. The A/B record.** Eight blinded dual reviews at ordinals
500–506 (M10-D was a design pass). Unilateral execution-class MAJOR
candidates: M10-1 R2 (the deep-tail `1 − erf` cancellation), M10-2 R2
(`Holds { measured: inf }`), M10-3 R2 (the second verdict-diff
engine), M10-4 R2 (the loft's silent zero), M10-5 R1 (the sweep that
never stopped) and R2 (the unreachable violation arm) — the program's
first symmetric pair; the coding is the blinded adjudication's. Two
lanes died at the account session limit before their first commit
and were redispatched fresh four days later against a main that had
moved 524 merges; both delivered. One reviewer's report arrived
without its rubric and the lane was resumed for it (the M9-3
missing-data shape avoided because worktrees are kept until a unit
concludes). One sample number collided with a concurrent recorder
(M10-P, #42→#43 by main's merge order). Two orchestrator-direct
hotfixes: an adopted probe row asserting unconditional door parity
against DL3 (#1193), and two adopted rows with absolute ε slack that
were vacuous at 1e-12 and red at 1e-6 (#1651).

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
| M10-6 | DISPATCHED — reporting, CI rows, MC lane, demo; PENDING |

## Open, named, not this program's

Issue 1191 (the certification-width class), 1143 (the poison-vs-widen
contract, `work/m10/certified-lane-non-real-contract-audit.md`), 1055
(the curved wall-clearance consumer — VERBS + M10 design question),
1254 (the k_stats escalation channel and its banked redo), 1255
(three per-node verdict shapes), 1274 (the reader census in worktree
checkouts), 1183 (`viewer::bounds` sampling where the interval lane
could certify), the E10 `build()`-gating sub-question (report-only
stands; an `[ev]` ruling if re-opened), the subgradient-at-a-kink
report mark (banked at M10-4), and the GUI's invisibility of a
distribution (M10-1 R1's note).

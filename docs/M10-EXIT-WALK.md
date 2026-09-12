# M10 — exit walk

STATUS: **PROPOSED FOR RATIFICATION** (re-cut 2026-09-12; first cut
2026-09-03 and made a DRAFT the same day on Ev's ruling that the
program stays open until certification is parameter-aware — "that's
the whole point of this machinery"; an `[ev]` PR — a program is
closed when its exit walk is ratified, and this walk is to be M10's
done-state of record). Every claim below is answered from main:
M10-D (#1146), M10-DI (#1154), M10-1 (#1147), M10-P (#1174), M10-2
(#1213), M10-3 (#1231), M10-4 (#1627), M10-5 (#1638), M10-6 (#1685),
M10-7 (#1725), M10-8 (#1828), M10-9 (#2048) and M10-10 (#2100) are
all merged; ERROR-DESIGN E12 and its E3 amendments are ratified
(#1712). Nothing in this walk is a design question left for Ev: the
one the first cut carried (the `min_clearance` arity) was ratified
with E12.

The plan's criterion rows are quoted VERBATIM from
`work/m10/plan.md`'s "Exit shape (proposed)" — including the row Ev
added at the first cut — and answered one at a time. Honesty rows
follow — the things a reader would be misled by if they were left
out.

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
rows 9–11). `Holds / Violated / Refused` over a certified leaf, every
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
STEP conclusion and by LOG, never by job name. Row 1 (assertion
gating): every registered document's assertions `Holds` over the
certified leaves with refused + tail mass inside that document's
RECORDED budget, priced or forced stated as a type; a completeness
guard reds a document that grows an assertion without a budget; the
register holds the two-hole plate, a `min_clearance` neck, a
Band-carrying placement and a document whose drive actually
subdivides — in the row's own registry, not the goldened corpus
(M10-6 D7). Row 2 (goldened accounting): M10-3's planted-flip and
terminal-sliver accounting bit-exact against three ε-KEYED goldens,
re-blessable by a documented env door, an unblessed ε red rather than
green. Row 3 (the driver K population): on the k-lint axis, rule 1
GATES and rules 2/3 are advisory by a flag no caller can widen — see
criterion 9, and honesty row 6 for the four days the row could not
red. Since 2026-09-04 main gates the WHOLE matrix on every run (both
lanes × three ε rows × five k-lint unifications; the sampled-matrix
trailers the first cut described no longer exist), and the tour cell
rides the `release-default` k-lint unification.

**8. "the two-hole-plate cell ships in the tour"** — MET, and the
cell's stop 1 is now the CERTIFIED study it was written to be.
`demos/tour/src/tolerance.rs`, two stops through public doors, its
own test asserting the numbers the captions print — leaf counts,
masses, the hull over every certified leaf and its slack at both
ends. Stop 1, the real study (±0.05 mm on the spacing, σ = 0.01 mm on
the radii), at the tour's 512 leaves: the requirement reads `Mixed`
— held on 0.8337 of the mass, VIOLATED on 0.0002, unresolved on
0.1661 — with the certified worst-case hull printed beside the RSS
figure and the padding finding's caveat (honesty row 3). At 1,024
leaves the same study certifies 431 leaves, 89.07 % of its mass, hull
`[0.419, 0.845]` mm against the 0.5 mm floor (criterion 11). Stop 2,
the ε-scale box, is kept as the record of what the first cut could
say. The cell was the site of one of the program's tally candidates
(honesty row 15): as first shipped it decided "the requirement FAILS"
on a raw float while the document's own `Assertion` read `Holds`; it
reads the verdict off the assertion node now, and the captions carry
the awkwardness findings as findings.

**9. "k_stats carries driver rows (the K re-open trigger armed with
real data)"** — MET, with two honesty rows on how (rows 6 and 7).
Hosted, on the k-lint axis at the program's last head: 74,423 driver
samples per ε row at 1e-6 / 1e-9 / 1e-12 — 48,039 decided as
symbolic THEOREMS, 140 through a registered identity, 26,244
numerically — rule 1 (an in-band indeterminate, the trigger E6 names)
= 0 at every row, against 6,862–7,081 rule-2/3 flags the population
piles up just outside the escalation band (a driver refines margins
toward zero by construction; the runbook's recourse 2, recorded as
the M10 addenda in `docs/K-REPORT.md`). Rule 1 is not demotable by
any caller. The outcome vocabulary (`Definite`, `Indeterminate`,
`Invalid`, `SymbolicZero`, `SignGated`, `Registered`) has ONE home
(`SampleOutcome::ALL`/`token()`), pinned across the workspace
boundary by k-lint's own test, since the day M10-7's review found the
row linting nothing (row 6). The population is smaller than at M10-6
(257,025 per row then) for two reasons stated in the rows: the
fixtures certify in fewer leaves as the tier strengthens, and the
plate fixture now certifies in one — "a stronger tier thins the K
sample" is a trend the next K re-open should watch.

**10. "every unit merged on its own green hosted head"** — MET. Every
unit's final code head carried its own green run on an asked-for or
drawn point (the rows in `docs/MODEL-AB-LOG.md` name each run); the
docs-only state-syncs rode the unit PRs on top of those heads.

**11. "a macroscopic box certifies — the two-hole plate's real study
returns certified leaves bounded by genuine flips, not by ε (E12;
Ev's condition for closing the program, 2026-09-03)"** — MET, and
the reviews made the sentence exact; read it at the leaf and at the
ceiling separately. At the LEAF: the plate's real study (±0.05 mm on
the spacing, σ = 0.01 mm on the radii) driven whole at 1,024 leaves
certifies 431 and refuses 593 on budget — 89.07 % of the mass, a
certified worst-case hull `[0.419, 0.845]` mm against the 0.5 mm
floor, the nominal in a certified chamber, both sensitivities
chamber-certified — and every refused leaf, refined to any depth by
both reviewers, is bounded by the document's own `assert_bound`
alone: the real flip, which enters the box at 0.625 of the study,
where the web can genuinely fall below the floor. Not by ε: at three
ε rows the numbers are the same to the digit. At the CEILING: the
widest box that certifies WHOLE is 0.263 of the study at ε = 1e-9
and 1e-12 (0.237 at 1e-6), and that number is bounded by DEPENDENCY
WIDENING of the assertion's own affine margin — `web − bound = 1e-4 +
2·Δhs − Δr_a − Δr_b` has true range `1e-4 ± 1.6e-4·s`, strictly
positive at s = 0.263 while its enclosure straddles zero — the class
M10-7's review filed (`work/m10/real-margin-dependency-widening`),
which subdivision resolves and which is what stands between 0.263
and the flip at 0.625. The road there is honesty row 1: four units
(M10-7 through M10-10), three of whose premises were wrong and were
corrected by review before they shipped. The tour's stop 1 is the
certified study (criterion 8). The annulus certifies 0.70–0.84 of
its study bounded by real margins; a link, a filleted bracket and a
rounded pad do NOT certify their studies (honesty row 2).

**12. "the walk convention applies at exit"** — this document.

## Honesty rows

**1. Certification is parameter-aware, and how it got there.** The
first cut's row 1 said certified numbers existed only over boxes a few
ε wide: the funnel's identity-shaped predicates (an edge endpoint on
its carrier, consecutive walls cosurface) widened with the box because
interval arithmetic cannot see that two occurrences of one parameter
are one number. Ev's ruling made that the program's exit condition,
and E12 (#1712) is the design: a symbolic tier beside the lane value
in which a margin identically zero in the parameters decides `Zero`
at any width. **M10-7** built it (`geom_core::sym`: a hash-consed DAG
over the parameters, a lazy exact-rational quotient normal form with
opaque atoms and a freezing budget, the driver replaying at
`Sym<Interval>` through one door) and moved the slab's certifying
half-width from ε/8 to 0.488 of a unit nominal — and missed every
arc, because an arc's carrier carries `sqrt` atoms the form cannot
reduce. **M10-8** measured the arc family's atom algebra before
shipping it and reported the algebra inert; both reviews showed the
negative result was an artefact of the implementation (the freezes
were `sqrt` of exact-rational CONSTANTS, which a value-free fold
relieves — R1 moved a bracket 10.4× the unit said could not move),
and the fix pass shipped the fold alongside the plain form on an
arbitrary-precision coefficient ring; the plate stayed at
`7.81e2 · ε`. **M10-9** took E12's provenance reserve — a constructor
registers a same-object identity it guarantees, verified at the
witness, counted apart from theorems — and discharged the arc's rim
and span identities completely; the ceiling did not move, and the fix
pass found why one level up: every "what bounds this document"
sentence since M10-7 had been read at TWICE the ceiling, where
evaluation order picks the name, and at the ceiling every document
was bounded by one predicate the door cannot reach (the carrier
against its own scaffold pushforward); a staged walk showed the plate
FOUR identity residuals from a macroscopic margin, worth 2× together
and then 1.68e5× — the family had to go at once. **M10-10** made the
arc's trig atoms exact (`sin`/`cos` of `q · atan(bulge)` in closed
form, halves on the positive branch by `atan`'s range, the chart's
phase by the syntactic non-negativity of `r²/√r²` — theorems of the
reals, no value read) with rules A/B per node in a walk made linear,
and the four residuals went at once: criterion 11. What the tier
does NOT do, each with its durable home: it reads no value (rule C,
the one clause-3 fold, is built and dial-off — `sign_gated` = 0
everywhere); a registered identity is an AXIOM whose soundness rests
on the registrant's proof, because at the interval scalar the witness
accepts any coincidence whose enclosures meet (M10-9 R1, pinned as
the limit); an iterated quantity — an SSI march point, a projection
foot — has no expression in the parameters and is S-CERT's frontier
(`work/cert/param-box-certification-of-implicit-quantities`); a
parameter bulge is outside the mechanism and a literal bulge other
than 1 leaves residue (`work/m10/rule-d-reaches-the-unit-bulge-only`).

**2. Three documents do not certify their studies, for a reason no
affordable dial changes.** A slotted link, a filleted L-bracket and a
rounded-corner pad — the reviewers' own documents — stay at identity
residuals (`carrier_matches_mapped_source`; the pad's `line_span`,
itself an identity of the fillet construction) whose squared
components FREEZE at the coefficient/term budget: M10-10's reviewers
raised the per-node cap eightfold and the budget eightfold and moved
nothing at 16× the leaf cost. What they wait on is the scaffold
residual's retirement for arc carriers, a PCURVE/D3 question
(`work/m10/plate-ceiling-is-now-the-scaffold-pushforward`), and the
fillet's declared tangency, whose centre the joint classifier
re-derives rather than receives
(`work/m10/fillet-tangency-is-not-the-constructors-node`).

**3. The certified hull is padded by the leaf width, not the lane.**
The stackup's worst case is the hull over certified leaves, and a leaf
that certifies whole pads it by its own width — `[0.419, 0.845]` mm
at 1,024 leaves is looser than the study's true range, which is why
the padding pins now assert the leaf count with the padding at both
ends and the tour's caption says so
(`work/m10/certified-hull-padding-is-the-leaf-width-not-the-lane`).

**4. The symbolic tier is expensive, and nobody has profiled inside
it.** S-TCOST measured the tier at 95 % of the M10-3 interval drive
(20.8× the suite; the degree dial is not a lever, as `drive.rs`'s own
note predicted) while M10-10 was frozen, and M10-10's own hosted
interval leg tripled on one shard (2.3 → 7.3 min); one waste was
removed (the trig pair memoized, a plate leaf 0.20 → 0.15 s), the
rest is disclosed with the job deltas and owed to the profiling row
(`work/m10/symbolic-tier-costs-95-percent-of-the-m10-3-drive`). The
tier is correct and expensive; a unit that makes it cheap has its
measurement waiting.

**5. The instrument that reads "what bounds this document" was wrong
from M10-7 to M10-9, and three units' premises with it.** M10-8's
`ceiling` helper reported the first refusal from a replay at 2× the
widest certifying scale, where several predicates are over the band
and evaluation order (validation before certification) picks the
name. M10-7's ceiling row named the wrong mechanism (R1, by
execution); M10-8's "the algebra is inert" was the instrument's
attribution (R2: a stale thread-local charged each replay's first
decisions to the previous predicate); M10-9's "the bound walks one
predicate per registrant" was the 2× read. Each was found by a
blinded review by execution, each correction is a pin, and the
over-band set at ceiling + δ is now the one spelling
(`m10_8_harness::{over_band_set, bound}`); the finding closes with
M10-10 (`work/m10/first-refusal-at-twice-the-ceiling-is-an-order-artefact`).

**6. The driver K gate could not red from M10-6 to M10-7, and the
row that found it was the review.** M10-7 added a K outcome token
that `tools/k-lint`'s parser did not know, so the driver CSV was
rejected at its first row and ZERO samples were linted while the step
stayed green: `ci.yml` read `PIPESTATUS` after `$?` had reset it — a
hole since M10-6 wrote the step. M10-7's R2 found it; the fix gave
the vocabulary one home and the status capture a form that can red,
and the `PIPESTATUS` pattern across the rest of `ci.yml` went to CIW
(`work/ciw/pipestatus-after-assignment-in-ci-yml`). M10-8 then found
1,054 corpus samples reaching the row `<unnamed>` and NAMED them
through an unlogged evaluator door (ledger F18) rather than
allowlisting the guard. Criterion 9's numbers are read from the job
LOG at every merge since.

**7. The K funnel row was built and not executed hosted for four
days** (criterion 9): the reports that found it misnamed the cause
as ε; the defect that was real was a row that panicked on an empty
population instead of reporting it. Fixed at #1670, together with two
pre-existing ε-fragile M10-5 witness-floor assertions (an absolute
`1e-9` against an ε-relative box) the same pinned point exposed.

**8. The profile lift was ratified WITH a hedge** (Ev, on #1151: "not
totally sure about this one, but I think we can proceed"). Guided
replay selects structure once at f64 and re-verifies every consumed
decision at the lane scalar; the f64 path is bit-identical and pass 2
defaults OFF. What it bought: interval and Dual seeds propagate
through profile dimensions (M10-3's door, M10-4's ∂gap/∂r = −1 through
the lifted cylinder carrier). What it does not: a loft's or sweep's
section stays f64 by C6/D9 and a seed on it refuses typed.

**9. A clearance `Violated` is a claim about carrier WINDOWS**, which
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

**10. The self-intersection arm reports a COINCIDENCE, never a signed
penetration depth**: the margin is a norm minus zero, so gross
interpenetration is reported as "these surfaces touch" with a ~1e-16
witness distance — reachable at all only since the fix pass's exhibit
arm (`work/m10/signed-penetration-depth.md`).

**11. The witness is the closest pair the f64 rebuild FOUND** (a
station lattice), which attains the true closest approach on flat
pairs and is a near pair on curved ones; and every `Violated` is
order-dependent in which witness it finds first (D9-fixed, a property
of the schedule).

**12. Three plan-named capabilities ship as seams, not machinery.**
`Dual<Interval>` contribution bounds (M10-4 deviation 3,
`work/m10/contribution-bounds-via-dual-interval.md`); the
monotonicity accelerator's oracle (`MonotoneOracle` with `NoTangents`
— a lying oracle is indistinguishable from the truthful one on every
buildable fixture, pinned and disclosed); and the issue-1055 curved
wall-clearance arm, ruled a STRETCH at Q5 and NOT landed — the valve
is a layering question (a curved gate above editor-core, or a
duplicate engine inside topo), filed with the cost figures.

**13. The measure's parallelism lever no longer floors at 1 metre.**
The first cut disclosed the floor; E12's E3 amendments (#1712) replace
it with an upper bound on the operands' extent — `reach(a) + reach(b)
+ ‖Δref‖`, a `Carrier` carrying its own reach, no floor — shipped in
M10-7 with a three-way ε-solved falsifier. `mate.rs` keeps the metre
where a datum names no length; that is filed
(`work/issues/mate-lever-needs-the-parts-extent`).

**14. Solver walls are vacuous** (ruling Q1: no W2 solver);
`Infeasible`/`Bifurcation` exist at the type, documented unreachable,
mechanically scanned unconstructed.

**15. The A/B record.** Thirteen blinded dual reviews at ordinals
500–511 (M10-D was a design pass); samples #39, #40, #43, #49, #50,
#114, #115, #118, #124, #145, #150, #173. Unilateral execution-class
MAJOR candidates: M10-1 R2 (the deep-tail `1 − erf` cancellation),
M10-2 R2 (`Holds { measured: inf }`), M10-3 R2 (the second
verdict-diff engine), M10-4 R2 (the loft's silent zero), M10-5 R1
(the sweep that never stopped) and R2 (the unreachable violation arm)
— the program's first symmetric pair — M10-6 R1 twice (the
window-superset enclosure sold as containment-true; the tour deciding
on a float against its own assertion), M10-7 R2 (the driver K gate
disarmed) and R1 (the ceiling's mechanism mis-attributed) — the
second symmetric pair — M10-8 R1 (the constant fold that moved a
ceiling the unit said could not move) and R2 (the declared tangency
as the door's consumer), M10-9 R2 twice (the four-identity walk to
the real margin; the bound mis-named in four documents with
`line_span` shown an identity), and M10-10 R1 (a wrapping shift
admitting a false theorem no document can reach) and R2 (the ceiling
is dependency widening, not a flip) — the third symmetric pair; the
coding is the blinded adjudication's. Twice the orchestrator amended
a spec BEFORE the review freeze because the implementer's honest
report showed the spec had counted wrong (M10-9 A1: the unit of scope
is the constructor, not the identity; M10-10 A1: a value-free fold
the spec's own clause excluded) — recorded in the rows as two
implementer segments each. Deaths and resumes: two lanes died at the
account session limit before their first commit and were
redispatched fresh four days later (M10-4/M10-5); M10-6's fix lane
died five times on API 529s and was resumed from its worktree each
time; M10-8's lane died twice on Fable limits (~21 h lost once) and
was resumed in place; M10-10's R2 died on the Fable limit before its
first step and was resumed in place five days later when the limit
reset — the orchestrator's own session was out on the same limit in
between — with no work lost. One reviewer's report arrived without
its rubric and the lane was resumed for it; two reviewers ended a
turn on detached work nothing could wake them for and were resumed
and told so. Two sample numbers collided with concurrent recorders
(M10-P #42→#43; M10-8 #144→#145) and were renumbered by main's merge
order. Four orchestrator-direct hotfixes: a probe row asserting
unconditional door parity against DL3 (#1193); two adopted rows with
absolute ε slack (#1651); a `TAG_INVENTORY` red on main another
program had routed to an inactive owner; and a selftest that rendered
its fixture at a calendar date the real date caught up with, red on
every run for a morning (#2099). One orchestrator incident, recorded
for the next brief: re-gating a lane's frozen head by resetting the
shared local branch ref displaced the lane's unpushed commits under
its live worktree — recovered from the reflog, nothing lost, the rule
written down.

**16. ERROR-DESIGN's `min_clearance` arity — RESOLVED.** The first
cut proposed amending the unary spelling to the pair form the
primitive shipped with; Ev ratified it with E12's E3 amendments
(#1712, "1712 lgtm"), and the design doc reads the pair form.

**17. `VerdictVector::certifying` moved the verdict-vector key of
EVERY assertion-carrying document** (M10-6 deviation D10, R2's
finding): a report node must not gate certification (the E10 v1
ruling), and without the filter a `min_clearance` assertion —
`Unevaluated` at every f64 witness by construction — would refuse
every leaf of every document carrying one. The move is intended and
pinned bit-exact on a `min_clearance` neck AND a plain-`Distance`
document; what was defective was the silence, and the unit's own
"keys bit-identical" claim was retracted for it.

**18. The MVP's reason-to-exist sentence is sayable now, with its
shape stated.** The first cut recorded that the demo could not show a
definite certified failure and a real RSS divergence at once at ε
(M10-6 D13). At the real study the tour's stop 1 now reads `Mixed`
with the masses on the line: the requirement HOLDS on 0.8337 of the
tolerance mass, is VIOLATED on 0.0002 and is unresolved on 0.1661
at 512 leaves (the corner where the web falls below the floor is
reached at 1,024). "x % of the tolerance mass has no valid build" is
therefore a statement the MVP makes as a certified lower bound with
an unresolved remainder, not as a point estimate — which is what
E12's "certified worst cases become statements about the study's
box" promised. The RSS figure stands beside it, labeled advisory.

**19. Two seams the reporting layer left typed but unfinished.**
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
| M10-7 | MERGED (#1725, sample #124) — E12 built: the symbolic identity tier, the extent lever; the slab certifies at 0.488 of a unit nominal; the arc family missed and named |
| M10-8 | MERGED (#1828, sample #145) — the arc family measured; the constant fold shipped alongside on an arbitrary-precision ring; rule C built dial-off; the plate unmoved |
| M10-9 | MERGED (#2048, sample #150) — the registered-identity door (E12's reserve taken); the rim and span identities discharged; the bound never moved: the 2× instrument found, the four-identity distance measured |
| M10-10 | MERGED (#2100, sample #173) — rule D and the linear per-node walk; the four residuals go at once; **the plate's real study certifies** (criterion 11) |

## Open, named, not this program's

M10's own open items on main, each with its home: the
dependency-widening class that bounds the plate's ceiling
(`real-margin-dependency-widening`); the tier's cost
(`symbolic-tier-costs-95-percent-of-the-m10-3-drive`, S-TCOST's
measurement — the next unit anyone opens on this tier); the reach
limits (`rule-d-reaches-the-unit-bulge-only`;
`plate-ceiling-is-now-the-scaffold-pushforward` — the link and
bracket wait on PCURVE/D3;
`fillet-tangency-is-not-the-constructors-node`;
`declared-tangency-needs-the-registered-identity-door`;
`revolve-carriers-state-only-the-rim`); the door's limits
(`the-span-identity-is-not-a-theorem-of-the-floats`;
`the-witness-slack-is-eps-independent`;
`sym-registration-flattens-two-axes`); the tier's hygiene
(`registered-is-spelled-five-times-and-pinned-once`;
`sym-rs-is-one-file-with-a-347-line-header`; `symbolic-tier-census`;
`interval-self-dot-straddles-before-rule-a`;
`derived-frame-placement-freezes-on-the-symbolic-lane`, DOCM's);
the hull padding
(`certified-hull-padding-is-the-leaf-width-not-the-lane`); the MC
lane's draws
(`mc-lanes-draws-are-not-reproducible-from-outside-the-crate`);
`coincidence-zone-priced-budget-at-the-floor`; the stringly
`MinClearanceRefusal` (row 19, `min-clearance-refusal-stringly-twin`);
`pncad-py-eval-err-variants-outside-the-tag-inventory`; and the
frontier — implicit and iterated quantities — at S-CERT
(`work/cert/param-box-certification-of-implicit-quantities`). Outside
`work/m10/`: CIW's `pipestatus-after-assignment-in-ci-yml` and
`probe-interval-lane-has-no-clippy-row`; the mate lever's metre
(`work/issues/mate-lever-needs-the-parts-extent`);
`symbolic-tier-and-clearance-engine`. Carried from the first cut:
issue 1143 (the poison-vs-widen contract), 1055 (the curved
wall-clearance consumer — VERBS + M10 design question), 1254 (the
k_stats escalation channel — since redone by PROPS as the bracket),
1255, 1274 (the reader census in worktree checkouts), 1183, the E10
`build()`-gating sub-question (report-only stands; an `[ev]` ruling
if re-opened), the subgradient-at-a-kink report mark, and the GUI's
invisibility of a distribution. Issue 1191 (the certification-width
class) is CLOSED by this program: E12 is its answer.

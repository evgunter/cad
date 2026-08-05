# M7 log — orchestrator record

Concurrent-orchestrator arrangement (Evan, 2026-08-04): this log
belongs to the M7 orchestrator (session cad-implement-m7); the M6
orchestrator's live record is docs/M6-LOG.md (read, don't touch).
Protocol: memories/concurrent-orchestrators.md; briefing:
~/.local/share/cad-work/handoff-prompt-m7.md.

## Session start (2026-08-04)

Checklist done: origin/main merged; monitors installed from this
checkout and armed (away-channel with
CAD_SIGNOFF_WATCHLIST=…/signoff-watchlist-m7.txt, disk watchdog,
hourly check-in); CPU canary 1.03s (healthy); disk 96G free;
cargo-slots.txt verified — slot 2 = cad-implement-m7, free; slot 1
= cad-implement-m6 (ev/ci-test-collapse, now PR #179).

**Plan + spec PR (#180)**: docs/M7-PLAN.md assembled from #169 +
D7 + #161 §2 (nothing newly proposed — self-merges per the
standing rule) plus the binding docs/M7-1-SPEC.md and this log.
Feasibility verified before speccing: topo's Euler-operator
vocabulary and geometry-attachment doors are public (step-export's
own test corpus builds bodies through them from outside topo), so
`crates/step-import` needs no kernel edits; hosted CI's
`--workspace`/closure scoping picks up a new member with zero CI
edits; the fixtures' declared uncertainty is 1e-9 and units are
metres (`.expect` volumes are mm³ — ×1e-9).

**A/B, block M7-1**: difficulty for M7-1 (import crate skeleton +
own-corpus round-trip) logged **L** BEFORE the draw — new crate
with a Part-21 parser, an Euler-op assembly algorithm over the
full entity subset (incl. composed_die's 89 faces, 42 reversed),
D7 adoption, and a six-row acceptance suite. Draw (after the
difficulty commit): byte 177 → **(fable, opus)** — M7-1 = FABLE;
opus remainder owed to the next A/B-eligible dispatch (expected:
M7-2, the FreeCAD-authored foreign corpus). Reviewer blinded as
always; row recorded AT MERGE.

**M7-1 DISPATCHED (2026-08-04)**: #180 (plan + binding spec +
this log) self-merged on green; implementer launched on lane
~/.local/share/cad-work/m7-import, branch ev/m7-import (cargo
slot 2 claimed in cargo-slots.txt); PR to be HELD for blinded
adversarial review per standing process. Fence in both the spec
(§0) and the prompt; report lands at
~/.local/share/cad-work/m7-1-report.md.

**Evan's five notes on #180 (comment, 2026-08-04), dispositions:**
(1) parser hand-rolling is necessity not preference — the F6
spike (references/notes/step-spike-report.md) found no usable
Rust STEP semantic layer; spec §Leg A corrected, and ruststep's
working *syntactic* layer + truck-stepio's `in::Table` noted as
precedented dev-dependency oracles. (2) Mäntylä notes
(references/notes/mantyla-ch9..15) are to be read BEFORE the
scans — relayed to the implementer with the main-checkout path
(references/ is git-ignored and absent from lane clones).
(3) reversed faces: the corpus's `.F.` faces are deliberate
S10/S11 output, not a bug — no healing now; reaffirmed to the
implementer. (4) adoption machinery should be reusable for GUI
remedies — refusals carry structured data, recorded in the plan's
contract section and relayed. (5) wild licensed STEP files inside
the subset as a late demo corpus — plan unit 4, deferrable. All
three docs amended; implementer messaged mid-flight (no
acceptance-row changes).

**M7-1 implementation COMPLETE (2026-08-04): PR #183 open, all
six acceptance rows reported green (8/8 nextest), ONE numbered
deviation, blinded adversarial review DISPATCHED.** Headline
measurement: the first re-export is byte-identical to the
COMMITTED fixture for **all 14/14** solid fixtures (row 2 only
required the second export to fix-point the first). Deviation 1:
five sidecars' EXPECT_EDGES record OCC's post-import
normalisation (pole edges, seam splits), not the kernel census —
resolved by asserting the kernel census quoted from the sidecars'
own comments; no fixture/sidecar touched; sidecar KERNEL_* fields
suggested as a design conversation. Architecture: hand-rolled
Part-21 parser (~350 lines, zero new deps; step-export enters as
dev-dependency oracle only); rotation-system Euler assembly
(σ(u)=next(mate(u)) fan orbits, mev/mef/mekr + ring_move/kfmrh
genus, strut+kemr hole-planting, strut+kev anchor rotation) with
a loop-cycle verification pass; file-order fixed-point discipline
(Shell::faces + Cycle::first); D7 adoption ladder
Intersection→TangentIntersection / Seam→MappedCurve with
structured (candidate, refusal) errors per Evan's remedy
directive; then mint_pcurves. Notable discovered facts: the
tangent gate accepts the full circular-trimline class (its
refusal text still names the M5 line-only class — stale, banked);
kev's unconditional Cycle::first re-anchor is the public
loop-rotation door (deserves a topo pin someday); STEP cannot
carry solid grouping (kiss_assembly imports 2 solids / 2 shells,
matching its sidecar). Impl ~441k tokens, ~1.9h wall. Review
assigned attacks: byte-identity provenance (anti-laundering),
deviation-1 adjudication, adoption-ladder corruption probes,
rotation-system stress (genus, permuted-order files), volume
tolerance teeth, same_sense flip fidelity, refusal coverage,
ε_in. Slot 2 → m7-import-review lane.

**M7-1 review returned (2026-08-04): APPROVE-WITH-FIXES,
1 MAJ / 3 MIN / 5 NOTE, deviations 1 reported (verified honest) /
1 silent, rubric 5/4/4; hosted CI 20/20 green; every assigned
attack executed.** The headline held: 14/14 byte-identity
reproduced AND proven un-laundered (perturbed radius flows
through to re-export with the matching closed-form volume;
no code path from source text to export). Deviation 1 adjudicated
HONEST (all five overrides quote the sidecars' own comments
exactly; only edge counts diverge). Adoption ladder has teeth
(four corruption classes → structured Adoption errors with
honest gate reasons; bonus: a tilted GREAT circle legally adopts
as a different valid body — honest, not a defect). Fixed point
holds on non-writer-ordered files (reversed DATA, renumbered
ids). Volume tolerance toothed (1e-7 radius corruption fails the
row) and both closed-form claims reproduced bit-for-bit.
MAJ-1 = the silent deviation: CONVERSION_BASED_UNIT length
contexts import silently as metres (unit check fires only on
instances containing SI_UNIT; refs unresolved) — the forbidden
silent-guess class. Fix pass DISPATCHED (inherits the arm):
MAJ-1 by-resolution unit check + inch-file test; full truncation
sweep (cap 400/7318 contradicted its own comment); MIN-4's three
silent drops → typed refusals (2nd curve set, mixed content,
orphan MSBs); NOTE-5 mojibake arm; 6.00-ulp correction; adopt
review/m7-1's 19 probes BY MERGE (authorship kept). Fenced
findings routed to fresh issue #184 (cross-orchestrator channel):
MIN-3 curved sense flips tier-invisible (kernel-scope — our-tiers
confirmation of the OCC text-level orientation note; executed on
washer, volume bit-identical) + NOTE-7 filleted_die.expect
comment overclaim + the sidecar KERNEL_* fields design
suggestion. Review ~181k tokens, ~35min wall.

**M7-1 MERGED as #183 (2026-08-04): THE KERNEL IMPORTS WHAT IT
EXPORTS.** Fix pass discharged all six items: units/uncertainty
checked by RESOLUTION (a subset SI length unit must exist; a
CONVERSION_BASED_UNIT inch file refuses typed; two distinct
declared uncertainties refuse as ambiguous); truncation sweep
runs every strict prefix (7k+ cuts, the only importing cut is
the trailing-newline one, asserted); the three silent drops are
typed `Structure` refusals (content resolved from the
representation structure — orphan MSBs refuse rather than
guess); string bodies refuse outside the Part-21 basic alphabet
(mirror of the writer's quoting refusal); 6.00 ulps; the 19
review probes adopted BY MERGE with authorship kept — plus the
fix pass caught the reviewer's unit probe being VACUOUS (its
#93 substitution never matched cube's #155; re-anchored id-free
and flipped to assert the refusal). Reported deviation from the
fix instruction, accepted: the probe suite is a third [[test]]
target, adopted verbatim (the two-target norm binds the
acceptance layout, not the adopted review surface). Hosted
27/27; crate suite 28/28. A/B row RECORDED AT MERGE. #184
exchange settled mid-flight: KERNEL_* sidecar shape locked both
sides (full-precision KERNEL_VOLUME_MM3; native-census
semantics with the kiss_assembly solids divergence documented;
no seam/pole accounting), export side implements after Evan's
👍, M7's consumer switch-over tracked for a later seam. Lanes
cleaned; slot 2 freed. Next: M7-2 (FreeCAD-authored foreign
corpus; OPUS block remainder; difficulty logged before
assignment at spec time).

**M7-2 substrate returned + spec written (2026-08-04)**: FreeCAD
1.1.2 dialect measured (13 files kept + full box.step walk;
~/.local/share/cad-work/m7-2-substrate/inventory.md). Headlines:
mm-prefixed SI units 13/13; NO FACE_OUTER_BOUND ever (outerness
must be inferred — multi-ring faces geometrically); FACE_BOUND
.F. is NOT redundant with face sense (4 planar-cap
counterexamples); cones always base-placed (r≠0; apex form never
appears); full sphere = ONE edge-free face bounded by a
VERTEX_LOOP (the genuinely new reconstruction case); periodic
faces arrive seam-unsplit but with the doubled seam edge our Seam
rung expects; NO NURBS-where-analytic, no ELLIPSE, no trim
params, no EDGE_CURVE .F.; 12-13 sig-digit truncation makes
pi-derived identities tolerance-budgeted, not bitwise.
docs/M7-2-SPEC.md written with three firm design elaborations
flagged for Evan: per-literal print-precision budget
(eps_in_eff = max(ε_in, half-ulp of the printed decimal)),
kernel-canonical sphere re-split as REPORTED D7 stage-3
normalization (file 1/0/… → kernel 2/2/2, mapping carried as
data), chart-based outerness inference with typed ambiguity
refusal. **A/B: M7-2 difficulty logged L BEFORE assignment**
(multi-front dialect work: units scaling, outerness, base cones,
vertex-loop reconstruction, structure roots, the first real
ε_in-scale interpretation); arm = OPUS (block M7-1 remainder,
predetermined at the draw). Substrate ~64k tokens, ~10min.

**M7-2 DISPATCHED (2026-08-04)**: spec merged via #187
(self-merge; the three design flags stand as firm proposals
absent Evan comment); implementer (OPUS) launched on lane
~/.local/share/cad-work/m7-2-freecad, branch ev/m7-2-freecad,
slot 2 claimed; PR to be HELD for blinded adversarial review;
report lands at ~/.local/share/cad-work/m7-2-report.md.

**Mid-flight (2026-08-04)**: Evan ruled on spec flag 1 (#187
comment) — flat ε_in replaces the per-literal eps_in_eff budget;
relayed to the implementer BEFORE the paperwork, spec amended via
#188 (self-merged; quantified concession: truncation ~1e-12·|x|
dominated by 1e-10 m under ~100 m; giant-model arm = typed
refusal + per-call override). Flag 2 approved; flag 3 stands.
Then hosted CI went RED on the four ε=1e-6 shards (cylinder
fixture, pcurve MapResidual at re-mint) — signature relayed with
the mm-scale-vs-absolute-ε lead.

**M7-2 implementation COMPLETE (2026-08-04): PR #189 open, CI
green after the ε-fix, review DISPATCHED with heavyweight
adjudications.** Implementer's own headline disclosures:
(1) TWO structure normalizations beyond the spec's sanctioned
sphere case (cone_apex ScaffoldingStrutVertex; torus
NegativeVolume with exact magnitude and INVERTED SIGN) — resolved
via the sphere mechanism with a stop-worthy question honestly
asked; review attack A1 adjudicates HONEST-RECONSTRUCTION vs
SYMPTOM-FLIP vs NEEDS-ESCALATION before I rule. (2) ε=1e-6 root
cause = mm-scale corpus vs absolute ε (the kernel refused
CORRECTLY); fix = CORPUS_EPS_CEILING=1e-8 with derivation, loud
skip above, always-on refused-typed row — attack A2 verifies no
silent matrix shrink. (3) **FIRST IN-BAND K LANDING of the
project** (the #89 re-open trigger class): ε=1e-7, cone_trunc,
props_rim_level_group, margin 5.590169943747308e-7 in
Band{1e-7, 1e-6} — attack A3 makes the measurement unimpeachable
BEFORE it goes to Evan; do not retune anything. (4) Four M7-1
refusal flips (S9 pattern audit = A6); (5) new interpretation
predicates not routed through Decide/K (spec §3 letter vs D7
interpretation-space argument = A7); (6) coincident-locus
MappedCurve rung (A5), 4th test target, 1-ulp cone_apex
fixed-point exception (A8). Impl ~422k tokens, ~2.6h wall incl.
the gate-red loop. Slot 2 → m7-2-review lane.

**M7-2 review returned (2026-08-04): APPROVE-WITH-FIXES, 2
blocking MAJORs (both on adversarial inputs, not the corpus),
fence clean, no silent deviations — all 11 numbered deviations
check out.** MAJ-1: the torus normalization is a SYMPTOM-FLIP —
all four orientation mutations of torus.step import as a
HALF-EDGE-IDENTICAL body certifying +1.2337e-9 m³ bit-identically
(full_torus discards the loop's cyclic order; use-multiset is
reversal-invariant), AND the kernel props torus contribution
never consumes sense_sign (kernel-scope, fenced — posted as a
#184 addendum; the sphere re-mint is honest by control, cone_apex
ruled HONEST-RECONSTRUCTION). MAJ-2: the coincident-locus rung
certifies surfaces but not the CURVE — an off-plane arc launders
through as MappedCurve. MINOR: ceiling-skipped tests print
empty PASS rows (invisible skip). A2 otherwise honest (no gates
widened, fixtures byte-identical, always-on refusal row real);
A4 chart fuzz clean at 1.2e-15 worst; A6/A8 clean. **A3: the K
landing is REAL and unimpeachable** — margin 5.590169943747308e-7
= √5/4×1e-6 m² exactly, an AREA-dimensioned two-length product,
quadratic in model scale (metre-twin ≈ 0.559 m²); standard rows
1e-6/1e-9/1e-12 swept CLEAN corpus-wide — the landing lives only
at ε=1e-7 between CI rows. **Reported to Evan on #89** (the
designated trigger, nothing retuned; comment watchlisted for his
disposition — framed as ε-vs-scale policy evidence, not a K
argument). Fix pass dispatched (inherits the arm): torus refuses
typed on inversion/undecidable orientation naming the kernel gap
(never import wrong); curve certification in the coincident
rung; ceiling-skips become refusal assertions (every ε row runs
real content); probes adopted by merge. Review ~169k tokens,
~48min wall.

**M7-2 MERGED as #189 (2026-08-04): FOREIGN GEOMETRY IS LIVE.**
The kernel now adopts FreeCAD 1.1.2 files across the full
analytic subset. Fix pass discharged all four items with the
torus fix done right: winding derived from the loop's CYCLIC
ORDER via mid-quadrant chart sampling; all four orientation
statements of torus.step behave per ISO (both legal
right-side-out encodings import to ONE body — pinned; both
inside-out encodings refuse typed naming the inversion AND the
kernel props limitation); the judgment call that minted
half-faces derive winding/sense from the material side (copying
would falsely refuse ISO's .F./CW legal encoding) is ENDORSED.
MAJOR-2: carrier_on_surface certifies the curve through the
shared door; the off-plane plant refuses with Surface1Residual.
The ceiling-skip fix introduced a principled third arm:
above-ceiling rows assert the sub-tolerance obligation and
tier 3 must be Ok OR AN ESCALATION (the kernel declining in
band), never a definite falsehood — declining vs answering
falsely, distinguished. Battery green at FIVE ε values
(default/1e-12/1e-7/1e-6/1e-5); oracle 13/13; hosted
19 pass / 6 skip (closure filter) / 0 fail. Deviations 11
reported / 0 silent; a 5th test target noted. Outstanding
question banked under deviation 9: coincident_surfaces budgets
its interpretation act bandless at ambient ε (review MIN-3) —
candidate for the M7 exit walk. A/B row RECORDED AT MERGE
(opus arm). Kernel-side ledger: props torus sense_sign gap on
#184 (with the M6 orchestrator's proposed unit strengthened);
K-landing disposition ask WATCHLISTED on #89. Lanes cleaned
post-termination (the new sequencing rule held); slot 2 freed.
M7 first-slice status: BOTH plan units 1-2 SHIPPED same-day;
remaining M7 = the M6-blocked unit 3 (NURBS faces), the
KERNEL_* consumer follow-up (blocked on #184's export side),
and the deferrable wild corpus (unit 4).

**Unit 4 GREEN-LIT (Evan 👍 on the #190 queue-state comment,
2026-08-05)**: the wild corpus pulled forward per the
orchestrator's recommendation (kernel support matured: two
dialects import, orientation laundering closed, out-of-subset
refuses typed; NURBS wild files become committed REFUSAL
fixtures, finally exercising the trimmed-B-spline refusal).
Substrate hunt DISPATCHED (license-verified web hunt + empirical
triage of every candidate through the real importer in a scratch
lane; panic/hang = headline; output
~/.local/share/cad-work/wild-corpus/inventory.md). Slot 2 →
wild-corpus-hunt. Spec + A/B block M7-2 draw (difficulty logged
FIRST) follow the inventory. Still pending on Evan: the #89
K-landing disposition (watchlisted).

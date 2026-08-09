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

**Hunt returned + M7-4 spec written (2026-08-05)**: 28 wild
files, 4 license-verified veins (NIST PD, STEPcode BSD-3,
Adafruit MIT, cadquery/build123d Apache; KiCad/OCCT/FreeCAD/
CAx-IF rejected on license). Triage: 0 import as-is, 28/28
refuse TYPED, ZERO panics/hangs — fail-loud held on every
foreign file. Blockers are dialect: (1) unit-context tolerance
17/28 (CONVERSION_BASED_UNIT inch/degree; unreferenced unit
clusters; unit-less 2D parametric contexts), (2) newline-wrapped
strings 8/28, (3) VECTOR magnitude ≠ 1 — scratch-surgery proof:
those three unlock all five Adafruit files with oracle-matching
censuses. docs/M7-4-SPEC.md written (legs = the ranked unlocks;
two S9 flips; rigid-transform assemblies; EDGE_CURVE .F. as an
evidence-gated stretch; 13-fixture committed corpus with
provenance headers + NOTICE; license judgment calls a/b/c ruled
and flagged for Evan). **A/B: M7-4 difficulty logged M BEFORE
the draw** (dialect plumbing + unit vocabulary + transforms; no
new geometry classes). Draw (after the difficulty commit):
byte 114 → **(opus, fable)** — M7-4 = OPUS; fable remainder owed
to the next eligible dispatch (expected: unit 3 NURBS-face
import when M6's assembly lands). Hunt ~88k tokens, ~21min.

**M7-4 DISPATCHED (2026-08-05)**: spec merged via #191 (license
judgment calls flagged; dispatch not blocked on them —
Evan can veto fixtures by comment); implementer (OPUS) on lane
~/.local/share/cad-work/m7-4-wild, branch ev/m7-4-wild, slot 2;
PR to be HELD for blinded review; report at
~/.local/share/cad-work/m7-4-report.md.

**M7-4 implementation COMPLETE (2026-08-05): PR #193 open, 7 of
13 wild files import first-class (volumes vs oracle 1e-16..1e-13
rel), 6 refuse typed by name, no-panic row permanent, 7 numbered
deviations.** Leg E TAKEN with evidence (sg1-c5-214 reaches its
exact oracle census 16/32/20 only through .F. sense
composition). Discoveries: a placement pair CANNOT state a
mirror (ISO's build_axes forces right-handed frames, det=+1
structurally — the mirror refusal moved to
CARTESIAN_TRANSFORMATION_OPERATOR_3D where a file can actually
say it); the ε_in window gained a FLOOR ([1e-10,1e-8] — at
ε=1e-12 a NIST inch translator's 12-digit truncation exceeds
the adoption band and ftc_09 refuses, correctly); a LATENT
M7-2 bug fixed (a solid named by two representations imported
twice — coincident duplicate solids, 316 faces where the file
says 158). STOP-AND-REPORT item, orchestrator ruling: OCC never
splits periodic faces, so lateral bands arrive as two rims with
NO seam generator → adopted ring-on-curved-patch passes tiers
1-2 but topo::mass_properties answers RingOnCurvedFace → tier 3
refuses. The implementer demoted nist_ftc_11 +
cq_red_cube_blue_cylinder to refusal fixtures rather than ship
a tier-3-invalid body. RULED CORRECT — StepImport::Solid
documents tier-validity at rest; shipping an unmeasurable body
pushes the failure downstream (the D4 anti-pattern). The remedy
(seam-generator minting for periodic bands — the M7-2 sphere
re-mint's class, four faces across two chart types in ftc_11)
is BANKED as the next M7 unit; the two fixtures flip back by
S9 then. One hosted red at delivery: the interval-square
tripwire on entities.rs (an x*x) — powi(2) conversion relayed,
in flight. Impl ~372k tokens, ~1.8h wall.

**Tripwire red resolved (2026-08-05)**: both hits were the KNOWN
false-positive class (vector × projection, a·a.dot(x)) — third
occurrence; resolved by the sanctioned named-binding restructure,
NO allowlist entry; matrix full green 19/0. The negative-lookahead
pattern fix stays the M6 side's queued hygiene item.
**Shared-infra fix #194 MERGED**: new-lane.sh refuses path-shaped
lane args loudly — my own briefs had been passing absolute paths,
silently nesting clones (why two review-lane clones went
untrackable); caught by the m7-4 reviewer, no orphans on disk,
briefs corrected to bare names.

**M7-4 review returned (2026-08-05): APPROVE-WITH-FIXES,
0 MAJ / 2 MIN / 3 NOTE, rubric 5/4/4, deviations 7/0 silent —
every headline independently CONFIRMED.** B1: refuse-correct
ruling CONCURRED (gate bypassed behind an env var:
RingOnCurvedFace genuinely fires, no cheaper honest arm). B2:
the ε_in floor is NOT a widened gate (declared uncertainties
down to 1e-23 m come back unraised; the window binds only the
test gate's ambient-ε obligations, loud skips + weaker
obligation outside). B3: duplicate-solid latency CONFIRMED at
merge-base (silently wrong tier-3-GREEN body); identity rule
keeps same-placement twins distinct. B4: five mirror-smuggling
attempts defeated. B5: oracle re-derived digit-for-digit on
upstream originals; KERNEL counts equal the files' own record
counts, independently proving the census divergence is OCC's;
hashes match upstream. B8: 620+ mutations, zero panics/hangs.
MIN-1 (inherited, in-crate): unbounded knot multiplicity →
16 GB alloc → SIGABRT (invisible to catch_unwind) — pulled INTO
the fix pass (the no-panic headline row demands it): parse-time
knot budget with typed refusal, probe_knot un-ignored. MIN-2:
one false test comment. Fix pass dispatched (inherits the arm).
Review ~167k tokens, ~73min wall.

**M7-4 MERGED as #193 (2026-08-05): THE WILD CORPUS IS LIVE —
the kernel imports files nobody here authored.** Fix pass
exceeded its brief: the knot bound is the SCHEMA'S OWN count
(n+d+1, checked arithmetic BEFORE allocation — one rule covers
the off-by-one, hostile multiplicities, and usize overflow);
probe_knot un-ignored with a 5s slowness ceiling AND a
schema-valid control (hardening that rejects valid curves would
be a regression in disguise); MIN-2's comment now tells the
PRESENT-never-traversed truth. Self-caught en route: the adopted
probes were default-ε-only and went red on the hosted 3-ε
matrix — converted to honest weaker claims outside the window
(probe_vol additionally asserts imported ⇒ measurable, exactly
the state the band refusal exists to prevent). Final: 27/0/0
FULL matrix (probe adoption widened the change filter — freecad/
admesh/persistence/corpus/interval rows all ran); 98 tests,
0 ignored, green at three ε values. A/B row RECORDED AT MERGE
(opus arm; fable remainder of block M7-2 owed to the next
M7-eligible dispatch). M7 unit 4 CLOSED; banked: the band-seam
unit (flips ftc_11 + cq_red_cube back), the KERNEL_* consumer
switch, unit 3 behind M6-3's completion — which is RUNNING
(sole-orchestrator pickup, see M6-LOG).

**du_of_rims DIMENSIONAL DEFECT diagnosed (2026-08-05, from
Evan's probe of the #89 landing's area-dimensioned margin)**:
geom-brep/src/props/curved.rs du_of_rims meters EVERY rim-level
comparand by × arm, correct for the sphere/torus payloads
(sin v / cos v, dimensionless) but WRONG for cylinder/cone
(level = v, already a length) — the classify comparand becomes
an AREA. Consequence on cone_trunc: the true rim separation is
~5.6e-4 m ≈ 5590ε — decisively separated — and the ×arm(~1e-3)
factor SHRANK it into the band; at large scale it would inflate
instead. The #89 landing is therefore the detector catching a
dimensional-metering bug, not scale policy evidence — #89
comment to be corrected honestly. Evan approved the fix
direction; the typed-margin design conversation (Length-typed
classify seam with blessed constructor doors) is drafted AFTER
the audit returns (its findings enumerate the doors). Also
green-lit in-chat: KERNEL_* sidecar unit ("do whatever you want
with the .expect files") — sequenced AFTER #192 merges (its
completion adds the 15th fixture to the same directory).
**A/B: du_of_rims fix + classify-comparand dimensional audit —
difficulty M logged BEFORE dispatch; arm = FABLE (block M7-2
remainder, predetermined at the byte-114 draw).** Dispatched on
lane rim-dim-fix (slot 2), branch ev/rim-dimensional-fix; PR
held for review. Design-conversation state from the in-chat
thread: typed margins take Evan's NON-GENERIC shape (concrete
erased newtypes, no dimension algebra) + Evan's follow-up
principle candidate — no kind-dependent payloads — sharpened in
chat to three clauses (Length-typed classify seam; no
dimensionally-heterogeneous uniform payloads, per-kind enum
variants instead; parameter-space→model-space crossings only
through per-kind metric doors). DESIGN.md amendment drafts
after the audit returns; waits for Evan.

**Side chunk (Evan, in-chat, 2026-08-05): PNG timestamp strip**
— OPUS by Evan's assignment, NO adversarial review (his call;
orchestrator sanity-check + merge), A/B-EXEMPT (rows 16/41
class). FreeCAD's saveImage stamps tEXt Creation Time + zTXt
Description (MIBA XML CreationDate) into every render; pixels
bit-identical (Evan-supplied IDAT-decode diagnosis). Unit:
chunk-strip post-pass in the render pipeline + in-place
normalization of the committed corpus (metadata-only, so no
re-render/FreeCAD/cargo — both build slots stay with the live
units); idempotence + IDAT-identity verification scripted.
Lane render-stamp, branch ev/render-stamp-strip.
*(MERGED as #196, 27/27; sanity-checked — surviving chunks are
the deterministic Author/Software/Title; lane cleaned.)*

**CPU PIN LIVE AGAIN (2026-08-05, canary 18.5s → 43.8s — worst
recorded)**: Evan terminal-pushed; both live lanes scoped down
to narrowest-local-signal + hosted-CI-carries-the-matrix; the
600s tool-timeout hazard relayed. One waiter-park caught on the
rim-dim lane (parked on its own topo battery), un-parked with
the standard nudge.

**Rim-dimensional unit COMPLETE (2026-08-05): PR #197 open,
HELD; review DISPATCHED.** Defect confirmed by execution TWICE
pre-coding (byte-exact landing reproduction + a native Probe
twin with the quadratic scale signature); honest correction to
my briefed number (true slant separation √5/2 mm = 1.118e-3 m —
my 5.6e-4 had the arm folded in). Fix = RimLevel enum
(Length(v) | Unit(s,c)) — the per-kind structural shape from
the design thread; mixed-kind comparisons poison typed.
Scale-twin pins (pre-fix ratio 1e6 → post-fix exactly 1000);
the A3 landing pin retired sanctioned. AUDIT: ~120 comparand
rows (docs/predicate-dimension-audit.md); EIGHT dimensionless
comparands fixed inline (bool_join_facing ×2, bool_strut_order,
bool_plane_orient ×2, pm_census_ee_parallel, point_in_loop_arm,
split_join_frame_arm + split_section_area factor-2); headline
DEFERRED finding F5: pcurve_chart_radial_moving's comparand is
an AREA — same class — and may partly underlie the freecad
CORPUS_EPS_CEILING (not raced: pcurve_cache.rs is an M6-3
collision file; ceiling re-derivation = own unit, BANKED);
F2-F11 deferred with dispositions; six notes feed the
typed-margin design. K delta: exactly one line (the landing
retires) + one masked false-coincidence retired; nothing else
moved. Review attacks: the 8 changed decision comparands
(riskiest surface), grouping-verdict invariance, scale-twin
re-derivation, audit-coverage independent sweep, F5 basis.
Impl ~290k tokens, ~3h wall under the pin.

**Rim-dim review returned (2026-08-05): APPROVE-WITH-FIXES,
1 MAJ / 2 MIN / 5 NOTE, rubric 5/4/4, 7/7 deviations honest.**
The fix/audit/retirement all HELD — and R1 found the fix
corrects real VERDICT FLIPS, not just margins (pre-fix silently
grouped 50ε-separated rims on a small body and spuriously
refused 0.5ε-coincident rims on a large one; post-fix honest
both directions). MAJ = the unit's own boolean-twin pin red on
3 hosted jobs: not ε-row-honest (at 1e-6 the mm twin hits
deferred F4's in-band area margin — .expect'd success; at 1e-12
the witness ratio assert fires on scale-invariant margins).
**F4 PRIORITY UPGRADED by execution**: bool_ring_run_winding's
area margin lands in Band{1e-6,1e-5} on REAL mm booleans at a
CI ε row — the banked dimensional unit (F4+F5 + ceiling
re-derivation) is sequenced IMMEDIATELY after the M6-3 merge.
R6 also executed the F5 linkage: the freecad cylinder refusal
at ε=1e-7 IS pcurve_chart_radial_moving, margin 5e-7 = 2r²
in-band. split_section_area's spec factor confirmed (2A/P; old
code 4A/P). Fix pass dispatched (inherits fable): per-ε-row pin
expectations with the three-outcome structure; scale-invariant
margins asserted AS invariant (stronger than exemption);
reviewer probes adopted; sphere pole-degeneracy audit note.
Review ~154k tokens, ~51min under the pin.

**Post-#192 unblock (2026-08-05): two dispatches staged.**
A/B block M7-3: difficulty logged BEFORE the draw for both —
**F3+F4 dimensional unit = M** (retire ops.rs's raw sign_within
bypass so margins route through the named funnel — its stale
attribution corrupts K telemetry, the second executed reason;
fix bool_ring_run_winding's area comparand — in-band on the
hosted 1e-6 row for real mm booleans; flip the F4 live-signature
pin; ceiling-table remnant check) and **KERNEL_* sidecar unit =
S** (15 sidecars gain KERNEL_FACES/EDGES/VERTICES/SOLIDS +
full-precision KERNEL_VOLUME_MM3 with native-census semantics
and the kiss_assembly divergence documented; export-suite
staleness row; check_step ignores; import row-1 consumer swap +
tolerance tightening — Evan's blanket .expect green light).
Draw recorded below after this commit.
Draw: byte 123 → **(opus, fable)** — F3+F4 = OPUS, KERNEL_* =
FABLE. Both dispatched (lanes f34-dim slot 1, kernel-sidecars
slot 2); PRs held for blinded review per standing process.

**PLATFORM OUTAGE (2026-08-05): the subagent-write safety
classifier went unavailable mid-dispatch.** The F3+F4 implementer
had every mutating tool blocked (~25 attempts); it stopped and
reported per the rule with the BOTH FIXES FULLY DESIGNED and
arithmetically pre-checked (F3: volume gates route through the
named funnel with honest lengths — ΔV/(A_got+A_bound) for the
backstop check, V/A for the operand gate on the validate.rs
positive_volume precedent, names volume_backstop /
volume_backstop_operand; F4: 2A/P mean-width margin at all three
winding sites, conic perimeter via the fail-loud upper bound;
pin flips drafted incl. retiring witness_at_mid_parameter's
KNOWN_NONLINEAR entries as pure F3 stale-name contamination;
collision rows verified retired — all five remote ev/ branches
are merged). Zero writes made. Kernel-sidecars lane created with
some writes landed before/around the outage; not yet reported.
Recovery plan: re-nudge the F3+F4 lane on the hourly cadence;
the design note hands off cleanly to a fresh implementer if the
transcript goes stale. Orchestrator-session tools unaffected.

**Block M7-3 COMPLETE + the docs estate de-rotted (2026-08-05).**
F3+F4 MERGED as #200 (opus): the audited family's last funnel
bypass retired via the dual-arm gate — sign-certain inequality
violations refuse dimension-free through the exact bit-hairline
band, BOTH arms on the metered comparand (a certainly-positive
lever cannot move a sign, so K attribution stays honest —
verified scale-linear and RED-with-arm-removed); the review's
hide-behind-area MAJOR (3mm cube on a 2m plate metered in-band
and PASSED) is pinned; editor-core expr.rs:656 recorded as
audit row F12 (deferred, attribution-hole class).
KERNEL_* MERGED as #199 (fable): 15 sidecars carry native-census
fields + full-precision volume + the ε-discovery
KERNEL_VOLUME_PAD_MM3 (enclosure midpoints move with ambient ε —
byte pin at declared ε, overlap rows elsewhere; the review's
planted-lie attack proved composed hiding room ZERO); the import
override table is GONE; tolerance claim corrected to measured
truth (3 fixtures up to 8.3e7× tighter, 11 honestly ~2-3× looser
at the 1e-6 mm³ scale, teeth proven at 1600×/8000×).
DOCS-ROT MERGED as #203 + tag archive/2026-08-05: all 13
contradictions, 18 stale items (K-REPORT landing-retraction
addendum), 9 over-spec trims; 49 files archived with INDEX;
memories consolidated (one lost standing rule caught and
restored). ORCHESTRATOR REJECTION during review: the unit had
CLOSED M6 and re-banked its ratified remainder — reverted to
honest OPEN (units 5 + ratified sense gate + hygiene remain;
closure is Evan's exit-walk call), recorded as ESCALATIONS
item 0. Banked follow-ups: ~30 code-comment citations of
pre-archive docs/M* paths (small code sweep); the √5/2-vs-√5/4
numeric cross-check in this log's rim-dim entry. NEXT: M7
unit 3 substrate (NURBS-face import); typed-margin design
draft (audit + F3+F4 outcomes now in hand).

**M7-3 substrate returned + spec written (2026-08-05).**
Measured: loft_prism's 4 non-rational B_SPLINE walls re-export
byte-identical below the header; NO pcurves cross the wire
(IsoCurve reconstructed at import); every certification door the
non-rational case needs is measured OPEN (traversal map with
file:line in the inventory). HEADLINE ADJACENT FINDING → issue
#207: sweep_body has ZERO successful callers (skin-fit weight
drift 1.0+2e-16 → bitwise-rational walls → speed_lower_bound
poison); only uniformly-spaced lofts export today; kernel fix
banked. Spec decisions: the surface_sig trap (all-NURBS-share-
one-key) fixed FIRST with a pin; the IsoCurve adoption rung;
rim-edge exemption per the Seam idiom; RATIONAL FORK proposed
firm as ARM B (import-with-typed-limitation — the imported body
lands in exactly the native state incl. the identical t3
refusal; refuse-at-import would reproduce the writer/reader
asymmetry the writer already exhibits) — flagged for Evan in
the unit PR. dm1-id-214 stays refused (stage-1 territory);
nist_ftc_10 premise was stale. **A/B block M7-4: M7-3
difficulty logged M BEFORE the draw** (per the substrate's
signal: doors open, work = two design decisions + the sig trap
+ flip bookkeeping). Draw after this commit.
Draw: byte 224 → **(fable, opus)** — M7-3 = FABLE; opus
remainder to the next eligible dispatch (candidates: the #207
skin-fit fix, M6 unit 5, or the margin-convention migration if
#205 ratifies). Spec + substrate ride the next state-sync;
dispatch now (lane m7-3-import, slot 2; the substrate lane on
slot 1 is done and cleans at this seam).

**#207 closed: the skin fit stops synthesizing a weight channel
(2026-08-05).** Root cause confirmed at the source: `skin_on`
built homogeneous `(w·x, w·y, w·z, w)` rows unconditionally, so
an INTEGRAL input (all weights bit-exactly 1.0) had its constant
weight column solved by LU and then divided back out — a
normalization round-trip that lands off 1.0 by an ulp for most
parameterizations (measured 1.0000000000000002 /
0.9999999999999998). Fix: integral input interpolates in
Cartesian ℝ³ and emits exactly 1.0; the rational lane is
untouched. NOT a snap — the weight channel is never manufactured,
and `integral` is decided by `==` on the input's own bits (C6
structure selection). Bitwise conservative for the uniform case
(`solve_square` factors once and substitutes each RHS column
independently; `p.x * 1.0` is `p.x`; the removed divide was by
exactly 1.0 wherever the old weights were exact) — pinned twice,
in-suite against the old lane and externally by the unchanged
`loft_prism` golden fixture. New pins: a quarter-torus elbow
`sweep_body` (the tree's FIRST successful curved-path caller) —
tier 1/2/3, Pappus bracket 4e-6 rel at 9 stations with the
certified pad four orders tighter, exported and reconstructed by
both Part 21 oracles as NON-rational; and two non-uniformly
spaced `loft_body`s (z = 0, 1, 3), one with a derived V = 12 m³.
Honest corrections landed at the three M6-3-era "sweep_body is
live" claims (DESIGN.md (c), `eval::wire::SWEEP_FRONTIER`, the
tour's lily narration): the M6-3 closure in fact covered only
straight-path sweeps and uniformly spaced lofts.

**Margin convention RATIFIED (Evan 👍 on #205; flip PR #208).**
The classify-seam migration is queued as the next block's unit.
**#207 fix dispatched in parallel** (slot 1; geom-curves/sweep
footprint, zero overlap with M7-3's step-import): **difficulty
logged S BEFORE dispatch** (localized fit-contract fix + two
door pins + the honest sweep_body doc corrections); arm = OPUS
(block M7-4 remainder, predetermined at the byte-224 draw).

**M7-3 MERGED as #209 (2026-08-05): M7 UNIT 3 IS CLOSED — the
kernel imports its NURBS faces.** Both surface arms parse; the
surface_sig trap fixed (injective under every constructed
collision); the IsoCurve rung reproduces the native description
verbatim; rim adoption via synthesized PlacedSegment; ARM B
landed, blessed by Evan, HOLED by the review (a different circle
through the same endpoints laundered on rational walls — the
uncertified-trust class), and REPAIRED to verified-not-trusted:
the rim residual gate samples the wall's own boundary against
the closed-form circle distance + lever-armed angular
containment (the reported role inversion — point-to-rational-
patch has no closed form; the angular clause kills the
complement arc). Evan updated on-thread; blessing carried.
#210 (skinfit) merged the same seam: sweep_body's FIRST
successful caller; curved sweeps + non-uniform lofts now export
— a widened exportable class the NEXT M7 seam folds into the
round-trip corpus (banked: elbow-class fixtures join
SOLID_FIXTURES; the ratified margin-convention migration opens
the next block). Watchlist empty; both slots free; lanes clean.
M7 remaining: the corpus-widening fold, the band-seam unit
(ftc_11/cq_red_cube flip back), stage-1 recognition (dm1-class),
exit walk. M6 remaining: unit 5, the ratified sense gate,
k-lint floor.

**Corpus-widening fold LANDED (PR TBD, 2026-08-05): the #210 class
is in the round-trip corpus.** Two committed fixtures, byte-golden
against the writer with hand-authored `.expect` sidecars carrying
both the FreeCAD/OCC reading and the full `KERNEL_*` block:
`nonuniform_loft` (`loft_prism`'s sections at z = 0, 1, **3** — the
minimal pair, isolating the section spacing that used to poison)
and `swept_elbow` (the quarter-torus sweep, `sweep_body`'s first
caller now its first FIXTURE). Corpus 15 → 17 everywhere:
`fixture_corpus`, the staleness row (17/17), the exactness /
NURBS-containment / K4-literal tables, `SOLID_FIXTURES`,
check_step.sh (18 files green locally, FreeCAD 1.1.2).
Both derivations are CLOSED FORM where one exists: the non-uniform
loft's V = 12.75 + 126.75/√19345 = 13.661304680798798 m³ falls out
of the chord-length middle parameter t = √73/(√73+√265), and the
kernel's certified midpoint sits 4 ulps away. The elbow has NO
closed form (the walls interpolate nine stations of circular
motion), so its EXPECT volume is the kernel's own certified value
and the oracle row states the claim an oracle can make — two
independent systems measure the SAME solid — at the corpus's first
non-default `EXPECT_VOLUME_RTOL` (1e-7, from a measured 1.94e-8
quadrature disagreement on the degree-3 walls). **The elbow's NURBS
walls went through #209's import machinery on a sweep body for the
first time with ZERO refusals**: full tier 3, fixed point, and
committed-byte divergence of exactly 3 tokens on each new fixture,
all in the documented `-0.0 → 0.0` class — no new class, pinned by
`step-import/tests/corpus_fold.rs`. No new in-band K landing in
local runs (the new bodies do not enter the k-probe corpus; hosted
k-lint stays the detector).

**Block M7-5 staged (2026-08-05): difficulties logged BEFORE the
draw.** Unit A — **classify-seam migration = M** (the ratified
margin convention's clause-(i) rollout: Length<T> erased newtype
at the classify/Band seam, blessed constructor doors, ~120 sites
per the audit ledger; mechanical but broad, every touched
comparand re-argued through a door). Unit B — **corpus-widening
fold = S** (the #210 exportable class joins the round-trip
corpus: elbow-class sweep + non-uniform loft fixtures with
KERNEL_*-bearing sidecars, byte-golden, check_step oracle rows,
SOLID_FIXTURES extension; measured feasibility from #209/#210's
own suites). Draw after this commit.
Draw: byte 220 → **(fable, opus)** — migration = FABLE, fold =
OPUS. Both dispatched: lanes margin-migrate (slot 1) +
corpus-fold (slot 2), disjoint footprints (geom-brep/topo
classify sites vs sweep tests + fixture corpus). PRs held for
blinded review per standing process.

**Block M7-6 staged (2026-08-06): montage refresh (Evan's #212
ask) — difficulty logged S BEFORE the draw** (scene
constructions for swept_elbow / nonuniform_loft / loft_prism /
tube_along_arc in the demos corpus, cells on both montage
lanes, renders through the stripped pipeline, cell-count pin;
the render-stamp unit makes the diff clean). Draw after this
commit; the block's remainder owed to the next eligible unit
(candidates: band-seam, stage-1 recognition, M6 unit 5).
Draw: byte 111 → **(opus, fable)** — montage = OPUS; fable
remainder to the next eligible dispatch. Dispatched on lane
montage-refresh (slot 2).

**Montage refresh PARTIAL delivered (2026-08-06, PR #215
merging on green): cell 19 (tube_along_arc, both sheets,
bit-exact minor_radius assertion executed in the stop; windowed
tube showing all three intent parameters; clean-re-render
verified twice). The three NURBS-walled scenes are BLOCKED on
the mesh crate's trimmed-NURBS tessellation lane** (banked at
M6-3 dev 8; tessellate.rs:93 refuses Surface::Nurbs ahead of
trim routing — executed: the tour panics after tiers 1-3 pass,
only mesh refuses). The agent correctly stopped at the design
boundary (placeholder cells would break the two-sheet
cell-for-cell contract); all three scene constructions are
WRITTEN and saved as
~/.local/share/cad-work/montage-skin-scenes.patch. **The lane
now has TWO consumers (the banked tour SceneBody stop + three
montage cells) — PROMOTED per the banked-until-consumer
principle: next unit = mesh trimmed-NURBS tessellation (FABLE,
block M7-6 remainder), difficulty logged M pre-dispatch**
(route the Nurbs arm to the trimmed lane; the certified-
conservative δ promise per D4's tessellation contract; flip
M6-3 dev 8's banked stop; apply the montage patch as the
completion rider).

**Classify-seam migration MERGED as #213 (2026-08-06): MARGINS
ARE LENGTHS BY SIGNATURE, WORKSPACE-WIDE.** The ratified
convention's clause (i) is structural: ~351 sites through
blessed doors, no raw construction path, byte-identical
23394-line census independently reproduced — and the migration
itself EARNED ITS KEEP by forcing F13/F14 into the open at
compile time, with the review catching the one laundered sine
(F15) via an executed scale-blindness probe. Three Evan design
rounds absorbed mid-unit and each made the convention sharper:
the door enumeration; consistency-not-accuracy as the
backstops' stated semantics; then the layering fork — the
volume backstops now live on a permanent INVARIANT LANE outside
the seam (bare margins, Corrupt-voiced ResultVolumeImplausible
with the bug-report affordance, per_boundary deleted), census
re-proven byte-identical after the restructure. Debt tracked as
#214 with a test-enforced count. Block M7-5 complete both arms.
Live: the mesh trimmed-NURBS lane (fable, block M7-6 remainder)
carrying the montage completion rider. Remaining ledger:
band-seam unit, stage-1 recognition, M6 unit 5 + sense gate,
k-lint floor, exit walks.

**Trimmed-NURBS tessellation lane LANDED (PR #218, ev/mesh-nurbs-lane, HELD for adversarial review):
the M6-3 frontier's second half — described NURBS faces RENDER.** The
banked lane `trimmed.rs:28` named is promoted with two consumers. The
per-triangle certificate is the torus derivation with a HULL-DERIVED
Hessian (`mesh::nurbs_cert`): second-derivative control nets by knot
differencing (NURBS Book 3.24 per direction, `derivative_coeffs`
iterated), sup by tensor-product convexity, anisotropic bound
(muu·a_u² + 2·muv·a_u·a_v + mvv·a_v²)/4 — never an estimate; grid
sizing budgets δ_s/2 per axis group and the chord pass grows the
adjacent-NURBS boundary tightening (the torus pattern, with per-axis
closed-form pcurve speed bounds — exact |pl| for IsoLine, amplitude
sums for Harmonic). Covered: described, non-rational, C¹ (degree ≥ 2
with interior mult ≤ p−1, or degree-1 single-span) — exactly the
1×2/1×3 loft/sweep wall class. Refused typed, naming the class
(`UnsupportedNurbsFace`): rational (a rational second derivative is
not a hull convexity fact — hull's deliberate absence), C⁰ creases,
degree-0; `Fitted` pcurves refuse on every chart (no certified UV
chord-step bound; consumer = the edge×NURBS-face boolean layer). The
tessellate.rs:93 first-arm refusal flips per S9 with its history
carried on `UnsupportedSurface` (placeholder-only now). Pins: the
δ+ε promise EXERCISED (coarse/fine δ pair on loft_prism, measured
max surface→mesh deviation dominated by δ+ε both times, pair
genuinely ordered); determinism bitwise on the elbow; hull-dominates-
sampled-Hessian; typed-refusal units. Consumers: (a) the skinned
SceneBody stop flips — the montage-refresh patch applied verbatim
(loft_prism / nonuniform_loft / swept_elbow, corpus fixtures cited
constant-for-constant); (b) montage 19 → 22 cells (31 scenes − 9
non-montage), README counts refreshed (STEP 41, curved 24; all three
new bodies 6 `.T.`/0 `.F.` — NURBS walls are authored outward by
assembly, nothing to reverse), renders on BOTH sheets with no
placeholder, admesh clean (1 part, 0 defects × 3), clean-re-render
verified twice through the stripped pipeline. Suites: mesh 73 + sweep
338 + stl + topo + editor-core all green.

**#218 review revisions (2026-08-06, same lane): the three cells
re-posed for silhouette legibility.** Evan's two visual findings, both
structural and both fixed at the geometry/pose level rather than
shading: (1) the loft minimal pair now shares a near-face-on xz
PROFILE camera (elev 10 / azim −80) — the flare is in x, so the
mid-height vs one-third bulge is the outline itself; (2) the sweep
cell's quarter-arc elbow was revolve-expressible (a square on ONE
planar arc is a partial revolve's orbit), so the cell becomes
`s_duct`: two OPPOSED R = 2 quarter arcs, degree-3 interpolant
through 17 exact points, 13 stations — curvature changes sign, which
no single-axis revolve can produce, and the S is posed edge-on. The
quarter-arc elbow REMAINS the corpus/suite constant (common/mod.rs,
m7_skin_integral, m7_nurbs_trimmed); the scene LEADS the corpus (lily
precedent) and the S sweep is the fixture candidate for the next
corpus fold. The tube cell keeps its torus-class shape with the
README stating that as its point (the door's exactness), the
not-a-revolve geometry living next door. Operational note: this
host's render passes contended with a concurrent review-lane FreeCAD
session (GL-context failures, one matplotlib-fallback near-miss
caught before commit); re-render serialized behind it.

**M6 unit 5 forks RULED (Evan on #217, 2026-08-06)**: F-a
Vec-only selections + enumerate-all helper (freeze semantics
made explicit and accepted — a selection is a commitment;
Rebind is the growth path); F-b v3 clean break (ratified
precedent); F-c N5-verbatim + Rebind third site; F-d TWO
sequenced PRs (clean split by consumer: the die's fillet is
terminal → PR-1 surgery-emitter + vocabulary + node + v3 +
eval + die registration at M, PR-2 whole-body totality at S);
F-e measure-first confirmed. Substrate correction on record:
the "banked N4 emitter" was zero code; the die document
pre-exists in hold-out shape. **A/B block M7-7: unit-5 PR-1
difficulty logged M BEFORE the draw.** Also this hour: Evan's
accidental cancellation disarmed the monitors (re-armed, all
three) and killed the mesh lane mid-render (resumed;
delivered as #218, review in flight).
Draw: byte 19 → **(opus, fable)** — unit-5 PR-1 = OPUS; fable
remainder to the next eligible dispatch. docs/M6-5-SPEC.md
written from the substrate + the #217 rulings; dispatched on
lane m6-5-fillet (slot 1); PR-1 held for review, PR-2 follows
its merge.

**Montage visual chunk scoped (Evan's #218 follow-up,
2026-08-06): difficulty logged S BEFORE dispatch; arm = FABLE
(block M7-7 remainder — unit-5's PR-2 inherited PR-1's opus arm
as one unit).** Scope: cut tube_along_arc + the two partial-die
cells from the montage (kept in the demo); replace/augment the
s_duct with geometry REVOLVES CANNOT DO (torsion via a
non-planar path, or continuously varying curvature — measure
the post-#210 path vocabulary first and demonstrate the
strongest reachable class); make the loft pair's nonuniformity
dramatic. Lane montage-v2 (slot 2).

**Montage-v2 delivered (2026-08-06, PR #221 HELD for Evan's
eyeball; watchlisted).** Item 2's measurement CONCEDED Evan's
read (the s_duct IS two glued partial revolves — demoted,
caption honesty-fixed) and produced the real answer: planar
varying-curvature and sub-half-turn non-planar paths all build
end-to-end; the new sweep cell is twisted_duct (square along a
twisted cubic, τ nowhere zero — beyond any revolve gluing);
profile twist verified unsupported (C11 scope). BANKED FRONTIER:
helix arcs ≥0.5 turn refuse typed (nurbs_span_meter ParamSpan —
the corner-path chord meter collapses under near-antipode frame
roll) — the long-turn sweep lane. Item 3 measured honestly (the
#218 pair was visually the prism rescaled; the re-spaced pair
overshoots silhouette-obviously, V = 9.7219 derived, quadrature
1e-13). One repair: the committed montage carried a stray
fallback frame from this session's concurrent-renderer accident
— recomposed from verified cells. Actions outage persists (zero
runs created; the #220 rerun queued ~4h) — #221 merges on
Evan's 👍 under the standing waiver, hosted evidence late via
task-#16's sweep.

**M6-6 substrate returned + spec written (2026-08-06).** The
executed truth table is STARKER than the ratifying thread knew:
NO gate catches any single-face curved flip (all four kinds
bit-identical or Zero-exempt), and fully inside-out
washer/cone/donut/lily certify GREEN positive — plus a NEW
executed import gap (cylinder/cone_apex flips import green; the
torus normalization check has no siblings — now the spec's
rider). Extension surface clean: one factoring
(boundary_material_sign) + a curved check-6 arm, combinatorial
(no comparand — margin convention satisfied by reuse of the
length-metered named decides). Coexist-not-subsume ruled for
the import torus refusal. No forks. **A/B block M7-8: M6-6
difficulty logged LOW-M BEFORE the draw.** Draw after this
commit.
Draw: byte 194 → **(fable, opus)** — M6-6 = FABLE; opus
remainder to the next eligible dispatch. Dispatched on lane
m6-6-gate (slot 1, replacing the substrate claim; the substrate
clone cleans per the placement rule — outputs stay).

**Fallback unit ratified (Evan 👍 on #221's affordance):
difficulty logged S BEFORE dispatch; arm = OPUS (block M7-8
remainder).** Scope: the matplotlib fallback writes to a
gitignored renders-preview/ (never the committed paths) + a
guard asserting committed PNGs carry the FreeCAD signature
chunks — a fallback frame becomes structurally uncommittable.
Lane render-guard (slot 2).

**C7 placement RULED (Evan 👍, #223 thread, 2026-08-07): option
(b) — M6 closes at its ratified boundary; the C7 join-lane
implementation OPENS M8 (the contact design was co-designed
with M8's signed clearance — the co-design stays load-bearing);
the lily rebuild rides there. The roadmap through both walks is
now fully ruled: M6 = unit 6 (in review) + k-lint floor → exit
walk (closure = Evan's call at the walk, with the 15-row lily
disposition table as walk evidence); M7 = band-seam + stage-1
recognition → exit walk; then M8 opens with C7.**

## TIE-UP (2026-08-07, Evan's ask): the resting state

Merged since the last state-sync: #218 (mesh trimmed-NURBS lane
— NURBS faces render, the empirical per-triangle falsifier
guards CI), #219+#220 (M6-5 whole — the composed die registered,
fillet naming total through both doors), #221 (montage-v2 — the
twisted_duct, curated cells, the conceded s_duct), #224
(render-guard — fallback frames structurally uncommittable; its
run confirmed ACTIONS RECOVERED). Filed: #222 (long-turn sweep
frontier). RULED: C7 opens M8, M6 closes at its ratified
boundary (Evan 👍). CLEARED: the Actions outage debt — main
went full-matrix GREEN on the tip containing every waiver-era
merge. A/B rows all recorded at merge (MESH fable, M6-5 opus,
MV2 fable, GUARD opus).

**The one open thread**: M6-6 (#223) is in its fix pass (lint
header + the conic-trim residual recording; the gate itself
survived every review attack incl. a byte-identical 51-row
census at three ε). On its green return it merges with its A/B
row — no re-dispatch needed, the pass is in flight.

**The runway after M6-6 merges (nothing dispatched yet, per the
tie-up)**: the k-lint baseline-floor refresh (last M6 hygiene) →
the M6 EXIT WALK (closure = Evan's call; the 15-row lily
disposition table is walk evidence); M7's band-seam re-mint +
stage-1 recognition → the M7 EXIT WALK; then M8 opens with the
C7 join lane + the lily rebuild. Small banked chunks: the
kernel-lane per-scene render timeout (the FreeCAD deadlock),
the #214 dimensional-debt riders, #222.

Slots: 1 = m6-6-review lane (cleans at #223's merge), 2 = free.
Watchlist: empty. Monitors: armed (re-armed post-accident).
Disk healthy. All logs, A/B rows, and memories committed.
**Tie-up completion: M6-6 MERGED as #223** (the open thread
closed itself — fix pass green on the fresh hosted run). M6's
RATIFIED CONTENT IS DONE. The resting state stands as written
above, with the runway's first step now the k-lint floor.
**Post-tie-up bookkeeping (Evan's check)**: A/B table verified
complete on main (all unit rows through M6-6; F3+F4 present;
two conflict-recovery duplicate rows deduped). The M6-6
residual family now has its tracking issue with per-residual
flip conditions and sequencing — walk material with named
owners, none blocking.

**Successor briefing written (2026-08-07):
~/.local/share/cad-work/handoff-prompt-next.md** — the resting
state, the ruled runway (k-lint floor → M6 walk → band-seam +
stage-1 → M7 walk → M8 opens with C7), the standing process
with every norm learned this session (CONFLICTING-silent-CI,
bare lane names, substrate placement, terminated-before-clean,
checkpoint cadence, arm-names-out-of-review-scope), A/B state
(blocks through M7-8 consumed; M7-9 next), and the first moves.
This session hands off clean: nothing running, all seams on
main.

## Successor session (2026-08-07): KLINT merged — the runway's first step is done

New orchestrator (same machine session; monitors inherited armed,
both slots free — the stale .holder PIDs were dead, #235's shape).
Handoff reading done in full; waited out Evan's requested pause;
noted the CONCURRENT LIB orchestrator (LIBRARY-DESIGN.md, own A/B
block series per §L8 — #232/#233/#236-era merges; no draw
collision).

**KLINT MERGED as #239 (2026-08-07): the k-lint baseline floor is
CURRENT and the advisory channel is clean.** Block M7-9 opened per
protocol (difficulty M logged pre-draw, byte 47 = opus,fable).
Floor 1.5e-3 → 4.0e-5 (P0 of the ε-independent ambient definite
population, 1.35M samples/row; the binding family is
`volume_backstop` on die_pips/die_composed — NOT the two families
M5-2 predicted, which rose out of the way). `props_quad_converged`
left both metre rules for rule 4 (|m| < 150·ε, calibrated on its
own |m|/ε population). **M7-F1 ruled (orchestrator + review
concurring): rule 2's definite arm capped at the floor** — uncapped
it prints 54 permanent known-feature flags at 1e-6 (the calibration
population itself; the M4-era 102-flag dead channel is the
existence proof against that posture). The honest residual is
MEASURED and pinned by adopted probes: at ε=1e-6 a definite margin
in [4e-5, 1e-3) is watched by no rule (empty window at tight rows);
that is ε-policy walk material, not a lint defect. Review
APPROVE-WITH-FIXES 0/1/4 (rubric 5/4/4), 5/5 mutations killed,
committed baseline byte-reproduced by a cold sweep; fix pass light,
14 tool tests. A/B row recorded AT MERGE. Advisory posture kept;
gating readiness noted as walk material. Snapshot contract written:
the baseline is a provenanced snapshot at its measured head, re-cut
when the distribution moves (main's tour grew path_junction_turn
mid-flight, +293 samples/row, lints 0/0/0 — no re-cut needed).

**M6's ratified content AND its last hygiene item are now both
DONE. Next: the M6 EXIT WALK** — drafted at docs/M6-EXIT-WALK.md
(every M6-PLAN criterion verbatim, M5-walk format); finalizing the
k-lint cell + hosted-green citation, then presenting to Evan via a
docs-only PR with the explicit closure affordance. After his
ruling: the two M7 units (band-seam re-mint takes the block M7-9
fable remainder), the M7 walk, and M8's opening (C7).

## Seam entry (2026-08-08): M6 CLOSED; k-lint GATED; M7-5 MERGED — one unit left before the walk

**M6 CLOSED** (Evan's lgtm on #243's affordance, comment 5224869607;
the ratified walk is docs/M6-EXIT-WALK.md; carried-items register =
#250). Riding the same ruling: **the k-lint row is a GATE** (#253 —
exit-2 findings voice carrying the interpretation discipline, exit-1
harness voice, baseline gates green; #250 row closed).

**M7-5 (band-seam re-mint) MERGED as #252** — both wild refusal
fixtures import first-class; census/oracles exact; the project's
second NOT-MERGEABLE-AS-IS (A/B row has the prose: the unwired
import-path backstop, the ~18° winding window minting silent
complements, the unpinned refusal — all closed, re-review APPROVE,
window proven GONE by dense sweep). Beyond-scope fact recorded in
#252's body: ordinary (non-band) imported solids get NO tier
validation at import — worth its own conversation before the M7
walk treats "imports are tier-valid at rest" as a blanket claim.

**A/B state**: v3 triples adopted (M7-10 open: byte 205 →
opus,fable,opus; slot 0 = the gate flip, MERGED, excluded class);
M7-9 completed as a v2 pair (KLINT opus + M7-5 fable, both rows
recorded at merge under the v3 discipline). Dual-review count:
U3 = 1, M7-5 = 2 (single review, verified G1 unmerged at merge);
the NEXT blinded-lane merge (likely G1 or stage-1) is row 3 → R2.

**Runway**: stage-1 NURBS recognition (M7-10 fable slot; substrate
→ spec → dispatch) → the M7 EXIT WALK (present to Evan) → M8 opens
with C7 per the standing ruling. Banked/watch: #235 (stale slot
holders — cosmetic), the FreeCAD per-scene timeout, #222, #214
riders, Q9 (fuse burning: the lib program approaches U9/U10).

## M7-8 — plane × NURBS intersection certification (declare-and-check)

The last M7 code unit, per Evan's #264 ruling ("definitely certify
plane×NURBS intersections … a 'declare and check that it actually
works out' case"). Spec: `docs/M7-8-SPEC.md`.

**The pin FLIPPED.** `cylinder_envelope_refuses_and_the_seam_orphan_
is_pinned` is retired and replaced by `the_seam_orphan_certifies_as_
a_plane_nurbs_intersection`: the arc-prism mixed promoted/stays-NURBS
body imports first-class, and its wall–wall seam (EDGE_CURVE #130 —
the edge that had no bitwise IsoCurve rung and no certification path)
now carries a certified `EdgeGeometry::Intersection` between the
promoted `y = 1` cap plane and the stays-NURBS arc wall. Re-derived
at ε_in = 1e-9: on-locus residual 2.48e-16 m, certified
between-samples sup 6.32e-12 m, uniqueness tube radius 0.25 m with
transversality 1.22 m over 32 boxes, min sin θ = 0.7071 (the 45°
the quarter-cylinder meets the cap at).

**The carrier is EVIDENCE, never truth.** The seam spline's middle
control point pushed 1e-3 m off the wall (staying exactly on the cap
plane, so the NURBS-side residual has to do the catching) refuses
through the importer with the measured bound in the payload —
`CertifyError::PlaneNurbs(Limb { .. })`, surfaced verbatim in the
adoption refusal's text.

**Shape.** The lane is INJECTED at the door rather than bound into
`topo`: `EdgeCurve::certify_nurbs_lane` takes a `NurbsLane` function
whose derivation needs `Decide + Bounds`, and `Body::set_edge_curve_
nurbs_lane` is a second attach door onto verbatim-shared
preconditions, adjacency rules and mutation. No door accepts the
class uncertified; the plain `T: Decide` door still refuses a
described NURBS operand with `Unimplemented` (pinned).

**Measured and REJECTED as scope expansion** (reported, not shipped):
widening `transform_rigid` and the six boolean entry points to carry
the lane. It type-checks, but it cascades into five `editor-core`
signatures and two `topo` tests for a capability no acceptance row
needs — transform/boolean re-certification of an imported plane ×
NURBS body is a real question, and it is BANKED for its own unit
rather than smuggled in here.

**Honest limits, reported.**

- *Tier-3 at rest is native-twin parity, not `Ok(())`.* The arc
  prism's wall is RATIONAL, so both the native body and the imported
  one refuse tier 3 with the same banked rational-quadrature refusal
  (M7-3's Arm B). The unit proves what it can: the at-rest pass finds
  NO certification failure — every edge, the new seam included,
  re-derives its certificate through `recertify_nurbs_lane`.
- *The near-miss row changed meaning.* Nudging the wall's mid-arc
  control point 5·ε off the cylinder no longer refuses, because the
  seam sits at the patch's `u = 1` edge where that point's basis
  weight vanishes. The row now pins that SEPARATION explicitly; the
  perturbation that must be caught is the carrier-side falsifier.
- *The flip is ε-DEPENDENT, and both postures are pinned.* The seam's
  certified between-samples sup is 6.32e-12 m — the promoted plane
  wall's boundary column and the arc wall's own column agree only to
  the arc endpoint's rounding, and the first-order envelope cannot
  say better. So the pin flips at ε_in = 1e-9 (default) and 1e-6,
  and at the 1e-12 matrix row the same geometry refuses TYPED,
  carrying that number: `Escalated { check: PlaneNurbsCertificate,
  cause: Indeterminate { margin: 6.31561637745462e-12, band: { zero:
  1e-12, escalate: 1e-11 }, predicate: "ssi_hull_sup_chart" } }`.
  This is the spec's clause 3 landing exactly as written — the bound
  too loose at ε refuses WITH its measurement, and nothing was
  widened to make the row green. Tightening it needs the banked
  algebraic spline-product hull certificate (the same bank that owes
  the cylinder track), not a gate change.
- *A COLD-LINT catch on inherited work* (×2): SU2's probe file shipped
  without its `#![allow(clippy::unwrap_used, …)]` header, and SU3's
  new `impl` block sat after `mod tests` in `topo/src/euler.rs`
  (`clippy::items_after_test_module`). Both would have failed the
  clippy gate; both were caught by the cold pass, not by a warm one.

**Tangency refuses in `certify`'s OWN vocabulary.** A planted
near-parallel plane raises `CertifyError::NotTransverse` — the same
variant the analytic arm raises, so a caller reads one refusal rather
than two dialects. No `TangentIntersection` adoption rung was invented
(the spec's STOP).

**Batteries, re-executed on the union with `origin/main`** (the
finisher's pass; every row a foreground run under a build slot):
step-import 147 passed / 0 failed at each of ε_in default, 1e-6 and
1e-12 — the ε-dependent seam posture is inside the pinned test, so the
COUNT is invariant across the matrix; geom-brep 220 passed / 0 failed.
Workspace `cargo check --all-targets` clean after the merge (the #274
lesson: the union is built explicitly, not assumed from two green
branches).

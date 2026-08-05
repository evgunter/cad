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

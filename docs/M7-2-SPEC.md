# M7-2 spec — the FreeCAD-authored foreign corpus (binding)

Mandate (docs/M7-PLAN.md unit 2): import FreeCAD 1.1.2-authored
STEP files of the entity subset we support — the first geometry
this kernel adopts that it did not write. Substrate: the measured
dialect inventory `~/.local/share/cad-work/m7-2-substrate/`
(gen.py + 13 kept .step files + inventory.md, incl. a full
170-entity walk of box.step and the ranked gap list) — read
inventory.md FIRST; every dialect claim below is its measurement.
This spec is binding: deviations are REPORTED (numbered, with the
executed blocker), never improvised.

## 0. The fence (unchanged from M7-1-SPEC §0, binding whole)

All work in `crates/step-import`. No step-export/CI/scripts/M6
edits. The 13 substrate files become COMMITTED fixtures under
`crates/step-import/tests/fixtures/freecad/` together with gen.py
(provenance; regeneration is manual, never a test dependency —
tests read the committed files, staying hermetic without FreeCAD).

## 1. Scope: the measured gaps, leg by leg

**Leg A — units and numeric hygiene (gaps 2, 10, 11).**
- Accept prefixed SI length units; resolve the full prefix table
  the subset needs (.MILLI. is what FreeCAD writes; structure the
  factor lookup so a prefix is data, not a special case). ALL
  lengths scale into kernel metres at Leg-B translation, including
  the declared uncertainty (1.E-07 mm → 1e-10 m becomes the
  default ε_in, D7's rule unchanged).
- Parse negative zeros (normalize to +0.0 at translation, stated)
  and 1e-16..1e-32 noise literals.
- **Interpretation budget (AMENDED per Evan, #187 comment,
  2026-08-04 — simplicity wins):** adoption/interpretation gates
  budget against **ε_in alone** — default = the file's declared
  uncertainty scaled to metres (1e-10 m here), per-call override
  unchanged (D7 verbatim). The quantitative ground: FreeCAD's
  12–13 significant digits put print truncation at ~1e-12·|x|
  relative, which the 1e-10 m absolute dominates for any part
  under ~100 m; if a giant model ever exceeds that, adoption
  fails TYPED with the residual named and the per-call override
  is the remedy — the fail-loud path, not a widened gate. (The
  originally-proposed per-literal `eps_in_eff` machinery is
  dropped as complexity without a driving case.) Exact `==`
  comparisons remain for the own-dialect (M7-1) paths.

**Leg B — bounds and outerness (gaps 1, 3, 9).**
- Accept plain FACE_BOUND-only faces. Outerness is an
  interpretation act with a stated rule: map each ring into the
  face surface's (u, v) chart via the kernel's own analytic
  projection, take signed area for orientation and ring-in-ring
  containment for outerness; a single bound is outer by
  definition (state it); ambiguity (containment undecidable at
  eps_in, or periodic wrap making the chart answer
  seam-dependent) refuses typed per D7 — never a guess. Where
  FACE_OUTER_BOUND IS present (our own dialect) it is honored and
  cross-checked against the inference (a mismatch is a typed
  error, not a preference).
- Honor FACE_BOUND orientation `.F.` as loop reversal composed
  into the realized half-edge directions. The substrate's measured
  counterexamples (4 planar caps: face `.F.`, bound `.T.`) prove
  it is NOT redundant with face sense — pin exactly those cases.
**Leg C — cones and the edge-free sphere (gaps 4, 5, 8).**
- Accept base-placement cones (the only form OCC writes): derive
  the kernel apex frame from (base origin, axis, radius,
  semi-angle) — a computation, budgeted at eps_in and pinned
  against the generator's closed forms. The M7-1 apex-form path
  stays (our dialect).
- VERTEX_LOOP: a full sphere arrives as ONE face, zero edges,
  bounded by a vertex loop. **Reconstruction (firm design,
  flagged in the PR):** the kernel cannot represent an edge-free
  closed face; adopt by minting the kernel-canonical sphere
  splitting (the native ball's own census — 2 faces / 2 edges /
  2 vertices), a REPORTED structure normalization on the import
  record (file census → kernel census mapping carried as data,
  never silent), volume/validity exact as always. This is D7
  stage-3 repair in its letter: the locus is fully explained; only
  the boundary-graph tessellation is re-minted. The vertex loop's
  point must lie on the sphere at eps_in (else typed error).
- Seam-unsplit periodic faces (cylinder/cone/torus, seam edge
  doubled inside one EDGE_LOOP, closed circles reusing one
  vertex): the substrate says this matches the importer's existing
  edge-twice precondition and Seam rung — verify, and pin one
  fixture per periodic kind; any gap found there is a numbered
  deviation, not an improvisation.

**Leg D — representation roots (gaps 6, 7).**
- Accept plain SHAPE_REPRESENTATION as a solid root (multi-MSB
  compounds — today refused as orphans by M7-1's structure rule;
  this narrows that refusal deliberately, keeping it for
  genuinely unreferenced entities).
- Assembly layer (NAUO / CONTEXT_DEPENDENT_SHAPE_REPRESENTATION /
  ITEM_DEFINED_TRANSFORMATION / REPRESENTATION_RELATIONSHIP):
  accept-and-traverse when every placement transform is the
  IDENTITY (the substrate's Import.export files); any
  non-identity assembly transform refuses typed naming the
  transform entity — full assembly instancing is a later unit
  (plan unit 4 may force it; do not build it speculatively).

**Leg E — acceptance corpus.** The 13 committed FreeCAD fixtures
+ hand-authored probes; expected censuses and closed-form volumes
derived from gen.py's dimensions (comment each derivation).

## 2. Acceptance rows (named tests; all binding)

1. **Foreign-corpus row**: every committed FreeCAD fixture
   imports; validity ladder green at default ε; certified volume
   matches the generator's closed form within quadrature pad +
   eps_in-propagated budget (derivation in a comment per
   fixture); censuses match stated expectations, with the sphere
   normalization mapping pinned explicitly (file 1/0/… → kernel
   2/2/2).
2. **Cross-dialect fixed point**: import(FreeCAD file) →
   `step_export::step_string` → import again → censuses and
   certified volumes bit-identical, and the SECOND export
   byte-identical to the first (the fixed point is in OUR
   dialect after one hop; byte-identity with the FreeCAD source
   is impossible and not claimed).
3. **Own-corpus regression**: every M7-1 suite stays green
   UNCHANGED — dialect relaxations must not weaken the own-corpus
   byte-identity fixed point or any M7-1 refusal that still
   applies (the narrowed orphan rule is the one deliberate
   change, named).
4. **Dialect rows**: mm scaling against closed forms (dyadic mm
   dims → exact m values where exactly representable, stated
   where not); base-cone apex derivation vs closed form;
   FACE_BOUND `.F.` pinned on the measured planar-cap
   counterexamples; outerness inference pinned on the multi-ring
   fixture (box_hole) AND an ambiguity-refusal probe
   (hand-built); negative-zero normalization.
5. **Refusal preservation**: EDGE_CURVE same_sense `.F.`,
   non-unit VECTOR, trimmed B-spline edges, non-identity assembly
   transforms, conversion-based units — all still typed, still
   entity-named (the substrate confirms FreeCAD never emits the
   first three; they remain the subset boundary, not dead code).
6. **ε_in rows**: scaled declared uncertainty (1e-10 m) read and
   exposed; a pi-derived-value fixture (cone semi-angle class)
   adopts cleanly under the DEFAULT ε_in — proving the flat
   budget absorbs print truncation on the real corpus; override
   still wins.
7. **Optional oracle row (loud-skip)**: re-exported FreeCAD
   fixtures validated through the local FreeCAD oracle (the
   check_step.sh admesh pattern, implemented INSIDE
   crates/step-import's tests — env-located freecadcmd, loud SKIP
   when absent so cargo stays hermetic; the export-side script is
   fenced and untouched).

## 3. Constraints (M7-1-SPEC §3 carries over whole)

f64 only; fail loud, structured refusal data (the GUI-remedy
directive); no new deps without the release-age policy; standard
Decide/K machinery for any new predicate — this unit is the first
real ε_in-scale interpretation work, i.e. the most likely first
source of #89 in-band landings: if k-lint rule 1 fires on these
suites, report it as a headline finding, never retune. Match
neighboring code voice; module headers state contracts.

## 4. Local battery scope

Crate suites at default ε as you build (foreground); one
`cargo check --workspace`; the optional oracle row locally (the
machine has FreeCAD) before the PR; no interval lane, no CI
mimicry. Hosted CI is the gate.

# M7 — STEP import as adoption (plan)

Ratified by construction: the boundary was decided at #169 (the
renumbering: "M7 should stay as just adopting STEP files") and the
content is DESIGN.md **D7** ("Import is adoption, not admission") —
this document assembles them; nothing here is newly proposed.
Core kernel work that import happens to *want* belongs to M6, not
here (#161 §2 relocated the census/declared-contact design work to
M6 by exactly this rule). M8 = error propagation.

**Concurrency note (2026-08-04):** M7 runs CONCURRENTLY with M6
under the fence recorded in `memories/concurrent-orchestrators.md`:
M7 touches only its new import crate + its tests + this plan;
no step-export edits, no CI-structure changes (append jobs only),
no M6-owned files. Any round-trip disagreement that tempts a change
to an export fixture, an `.expect` sidecar, or `check_step.sh`
semantics goes to a design-conversation PR seen by both
orchestrators and Evan — never a direct edit.

## The contract (D7, restated as obligations)

Import **reconstructs the intensional description** the extensional
data satisfies; imported bodies end first-class, their caches
recomputed and certified at ε by the kernel's own machinery.
Interpretation is governed by a separate per-import **ε_in**
(defaulted from the file's `uncertainty_measure_with_unit`,
overridable per call); healing may move geometry by up to O(ε_in)
as a **reported** model change, never a loosened certification.
Data ambiguous at ε_in fails with a typed ambiguity error; the
unhealable fail loudly, naming entities (D4 ¶5). Feature
recognition is a non-goal.

*Design consideration (Evan, #180 comment, 2026-08-04):* the
adoption machinery will likely be reused to offer GUI users the
appropriate **remedy** instead of an error — refusal types carry
structured data (entity, failed interpretation, what would be
needed), so a future remedy flow never parses messages.

## First slice: import what we export

The export corpus (14 solid fixtures + `nurbs_wireframe` under
`crates/step-export/tests/fixtures/`) covers the kernel's whole
geometry vocabulary as **native, exact AP214 entities** (M5 PR 13;
`memories/step-curved-subset.md`): PLANE / CYLINDRICAL_ / CONICAL_
/ SPHERICAL_ / TOROIDAL_SURFACE surfaces; LINE / CIRCLE / ELLIPSE
/ B_SPLINE_CURVE_WITH_KNOTS carriers. For this subset, D7 stage 1
(NURBS→analytic recognition) is mostly the identity — the entities
arrive already analytic — so the first slice exercises **stage 2
(edge adoption) and the certification path**, which is where the
inverse problem actually lives.

### Units, in order

1. **M7-1 — import crate skeleton + own-corpus round-trip**: new
   crate `crates/step-import` (own tests; workspace member —
   root-Cargo.toml member line is the only out-of-crate edit).
   Parse the AP214 subset the writer emits; adopt per D7 into
   kernel bodies; acceptance: for every solid fixture,
   export → import → **censuses, certified volumes, and validity
   match the source body**; the committed fixtures import to their
   `.expect` counts. `nurbs_wireframe` (curve-only) gets the
   disposition its geometry supports, stated, not skipped.
2. **M7-2 — foreign corpus: FreeCAD-authored files** of the same
   entity subset (FreeCAD 1.1.2, the version-matched oracle —
   `memories/freecad-oracle.md`): the first geometry this kernel
   adopts that it did not write. Validity + expected censuses /
   volumes; ε_in exercised for real (OCC's default write
   uncertainty is coarser than kernel ε).
3. **Later M7 (blocked on M6 units, not started early)**: NURBS
   *faces* (`B_SPLINE_SURFACE_WITH_KNOTS` arrives with M6's
   loft/sweep assembly — its import waits for its export);
   genuine stage-1 recognition (foreign NURBS within ε_in of an
   analytic surface, promoted); the healing ladder beyond what
   M7-2's corpus forces.
4. **Wild corpus (late; may defer past the slice — Evan, #180
   comment, 2026-08-04)**: suitably-licensed STEP files found in
   the wild that fall inside the supported subset (no NURBS), as
   a demonstration that import works on files nobody here
   authored. Sequenced near the end of the work, and deferrable
   until the underlying kernel support is more mature.

## K telemetry (standing, #89)

The import corpus is the **designated re-open trigger** for #89
(K=10, CLOSED — `docs/K-REPORT.md`): the expected first source of
IN-BAND LANDINGS, detected by k-lint rule 1 at the next hosted run.
A landing is a headline finding — **report it to Evan; never
quietly retune**. Known stale item, not this milestone's to fix
silently: the large-K lint's 1.5e-3 baseline floor (named M6
pickup).

## Exit shape

Every STEP file the kernel can currently export imports back to a
first-class body whose censuses, certified volumes, and validity
match; FreeCAD-authored files of the same entity subset adopt
cleanly or fail with the typed errors D7 promises. Anything beyond
that subset is a typed refusal naming the unsupported entity —
the S9 flip pattern applies when later units retire refusals.

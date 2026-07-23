# M4 Implementation Log

Orchestrator's running log for M4 (parametric model layer). Ratified
work order: `docs/M4-PLAN.md` (#80, 2026-07-23, F1–F9 + Evan's F6
early-spike amendment). Binding pre-M4 design: `docs/NAMING-DESIGN.md`
(#74), `docs/SOLVER-DESIGN.md` (#79). Obligations grounding:
`<main-checkout>/references/notes/m4-obligations-inventory.md`.
L-numbering continues (counter at L7, unused since M0).

## Process conventions (inherited from M3 unchanged)

- One implementer + one adversarial e2e reviewer (falsification
  assignments, real consumer programs) + one fix pass per PR;
  reviewer suites promote as `review_m4_prN*`; self-merge with full
  writeups on green gate; genuine design forks wait for Evan.
- Branches `ev/m4-<n>-<slug>`, merge commits only; OUTPUT DISCIPLINE
  header in every agent spec; push after EVERY commit.
- Merge gate: `scripts/gate.sh <merged sha>` while hosted CI is down
  (keep ci.yml in sync); all topology-determining comparisons through
  Q1 trileans into `geom_core::k_stats`.
- Evan's sequencing principle (recorded at #80 ratification): he
  holds sequencing opinions only where order could affect the final
  design (stopgap-entrenchment risk); surface exactly those.

## PR 1 (editor-core: recipe substrate) — launched 2026-07-23

Binding spec `docs/M4-PR1-SPEC.md` (D1–D9): crate editor-core
(geom-core dep only), Doc-as-value + pure apply, RecipeNodeId
(monotone, never reused), F4 node vocabulary as data,
structural-vs-continuous as distinct typed slots, expression
sublanguage v1 (F1 lattice with same-dimension ratios REFUSED in v1;
F7 AST, no conditionals; scalar-generic evaluator with Interval
instantiation pinned), ExprPath {node, slot, path} with stability
tests, DocEdit v1 arms + reserved-arm plan, replay-identity + doc
diff. Acceptance: the die authored as a document through apply.

## STEP spike (F6, early per Evan's #80 amendment) — launched 2026-07-23

Parallel adopt-vs-in-house evaluation of ruststep/truck-stepio for
the AP203/214 analytic-subset EXPORT (import stays M7).

**F6 DECISION (2026-07-23, spike report
`references/notes/step-spike-report.md`): IN-HOUSE subset writer;
adopt nothing at runtime; ruststep (Part 21 parser) +
truck-stepio's importer become DEV-DEPENDENCY parse-back oracles in
tests.** Grounds, executed not estimated: (1) ruststep cannot write
STEP at all (serialization is its own open roadmap item, ruststep#13)
and its AP203 semantic layer failed on a minimal two-entity file;
(2) truck-stepio's writer ships conformance defects unfixable
through its API (resource-schema FILE_SCHEMA over an AP214 data
section; FACE_SURFACE where ADVANCED_BREP_SHAPE_REPRESENTATION
requires ADVANCED_FACE; hardcoded units/empty product/unwrapped
uncertainty; no analytic-surface printers — wrong for the M5
carrier story); (3) the spike's ~120-line prototype produced a
152-entity AP214 cube that an independent importer reconstructed as
exactly 6/12/8, already MORE conformant than truck's output;
generalized writer ≈ 450–650 lines M4 scope. Hybrid rejected: it
buys the trivial record-printing 30% while denying control of the
acceptance-critical preamble. **Open caveat for PR 7: no FreeCAD/OCC
tool exists on this machine, so the external-import acceptance is
NOT yet discharged — PR 7 needs a FreeCAD import run where one is
available.**

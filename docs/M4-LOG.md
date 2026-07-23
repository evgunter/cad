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
the AP203/214 analytic-subset EXPORT (import stays M7): hand-built
reference parts through each crate, parse-back + external-import
check, dependency-policy review. Decision records here + in the PR 7
spec when the report lands.

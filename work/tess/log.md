# TESS log

## Opened at S-MESH's exit (2026-09-16)

Opened by S-MESH's orchestrator in the PR that proposes
`docs/S-MESH-EXIT-WALK.md`, as `work/README.md`'s closing rule directs:
S-MESH's open mesh findings, its parked MESH-9 and its Track R rows
cohere into one track on one territory (`crates/mesh`,
`topo::coherence`), so the closing program opens the successor and
moves them here. Band 5100–5199 recorded in the ledger's banding entry
in the same commit. No unit dispatched; the first sitting picks from
`work/tess/plan.md` §The slate (the NURBS face bound's unsoundness
first — a wrong certificate outranks every style row). Ev's sign-off
on the exit walk ratifies the opening.

## First sitting (2026-09-18)

Orchestrator seated; branch `tess/orchestrator`, in a lane clone
(`cad-work/tess-orchestrator`), not the main checkout. S-MESH's exit is
ratified (`docs/DOC-LEDGER.md` sweep 16), so the opening stands.

**Sequencing (mine, per the standing rule; alternatives recorded).**

1. `nurbs-face-bound-unsound-on-a-random-rational` first, as the opening
   entry directs. It is TWO defects by the row's own evidence — a 1.7 %
   `vv` excess (a wrong bound) and a two-ULP `uu` excess (a rounding
   comparison) — so the unit is preceded by a **diagnostic lane**
   (`tess/nurbs-bound-diag`, no fix, no PR, no A/B row): reproduce both
   seeds, localise which hull claim of the rational recurrence the truth
   violates, decide for the ULP case whether the certificate or the
   test's bare `<=` is the defect, and measure the failure rate across
   seeds. The spec is written from its report. Rejected: dispatching an
   implementer straight at the row — the row's cause is inferred from
   the assertion text, and `patch_bound.rs` is PROPS', so which program
   owns the fix is not known until the wrong step is named.
2. Then the door/refusal rows, because each is a shape a user can reach
   today: `rim-only-sphere-cap-panics-at-census` (release build emits a
   non-manifold patch silently — measured first), then
   `tessellate-refuses-approx-face-without-caches` and
   `lofted-circle-sections-…` (the latter's two cheaper remedies are
   outside `crates/mesh`; its owner is settled when it is specced).
3. Then the contract rows (`mesh-materialized-form-is-writable`,
   `degenerate-triangle-normal-is-substituted` with its rider,
   `tessellation-chart-frame-…`, `dist-line-triangle-…`), then the
   test-tightness rows (D300, S237, S236), the small ones (D303, D304,
   `memo-dumps-…`, `chart-azimuth-…`, `one-element-grid-axes-…`, C23
   premise-first), and S28 last — it is the largest and every other row
   moves the code it would unify.

**Review tiers (Ev, in-chat, 2026-09-18).** TESS takes EDIT's tiers:
full v6 dual on a kernel unit; style review plus a correctness arm, no
row, for an easy-but-not-trivial unit; green CI and the orchestrator's
read for a rename or prose row. Written into `plan.md` §Process.

**Plan rot fixed.** `plan.md` said the Track R rows "land per §D's
conventions" and wanted them "empty in §D" at exit; the register §D
belonged to left the tree on 2026-09-11 (`work/README.md`, sweep 11) and
Ev did not recognise the reference. The clauses now say what they can
mean: the seven rows are closed in `work/tess/`.

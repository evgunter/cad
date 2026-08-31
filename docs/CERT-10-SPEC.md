# CERT-10 — patch-hull consolidation (issue 1006, under the Q2 ruling)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **M/L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1006 is the primary specification; the Q2 ruling
(`docs/S-CERT-PLAN.md` §Rulings) fixes the decisions — they are
RULED, not yours to re-open. Branch `cert/10-patch-hull`.
Sequencing gate satisfied: CERT-5 (f63f2f3f) and CERT-7 (0fa3f176)
are both merged; work from current origin/main.

## The Q2 ruling, operationalized

Three spellings of tensor derivative-net assembly exist
(issue 1006's real distribution): `geom-brep/src/patch_bound.rs`
(the lift — per-cell, both readings), `props/quad.rs`'s `PatchGrid`
(~:1092 — missed by the original sweep), and
`mesh/src/nurbs_cert.rs`'s whole-face integral arm
(`integral_face_bound` + the three `*_derivative_hull`s). The
ruling:

1. **Shared home**: the derivative-net assembly moves to
   `geom_core::spline`, beside `compose`. Both `patch_bound` and
   `PatchGrid` consume it from there. What stays behind in each
   caller is its own READING of the nets (hulls, ladders), unless
   a reading is itself duplicated — then it moves too, argued.
2. **The whole-face arm collapses into a fold over `patch_bound`'s
   cells.** Per-cell-then-union is tighter or equal, so the bound
   improves or holds — demonstrate this on measured fixtures, do
   not just assert it. **Fold cost measured against the whole-net
   hull BEFORE the shape is chosen**: record both numbers (time
   and bound width, a rational and a non-rational face, a
   high-knot-count face) in the PR body, then choose, with the
   choice argued from the measurements. If the fold LOSES on cost
   somewhere material, the measurement is the deliverable and the
   shape decision cites it — the ruling requires the collapse
   measured, not the collapse regardless.
3. **The magnitude reading retires** in favour of the strictly
   tighter signed one. This coarsens/re-sizes rational-face grids
   and moves render and tess-budget baselines — the re-baseline is
   OWNED BY THIS PR: what moved and why stated per the render-lane
   conventions, every moved baseline re-derived with the argument,
   never re-baselined silently. Bit identity is explicitly NOT the
   bar (`memories/output-stability-as-justification.md`); affected
   pinned rows are RE-DERIVED, not preserved.

## Order of work

1. Red-first where the defect is measurable: a row pinning the
   whole-face arm's bound vs the per-cell fold's on the same face
   (the tighter-or-equal claim as an executable inequality), and a
   row pinning the signed-vs-magnitude width gap on a rational
   face.
2. The `geom_core::spline` home; `patch_bound` and `PatchGrid`
   re-pointed. Mechanical call-site updates in consumers
   (`offset_fit.rs` included) are in scope; behaviour changes
   there are not.
3. The whole-face-arm collapse, shape chosen from the recorded
   measurements.
4. The magnitude retirement + owned re-baseline, LAST — it has the
   widest blast radius; keep it a separable commit cluster so the
   re-baseline diff reads on its own.

## Fences / posture

- **Sibling lane fence**: CERT-8 runs concurrently on
  `topo/src/pcurves.rs`, `topo/src/chart_region.rs`, and
  `geom-brep/src/pcurve_cache.rs`. Do NOT edit those files.
- In `offset_fit.rs` you touch only call sites: issues 1320
  (projection bound through D), 1321 (BudgetExhausted split) are
  NOT in scope. Their acceptance instruments must not silently
  move: if the micron row's digits (3.222e-4 / achieved 3.791e-7)
  shift because the underlying hulls tightened, re-derive with the
  argument and name issue 1320 in the PR body so its baseline is
  updated knowingly.
- Issue 1322 IS in scope as invited ("cheap fix for whoever is
  next in the file"): pin the limb and the bound's shape in
  `offb_r1_probes`' wrong-sign row, matching the stronger sibling
  spelling at `offset_fit:666`.
- Issue 1368 (`props/quad.rs` — the `perimeter_lo <= perimeter`
  debug_assert and its ten-row disposition pass) is NOT in scope
  unless your `PatchGrid` rehoming already forces you through
  those probes; if it does, take the guard with the row-by-row
  pass, else state untouched. Do not break CERT-6's
  `boundary_chord_perimeter_lo` machinery; its calibration record
  has one durable home — if your grid re-sizing moves those
  figures, update the one home, not scattered copies.
- The mesh fence: `nurbs_cert.rs`'s integral arm is yours (the cut
  assigns it via issue 1006); do NOT widen into `mesh::walk`'s
  `closing_column` or other mesh ground.
- ε/lane posture per the issue-1356 practice: this unit's claims
  are interval-lane claims — `CI-Config: lane=both`. Enclosure
  hulls consult no tolerance but the fixtures ride
  `Tol::witness()`; run the three-ε local sweep on every
  new/changed row, state per-band premises, and pin the trailer ε
  with the argument.
- No `Co-Authored-By`; issues spelled out ("issue 1006"); push
  early to `cert/10-patch-hull`; the gate runs when the
  orchestrator opens the PR — report local evidence as local.

## Acceptance

- One assembly spelling, homed in `geom_core::spline`, consumed by
  both former assemblers; the third spelling collapsed or its
  retention argued from the recorded cost measurements.
- The tighter-or-equal demonstration and the fold-cost table in
  the PR body, with the shape decision citing them.
- The magnitude retirement's re-baseline table: every moved
  baseline, old → new, with the derivation; grid re-sizing digits
  for a rational face.
- Sweep obligation (assume a class): any remaining derivative-net
  differencing outside the new home (the original sweep missed
  `PatchGrid`; yours must say what it searched and what the
  pattern cannot match).
- Deviations stated; any refusal minted/changed classified per the
  D2 addendum.

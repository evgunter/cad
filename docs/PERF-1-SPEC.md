# PERF-1 — torus chart sizing: a per-direction doubly-curved chord bound

**Status: ratified at dispatch (PERF orchestrator, 2026-09-10).** Binds
the implementer of unit `PERF-1`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/torus-grid-step-one-step-both-directions.md`
(claimed from `work/mesh/`; read it — its "honest caveat" is this
spec's §2).

## 0. The finding this executes

`mesh::sizing::torus_grid_step(δ_s, R, r) = √(δ_s / (3(R + 2r)))`
(`crates/mesh/src/sizing.rs:448`) is one angular step applied to BOTH
chart directions of every torus face (`curved.rs:1030`:
`(ceil_count(uspan, h), ceil_count(vspan, h))`) and, through
`torus_step`, as a lower bound on the chord count of every circle edge
adjacent to a torus (`chords.rs:104`). On the tour's `hollowring`
(R = 0.30 m, r = 0.07 m) at δ = 0.1 mm that is 1021 divisions each way
and 3.98 M triangles; the per-direction sagitta asks 83 × 191. Measured
on a torus across four decades of δ by the PERF kernel lane: the grid is
110–145× the naive chordal cell count at every δ, with the correct 1/δ
scaling — the asymptotics are right and the constant is ~11× per
direction. This is why the ring documents are million-triangle
documents, why the display budget has to coarsen them, and why the
tour spends 71 % of its wall tessellating.

## 1. What this unit delivers

Two steps, `h_u` for the major (θ) direction and `h_v` for the minor
(φ) direction, each derived from a chord-deviation bound that is
**sound for a doubly-curved chart**, replacing the single step at all
three sites, with the certifier as the pin (§3) and the tess-budget
baseline re-cut deliberately (§4).

## 2. The bound — derive it, do not sketch it

The per-direction sagitta is NOT sufficient. A triangle of the grid
spans both directions, and the deviation of a linear interpolant over
a parameter cell `[0,h_u] × [0,h_v]` of a C² map `P(θ,φ)` is bounded by
the second derivatives including the mixed term — for the linear
interpolant on either triangle of the cell, a bound of the shape

    dev ≤ (h_u² · sup‖P_θθ‖ + 2·h_u·h_v · sup‖P_θφ‖ + h_v² · sup‖P_φφ‖) / 8

(Taylor with integral remainder over the triangle; state the constant
you prove, and prove it — the crate's own `sagitta_step`,
`curvature_step` and `ellipse_step` docs are the register for how a
step is argued here, and this one joins it). For the torus
`P = ((R + r cos φ) cos θ, (R + r cos φ) sin θ, r sin φ)`:
`‖P_θθ‖ = R + r cos φ ≤ R + r`, `‖P_φφ‖ = r`, `‖P_θφ‖ = r|sin φ| ≤ r`,
over the face's actual φ window where that is tighter (a face is an
iso-rectangle in (θ, φ), `curved::require_iso_rectangle_face`, so the
window is known).

Choose `h_u`, `h_v` to meet `dev ≤ δ_s` with a documented split of the
budget between the three terms (a named constant, not a tuned one —
`docs/TESS-BUDGET.md`'s aspect-policy ruling is the precedent for how a
choice like this is written down). The equal-split `h_u = √(8δ_s/(3·sup‖P_θθ‖))`,
`h_v = √(8δ_s/(3r))` does NOT satisfy the mixed term when `R ≫ r`
(check it: `2·h_u·h_v·r/8 ≤ δ_s/3` fails once `4r < R + r`), so the
split must be solved, not assumed; a clean choice is to bound the
mixed term by the geometric mean of the other two and solve the
resulting quadratic in `h_u/h_v`, or to fix the aspect `h_u·(R + r) :
h_v·r` and solve for the scale. State what you chose and its cost
against the ideal in triangles.

Both steps are capped by `cap_angular` as today; the `torus_cap_regime_is_sagitta_capped`
row in `sizing.rs` pins the cap-regime claim for the single step and
must be re-derived for two — decide whether the claim still holds per
direction and re-pin it, or delete the row with the reason.

## 3. The pin is the certifier, and it must be able to go red

- `cert::cert_torus(R, r, uv)` (`crates/mesh/src/cert.rs:141`) bounds
  a torus triangle's deviation from its uv corners, and the curved lane
  refuses typed when `worst > δ` (`curved.rs:319` onward). The new
  sizing must pass it on every torus the tree builds (the tour, the
  gallery documents, `crates/mesh`'s own torus rows) — that is the
  soundness pin, and it is mechanical.
- Add a row that sweeps `(R, r) ∈` a grid spanning `R/r` from 1.2 to
  50, δ across three decades, and asserts (i) the certifier accepts,
  (ii) the measured `worst` is at least 25 % of δ on the coarsest cell
  (a bound that is sound but ten times too loose is the defect this
  unit exists to remove — write the row so it goes red if the
  constant drifts loose again), and (iii) the triangle count is
  within a stated factor of the ideal your derivation names.
- `chords.rs:104`: an edge adjacent to a torus tightens against the
  step of ITS direction — a circle of latitude (axis parallel to the
  torus axis) against `h_u`, a meridian circle against `h_v`. The
  boundary polyline and the interior grid must coincide on the shared
  rows, as today; the watertightness census at the end of
  `tessellate` and `crate::validate::check_mesh` are the pin. Say how
  you classify an edge's direction and what refuses if neither applies.
- D9: byte-identical meshes for identical `(body, δ)`; nothing here
  may read a hash order.

## 4. The baseline re-cut

`docs/TESS-BUDGET.md`'s baseline was verified "exactly against the
torus grid step" and `tools/tess-lint`'s censuses assert the sized
rows. Re-cut under **"Re-cutting the baseline"** in that document:
`scripts/tess_budget_sweep.sh`, read every moved row, and write in the
PR what moved and why — the `hollowring` rows will move by an order of
magnitude and that is the finding, not drift. A `vanished` or
`uncovered` row is read, never folded. The tour's committed renders
will change (`demos/renders`); per `implementer-discipline.md` §3 the
question is only whether the new mesh is right, and the certifier
answers it. Re-baseline and say what moved. The display-budget doc on
`viewer/src/scene.rs:803-812` quotes the 65× figure — update the
sentence to what is true after this unit.

## 5. Measurement to report

`benches/` has no torus row; add `tessellate/torus/<δ>` beside the
washer rows (a criterion row, release, debug assertions off) and
report before/after on your box with the spread. Report the tour's
wall before and after (`demos/tour`, release, its own profile) and
`hollowring`'s triangle count at the viewer's δ = 0.1 mm.

## 6. Out of fence

The NURBS split schedule (`nurbs_cert`), the sphere/cylinder/cone
sizing, the display budget's probe (`fit_delta`, a separate unit), and
`tessellate.rs`'s id minting (PERF-3 edits that concurrently; this unit
does not touch `tessellate.rs` beyond what the chord pass needs).

## 7. Report

≤150 lines: the bound with its proof sketch and constant, the split
chosen and its cost against the ideal, the certifier sweep's numbers,
the re-cut's moved rows, and the measurements in §5.

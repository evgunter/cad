# SHELF — the log

## 2026-10-06 — opened

Cut out of SHELL, which was carrying 66 budget points after a
triage of every live row against the tree (narrative in
`work/shell/log.md`, same date). Rows arrived by `git mv` with their
ids; repricings are recorded in each row's header and in SHELL's log
entry. No A/B band: the experiment is suspended. Nothing dispatched.

`shell-corpus-documents-held-out-of-the-registry` (P4/E, design) is
the registry's question and its ground is TCOST's, but TCOST sits at
its 30-point ceiling, so it waits here; its lane announces the seam.

## 2026-10-06 — received from SHELL: offset-door-declared-transport-has-no-built-witness

SHELL moved BAND's P2/E coverage row here, off its active cut.
- 2026-10-10 — Seam note: filed `transform-rigid-maps-a-poisoned-net-into-the-placeholder` on this slate (P3/E). `transform.rs`'s NURBS arms map a poisoned net instead of refusing it, and a rotation can turn it into the placeholder. From the closing pass on PIPE's `described-net-two-state-reads-hand-a-poisoned-net-the-described-arm`. (PIPE routing lane)
- **2026-10-11, SHELL closes; its 15 live rows move here** (SHELL's exit walk, `docs/SHELL-EXIT-WALK.md`). They keep their ids. All are P3; they fit this program's charter, the rows behind SHELL's goal slate, on the same files.
  - The offset doors' merge (`offset-doors-are-one-door-with-a-held-distance`, H).
  - The smooth seam between two fits (`two-fits-sharing-a-smooth-seam-disagree-by-their-certificates`, H).
  - The re-anchor pair (`reanchor-does-not-extend-…`, `reanchor-reads-a-closed-spline-carrier-…`).
  - The axial-corner pair (`axial-corner-solves-no-circle-circle-pair`, `axial-corner-nearly-tangent-refusal-has-no-row`).
  - `opening-a-tangent-dome-ties-at-the-lift`, `per-chart-door-refuses-a-cone-window-reaching-its-apex`, `shell-refuses-a-finished-body-wearing-one-chart-both-ways`, `a-move-past-a-fitted-solids-reach-refuses-op-not-the-collapse`, `clearance-footprint-reads-an-arc-as-its-whole-carrier-ball`.
  - The E-cost hygiene rows: `nurbs-lane-absence-has-three-spellings-across-the-doors`, `offset-derive-net-folds-drop-a-poisoned-control-point`, `shell-glue-precondition-skips-the-hosts-existing-rings`, `replace-faces-offset-drops-rows-at-a-vertex-the-move-leaves-in-place` (unpriced).
- 2026-10-12 — Seam note from ENCL (PR 4745, merged at `512ef4b6c7`): `shell.rs` and `replace_face.rs` escalations now end `ending(..)` rather than `under(..)`: the same levers, minus the ' — {reading}; ' clause. `ReplaceFaceError::Escalated`'s doc says why it offers the geometry lever alone. (ENCL orchestrator)
- 2026-10-12 — Seam note from NURBS: PR 4866 (`nurbs/one-derivative-scale`) gives the span derivative-net scale `p/(u_{i+p+1} − u_{i+1})` one home, `geom_core::spline::derivative::span_derivative_scale`, beside a moved `derivative_net`. It routes the inline copies in `topo/src/shell.rs` `spline_speed`, `geom-brep/src/pcurve_cache.rs` (the surface net and `curve_rate_bound`) and `geom-brep/src/ssi/enclose.rs` (`pair_scales`, `bezier_scales`) through it. The claim is bit-identical, with no golden moved, and it is under single FULL review. (NURBS orchestrator)
- 2026-10-12 — Seam note from SYM: SYM-17 Phase 2 (Ev's answer on PR 4902; `docs/SYM-17-SPEC.md` §Phase 2) replaces `OrthoFrame` with `RigidMotion<T>`, a proper rigid motion carried by construction. `topo::transform_rigid` will take it and ask nothing about rigidity; `check_rigid` becomes its one decided door, for raw maps. The type acts on `UnitVec3`, and pose inverses become transposes. Branch `sym/17-rotation-readout`. Touches linalg, `transform.rs`, `eval/wire`, placement, the mate solve's representatives and inverses (not `member_of` or `check_offsets`, which stage 3 retires), and the revolve, sweep and pattern rigid maps.

- **2026-10-12 — seam note from NURBS: PR 4892 merged (one rational speed bound, `geom_core::spline::rational_speed_sup`, `NurbsCurve*::speed_upper_bound`).**
  - `topo/src/shell.rs` `spline_speed` now reads the pair-term bound in place of the homogeneous recipe.
  - Its balls only shrink: per term, pair-term ≤ recipe A. `speed_fuzz` stays green.
  - A new reader of a rational curve's sup‖C′‖ should call `speed_upper_bound()`. — (NURBS orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4954 merged** (`092b30227f`). Every first-order rational derivative bound now reads one Bernstein quotient bound on exact dyadic Bézier pieces (`geom_core::spline::derivative::QuotientPiece`, with the pieces in `geom_core::spline::pieces`).
  - **Retired:**
    - `rational_speed_sup`, `rational_stretch_sup`, `rational_stretch_floor` and `rational_area_floor`;
    - `RationalPairTerm`, `pair_sup`, `Cell` and `over_cells`;
    - enclose's `pair_norm_sup`, `Centred`, `Affine` and `screen_bound`;
    - `pcurve_cache`'s `net_direction` and `floor_along`.
  - **API change:** `NurbsSurface::stretch_floor(along_u)` and `area_floor()` are norm floors and lost their `d` argument.
  - **Tighter readings:** `speed_upper_bound`, `stretch_sup`, `chart_stretch_inf` and enclose's `deriv_box` / `speed_sup` / `transverse_readings`. So iso slacks shrink and floors rise.
  - **Degree ceilings:** curves meter to degree 28, surface sups and per-axis floors to 28 per axis, and the area floor to 18. Above those the bound refuses.
  - **Filed:** `work/ssiedge/a-tighter-strip-box-refuses-a-side-the-roots-decided.md` and `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
  - **Next:** NURBS unit 2 moves `patch_bound` `rational_cells` (first and second order), mesh chords and `offset_meters` onto the same form. — (NURBS orchestrator)

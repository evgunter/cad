# Review probes — gui1-r1 (PR #1093, frozen head 568bda33)

Mutants executed against the shipped rows and this lane's consumer
suite (applied, run, reverted; recorded for reproducibility). All runs
via `local-scripts/with-build-slot.sh -- cargo test ...` in the
r1 review worktree.

## Mutant A — `Bvh::ray` sort dropped
`crates/bvh/src/tree.rs`: final `out.sort_unstable_by(...)` removed.
Result: `ray::candidates_order_by_entry_then_index` FAILS and
`ray::sweep_matches_brute_force_and_never_misses_true_hits` FAILS
(realized == idealized asserts order), plus both r1 sweeps. The
sweep's set-AND-order claim genuinely gates — claim 3's "can go red"
verified.

## Mutant B — tree-shape leak (items inherit the leaf hull's t_enter)
`crates/bvh/src/tree.rs` leaf arm: `RayCandidate { item, t_enter }`
with `t_enter` taken from the LEAF hull's `slab_enter` instead of the
item box's. Result: `ray::candidates_order_by_entry_then_index` FAILS,
`ray::sweep_matches_brute_force_and_never_misses_true_hits` FAILS —
the realized==idealized row detects tree-shape leakage into `t_enter`,
not merely into membership.

## Mutant C — pick tie-break flipped
`crates/editor-core/src/resolve/pick.rs`:
`(target_pos, cand.item) < (b.target_pos, b.tri_pos)` → `>`.
Result: `gui1_pick::edge_ray_between_two_faces_resolves_deterministically`
FAILS (the shipped shared-edge row genuinely pins the documented
patch-major winner), and this lane's
`review_gui1_r1::dyadic_battery_pins_faces_edges_corners_and_tiebreak`
and `review_gui1_r1::coplanar_cross_target_tie_resolves_by_target_position`
FAIL.

## Behavior probes (scratchpad, not committed)

- `slab_enter` d=0-outside sign asymmetry: with a finite constraint on
  any other axis, both infinity signs prune (doc's "exact prune"
  holds); with NO other finite constraint (e.g. a zero-direction
  point-ray strictly below the slab), the +∞-product side returns
  `Some(≈f64::MAX)` — kept as a candidate — while the −∞ side prunes.
  Conservative, but the `axis_interval` doc-comment's "the prune is
  exact" is one-sided. Probe output:
  `point-ray below slab: Some(1.7976931348623151e308)`,
  `point-ray above slab: None`.
- e2e consumer (pncad-authored scenes; editor-core picking): occlusion
  and target-order behavior as documented; `t` scales with `|dir|`;
  silhouette grazes (ray in a face's plane) HIT the side face's closed
  boundary (the in-plane face itself is a parallel miss); far origins
  fine at 1e9 and 1e300; a 1e-9 sliver profile is REFUSED upstream by
  profile authoring (margin 1e-9), a 1e-6 sliver tessellates and both
  its wafer cap and its 1e-6-tall side face pick correctly.

---
id: a-plane-poses-normal-read-off-a-face-is-opaque-to-the-unit-relation-at-sym
kind: issue
title: A plane pose's normal read off a face is opaque to the unit relation in the Sym replay
status: open
opened: 2026-10-23
priority: P2
cost: M
---

Found by the designer pair on MSOLVE's `a-plane-poses-foot-outgrows-the-coincidence-doors-budget` (record on `analysis/design-fork/msolve-plane-foot-sym-budget`), and filed by its implementer on PR 5624.

The tier folds `|n|² = 1` for a unit vector built from plain symbols. It does not fold it for the normal of a plane pose read off a face in the `Sym` replay. That normal is the carrier's stored normal, minted by Newell's method (`work/lathe/an-extrudes-cap-carrier-is-minted-by-newell-not-by-its-frame.md`) and normalised again by the pose read (`read_off` in `crates/editor-core/src/eval/wire/pose.rs`, through `pose_unit`).

Measured on PR 5624's branch:
- The split tool read off a box's cap has a normal whose `x` and `y` have value `0.0` but are not the literal zero node.
- The reunion's four crossing-vertex `OnCarrier` margins (281-node DAGs) do not discharge at 256, 300, 384 or 512 terms. They do discharge at 2048.
- On designer A's probe (a split on another body's top face, `coincidence_door::a_split_on_a_faces_plane_lays_its_crossings_on_that_face`), the crossings' margins do not discharge even at 16384 terms.

So this is not a budget problem.

Suspects, unmeasured:
- `pose_unit`'s re-normalisation of an already-unit carrier normal;
- a frozen node on the Newell sum;
- an entry of `scripts/gates/from-f64-sites.tsv`'s `launders(…)` class on the carrier path.

PR 5624 makes rung 1 prove these rows without the replay, so no row depends on this today.

**Done when** a plane pose read off an extruded cap has, in the `Sym` replay, a normal on which `|n|² − 1` discharges, or the reason it cannot is stated with its site.

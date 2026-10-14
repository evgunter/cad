---
id: the-near-eps-rigid-map-rows-are-red-at-1e-12-on-main
kind: issue
title: rigid_map_near_eps_approx's two rows are red at eps 1e-12 on main: no rotation moves the near-eps face's bound past eps (d = 0.05)
status: open
opened: 2026-10-14
priority: P2
---


## What

`crates/topo/tests/rigid_map_near_eps_approx.rs` has two rows that are red at `CAD_TOLERANCE_EPS=1e-12` on main `e9e5df7c8a`. GERM reproduced them locally with `cargo nextest run -p topo -E 'test(/minted_near_eps/)'`:

- `the_image_of_a_face_minted_near_eps_refuses_re_derivation` panics at `:208`: "d = 0.05: no rotation moved the bound past ε, so the face does not reproduce the refusal".
- `a_rigid_map_moves_a_face_minted_near_eps` panics at `:266`: "d = 0.05: both answers must occur — 32 imaged, 0 re-fitted".

Both pass at 1e-6. The rows are in the slow set, and the hosted job runs the slow set at the default ε only, so no PR's CI shows them. GERM's PR 5012 fix-pass lane hit them first; they are not that PR's.

This is the shape ENCL re-found once before (`fcf725662e`, "re-find the near-eps rigid-map witness on round 2"): a certificate change moved the witness face's bound, so no rotation at d = 0.05 pushes it past ε at 1e-12. The commit that moved it has not been bisected. The rows were already red at `6f2656af50` (2026-10-13 22:33 UTC), which is before GERM's PR 4966 (`geom::square_frame`) merged, so PR 4966 is not the cause. PCERT's check-4 norm readings, which landed before that commit, are one candidate.

## Done when

- Bisect the commit that turned the rows red.
- Re-find a witness that reproduces both answers at every ε row, or show why 1e-12 can have none and state it in the rows.

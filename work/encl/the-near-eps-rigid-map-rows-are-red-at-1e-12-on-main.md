---
id: the-near-eps-rigid-map-rows-are-red-at-1e-12-on-main
kind: issue
title: rigid_map_near_eps_approx's two rows are red at eps 1e-12 on main: no rotation moves the near-eps face's bound past eps (d = 0.05)
status: closed
opened: 2026-10-14
priority: P2
branch: nurbs/fix-shelling-pin-after-4984
closed: 2026-10-14
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

## Resolution (NURBS, 2026-10-14)

Bisected with a fresh `CARGO_TARGET_DIR` per commit: PR 4984 turned
the rows red. They pass at `a71403e3` and fail at its merge
`2639b944`. 4984 made the offset regularity floor the area quotient's
inf. That tightened the round-2 bound's ambient-frame terms, so the
worst map's drift at `d = +0.05` and 1e-12 fell to `2.16e-4`, under
the `2.28e-4` that the fixed `MARGIN = 1/4096` landed the face at.

A fixed margin cannot be re-picked. At round 2 the scaled fit's
departure from `s` times the unscaled bound is up to `±1.7e-4` and
jumps between neighbouring scales: `1/6144` holds at 1e-12 and 1e-9
and mints a face `3.3e-4` under ε at 1e-6. So `near_eps_face` now
walks a ladder of margins. It keeps the first round-2 face whose
images include both a hull-limb refusal and a certificate, and panics
if none does. All three rows pass at 1e-6, 1e-9 and 1e-12.


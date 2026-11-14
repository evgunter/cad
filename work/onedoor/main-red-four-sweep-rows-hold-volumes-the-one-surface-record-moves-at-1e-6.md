---
id: main-red-four-sweep-rows-hold-volumes-the-one-surface-record-moves-at-1e-6
kind: issue
title: Main red at eps 1e-6: since PR 6505 four sweep rows build bodies whose volumes miss their closed forms by a fraction of eps (keel halves, cone-cap union, two parallel-cylinder rows)
status: open
opened: 2026-11-14
priority: P0
cost: M
---


Found by GREEN's ε 1e-6 lane (`green/eps-1e-6`), from nightly 43371638371 (main `e36675c1`). Red at ε 1e-6 only; all four passed in the nightly of 11-11 (main `b2916558`) and failed in the nightly of 11-12 (main `da77d875`).

**What fails** (`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E 'test(=<name>)'`, on main `b363f5fa1`):

| row (`sweep::all`) | first message | off by |
|---|---|---|
| `a_curved_keel_in_the_plane::a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur` | `the V turned -20°, its tip 0.6 ε off the plane, the plane's normal 1 y: the halves hold 40.32583257462023, the body 40.3258330933859` | 5.2e-7, against `1e-9 · whole` |
| `a_plane_moved_to_a_cone_apex::a_half_space_flush_with_a_frustum_wedge_cap_builds_the_union` | `y0 = 0.25, tan α = 0.015625, 0.5·ε over the cap, wedge ∪ half-space (start 32.004·ε): volume 24.0000679165704 against 24.000068913518643 (± 0.00000003864850641351864)` | 1.0e-6, against 3.9e-8 |
| `parallel_cylinder_join::one_face_walls_read_their_axes_as_an_angle` | four runs tipped `2e-7` (`0.2·zero`): `seams at 0 \| S ba`, `seams at π \| U ab`, `U ba`, `S ba`, each `OK BAD t2=true t3p=true cert=true operand=true`, `v=1.348201211 want=1.348201331` and `v=7.631386518 want=7.631386638` | 1.2e-7, against `VOLUME_CLAIM = 1e-7` |
| `parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region` | six runs tipped `4.6875e-7` (`1.5·zero / 3.2`), about `x` and about `y`: `U ab`, `U ba`, `S ba`, `v=8.853981355` or `8.853981433` against `want=8.853981634`, `v=2.570796126` against `2.570796327` | 2.0e-7 to 2.8e-7, against 1e-7 |

Each row stops at its first failing assertion, so the cells after it are unread. Every body that fails passes tier 2, tier 3′, the certificate and the operand check; only the volume misses.

**Bisected, first-parent, at ε 1e-6**, all four together:
- green at `6d4664849`, `52444e25a`, `97af75510`, `690c83eb6` (PR 6511) and `e996c0049` (PR 6672);
- red at `9e1484043`, the merge of PR 6505 (`onedoor/one-surface-record`), whose first parent is `e996c0049`;
- red at `90998fa0b`, `ae97403b4` and main.

**What this row owes.** Each pose puts two carriers a fraction of ε apart: a tip `0.6 ε` off the split plane, a brick face `0.5 ε` over a cap, axes tipped inside the zero band. That they are now decided one carrier, with a face moved onto the kept record, is the reading the merge suggests; this lane did not measure it. The far-origin union's shift of `4/3 ε` was closed as correct with its bound re-derived (`a-far-origin-rods-caps-moved-onto-the-plates-record-move-the-union-volume-by-four-thirds-eps`). These four have not been derived. For each, decide whether the shift is the one record's by construction. If it is, restate the row's bound from the band and the faces' areas, with the argument. If a shift is larger than that, it is a wrong volume. This lane did not derive any of them, which is why the row is banded P0 until one is.

`band_strip_mouth_cups::a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes` went red at the same merge with a different stop (the fixture's boolean returns no finished body); it is `work/band/a-cups-fixture-boolean-is-unfinished-at-eps-1e-6.md`.

**Pinned** (`green/eps-1e-6`): each of the four is under `test_utils::pinned_red::row` at ε 1e-6, on the messages above. Take the pins off with the fix.

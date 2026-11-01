---
id: review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose
kind: issue
title: review_pick_r2's pinned tally moved at PR 4861's merge; main is red on the slow set
status: closed
opened: 2026-10-14
closed: 2026-10-26
---


(Found by the NURBS program while landing PR 5123, whose downstream run saw the row red on main as well as on its own head. A bisect lane then attributed it.)

## What

`crates/viewer/tests/review_pick_r2.rs` asserts a pinned corpus tally: rays, answered at the aimed vertex, refused candidates, rays with a refusal. It was last re-pinned in `f4bc67d69c`.

**Bisect.** It ran over `git log --first-parent dee7347bd3..b41d2fe38f`, building every tested commit with its own fresh `CARGO_TARGET_DIR` and running `cargo test -p viewer --test all -- review_pick_r2`. The tally moved exactly once:

| merge | tally |
|---|---|
| `9235f6d13a` (#5112, last good) | (441198, 141261, 14088, 7230) = the pin |
| `1d6c939490` (#4861, `cleave/section-ellipse-re-pose`, first bad) | (441198, 141201, 12294, 6504) |
| `b41d2fe38f` (main, 2026-10-14) | (441198, 141201, 12294, 6504) |

Only the pin assertion fails:
- No genuine crossing is refused, and there are no wide winners.
- The ray count is unchanged, so no retessellation changed the vertex count.
- The best conditioning refused went from 4.79e-16 to 4.71e-16.

**Direction:**
- 60 fewer rays are answered at the aimed vertex.
- 1794 fewer candidates are refused.
- 726 fewer rays carry a refusal.

This fits #4861 re-posing the ellipse carrier (`u_ref` on a principal axis), which shifts vertices on elliptic section edges by rounding. It was not broken down per document.

**Why it landed red:** `review_pick_r2` is in the slow set (`.config/nextest.toml`), which per-PR CI does not run.

**Moved again by INTENT stage 3 A′** (PR 5243, measured 2026-10-17): main at `82d0f5c2b7` still answers (441198, 141201, 12294, 6504), and the A′ branch answers (441198, 141763, 12306, 6534). A′ gives a revolve's axis a heading, so the gallery ring (`crates/viewer/tests/gallery_ring.pncad`) turns about an axis whose quarter-turn cosine rounds to 6e-17, and its meridian vertices sit about 3e-19 off `y = 0`. 562 more rays are answered at the aimed vertex than on main, 12 more candidates are refused, and 30 more rays carry a refusal. A′ did not re-pin the row.

## What is open

The owner decides whether the 60 lost aimed-vertex answers are acceptable, and re-pins with that reading stated, or fixes what lost them. Until then, every program's downstream run sees this row red on main.

**Re-pinned, then moved again (2026-10-19).** INTENT stage 3 C (PR 5267) re-pinned the row to A′'s tally (441198, 141763, 12306, 6534). PR 5225 (`pcert/azimuth-placement`) then moved it to (441198, 141805, 13524, 6936), and the 2026-10-19 nightly is red at every ε. That movement is filed on PCERT's slate as `review-pick-r2-tally-moved-at-5225-azimuth-placement`. The question above about PR 4861's lost answers is still open.

## Closed (2026-10-26)

Stale: the red this row carried is gone. `crates/viewer/tests/review_pick_r2.rs` has been re-pinned three times since (PR 5267, PR 5068, PR 5370), each re-pin recording that no genuine crossing is refused either way; the 4861 movement measured here (60 fewer aimed-vertex answers, fewer refusals, no genuine crossing refused, an ulp re-pose) is of the class those re-pins accepted. The 5225 movement is PCERT's `review-pick-r2-tally-moved-at-5225-azimuth-placement`, and PR 5370's pin starts from its tally. (CLEAVE slate triage)

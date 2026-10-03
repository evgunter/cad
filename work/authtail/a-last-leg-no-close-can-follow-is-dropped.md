---
id: a-last-leg-no-close-can-follow-is-dropped
kind: issue
title: viewer: a chain's last leg that no provisional close can follow is dropped from the preview, and the cross or tip lands one vertex early
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam]
---


Found by AUTH-13 (`author/geometry-close`, PR 3579) and its reviews.

**What happens.** `sketch::preview` draws a chain that does not close by replaying it under a provisional close (`sketch::replay_provisionally_closed`), and walks back (`sketch::prefix_loop`) when that close is refused. A last leg whose tip no close can leave is therefore never in any replay. The chain is drawn short of it, and the refused step's cross, or an unfinished chain's tip, sits one vertex early. Measured on the branch; each case is held by `sketch::tests::a_last_leg_no_close_can_follow_is_walked_back`, which is the row to re-pin:

- **Onto the start, by a step that does not name the start.** Examples: `at, toward, line, (turn π/2, line)×3`, and a square whose last `line_to` is `5.5e-17` off the start. Every close from a tip on the start has no length. The step cannot be re-spelled as the close: a `line` names no end point, and the target is not the start as written. Far-end anchors (no `Start` form) and arc specs with no target are the same case. `replay` exposes no tip of a chain that does not close.
- **The start behind the last leg on its own line.** Every close reverses the leg (`JunctionCusp`). Example: `at (0,0), line_to (0.005,0.01), line_to (0.005,0), line_to (0.01,0)`. A chain that passed through its start and went on (`…, line_to (0,0), line_to (-0.01,0)`) is drawn as the closed loop its own steps made, with the leg after it dropped.
- **Inside the kernel's ambiguity band.** `line_to Start` escalates as too close to call, and the preview says so rather than re-spelling it. A last leg that turns by `1e-9` from straight-on is one example; there `continue_to Start` escalates too. Another is a long leg with a short close: `two_legs`, then `line_to (0.0010000004, 0.001)`, anywhere in a window of at least `d` from 2e-10 to 1e-9 in `line_to (0.001 + d, 0.001)`. There `continue_to Start` would replay. The viewer re-spells only a *decided* tangency, though. An escalation is the kernel declining to say whether the close runs straight on, and another spelling that happens to pass is not that answer. Drawing the leg here would show the author a straight-on close the kernel has not called straight on.

**Shape of a fix.** The start-on-tip case wants a door from the kernel: a typed refusal for a close of no length (`work/paths/a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`), or a replay that returns an open chain's tip. With the first, a straight last leg can be drawn as the prefix before it closed, which is the same segment. The cusp and band cases have no close at all, so drawing their last leg needs a drawing that is not a closed replay. That is a design question about what the preview is.

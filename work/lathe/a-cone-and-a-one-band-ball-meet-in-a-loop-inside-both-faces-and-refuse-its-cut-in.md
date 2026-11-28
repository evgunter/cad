---
id: a-cone-and-a-one-band-ball-meet-in-a-loop-inside-both-faces-and-refuse-its-cut-in
kind: issue
title: A cone and a one-band ball off its axis meet in a loop inside both faces, and the cone's cut-in is not built (FallbackExtentUnsupported) where main's two lunes build
status: closed
opened: 2026-11-26
closed: 2026-11-28
priority: P1
cost: M
refs: [a-ball-against-a-donut-turned-a-quarter-refuses-its-cut-in-at-1e-6, close-cuts-is-not-total-on-crossings-path-cut-ins]
---

## What

`sweep::all a_cone_with_a_ball_or_a_cone_in_every_order::a_cone_and_a_ball_off_its_axis_refuse_their_loop_in_every_order`
is main's row (`e27a7a9ea8`, `cfac4e0abd`), named
`a_cone_and_a_ball_off_its_axis_build_in_every_order` there. On main `5e04ac6561` it
builds every op in both orders. On PR 7414's head, after main's merge
(`f2731c9f39` and later), every run refuses, typed, at ε 1e-9, 1e-6 and 1e-12:
`FallbackExtentUnsupported { operand, face: FaceKey(2v1) }`, with the
operand the cone in either order. The message reads: "a curved face
meets a face of the other solid in a closed loop interior to both
faces, which no edge event and no vertex probe can see; the loop is
certified, and cutting it in is not built". The row is not on the
one-band stack before the merge (`a8a9d3a9b6`).

The pose is the ball of radius 0.4 about `(0.9, 0.6, 0)`, poled along
`z`, against the cone `K` (base radius 1 at `y = 0`, apex `(0, 2, 0)`).
They meet in one loop on the cone's side. On main the ball's two lune
seams are the meridians through `±u_ref`. On head the ball has one wrap
edge, and no edge event reads the loop. So the loop reaches the
no-crossings path as interior to both faces, and the cut-in it asks
for on the cone's face is not made. The section pass's refusal stands:
typed and loud, with no body shipped wrong.

## Not yet known

Whether the cut-in is refused because the cut face is a cone (the
no-crossings path's cut-in reads a sphere's meridian), or at one of
the cut-in's readings, is not traced. Nor is whether turning the ball
so its wrap edge crosses the loop restores main's outcome.

## Pin

The row pins each run's outcome. For this pose it pins the refusal
above in all six runs (`BALL_PINNED`, citing this row); the two-cone
row stays `built`.

- 2026-11-26 — Re-banded P2 → P1 (LATHE orchestrator). Main builds this pose; the one band refuses it typed. That makes it a capability regression the one-band landing introduces, which takes P1 as `a-revolved-cone-cut-along-two-rulings-through-its-apex-refuses-the-pinch` did. The refusal is loud and owned, so it did not block the landing.
- 2026-11-27 — Not part of `close-cuts-is-not-total-on-crossings-path-cut-ins` (LATHE, `lathe/close-cuts-total`). This pose never reaches the closing. A cone's pair with a ball takes no cut-in by design (`topo::boolean::ops::interior_cuts`'s module doc and `cut_in`: "A ball's pair with a cone takes no cut-in, and its pair's refusal stands"), so the no-crossings path's section certificate refuses it before any cut exists. Closing it needs a cone cut-in, or a ball cut-in read against a cone partner. That is a cut-in question, not a closing one. The priority stands: the regression and its loud refusal are unchanged.

## 2026-11-28: closed (LATHE, `lathe/cone-ball-cut-in`)

The pose builds every op in either order at ε 1e-9, 1e-6 and 1e-12, through the sphere face's loop cut-in. The row is `a_cone_and_a_ball_off_its_axis_build_in_every_order` again, main's name, with its pins at `built`. Each body passes tier 3, tier 3′ and the certificate. The intersection matches a slice-by-slice integral of the two discs' lens to about 1e-13, in both orders.

Two stops, each traced:

- **The cone's arm was missing from `interior_cuts::cut_in`.** That was an omission from when the operand gate stopped refusing the pair (426d22ec8e), not a rule. The cut is made on the sphere face, so the cone's face need not be simply connected.
- **`apply_cut_ins` read the ball's pole vertex against the loop's tube heights.** The tube's boxes reach past the sphere (to `0.4186`, with r `0.4`), so the pole read as inside the loop's heights. The loop lies decidedly in the cut's open half-plane, so a pole, the half-meridian's end, lies past every crossing. A hit read on the axis is now placed by its own height.

The cut-in also reaches `germ_sphere_no_crossings::a_ball_against_a_cone_face_is_the_section_passs`, whose small ball (radius `0.05`) on the quarter cone's lateral face refused at the section pass before. It now asks for one cut-in.
- At ε 1e-9 and 1e-12, every op builds in either order, at the slice integral of the ball's part (`sweep::test_support::lens`) and the closed forms with it.
- At ε 1e-6, every op escalates, typed, at the pierce curvature: a margin of `1.26e-6` read in the band `[1e-6, 1e-5]`.

PR 7839's review tightened both rows.
- **The pad ceiling is per ε:** `1e-6` of the volume, and `5e-6` at ε 1e-6, as measured.
- **Each comparison reads its own pads** and the oracle's error estimate, with no fixed slack.
- **The lateral ball's ops** are each held relative to their own volume against the oracle's integral. Its escalation at ε 1e-6 is pinned by decision, margin and band.
- **The pole verdict is the one each hit is made with** (`bool_sphere_cut_pole`), so no reading is added for hits off the axis.

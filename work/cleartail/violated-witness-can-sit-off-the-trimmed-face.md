---
id: violated-witness-can-sit-off-the-trimmed-face
kind: issue
title: clearance's Violated witness is verified for distance but not for membership, so it can sit off the trimmed face
status: open
opened: 2026-09-14
priority: P3
cost: E
---


## What was measured

Reviewer r2 of PROPS's sign-hull unit (PR #2468) attacked
`editor_core::clearance`'s `Violated` arm through its public door and
found the witness it hands back can name a point the face does not
carry. Two instances, both at `Interval` over an ε-scaled leaf:

- **A 30°-tilted prism against a copy 0.3 m away**, bound 0.2 →
  `Violated`, witness `b = (1, 0.0453, 1.1684)`. Mapped back to the
  extrude's own parameter, that point is at `t ≈ −0.31`: below the
  copy's start cap, on the carrier plane but off the trimmed face.
- **Two facing chamfered cubes** at `t = 1.05`, whose nearest real
  planes are 0.2475 apart, bound 0.2 → `Violated` at a separation of
  `0.182 = √(0.05² + 0.175²)`. That distance is only reachable at the
  edge the chamfer removed.

## Why

`verify_witness` (`crates/editor-core/src/clearance.rs`) re-evaluates
the document at `f64`, reads the two `(u, v)` pairs in the same stored
charts the interval pass subdivided, and checks the DISTANCE between
the two points against the bound. It does not check MEMBERSHIP: that
each `(u, v)` is inside its face's trimmed region. A cell of a carrier
window that covers more than the face — the same window model
`clearance-window-tightening-needs-chart-boundary` (`work/trim/`)
describes — can therefore carry a verified-by-distance witness the body
does not touch.

The verdict is the load-bearing part: `Violated` is the arm a consumer
acts on, and the witness is what it acts on it WITH. A witness off the
face is a report of interference at a place with no material.

## Not the frame's, and not PROPS's

Frame-independent. The windows in both instances are axis-aligned or
blend-minted, so the frame the sign-hull unit changed is not what puts
the witness off the face; re-running the same fixtures under the old
construction reproduces the class. It is the window model plus the
missing membership check, which is this program's ground
(`crates/editor-core/src/clearance.rs`).

## What a fix would look like

The same dependency `work/trim/clearance-window-tightening-needs-chart-boundary.md`
names: the face's boundary in chart coordinates. With it,
`verify_witness` can ask whether each `(u, v)` is inside the loop's own
2-D extent and refuse the witness (not the verdict's arithmetic) when
it is not — or the subdivision can drop the cell before a witness is
ever probed there, which closes both this row and that one. What must
NOT happen is a tolerance: membership is a boundary question, decided
by the description or not at all.

## Home

`clearance::verify_witness` and the `Violated` arm of
`clearance`/`self_intersection`. Found by PROPS's sign-hull dual review
(r2), filed here because `clearance.rs` is SHELL's ground
(`python3 scripts/work.py territory`); related to `work/trim/`'s window
item by mechanism, not by owner.

## Update (2026-11-02, DOORS' `a-clearance-assertion-is-decided-by-the-sweep`)

User-facing now, and for an upper bound's PROOF as well as a violation:
a clearance assertion's `Within` pair is a verified pair of face points
drawn by the viewer. The verifier is offered only stations the faces'
point doors place on their faces (`Candidate::on_face_stations`), for
both questions. Two rows pin it on non-convex faces whose windows cover
material the faces lack:
`m10_6_r1_probes_interval::an_upper_bound_over_a_notch_is_proved_only_by_face_points`
(the windows' pair across the notch is 0.1 apart, the faces' 0.269) and
`r2_m10_6_probes_interval::a_clearance_between_two_separated_bodies_reads_their_gap`.
The membership question at a station the point door places `OnEdge`
within the band is the row's remaining edge.

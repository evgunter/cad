# ENCL — the plan

certified enclosures: what a certificate claims, and what it is worth

## Where the slate stands

Every row the program could act on without another program has been worked.
Those rows have landed through a series of merged PRs, which the log names one
by one. What is left falls into three groups.

**Waiting on PROPS' `f64-refinement-inside-an-enclosure-has-five-more-sites`.**
The fix is the convex form of `insert_once_ring` in
`crates/geom-core/src/spline/compose.rs`. ENCL does not take that site; it
belongs to PROPS, and PROPS has been told on its log twice.

| pri | item | why it waits |
|---|---|---|
| P0 | `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart` | the certified bound's floor is that site's insertion width |
| P1 | `patch-bound-offset-fit-recentring-origins` | measured once the insertion width is gone |
| P2 | `one-pass-refinement-would-cut-the-rational-bounds-widening-tail` | the same enclosure-width family |
| P3 | `a-rigid-map-still-refuses-the-bowed-approx-fixture-at-eps-1e-12` | the frame-dependent width is the same insertion width (measured: the convex form alone gives 0 of 93 refusals) |

When that fix lands, the P0 is taken first. It re-measures the offset fit's
tight-ε floor and its budget numbers.

**Waiting on SHELL's `shell-refuses-every-lofted-body-at-a-wall-seam-carrier`.**

| pri | item | why it waits |
|---|---|---|
| P3 | `a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters` | latent until a body with a curved `Approx` face and edges can be moved. It is a design fork, weighed by two designers once it is reachable. |

**Open and in flight.**

| pri | item | state |
|---|---|---|
| P3 | `certify-escalation-renders-the-coincidence-menu-unlabelled` | a lane is running (style review) |
| P4 | `domain-grid-homing-residue` | open. It is the residue left by the grid homing, and it is taken if the track has room. |

## Review posture

The review tiers are those of `memories/orchestration-model.md`, and every lane
runs on Opus. The log names the tier for each unit at dispatch.

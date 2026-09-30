---
id: a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam
kind: issue
title: profile: a close from a tip that sits on its start is refused as a tangent seam, with a recourse the kernel refuses in turn
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [a-close-refused-on-its-geometry-draws-nothing, a-last-leg-onto-the-start-is-dropped-unless-the-loop-opens-with-at, a-last-leg-no-close-can-follow-is-dropped]
---


Found by AUTH-13 (`author/geometry-close`, PR 3579), by running probes through `profile::replay`.

## What

A chain whose last leg lands on its start and is then closed by `line_to Start` has a closing leg of no length. `replay` refuses it `PathError::SeamTangent` (`crates/profile/src/path.rs`, the `SeamTangent` arm), whose sentence says the seam "arrives tangent to the entry's first side … and nothing declared it" and offers "DECLARE the joint on the target with Start.arrives_tangent()". The leg does not arrive at all, and the recourse is refused in turn. Measured, each from a tip on the start:

| chain before the close | `line_to Start` | `line_to (Start.arrives_tangent())` | `continue_to Start` |
|---|---|---|---|
| `at (0,0), line_to (0.01,0), line_to (0.01,0.01), line_to (0,0)` | `SeamTangent { margin: 0 }` | `SeamArrivalLeverTooShort { arm: 0 }` | `NonpositiveLeg { length: 0 }` |
| `at (0,0), toward (1,0), line 0.01`, then `turn π/2, line 0.01` three times | `SeamTangent { margin: 1.7e-18 }` | `SeamArrivalLeverTooShort { arm: 2.5e-18 }` | `NonpositiveLeg { length: 1.7e-18 }` |
| the square by `line_to`, its last target `(5.5e-17, 0)` | `SeamTangent { margin: -6.7e-33 }` | — | — |

So the first refusal names a cause (a tangent seam) and a recourse that are not what stops the close. What does is that the tip already sits on the start: the author's last leg is the close, and has to be spelled as one.

## Why the viewer cares

The profile editor's preview says a geometry-refused close in the driver's own words (`viewer::sketch::PreviewError::Geometry`, AUTH-13). A chain whose last leg lands on the start but cannot be re-spelled as the close reads this sentence under its step list. That is a `line` by length, a far-end anchor (no `Start` form), an arc spec with no target, or a `line_to` a hair off the start. Nor can the viewer draw that leg: no close replays from a tip on the start, and `replay` exposes no tip of a chain that does not close. The drawing half is `work/author/a-last-leg-no-close-can-follow-is-dropped`.

## Shape of a fix

Refuse a closing leg of no length as that, with a recourse that names the respelling: target `Start` from the last leg itself. A typed refusal the viewer can read as "the tip is on the start" also answers the drawing half, at least for a straight last leg: the prefix before that leg, closed, is the same segment.

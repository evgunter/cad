---
id: a-revolve-traced-tangent-has-no-axis-read-off-the-face
kind: issue
title: The trace statement's revolve arm has nothing to read: a revolve's axis is typed in the profile plane, and a sphere's centre needs a kind the door does not hold
status: open
opened: 2026-10-30
priority: P2
cost: M
blocked_on: [coaxial-unions-with-a-torus-or-a-sphere-and-a-tangent-cone-refuse]
---

Found while building INTENT stage 6 D (`work/intent/the-door-proves-a-constructed-tangency.md`). D builds the curve tier of `[coincidence.structural]`'s trace statement for an extrude. Ev's motivating scenes are a cone revolved tangent to a sphere or to either branch of a torus, and their revolve arm cannot be built as ratified. Two reads are missing, and the GERM row this is blocked on keeps either scene from building today.

1. **The revolve's axis is never one construction with the face's.** `Node::Revolve`'s axis is typed in the profile's own plane (`axis_through`, `axis_heading`, `editor-core/src/node.rs`), and its lift is the node's `axis` output. Nothing reads it off the traced face. So rung one's pose row ("the sweep's axis is one construction with F's axis", PR #6006) can hold only through a mate, which relates two spaces, while the traced profile and its face share one. A revolve whose axis lies on the frame's `x` at `[0, 0]`, heading `0`, coincides with the face's axis by typed constants. Whether those constants count as construction is a design question.
2. **"Passes through F's centre" needs F to be a sphere.** The kernel holds `Subgroup` per pose kind (`VarKind::symmetry`), and rung one reads no carrier kind. D reads a face's symmetry off its own construction instead (`coincide.rs` `symmetry`): an extrude's wall is cylindrical about any axis it has, and a revolve's band is revolute about its `axis` output. A `Point` read off a face is a sphere's centre, a torus's centre or a cone's apex, and only the sphere's symmetry turns about every line through it. A revolve band is a sphere only by value (an arc centred on the axis).

Done when a cone drawn on a sphere's (or torus's) trace and revolved reports its `SurfaceTangent` row `Structural`, with a typed or valued axis staying unproven. This needs the boolean arms (GERM) and a design answer to (1) and (2), which goes to Ev as an `[ev]` PR.


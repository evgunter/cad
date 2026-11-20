---
id: a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face
kind: issue
title: A cylinder, cone, sphere or torus is stored by an axis point, apex or centre that may lie far from the face it carries, the conic's far-centre defect one level up
status: open
opened: 2026-11-20
priority: P3
cost: H
design: true
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Raised, independently, by both designers who weighed the conic's stored
form for PR 7194 (2026-11-20). By reading; nothing was run or measured.

`geom::Surface` (`crates/geom/src/surfaces.rs`) stores each quadric by a
datum on its axis: `Cylinder { origin, axis, radius, u_ref }` with
`S(u, v) = origin + radial(u)·radius + axis·v`, and the cone, sphere and
torus likewise by an apex or a centre. A face of a large-radius
quadric lies near the model while that datum lies a radius away, so a
point of the face is summed from terms as long as the radius and
carries about `u·radius` of rounding, however small the face. That is
the "stored form that loses digits" of `crates/geom-core/README.md`,
"Float rounding", which names the conic's centre form as its instance.

The conic's instance is measured and has its answer in flight
(`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`;
PR 7194 for the form). The surface's is neither measured nor
surveyed:

- which verdicts read a quadric's point or foot at `u·radius` and
  decide on it (the conic's phase-1 probe found its wrong answers only
  at ε = 1e-12 from a far datum of 1e5, and a wall whose arc edge
  refuses to certify from 1e7 at ε = 1e-9; a surface's thresholds are
  unknown);
- which mints hand a quadric a far datum (a large-radius fillet wall, a
  shallow revolved face, an imported cylinder of radius past the
  session box);
- whether the answer is the conic's (a form anchored on the face, its
  axis datum derived) or something the surface's readers need
  differently (the chart, the closed forms of the intersection table
  and `props` all read the axis frame).

`design: true`: several stored forms are viable and the choice binds
every reader of a quadric. Weigh it after PR 7194 is answered, since
the conic's answer is the nearest precedent. Before any designer is
asked, a probe owes the measurement the conic had: where a far-datum
quadric first reads wrong, at which ε, and through which door.

P3 by `work/README.md` Priority (latent unsoundness, on the conic's
measured precedent); the probe may raise it.

**Where to look first (the FULL review of PR 7146, 2026-11-20, likely).**
The same cancellation, two terms of a far datum's size summing to a
small one, was found in three places by that unit: the split's section
area (fixed there), `geom_brep::extent`'s fold
(`the-extent-folds-sagitta-is-read-as-r-times-one-less-cos`) and
`props::curved`'s cylinder face
(`work/boxfinish/a-cylinder-faces-flux-sums-two-terms-of-its-far-axis-origins-size.md`).
The reviewer names the cone, sphere and torus lanes of
`props/curved.rs` and `props/quad/rotational` as the next to read.

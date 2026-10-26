---
id: a-recorded-skip-scaffold-rests-declared-by-its-own-pushforward
kind: issue
title: A scaffold left at rest in a recorded merge skip is restated Declared by its own pushforward, which may be a kernel-minted one nothing declared
status: open
opened: 2026-10-13
priority: P3
cost: M
---


## Finding

Raised by PR 4856's review (NOTE-6). Measured in the PR's fix pass.

`boolean::ops::describe_edges`' stale arm restates a stale edge with
`EdgeCurve::restated_in_chart`, which carries the edge's authority
record as it was.

A `Scaffold` description reads as `EdgeAuthority::Declared(mc)`
(`geom_brep::description::authority_of`). So a scaffold left at rest
between the faces of a recorded merge skip comes out `Declared` by its
own pushforward.

Measured in the scene below, the half rod `y ≥ 0` on `z ∈ [0, 2]` united
with a coaxial rod of the same radius on `z ∈ [1, 3]`:
- Order half ∪ rod, with the rod not turned (its seam rulings at
  `x = ±1`, on the rulings): two stale line edges reach the arm with a
  `Scaffold` description and `is_declared() == true`. They are the
  rulings at `x = ±1` over `z ∈ [1, 2]`, now lying between the two walls.
- The same order with the rod turned 1.3 rad (the pin's pose): the same
  two rulings arrive as derived `Intersection`s. So whether a scaffold
  is involved depends on the seams coinciding with the rulings.
- Order rod ∪ half: the only stale edge is the rod's rim circle, and it
  is derived.

## The question

Who minted those scaffolds?
- If a sketch entity declared them (an extrude's profile vertex), the
  declaration is real and carrying it is right.
- If a kernel step minted them (a boolean's chord, a split's scaffold),
  the "made-up `Declared`" defect PR 4856 removed from the circle and
  line lanes survives on this path. That matters because `replace_face`
  refuses a declared record under a non-rigid offset, and tier 3's
  `TransverseNotIntrinsic` reads `is_declared`.

Measure which producers apply. Then decide whether a kernel-minted
scaffold should rest `Derived`.

## Slate triage (2026-10-26, CLEAVE)

- Band P3: a kernel-minted scaffold may carry a made-up `Declared` authority: latent unsoundness, unreached by any known input, P3 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

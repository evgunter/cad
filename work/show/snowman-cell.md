---
id: snowman-cell
kind: unit
title: a new montage cell: two coaxial balls under union, subtract and intersect, and the union's waist rolled into a torus band
status: review
opened: 2026-10-02
priority: P3
cost: M
pr: 3787
---

## What

The one NEW montage cell SHOW adds. REACH PR #3659 landed the
circle×sphere and line×sphere root lanes and the sphere-pair join on
the radical plane, so two coaxial balls of revolution now build under
all three booleans, and `BlendArm::SphereSphereTorus` rolls the
union's waist into an exact torus band. Nothing in the tour shows it;
lily pins only the TILTED pair, as a refusal.

One cell, four bodies side by side under one camera:

- `A ∪ B` — the snowman — with its waist filleted (the hero body);
- `A ∖ B` — a ball with a spherical bite;
- `A ∩ B` — the lens;
- optionally the unfilleted union, if the fillet's band reads better
  against it than alone (the scene's call, measured on the render).

## Oracle

`crates/sweep/tests/snowman.rs` is the kernel's own evidence and has
the closed forms: cap and lens volumes for the three booleans
(`the_snowman_builds_under_every_boolean`), and the torus band's spine
and radii for the fillet (`the_snowman_waist_fillets`). Build the
scene through the public doors the way a user would (two `revolve`s of
semicircles, `union`/`subtract`/`intersect`, `fillet_edges` on the
waist circle selected by `select_where`), and assert the same closed
forms. Volume after the fillet: the band's ΔV by Pappus on the
fillet's meridian section.

## Constraints known going in

- Only the coaxial, coplanar-seam pose builds; a ball spun off the
  shared axis refuses (`reach/tilted-sphere-pair-section-refuses-at-the-polar-gate`).
  Do not pose around that: author the natural snowman (coaxial is what a
  snowman is), and say in the narration that the tilted pair is lily's
  wall 7.
- Pick radii a person would pick (a bigger bottom ball), not ones
  chosen for dyadic oracles; state where an oracle is closed-form
  rather than bit-exact.
- New cell ⇒ a `demos/manifest.py` entry (CIW's file: announce the
  crossing) and a stops-table row in `demos/README.md`.

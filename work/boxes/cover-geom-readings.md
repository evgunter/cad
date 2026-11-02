---
id: cover-geom-readings
kind: unit
title: geom::cover: the Cover value, Uncovered, meet and hull on the Result, its methods, and geom's readings of a bounded carrier whole and over a span; the whole-carrier doors read it
status: open
opened: 2026-11-02
priority: P1
cost: H
---


## The build this unit opens

`docs/DESIGN.md` `[one-way.places]` ("The region a carrier, an edge or
a face can occupy") and `[one-way.readings]` (the paragraph "The region
an entity can occupy is a reading"), ratified by Ev on PR 6334
(2026-11-02): *"we can accept this for now and if it needs to be
revised because of issues encountered when implementing then that's
ok"*. So a unit that hits a real problem with the text reports it as a
finding, with the evidence, and proposes a revision. It does not work
around the text. The survey of today's doors is in `log.md`
(2026-11-02, "the extent survey"); the designers' notes are under
"the extent-home fork goes to Ev". Units, in order:
`cover-geom-readings` → (`cover-edge-doors`, `cover-topo-faces`) →
(`cover-levers`, `cover-app-readers`) → `cover-kind-match-gate`.

## This unit

In `geom` (one carrier; no body):

- **`Cover<T>`**: a value over a closed set of shapes, each with a closed-form support:
  - a point set, a segment, a conic arc over its span;
  - an axial slab, a cone frustum;
  - a chart rectangle or latitude zone on a sphere or torus (`R = 0` gives the sphere's);
  - a whole ball, a tube, a control net;
  - meets and hulls of these.
  
  `Cover` has **no whole-space value**.
- **`Uncovered`** names the entity and what was not known, and combines on a meet of two (a list of (entity, what) pairs).
- **`meet` and `hull`** are defined on `Result<Cover<T>, Uncovered>`:
  - `Ok ∧ Err = Ok`: the meet's identity;
  - `Ok ∨ Err = Err`: the hull's absorber.
- **The methods, on `Cover` only.** No bound is a method on `Uncovered`.
  - `support(dir)`.
  - `box_in(frame)`: the support along ± each row. A frame type at `T` replaces `boxes::BoxFrame`.
  - `distance_range(point | line)`: the nearest and farthest distance.
  - `ball()`.
  - `aabb(pad)`, which takes the bracket once, on the answer. It is the only `Bounds` site. 
  
  Each piece charges its own rounding outward at its own scale, as `sphere_reach` does today, so a cover encloses at every scalar with no reader-side pad.
- **A parameter-window type**, data with no arithmetic, for chart rectangles and axial windows. It replaces what `TorusWindowPair`/`Span` carry as windows.
- **The readings.** These are the only kind matches:
  - `Surface::cover()`, or a free function: the whole bounded carrier (sphere ball, torus tube with the exact support `R·|e⊥| + r`, net), refusing `Uncovered` for an unbounded kind;
  - `Curve3`'s arc cover over `(t0, t1)`: segment, conic arc, spiric as tube ∧ period box (`geom::spiric_period_box` becomes internal), net.
  
  Sign handling (`|R|`, `|r|`) is written once, here.
- **Repoint the whole-carrier doors onto it:**
  - `topo::boolean::trace::carrier_extent`. Its plane and cone arms may encode a pose precondition rather than an extent; check.
  - `geom_brep::ssi::analytic_box`.
  - `geom_brep::ExtentBall::of_carrier` / `of_torus`, which become `of_cover` or go.
  - every literal torus ball `major_radius + minor_radius` that bounds a region. The survey lists them.
  
  `boxes::torus_extent`'s overreach (`torus-extent-overreaches-the-support-along-a-turned-direction`) goes with the tube's exact support.

Review tier: **dual**: this unit fixes the type every later unit builds
on, and it is hard to change once they do.

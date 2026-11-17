---
id: a-face-with-no-stored-image-refuses-differently-by-its-route
kind: issue
title: A curved face with no stored chart image refuses DomainUnread or a chord-pass UnsupportedCurve by which route reached the trimmed lane: two readers of image presence
status: open
opened: 2026-11-17
priority: P3
cost: E
---

## What

Two readers decide what a trimmed face with no stored chart image answers,
and which one reads first depends on the face's route:

- **The route's domain read**, for a face the doors decided no rectangle:
  `tessellate::route_of` reads the domain before the chord pass
  (`read_domain`, `crates/mesh/src/tessellate.rs:373`, called from the
  `Shape::NoRectangle` arm at `:328`), and the lift's `Gap` is the refusal:
  `DomainUnread`.
- **The chord pass**, for every other trimmed face (`Trim::Structural`: a
  ring, a non-iso image, a split straight side, a spline chart), whose
  domain is read at its turn after the chord pass: the chord pass's
  `Reads::ChartRows` arm (`crates/mesh/src/chords.rs:547`) finds no
  `body.pcurve(hek)` and refuses `UnsupportedCurve` ("trimmed analytic face
  half-edge carries no stored pcurve cache"), before the domain is read.

So the same fact of the body (no image stored) is two refusals. The four
unminted D9 witnesses answer `DomainUnread`; a ringed or split-side face
with no image would answer the chord pass's. Found by PR 7041's delta
re-review.

## The shape of a fix

One reader of image presence, ahead of both: the route's, so a face that
stores no image refuses `DomainUnread` whatever sent it to the trimmed lane,
and the chord pass's arm becomes the defect it then is.


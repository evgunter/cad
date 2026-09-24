---
id: refusal-messages-render-floats-through-f64-display
kind: issue
title: Refusal messages render f64 knots and weights through Display, so 1e308 prints as 309 digits
status: review
pr: 3178
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

`f64`'s `Display` prints a large-magnitude value in full positional
notation: `1e308` comes out as a 309-digit decimal. Every typed refusal
in the spline stack that names a knot, a weight or a parameter
interpolates it with a bare `{}`, so a refusal about a domain near the
`f64` ceiling renders as a wall of digits rather than as a number a
reader can see.

The class, as of this filing:

- `crates/geom-core/src/spline/algebra.rs`, `KnotAlgebraError`'s
  `Display`: `{u}` at four arms (parameter not strictly inside the
  domain; insertion exceeding the interior budget; not an interior
  knot; removal exceeding multiplicity).
- `crates/geom-core/src/spline/knots.rs`, `SplineError`'s `Display`:
  `{weight}` at `NonPositiveWeight` and `NonFiniteWeight`.
- `crates/geom/src/surfaces/nurbs.rs`, `KnotMirrorError`'s `Display`:
  `{lo}`, `{hi}`, `{knot}`, `{mirror_knot}` — `ReflectionNotFinite` on
  a `[1e308, 1.5e308]` domain is the worst case in the tree today,
  since that variant EXISTS to report a domain at the ceiling.

## Why it is filed rather than fixed

VREV's fix pass (PR 2627) was asked to make `KnotMirrorError` follow
the crate's convention for rendering floats in a refusal. Grepping the
two neighbouring error types found the convention IS the bare `{}`, so
matching it and fixing it are the same edit at three sites in two
crates — a class, not a variant. `KnotMirrorError` was left matching
its neighbours.

## What the fix looks like

One rendering helper in `geom-core` (`{:e}` above some magnitude, plain
below, so ordinary knots in `[0, 1]` read exactly as they do now) and
the three `Display` impls through it. The rows that assert on message
text (`KnotAlgebraError`'s and `KnotMirrorError`'s) move with it, and
the new spelling is what they then pin.

## Was

Filed by SCALAR's VREV fix pass (PR 2627).

## Resolved (PORT, `port/wrap-a`)

`geom_core::Readable` (`crates/geom-core/src/readable.rs`) is the one
rendering: positional on `1e-4 ≤ |x| < 1e16` and at zero — std's own
boundary for `f64`'s `Debug` — and scientific outside, both shortest
round-trip; formatter flags are not honoured. The whole is `{:?}` less a
trailing `.0`, so inside the band it reads exactly as the bare `{}` and
every ordinary knot, weight and length message is unchanged.
`every_binade_renders_short` holds every finite binade to 24 characters
and a round trip; `the_rendering_is_debug_without_its_integral_tail`
holds the identity.

Routed through it: the three `Display`s the row named; nine sibling
arms the sweep found in the spline stack — `SplineError::DomainInvalid`
(missing from the row's list; the variant whose width-overflow case is
exactly this magnitude), `spline::compose::ComposeError::DomainMismatch`,
`LsqError::LsqDegenerate`'s pivot, `geom::curves::compose::ComposeError`'s
`SeamGap`, `SeamNotTangent` and `DegenerateSpan`,
`FitError::InvalidTolerance`, and the curve and surface
`*ProjectionInconclusive` refusals (parameters AND residuals, so one
message carries one spelling); and `KnotMirrorError`'s evaluated
`lo + hi`. New pins: `1e308` in `insertion_refusals_are_typed`,
`[-1e308, 1e308]` in `on_domain_refusals_are_typed`,
`[1e308, 1.5e308]` in `a_domain_with_no_finite_reflection_refuses`. No
existing pinned string moved.

Two more homes of the same rendering: `viewer::props::render_number`
was a hand copy and now calls `Readable`; `quantity::fmt`'s
`render_shortest` stays a copy (neither crate depends on the other) and
each names the other. `Stackup::render`'s own floats (in `editor-core`,
touched for the sibling row) go through it too.

The rest of the class, outside the spline stack — and the existing
`{:e}` sites, a second spelling of the same concept — is filed as
`work/issues/refusal-floats-outside-the-spline-stack-render-through-f64-display`.

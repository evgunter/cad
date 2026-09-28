# CONTACT-4 — contfp reads every loop on its carriers

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-4.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/4-contfp-carriers`. Rows carried (read both in full):

- `work/contact/contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop` (P0);
- `work/contact/point-on-arc-endpoint-zone-compresses-by-sin-half-width` (P1).

Also read the ATREST note of 2026-09-28 on `work/contact/log.md`.

## What is wrong

`boolean::contain::contfp` answers a `LoopShape::ArcParity` loop from its
vertex polygon, which is not the loop's region. An outward arc makes
it answer `Out` for points that are in. The general walk now exists:
`splitting::containment::point_in_carrier_loop` (ATREST-9, PR #3204)
reads each edge on its carrier and has been measured at about 43k
adversarial probes with zero wrong answers.

The second row is `contfp`'s boundary pre-pass. It asks
`point_on_arc`, whose endpoint zone compresses by `sin(w/2)`, so a
point on a short or near-full arc can read as off the boundary. The
carrier walk has its own boundary pre-pass, which asks an arc its own
carrier and window. Switching the walk is likely to retire the second
row. Measure whether it does; do not assume it.

## Settled design

**S1. One walk.** `contfp` reads its outer loop and rings through
`point_in_carrier_loop`, together with that walk's boundary pre-pass.
- Its `None` (a spiric or spline edge that could matter) becomes a
  typed refusal, in the same shape `contfp`'s callers already handle
  (`ContainError`).
- Decide whether `LoopShape::Polygon` and `LoopShape::Disc` still need
  their own arms. The carrier walk is exact on both. Keep an arm only
  if it earns its place: measure that it is faster where that matters,
  or show that it answers something the walk refuses. Otherwise
  delete it.
- If `point_on_arc` and `boundary_pre_pass`'s `Chord` arm lose every
  caller, delete them.

**S2. `LoopShape`'s other consumer is ATREST's.** Tier 3's check 9
gates its nesting arm on `LoopShape`, and ATREST-12 (specced) moves
check 9 onto the carrier walk. Do not edit `validate.rs`. If `LoopShape`
keeps consumers only in check 9 after your change, leave it and say so
in the PR; ATREST retires it. Announce the seam on `work/atrest/log.md`.

**S3. Every caller's contract holds.** For each caller, check that the
new walk's refusal and escalation shapes map to what that caller does
today with `ContainError`, and state the mapping in the PR:
- `reduce.rs` (three sites)
- `ops.rs`
- `census.rs`
- `chart_region.rs`

Where a caller used to get an answer and now gets a refusal, list it
with its reason. Where a caller used to get a WRONG answer and now gets
the right one, list that too, with a row.

## Rows

- The bored D-rod transverse cap's lune point through `contfp`: `Out`
  on the base, `In` at the head. If the D-rod reaches `contfp` through
  a public door (a boolean or the census), add that end-to-end row as
  well.
- `point_on_arc`'s compressed zone: a point an arc length `s` past the
  end of a short arc (w = 0.02 rad) with `ε < s < ε / sin(w/2)`, and
  the near-full-arc mirror. Each must read as off the boundary where it
  is off, and ON where it is on.
- A slot and a rounded rectangle, the shapes `ArcParity` was measured
  on, must answer as before.

## Review

**Single full review.** The walk itself is well measured. The risk is
in the swap under five live callers.

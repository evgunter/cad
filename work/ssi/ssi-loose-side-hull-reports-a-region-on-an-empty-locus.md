---
id: ssi-loose-side-hull-reports-a-region-on-an-empty-locus
kind: issue
title: a side whose Bernstein hull straddles zero though phi along it does not reads in band, not clear: the search reports a Side region on an empty locus
status: closed
opened: 2026-10-04
closed: 2026-10-06
branch: ssi/loose-side-hull
priority: P2
cost: M
refs: [limb3-at-rest-proves-the-graph-not-the-arc]
---


## Found (PR 4012's delta review, probe `d_loose_phantom` on `analysis/limb3-rest-review/delta`; filed 2026-10-04)

A wall `z = k·x + h(y)` over `[0, 1]²`, `h` quadratic over `m` C0
spans with Bernstein coefficients `(P, −N, P)` on each, `P > N > 0`.
Along the side `x = 0`, `φ = h ≥ (P − N)/2 > 0`, and the wall rises
inward, so the exact locus over the wall is empty: #3862's rule reads
this side clear. But every piece's hull holds `−N`, so no piece of the
side is certified one-signed:

- `boundary_section` (`section.rs`) takes `side_of_plane` from
  `Pieces::distance`, the hull of the whole side's Bernstein ratios,
  which straddles zero; `Pass::side_region` (`boundary.rs`) then never
  reads `clears` true, and the side's cover is within ε, so the search
  reports a `Side` region (reach ≈ 1.6ε) on an empty locus.
- At rest, `read_stretch` cuts the stretch into 64 pieces, each still
  a hull over whole Bernstein spans when `m ≥ 64`, so no piece clears
  either, and limb 3's side arm would accept the side. Only limb 2
  (`HullSup`) refuses a carrier along it today.

Measured at the head of PR 4012, ε = 1e-9, `P = 0.6ε, N = 0.2ε` (and
`0.9ε, 0.5ε`), `k ∈ {10, 100}`, `m ∈ {64, 256, 1024}`: the search
answers 0 branches and one `Side { fixed: U, end: Low }` region at
every row; the declared carrier `(0,0,0) → (0,1,0)` refuses
`Limb { HullSup }`.

## Whether it matters

Unsure. Under D4 the plane lies within ε of that side, so a `Side`
region there is in band; what is lost is #3862's exact empty answer,
for an input whose own net hides it. The doors still agree on
certifying nothing.

## Repair shape

Decide a side's sign on a refined hull: subdivide a piece whose hull
straddles zero (or degree-elevate) before reading it unsigned, in the
one reader both doors share (`section.rs`'s `Pieces`), so the clear
test and the side arm move together.

## Closed (2026-10-06)

A side's sign is read on a refined hull in `section.rs`'s one reader.
`refined_sign` reads each piece's `h` coefficients (the weights are
positive, so `φ` shares `h`'s sign) and halves (`sub_piece`) every
piece whose hull straddles zero. It gives a sign where every piece reads
certified one-signed, all of one sign, and reads the side in band (no
sign, not clear) at a refused hull, two pieces of opposite sign (the
pieces as given are all read first, so order does not matter), or a
straddling piece whose halves' hull is no narrower than its own: the
arithmetic's floor at that piece, with no tuned constant. Only halvings
spend the budget, `SIDE_SIGN_HALVINGS` per side read: the whole side at
the search, all 64 pieces of a stretch together at limb 3. Past it the
search refuses `SsiError::SideSignBudget` naming the side, and limb 3
reads the stretch `Refused`. `boundary_section`'s `side_of_plane` and
`read_stretch`'s per-piece sign both read it, so the clear test moves at
both doors together.

Measured reads (the PR 4104 reviews' probes): the phantom takes 3 per
span (one halving, both halves one-signed). A transversal crossing stops
in a few reads. A tangential touch takes up to about 750 reads, bounded
by underflow: an exact double root at a non-dyadic point halves exactly
for about 250 levels. Planted zeros of multiplicity 4 take up to about
1500. In exact arithmetic the stop fires only where a piece's hull ends
are its end coefficients, `φ`'s values at the piece's ends, of opposite
sign or touching zero. Under intervals it fires where an end
coefficient's enclosure touches or straddles zero, which costs only
rounding-level precision, and never loses a sign the unrefined hull
gave.

Measured on the reported grid (P/N ∈ {0.6ε/0.2ε, 0.9ε/0.5ε},
k ∈ {10, 100}, m ∈ {64, 256, 1024}) at ε 1e-6, 1e-9 and 1e-12: the search
answers no branch and no region (before: one `Side { U, Low }` region),
and a window of limb 3's on the side reads `Clear` and holds no
side's piece (before: the side arm took it). The declared carrier still
refuses `Limb { HullSup }` first at the public door. Rows:
`a_side_whose_hull_is_loose_reads_clear_at_both_doors` (one row per PR,
with a control whose spans each hold two zeros of `φ`, still a `Side`
region) and `…_over_the_grid` (the slow set),
`a_window_on_a_side_whose_hull_is_loose_holds_no_sides_piece`,
`a_stretch_past_its_halvings_reads_refused`, and `section.rs`'s
`refined_sign` and `section_within` rows.

The trough rows of
`ssi-a-corner-on-a-side-the-plane-runs-along-refuses-as-a-graze` answer
as before at all three ε: the graze is on a side whose hull is already
tight.

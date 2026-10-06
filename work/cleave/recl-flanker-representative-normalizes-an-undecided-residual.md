---
id: recl-flanker-representative-normalizes-an-undecided-residual
kind: issue
title: recl's flanker representative normalizes an undecided residual against the common line
status: dispatched
opened: 2026-10-01
priority: P2
cost: M
branch: cleave/recl-flanker
---


## What

`crates/topo/src/boolean/recl.rs` `resolve_edge_edge` builds each
flanker's representative as `((v - axis * v.dot(axis)).normalize(),
reach)` — a hand Gram–Schmidt residual against the common line, with
its length neither decided nor refused. `axis` itself is
`a_sectors[fa_s].start.normalize()`, also undecided.

The doc calls the representative the flanking sector's NONCOPLANAR
bound, which would make the residual nonzero if that bound were
decided off the line somewhere upstream; this filing did not find
where that is decided, and did not build a fixture for it. A bound
within the band of the common line normalizes to a direction made of
rounding, and the membership test then reads a definite side off it.

## Shape

`geom_core::OrthoFrame::from_aim_and_reference` (or `UnitVec3::new`
on the residual, under a `bool_*` K name) decides the length and
refuses typed. Measure first: whether a body reaches a bound inside
the band of the line through the public boolean doors.

Found by the `linalg/decided-not-minted` sweep for the hand
Gram–Schmidt shape.

## Built (branch cleave/recl-flanker)

**Measured first.** The residual's length, the common line's norm and
the site's arm were logged at every `resolve_edge_edge` call over the
`topo`, `sweep` and `editor-core` suites (7444 tests; 29,744 flanker
readings from 210 tests, at `f64`, `Interval`, `Dual` and the symbolic
scalar). No body reaches a flanking bound near the common line through
a public door: the smallest residual was 0.148 (about 8.5°), the
smallest `|residual| × arm` 1.6e7 band zeros, and the common line's
norm never left 1 by more than 2.2e-16. The only readings near the
band were the in-crate fixtures that hand `resolve_edge_edge` an
in-band arm (`offer_rows`' short-arm flanks, the membership-tie test),
and there the residual was 1: the arm, not the residual, was what sat
in band.

**Not decided upstream.** The A flankers' residual is `sector_reflex`'s
sine for an unsplit sector, but levered at that sector's own arm, and
for a split one the half-angle, `≥ |sin θ| / 2`; the B flankers' is
read against A's line, which `bool_ee_collinear` grouped with B's own
only to within the band — so no upstream decision covers it at the
site, and a proof-at-site would have been false.

**Landed.** `flank_rep` decides each residual's length times its
bound's own reach (`bool_flank_offset`, `decide_positive`) — the
bound's far end's distance off the line, in metres, where `side_code`
reads the bound — and refuses an in-band or decided-zero length as
`Coincidence(Sectors, Moot)` with its decided margin, as
`wedge_is_reflex` does. The common line's direction is minted through
`UnitVec3::levered` at its bound's reach (`bool_flank_axis`); every
refusal there is a `ClassificationInvariant`, since `sector_shape`
built that bound from a chord its arm rung decided long. Both names are
rostered (`docs/K-REPORT.md`, `docs/predicate-dimension-audit.md`).

---
id: cone-chart-nappe-zero-says-the-plane-meets-only-the-apex
kind: issue
title: geom-brep: pcurve_cone_chart_nappe's Zero refusal says the apex plane meets the cone only at the apex, and prints no margin
status: closed
closed: 2026-10-20
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from PR 4976's review of its sweep's blind spot: a Zero verdict carried as a bare `Sign::Zero`, whose refusal states the zero as an exact fact.)

A `Sign::Zero` from `decide` is a band-decided reading — an exact zero, a straddling enclosure, or a nonzero size inside the zero band — not a proof of equality. The refusals below state it as exact, hedge nothing, and print no margin (D4 ¶1 (i): the margin rides the payload). geom-brep's Zero-arm payloads now say "… is not shown … (margin … m)".

## What

`crates/geom-brep/src/pcurve_cache.rs`, the `decide("pcurve_cone_chart_nappe", Margin::of(h), band)` call (the rim's height above the apex), `Sign::Zero` arm: "the apex plane meets the cone only at the apex, so no rim lies there". A rim at a height inside the zero band is a real circle of sub-tolerance radius, not the apex plane. `work/predtail/cone-nappe-is-decided-in-five-places.md` covers where the nappe is decided, not this wording.

## Repair shape

Say the rim's height "is not shown away from the apex (margin … m)", carrying the margin into `off_chart`'s text.

## Fixed (PCERT E1, `pcert/lane-choices`)

`PcurveCertifyError::CarrierOffChart` carries an optional `margin`
(`OffMargin`: `Past` for a reading decided past the degenerate pose,
`InBand` for a band-decided Zero), printed as "(margin … m)", with
"within the band" for `InBand`, whose Display does not call the edge
off its face. Every `pcurve_cone_chart_nappe` Zero that can still be
reached now says the reading "is not shown" rather than stating the
zero as exact. By arm (`crates/geom-brep/src/pcurve_cache.rs`):

- **The rim** (`derive_class`, the cone's circle arm) reads the nappe
  through `decide_reported`, and its Zero quotes the margin as
  `OffMargin::InBand`.
- **`cone_conic_incidence`** reads the conic's lowest point through
  `decide_reported`. It splits a definite `Negative` ("reaches past the
  apex's height", `OffMargin::Past`) from the not-shown `Zero`
  (`OffMargin::InBand`).
- **The section ellipse** reads its centre's height with `decide`, and
  its Zero and in-band arms are `unreachable!`. `incidence::incident`
  runs `cone_conic_incidence` first, which decides the ellipse's lowest
  point past the band off the apex's height, so the centre's height is
  definite by then.
- **The line**, on the ruling's direction gate, where the anchor's
  height is Zero, reads through `decide_flagged_reported`. Its Zero
  ("the line is not shown off the plane through the apex") quotes the
  margin as `OffMargin::InBand`. It is reachable only on a sub-band
  span, because the line's incidence test runs first.

Pinned by `chart_incidence::a_rim_at_the_apex_height_refuses_as_not_shown_with_its_margin`
and by `pcurve_cache`'s
`an_apex_plane_line_on_a_sub_band_span_is_not_called_off_its_face`,
both at `f64` and at `Interval`.

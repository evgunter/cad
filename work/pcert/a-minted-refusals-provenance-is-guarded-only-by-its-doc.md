---
id: a-minted-refusals-provenance-is-guarded-only-by-its-doc
kind: issue
title: The lane relabel's mint provenance is guarded only by minted_refusal's doc: a caller that hands it a stored row ends a wrong row 'not yet'
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Filed by PCERT unit E4 (`pcert/circle-general-lane`, PR 6485), from
the delta verifier's fix list X (X2, Q7).

## What

The projected lane relabels its refusals ("not yet", `LaneGap::Pose`
or `LaneGap::Envelope`) only when the image was just derived for the
edge. A stored row's envelope refusal says the row is wrong, and has
to keep its own term (`a_stored_rows_envelope_refusal_keeps_its_term`,
`a_foreign_stored_row_never_ends_not_yet`). One home does the
relabel: `general_lane_refusal`
(`crates/geom-brep/src/pcurve_cache.rs`). Two places reach it:
`chart_image`, for the projection's own refusals, and the `pub`
`geom_brep::minted_refusal`, for the certificate's refusal of the
mint's fresh image.

Nothing but `minted_refusal`'s doc says the image must be fresh. Its
callers are topo's mint walk (`certify_walked`,
`crates/topo/src/pcurves.rs:3219`; `read_loop`'s mint arm,
`pcurves.rs:5085`) and `tests/pcurve_villarceau.rs`. The next caller
that hands it a stored row would end a wrong row "not yet", a lever
offer, rather than as the contradiction it is. That repeats E4's M1
from the type's side.

## Why not fixed in E4

A provenance type (`chart_pcurve_over` returning a `Fresh`/`Minted`
wrapper that `minted_refusal` takes) does not stay local. The walk
moves the minted image before certifying it (`shift_v_branch`, the
sphere's twin), and `Walked::pcurve` also holds images the walk builds
itself (an `IsoLine` at the seam). So the wrapper would have to thread
through every `Walked` construction in `topo::pcurves`, and through
the branch shifts that keep it fresh, which is wider than E4.

## Fix direction

Carry provenance on the walked row (`Walked::fresh`, or a
`Walked<Minted>` / `Walked<Stored>` split) and have `minted_refusal`
take that row, not a bare `Pcurve`. That way a stored row cannot reach
the relabel without being constructed as minted.

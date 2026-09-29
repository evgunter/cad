---
id: surface-field-walks-and-source-theorem-checks-have-no-one-home
kind: issue
title: four field-by-field surface walks and four checks of 'same source => same bits', with no shared spelling
status: closed
opened: 2026-09-29
priority: P1
cost: M
refs: [set-surface-source-stamps-a-recipe-without-checking-the-descriptions-agree, three-spellings-of-one-chart-answer-the-same-question-differently]
branch: origin/one-surface-walk
closed: 2026-09-29
pr: 3429
---


Found by the review of PR 3414 (the row doors reading identity), as a
class; filed at adjudication.

**Four walks of one shape.** Each of these compares two `Surface`
values kind by kind, field by field, in the same field order, and
differs only in its comparator:

- `source.rs` `surface_bits_witness` / `analytic_scalars` (PR 3414,
  bit channel, debug-assertion only);
- `chart_region.rs` `surface_bits_equal` (exact bracket, production);
- `source.rs` `plane_bits_witness`, which omits `u_ref` where the
  others read it;
- the plane compare behind `merge_faces.rs`'s declared rung and
  `boolean/plane_eq.rs`.

No text at any site names the others, so a new surface kind or field
has to be added four times by hand, and one of the four has already
drifted (`u_ref`).

**Four checks of one theorem.** "Same `GeomSource` ⇒ bit-identical
descriptions" (N6) is checked at the stamp door (PR 3414's debug
assertion), and read-side at `chart_region.rs`'s source rung, the
merge door and `plane_eq.rs`. `chart_region.rs`'s comment still
describes `set_surface_source` as a door any caller can attach an
unchecked claim through, and nothing records how the write-side and
read-side checks relate.

**What a taker decides.** One field walk parameterised by its
comparator (the shape is open: a visitor over `Surface`'s scalars, or
`Surface` exposing its scalars once), and where the theorem is
enforced once rather than re-checked per reader. The second half
depends on Ev's answer on PR 3410 (a production comparator, or not).

## After PR 3410's ruling (2026-09-29)

No production comparator: the "theorem enforced once" half is answered
as assertion-side only — the four checks become one assertion home
plus the readers that need a verdict (chart-region's bracketed read).
The field-walk half is unchanged: one walk parameterised by its
comparator.

## Closed (2026-09-29, PR 3429)

One walk: `Surface::data()` returns `SurfaceData { Analytic(AnalyticData),
Nurbs(&Arc), Approx(&Arc) }`, the analytic fields destructured without
`..` in one place (`crates/geom/src/surfaces.rs`), and every reader is a
fold — the debug bit witnesses (`source.rs`), chart-region's bracketed
read, `validate::poisoned_datums`, the mesh memo key and step-import's
dedup signature (both byte streams proven identical). The drifted
`plane_bits_witness` (it skipped `u_ref`) is deleted. Rows go red when
any reader skips a scalar (topo and mesh). Theorem checks: the stamp
door is the one assertion home; the merge door and `plane_eq` rung 1
keep theirs because they compare pairs the stamp door never sees
(grafted stamps; mirrored and cross-body planes) — a disclosed, argued
departure from this row's "one assertion home", accepted.

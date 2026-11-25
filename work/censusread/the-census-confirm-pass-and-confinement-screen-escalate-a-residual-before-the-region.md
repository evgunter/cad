---
id: the-census-confirm-pass-and-confinement-screen-escalate-a-residual-before-the-region
kind: issue
title: The census's record confirm pass and its crossing rung's confinement screen escalate a plane residual or carrier gap before reading the face's region, where both readings would refuse the same way
status: open
opened: 2026-10-17
priority: P2
cost: E
---


## What

Found by the edge-face lane's sweep (PR for
`the-census-edge-face-lane-escalates-a-plane-residual-at-an-end-far-outside-the-face`).
That PR moved passes 3 and 4 onto "escalate a residual only where the
readings it could take disagree". Four more sites in
`crates/topo/src/census.rs` decide a plane residual or a carrier gap and
escalate it before reading the face's region. In each one, every reading
of the residual refuses the same way wherever the region read would say
`Out`:

- `confirm_edge_face` (`pm_census_confirm_ef`): an end's residual
  escalates and returns. Read `Zero`, the cells come next, and none
  means stale. Read off, it is stale too.
- `confirm_vertex_on_face`, through `face_holds` →
  `boolean::face_placement` (`pm_census_confirm_vf`): the on-plane
  decide escalates before the trim. `Out` there is stale either way.
- `pair_holds_point` (`pm_census_confined_carrier`, the crossing
  rung's confinement screen, under `ee_cross_backed`): the residual
  escalates before `contain`. `Out` there means "not confined" either
  way, and the crossing then stays an unbacked finding.
  `pair_holds_edges` reads eight residuals with no region at all, so
  only the first refusal's typing is at stake there.
- `census/point.rs` (`pm_census_point_gap`, `pm_census_point_plane`,
  the curved point-touch confirm): the carriers' gap escalates before
  `in_face` reads the trims.

In 3′ each site refuses the body either way, as an escalation or as a
`StaleContactDeclaration` / unbacked finding, and the escalation tells
the reader "undecided, try another tolerance" where the region read
says the record is stale or the crossing unbacked. Through a carry the
difference is behaviour, not typing (below).

## The shape to give

These sites need a disagreement test, not `census::read_undecided`.
`read_undecided` escalates where ANY reading finds something. That is
"the readings differ" only where the margin's off reading finds
nothing. Here the off reading finds something itself: a stale record,
or an unbacked crossing. So `read_undecided` would keep every
escalation.

What is owed: decide the margin, and when it escalates, run each
reading's branch (the off reading's included). Escalate only where the
branches refuse differently: say stale under one and confirmed under
another, or unbacked under one and backed under another. Where every
reading refuses the same way (stale, or unbacked), push that refusal
instead of the escalation. A sibling of `read_undecided` comparing the
branches' answers is the natural home. Carry the escalations the
branches met as `census::Branch::carries` does.

## The carry makes the typing a behaviour (2026-10-19)

From `contacthold/ff-divided-reads-stale` (PR 5390). When an op divides
a recorded cell, `boolean::records::landings` confirms the row on each
pair of the cells' images through `census::RecordConfirm::meets`: a
stale pair is passed over and the row lands on the pair that holds the
touch, while any other finding refuses the op
`CarryRefused { Found, Unread }`. So a piece pair these sites escalate,
where the region read says `Out`, refuses an op that should land the
record on another piece. The sense-first sibling in `confirm_patch` is
its own row:
`a-rest-patch-records-door-one-contradicts-a-piece-its-door-two-would-read-stale`.

Moved from RESTREAD to CONTACTHOLD on 2026-11-20, by `git mv`, when RESTREAD merged into CONTACTHOLD: RESTREAD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

## Merged in: `census-confinement-screens-keep-a-first-face-escalation-the-second-face-would-decide-off` (2026-11-25)

Folded in at the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`): the same mechanism, fixed by the same change. Its title was: The census confinement screens stop at the first face's undecided residual and keep its escalation where the second face would decide the point off. Its body follows, headings demoted.

Found by the §5 sweep of CONTACTHOLD's one-tangent-interiors-reading
lane (loops over two cells that stop at the first non-answer, pattern
`for (face|f|cell|c) in \[[a-z_]+, [a-z_]+\]` over `crates/topo/src`).

`census::pair_holds_point` and `census::pair_holds_edges`
(`census.rs`) loop `for f in [fa, fb]` and return `false` at the first
residual `signed_is_zero` does not read zero. Where `fa`'s residual is
in band, its escalation is pushed and the loop stops, so `fb`, which
may decide the point definitely off its plane, is never read: the
screen's answer is `false` either way, but the escalation stays in the
report where a decided reading would have dropped it, and which face
is listed first decides whether it does.

Not measured on a fixture; read from the code. The rule is
`census::in_both`'s: read both faces, keep the escalations only
where neither decides off.

Moved from RESTREAD to CONTACTHOLD on 2026-11-20, by `git mv`, when RESTREAD merged into CONTACTHOLD: RESTREAD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

## Merged in: `census-vertex-cut-escalates-a-vertex-past-the-end-within-the-band` (2026-11-25)

Folded in at the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`): the same mechanism, fixed by the same change. Its title was: The edge-face vertex cut escalates a boundary vertex lying past the edge's end by less than the band where its distance to that end is undecided, though a vertex there can cut nothing. Its body follows, headings demoted.

### What

Found by review 2 of PR 5261 (its NOTE 8). `crates/topo/src/census.rs`
`ef_overlap_cells` cuts an edge at a face's boundary vertex through
`on_edge_interior_reading`, which is `geom_brep::curve_pair::Span::contact`.
`contact` reads the span first, through `place_at`. For a foot past the
edge's end, the chord from the foot to that end is decided
(`curve_pair_span_end_chord`, then `_side` / `_straddle`). Where the
vertex lies past the end by less than the band, and that chord's
reading is undecided, the escalation is pushed.

But a vertex there can cut nothing. Within the band of the end, it is
the end's own cut, and past it, it is off the segment. So the census
refuses a configuration that has one answer: a sub-band proxy, the
shape this PR removed at the band's scale.

### The shape to give

Read the place's escalation through `census::read_undecided`. Under
both readings, end and past, the cut is "none". If the vertex's gap off
the line is not `Zero` either, nothing else is at stake. The reading
belongs to `contact`'s callers, of which pass 2's (`pair_vertex_edge`)
has the same question. Measure first: the probe's default grid showed
no `curve_carrier_*` / `curve_pair_span_*` escalation from pass 4 at
ε = 1e-9, 1e-12 or 1e-6 after PR 5261, so this needs a constructed
witness.

Moved from RESTREAD to CONTACTHOLD on 2026-11-20, by `git mv`, when RESTREAD merged into CONTACTHOLD: RESTREAD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

Moved from `work/contacthold/` to `work/censusread/` on 2026-11-25, when CONTACTHOLD closed (`docs/doc-ledger/contacthold-leaves-the-tracker.md`); id unchanged. The cleanup re-read the row against main at `6003877ad` and found it live. Re-banded P3 → P2: it absorbed the P2 `census-confinement-screens-keep-a-first-face-escalation-the-second-face-would-decide-off`.

---
id: the-ladder-has-no-climb-for-a-disagreement-that-answers
kind: issue
title: The ladder has no climb for a disagreement that answers: two loops hand-roll it and refuse on a kept reading ahead of a decided disagreement
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Filed from PR 6750's review (F1), on the orchestrator's ruling.

## The shape

A loop reads one answer per rung, and **two decided rungs that differ are an answer of their own**. Any such pair answers: a theorem, whatever the other rungs read. Only where none differ does the agreement stand, and it needs every rung.

`geom_core::ladder` has no combinator for this:
- `climb_agreeing` treats a disagreement as `Err(Disagree)` and answers the decided agreement past a reading kept in band, without telling its caller one was kept;
- `climb(Any)` has no notion of two answers differing.

So both sites hand-roll it. Each refuses on the **first** reading in band, ahead of a decided disagreement later in the loop, through a `?` no register reads (the kept-reading-ahead-of-an-answer shape).

**The combinator wanted:** answer on the first decided pair that differs; else, if every rung is decided, the agreement; else undecided, with the kept evidence ranked as `Evidence` ranks it.

## The sites

| site | the disagreement's answer | in-band first |
|---|---|---|
| `topo::boolean::vtxfac::partner_side` (the bounds' side codes) | `PartnerRead::Across` | each bound's `datum.side(..)?` |
| `topo::boolean::rim_wedge` `FaceOnLocus::at` (the riding edges' signs) | `Cover::RidesBothWays` | each ride's `inside(..)? && inside(..)?` |

**Reach, measured** (2026-11-10). Probes, not committed, logged every call that refuses in band, and whether two decided rungs differ anywhere in it. They covered the topo, sweep and editor-core `ci` sets, at the default ε and at `1e-12`:
- **`partner_side`:** 6 calls per ε, all in the unit test `boolean::vtxfac::tests::a_partners_side_of_a_face_is_read_off_its_bounds`, none with two decided bounds on opposite sides.
- **`FaceOnLocus::at`:** 0 calls. A positive control saw 120 calls in topo alone, none in band.

So no in-suite outcome would move. The fixture for `partner_side` is sketched on `t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer`:
- rays `[[1, 0, h], [0, 1, 1], [-1, -1, -1]]` over the plane datum `z = 0`, with `h` mid-band;
- today the escalation, ported `Across`, in every rotation.

`FaceOnLocus::at`'s would be a locus point covered by three rides: the first in band of its span's end, the other two decided with opposite signs.

## Not this shape

`topo::boolean::contact_verify::tangent_pass`'s per-sample loop **ranks refusals**. A span bound left unsettled is the pass's answer only where every other reading decides, and an in-band bound stands before one beyond the band. Nothing there is a disagreement that answers. Listed so the combinator's reader does not port it.

## Done when

- The combinator exists in `geom_core::ladder`, with its own rows.
- Both sites climb it, each with a pin: a reading kept in band ahead of a decided disagreement, red under today's first-refusal loop.
- Their "deviation scheduled" comments go.

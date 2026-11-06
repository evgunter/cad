---
id: question-mark-ladders-the-gate-registered
kind: issue
title: Loops that return a band decision's escalation by ? ahead of a later answer, registered by escalation-question-marks.sh
status: open
opened: 2026-11-06
priority: P2
cost: H
---


Found by `scripts/gates/escalation-question-marks.sh`'s first register
(`onecopy/refusal-standing`, tranche 4; `edge_join` moved here by PR
6553's review). Each site below returns a band
decision's escalation by `?` inside a loop, ahead of a later reading
that could answer definitely. It is a hand-rolled search or identity,
the class of `an-escalation-returned-by-question-mark-ahead-of-a-later-answer`.

Each was dispositioned from its loop header and its hit line. **None is
measured yet.** Read each in full before porting it.

| site | shape | port |
|---|---|---|
| ~~`boolean::solid_contain::point_in_faces` (six on-face reads)~~ | **ported** (`onecopy/refusal-standing`, tranche 5): the pre-pass climbs its faces | — |
| `boolean::rim_wedge::departures` (`abut`) | either pair abutting answers | `read_identity`-shaped, `Any` |
| `boolean::rim_wedge::rides` | either end off the line answers "not" | `read_identity` |
| `splitting::containment::certify_plane` | any margin decided apart answers "off the plane" | `read_identity` |
| `boolean::sectors`' crescent search (two reads) | a crescent found answers | `climb`, `Any` |
| `splitting::join::conic_pairs` (the first cut) | which gap is first decided apart | `climb`, `First` |
| `shell`'s moved-walls overlap search | an overlap found answers | `climb`, `Any` |
| `boolean::surface_group::Rim::holds` | any coaxial margin decided apart answers "no" | `read_identity` |
| `boolean::reduce`'s line-cone apex end search | an end at the apex answers | `climb`, `First` |
| `ring_path`'s lead-point read | an arc that is not a point answers `None` | `climb`, `Any` |
| `offset_derive::nearest` | an argmin | see below |
| `sweep::loft::stacking::rim_pinned` | any pair decided apart answers "not pinned" | `read_identity` |
| `boolean::edge_join::joinable` (the chart-class read over the two surfaces) | which surface's class decides | `climb`, `First` |

`offset_derive::nearest` is listed for its argument, not as a clear
port. An in-band comparison between the best so far and a candidate
leaves the minimum open, unless a later candidate is decided nearer than
both. Whether the ladder's rule applies to an argmin is a design
question.

## What closing it takes

Port each site, measure its reach over the `ci` sets, and pin every
outcome or ranking move. Lower its file's count in
`scripts/gates/escalation-question-marks.counts` in the same commit.
`solid_contain` is the largest and the most used: the point-in-solid
door.

## The callee shape (tranche 5)

`escalation-question-marks.sh` now also registers a `?` on a callee
that reads the band, inside a loop that answers early
(`escalation-question-marks-callee.counts`). These are the HAND-ROLLED
entries of its first register. Each was read in its loop, and none is
measured yet.

| site | shape | port |
|---|---|---|
| `geom_brep::curve_pair::pose::one_line` | either line off the other answers "not one line" | `climb`, `Any` |
| `geom_brep::props::curved::sphere_region_material_sign`, `pole_side` | an arc against the pole answers | `climb`, `Any` |
| `profile::seg::arc_arc` | any end over the other arc answers `Overlap` | `climb`, `Any` |
| `boolean::carrier_touch::off_face`, `ball_off_face` | any cluster or edge not off answers "not off" | `climb`, `Any` |
| `boolean::region::on_boundary` | any edge holding the point | `climb`, `Any` |
| `boolean::sectors::rays_within` | any ray not within answers "no" | `climb`, `Any` |
| `boolean::sectors::crescent` (its callee reads, beside its decide sites) | a crescent found | `climb`, `Any` |
| `boolean::rim_wedge::departures` (its cover reads) | either pair's departure | `climb`, `Any` |
| `census::span_clear` | anything meeting answers "not clear" | `climb`, `Any` |
| `chart_region::polygon_relation` | the first decisive vertex | `climb`, `Any` (check every non-boundary vertex reads alike) |
| `ray_walk::on_boundary` | any segment holding the point | `climb`, `Any` |
| `replace_face::carried_alike` | any sample apart answers "not alike" | `read_identity` |
| `shell::band_seam`, `wraps_its_period` | any side not winding once, any start winding once | `climb`, `Any` |
| `stands::face_interior_point` | any certified interior point | `climb`, `Any` |
| `stands::witness_insides` | its own ranking of `Witness` (`On`, `Blocked`, `InBand`) | its witnesses' standing, then `climb` |

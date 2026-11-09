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
| ~~`boolean::rim_wedge::departures` (`abut`)~~ | **ported** (`onecopy/refusal-standing`, QML T1): `climb(Any)` over the two end pairs, a theorem (either abuts) | — |
| ~~`boolean::rim_wedge::rides`~~ | **ported** (`onecopy/refusal-standing`, QML T1): `read_identity` over both ends | — |
| ~~`splitting::containment::certify_plane`~~ | **ported** (`onecopy/refusal-standing`, QML T1): one `read_identity` over every datum; the datum apart names the cause | — |
| ~~`boolean::sectors`' crescent search (two reads)~~ | **ported** (`onecopy/refusal-standing`, QML T2): `climb(Any)` over the four bound pairs, a theorem; a pair's premise in band is kept (`BooleanError::margin_or_abort`) | — |
| `splitting::join::conic_pairs` (the first cut) | **WHICH** (place_off ruling, QML T1): the cut rotates the pairing; the `?` stands, the site says so | — |
| ~~`shell`'s moved-walls overlap search~~ | **ported** (`onecopy/refusal-standing`, QML T2): `climb(Any)` over the pieces, read lazily; the overlap only names the fault | — |
| ~~`boolean::surface_group::WrapRims::holds`~~ | **ported** (`onecopy/refusal-standing`, QML T1): `read_identity` over the two coaxial margins | — |
| `boolean::reduce`'s line-cone apex end search | **WHICH** (place_off ruling, QML T1): the end is the root whose spread is read next; the `?` stands | — |
| ~~`ring_path`'s lead-point read~~ | **ported** (`onecopy/refusal-standing`, QML T2): `path_parity` climbs every (piece, arc) pair; any pair at which the path says nothing answers `None`, a pair in band is kept, the rest fold the parity | — |
| `offset_derive::nearest` | an argmin | **filed** (`onecopy/refusal-standing`, QML T2): `an-argmin-by-decided-comparison-is-spelled-three-ways` (a copy of `join::keep_nearer`, and the argmin rule is a design question) |
| ~~`sweep::loft::stacking::rim_pinned`~~ | **ported** (`onecopy/refusal-standing`, QML T1): `read_identity` over the rows | — |
| `boolean::edge_join::joinable` (the chart-class read over the two surfaces) | **WHICH** (place_off ruling, QML T1): the carrying surface is in the locus compared; the `?` stands | — |

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
| ~~`boolean::sectors::crescent` (its callee reads, beside its decide sites)~~ | **ported** (`onecopy/refusal-standing`, QML T2), with its decide sites | — |
| `boolean::rim_wedge::departures` (its cover reads) | either pair's departure | `climb`, `Any` |
| `census::span_clear` | anything meeting answers "not clear" | `climb`, `Any` |
| `chart_region::polygon_relation` | the first decisive vertex | `climb`, `Any` (check every non-boundary vertex reads alike) |
| `ray_walk::on_boundary` | any segment holding the point | `climb`, `Any` |
| ~~`replace_face::carried_alike`~~ | **ported** (`onecopy/refusal-standing`, QML T1): `climb(Any)`, any sample past ε answers "not alike" (a theorem) | — |
| `shell::band_seam`, `wraps_its_period` | any side not winding once, any start winding once | `climb`, `Any` |
| `stands::face_interior_point` | any certified interior point | `climb`, `Any` |
| `stands::witness_insides` | its own ranking of `Witness` (`On`, `Blocked`, `InBand`) | its witnesses' standing, then `climb` |
| `boolean::insert::held_cut` (`nested` and the two `tied_held` reads) | any cut held answers; its owner only names `SharedVertexCrossings` (PR 6574's review) | `climb`, `Any` |

## The assignment-exit loops (outside the gate)

The gate's callee register does not see a loop whose answer leaves by
assignment (`found = Some(..)`, `answer = Some(s)`, read after the
loop): its header names the gap. PR 6574's review asked for these to be
kept somewhere checkable, so they are listed here. **34 lines in 24
loops**: the sweep's wider exit shape (an assignment of `Some(..)` or to
`found`) less every loop the gate's own exit rule (a `break`, or a
`return` of a value or a bare binding) already reads. The count is the
same at 6574's first head (`42d9cc9759`). The review's 43 was the
sweep's 126 lines that exit early less the register's 83, which counts
a line the two scans read differently on both sides. Each loop was read
once at its hit lines and its exits.

| loop | `?` lines | disposition |
|---|---|---|
| `geom_brep::certify::run_checks` | 4367, 4546, 4572, 4579 | NOT A SEARCH: every sample is checked; `tangent_prev` is running state |
| `geom_brep::pair_section::torus_arms::square_folds` | 948 | NOT A SEARCH: both folds are collected |
| `geom_brep::pair_section::matched` | 1584, 1598 | NOT A SEARCH: a unique-holder read (two holders are a `Desync`), so every branch is read |
| `geom_brep::pair_section::ended_by` | 1718, 1757 | NOT A SEARCH: a unique-holder read |
| `geom_brep::pair_section::site` | 2684 | NOT A SEARCH: a unique-holder read (a site on two components refuses) |
| `geom_brep::pcurve_cache::projected::met_floors` | 2354 | NOT A SEARCH: a fold |
| `geom_brep::props::curved::unanimous_rim_side` | 1283 | **ported** (`onecopy/refusal-standing`, QML T1): `climb_agreeing` over the rims. No move is reachable: every rim lies at `lo` or `hi`, so each rim's margin is ±(hi − lo), and all are in band or none |
| `geom_brep::props::curved::torus_chart` | 5389 | NOT A SEARCH: a fold over every loop |
| `step_import::entities::loop_u_wrap` | 2084 | NOT A SEARCH: every use is read (an `Option` `?`, running `prev`) |
| `sweep::blend::surgery::ring_clearance_pass` | 2874, 2887, 2896 | NOT A SEARCH: a minimum over every ring. It raises the argmin question `offset_derive::nearest` raises above |
| `topo::boolean::contact_verify::tangent_pass` | 504 | NOT A SEARCH: every sample is checked |
| `topo::boolean::fate::piece_beside` | 1282 | NOT A SEARCH: every holding face is collected |
| `topo::boolean::insert::mint_plans` | 932 | NOT A SEARCH: every plan is keyed |
| `topo::boolean::insert::fan_germ_dir` | 2590 | NOT A SEARCH: a unique-holder read (two rays within one pair refuse) |
| `topo::boolean::join::surface_pair_reach` | 2976 | NOT A SEARCH: a maximum over every pair |
| `topo::boolean::join::seam_germ_side` | 5653 | **HAND-ROLLED, the agreeing shape**: ported to `ladder::climb_agreeing` (`a-ladder-that-checks-its-answers-agree-has-no-home`); its `?` is now a rung |
| `topo::boolean::ops::apply_cut_ins` | 7211, 7212 | NOT A SEARCH: a unique-holder read (a second holder is unread) |
| `topo::boolean::recl::resolve_edge_edge` | 1614 | NOT A SEARCH: each slot is filled from its own face |
| `topo::boolean::reduce::face_tree` | 918 | NOT A SEARCH: every face's box is collected |
| `topo::boolean::solid_contain::ray_crossings` | 4521 | NOT A SEARCH: every meeting is counted |
| `topo::boolean::vtxfac::classify_vertex_on_face` | 867 | NOT A SEARCH: every sector is collected |
| `topo::census::chord_on_edge` | 9213, 9239, 9244 | NOT A SEARCH: both halves are collected |
| `topo::offset_nappe::group_nappe` | 181 | **ported** (`onecopy/refusal-standing`, QML T1): `climb_agreeing` over the faces; a face in band is kept, a stale or straddling face stops the read |
| `topo::pcurves::pcurve_pass` | 5402 | NOT A SEARCH: every face's findings are collected |

A unique-holder read is not the agreeing shape: two holders are a
contradiction, so the read needs every element, and a `?` in band
refuses what the uniqueness needs.

## Other agreeing copies (PR 6606's review)

Loops that read every element and compare the answers, beside the
three in the table above. Each is a candidate for `ladder::climb_agreeing`;
read each before porting it. In two of them a disagreement is a valid
answer of its own, not a fault (`climb_agreeing` leaves that to its
caller):

- `topo::link::second_order` (≈`link/second_order.rs:151`);
- `geom_brep::curve_face::composite::one_signed` (≈`composite.rs:462`);
- `geom_brep::ssi::section` (≈`ssi/section.rs:424`);
- `topo::boolean::vtxfac` (≈`vtxfac.rs:301`): a disagreement maps to a
  valid answer;
- `topo::boolean::rim_wedge` (≈`rim_wedge.rs:500`): a disagreement maps
  to a valid answer.


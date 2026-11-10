---
id: t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer
kind: issue
title: 25 of 44 production Any ladders have no pin for a kept reading ahead of an answer; the any-climb-pins register names each
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Filed by QML T4 (`onecopy/refusal-standing`) on the orchestrator's ruling (c), 2026-11-10. Each of T4's ports onto `geom_core::ladder::climb(Any)` should have a pin: the production function, with a reading kept in band ahead of a later rung that answers, red under the old first-refusal loop. A mis-wired abort or `Blocked` arm then goes red.

T4 pins four:
- `ray_walk::on_boundary`;
- `boolean::region::Region::on_boundary`, by a stub region over the production method;
- `boolean::sectors::rays_within`;
- `chart_region::polygon_relation`.

These four are not pinned, because no cheap fixture reaches a kept rung ahead of an answer. The reach probe saw none in-suite either. Each needs the fixture named:

- **`boolean::carrier_touch::ball_off_face`** (`crates/topo/src/boolean/carrier_touch.rs`). A member's reading refuses only where `edge_clear_of_ball`'s `reduce::certified` refuses: an isolated member's `clear` and the box test never refuse. **Fixture:** a body whose face boundary holds an edge with an uncertifiable curve geometry (an in-band certificate), ahead of a member inside the ball.
- **`boolean::carrier_touch::off_face`** (same file). A cluster refuses only through `ball_off_face`, so it needs that fixture, plus a carrier whose clusters reach that face: a cluster whose ball meets the uncertifiable edge, ahead of a cluster whose ball holds a face point.
- **`shell::band_seam`** (`crates/topo/src/shell.rs`). **Fixture:** a band face on a periodic chart whose two seam sides are half-edge runs, one read in band by `winds_once`, the other decided not to wind once. `winds_once` refuses where its first point's radial lever is in band of the chart's axis (`shell_chart_lever`) or its winding is in band of half a turn (`shell_chart_winds`); the lever is the cheaper, a side starting within the band of the axis.
- **`shell::wraps_its_period`** (same file). **Fixture:** a face with two boundary cycles on a periodic chart, the first read in band by `winds_once` (as above, starting within the band of the axis) and the second winding once.

**QML T5 adds two** (`onecopy/refusal-standing`, after PR 6722). The reach probe saw neither answer past a kept reading in-suite.
- **`stands::insides_at`** (`crates/topo/src/stands.rs`, `witness_insides`' reading). **Fixture:** a body of three shells. One witness of the first lies in band of the second (a `point_in_solid_faces` refusal the walk reads in band) and on the third's surface: the witness reads `On` past the kept shell. A first-refusal loop returned the in-band reading. A second witness, at a limit on one shell and in band of another in that order, would pin the ranking change (a limit before an in-band reading, where the old loop returned the first).
- **`boolean::insert::held_cut`** (`crates/topo/src/boolean/insert.rs`). A cut refuses in band only where `walks_before`, `cut_order` or `nested`'s `holds_whole` reads two germ lines in one entry in band of each other. **Fixture:** an orbit's sectors whose first other cut lies in band of the run's end germ, and whose second cut the run holds: the second's owner answers. The `an_in_band_germ_line_is_the_kernels_own_check` sectors are the nearest existing fixture.

**QML T6 adds one** (`onecopy/refusal-standing`). The reach probe saw it answer past no kept reading in-suite.
- **`census::span_clear`** (`crates/topo/src/census.rs`, the edge–face pierce census's clearance). **Fixture:** an edge whose span runs over a face, the face's boundary holding first a side whose end reads in band of the span (`on_edge_interior_reading` escalates), then a side whose interior meets it (`interiors_meet`). `span_clear` answers "not clear" and pushes nothing. A first-refusal loop pushed the first member's escalation and returned `None`.

## Not yet ported: `vtxfac::partner_side` (filed 2026-11-10, measured)

`topo::boolean::vtxfac::partner_side` reads every bound's side with `?`, so a bound read in band refuses the whole read. It does so even where two other bounds are *decided* on opposite sides, which by itself is `PartnerRead::Across` (a theorem). That is the kept-reading-ahead-of-an-answer shape. It is not a `climb_agreeing` port, since there a disagreement is an answer (PR 6750's site comment). Its port is its own: answer `Across` where two decided bounds differ, else refuse on a kept bound, else the side.

**Reach, measured:** a probe (not committed) logged each call that refuses in band, over the topo, sweep and editor-core `ci` sets at the default ε and at `1e-12`.
- 6 calls at each ε, all in one unit test, `boolean::vtxfac::tests::a_partners_side_of_a_face_is_read_off_its_bounds`.
- 2 of their 6 bounds are in band, and **none** has two decided bounds on opposite sides.
- So no in-suite outcome moves.

**Fixture**, in that test's own frame (`cone(rays)` over the plane datum `z = 0`, `read` checking every rotation):
- rays `[[1.0, 0.0, h], [0.0, 1.0, 1.0], [-1.0, -1.0, -1.0]]`, with `h = 0.5 · (band.zero() + band.escalate())`;
- the first ray's bound reads in band (`Escalated { decision: Coincidence(SectorSide, Moot) }`), and the other two are decided on opposite sides.
- Today: the escalation, in every rotation. Ported: `Across` in every rotation.

## Every Any ladder, held by a gate (the any-pins unit, after PR 6739)

`door-searches-left-on-the-hand-rolled-ladder` closed into this row. Its last item, "the `Any` record paths are unreached", was the same debt for every `Any` port, not only QML's. `scripts/gates/any-climb-pins.sh` now registers every production `Answering::Any` ladder, keyed by its function (44 at filing), in `any-climb-pins.counts`:
- 19 name a pin: a test reading the production function with a kept reading ahead of an answer, or two kept readings ranked;
- the rest name this row.

A new `Any` ladder reds until it is named there. So **the register is this row's list**, and this row closes when no entry names it.

A pin's name is checked by the gate (it must be a `#[test]` or `#[rstest]` fn); that it reaches the answer past a kept rung is checked by `scripts/any-first-mutant.sh`, nightly, which runs every pin under `--cfg any_first_mutant` (`climb_evidence` refusing an `Any` ladder at its first kept rung) and reds one that stays green. A new pin is run under it before it is registered.

The 18 beyond the seven above, each with the fixture its pin needs (a kept rung read ahead of an answering one, through the production function):
- `geom-brep` `chart_singularity::holds`: a chart point whose first bounding reading is in band of the singular point, a later one decided outside.
- `geom-brep` `edge_nurbs::measured_ends`: a spline edge whose first end measures in band, its second decided.
- `topo` `boolean::fate::rehome`: a ring whose first vertex reads in band of the run's loop, a later one decided.
- `topo` `boolean::rim_wedge::rim_exists` and `departures` (the abut climb): rim ends, one pair in band ahead of one that abuts.
- `topo` `boolean::sectors::crescent`: a sector pair whose first bound pair is in band, a later one opening a crescent.
- `topo` `boolean::slit::holding`: an edge in a slit face whose first placement reads in band, a later one decided. `an_unread_piece_refuses_ahead_of_an_earlier_in_band_one` climbs the production rung (`piece_rung`) by hand, so it is one call away.
- `topo` `census::{planar,arc}_interiors_meet`, `shared_run`, `shared_interior_witness`: two interiors whose first piece reads in band and a later one meets. `interiors_ranking`'s rows climb by hand.
- `topo` `chord_join::ring_side`, `holds_outer`: a ring whose first vertex reads in band of the run. `an_unreadable_side_refuses_ahead_of_an_earlier_in_band_one` climbs the production rung (`vertex_side`) by hand.
- `topo` `link::place`: a link whose first piece reads in band ahead of a placing one. `a_degenerate_note_outranks_a_piece_in_band` reads `unread_or_rest`, not `place`.
- `topo` `offset_nappe::face_nappe_reading`: a face whose first nappe reading is in band.
- `topo` `replace_face::carried_alike`: one side's transport escalating ahead of a side with no image.
- `topo` `side_reader::cast_aims`: an aimed ray's `Pair` in band ahead of a ray that decides (the door row's cheapest-named case; no fixture builds the pair today).
- `topo` `splitting::finish::shell_side`: a shell whose first witness reads in band.

## Done when

- No entry of `scripts/gates/any-climb-pins.counts` names this row: each has a pin as described, red under a first-refusal loop, or the register names a stated reason it cannot be reached.
- `partner_side` is ported as above, with the fixture as its pin.

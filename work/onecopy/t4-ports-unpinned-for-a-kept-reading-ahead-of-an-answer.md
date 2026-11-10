---
id: t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer
kind: issue
title: Seven of QML's Any ports (T4's four, T5's two, T6's one) have no pin for a kept reading ahead of an answer
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

## Ported: `vtxfac::partner_side` (filed 2026-11-10, measured; ported on `the-ladder-has-no-climb-for-a-disagreement-that-answers`)

**Ported** onto `geom_core::ladder::climb_differing`, with the fixture below as its pin (in `a_partners_side_of_a_face_is_read_off_its_bounds`, `h = 3 · band.zero()`, a lift that test already shows in band). What follows is the row as filed.

`topo::boolean::vtxfac::partner_side` reads every bound's side with `?`, so a bound read in band refuses the whole read. It does so even where two other bounds are *decided* on opposite sides, which by itself is `PartnerRead::Across` (a theorem). That is the kept-reading-ahead-of-an-answer shape. It is not a `climb_agreeing` port, since there a disagreement is an answer (PR 6750's site comment). Its port is its own: answer `Across` where two decided bounds differ, else refuse on a kept bound, else the side.

**Reach, measured:** a probe (not committed) logged each call that refuses in band, over the topo, sweep and editor-core `ci` sets at the default ε and at `1e-12`.
- 6 calls at each ε, all in one unit test, `boolean::vtxfac::tests::a_partners_side_of_a_face_is_read_off_its_bounds`.
- 2 of their 6 bounds are in band, and **none** has two decided bounds on opposite sides.
- So no in-suite outcome moves.

**Fixture**, in that test's own frame (`cone(rays)` over the plane datum `z = 0`, `read` checking every rotation):
- rays `[[1.0, 0.0, h], [0.0, 1.0, 1.0], [-1.0, -1.0, -1.0]]`, with `h = 0.5 · (band.zero() + band.escalate())`;
- the first ray's bound reads in band (`Escalated { decision: Coincidence(SectorSide, Moot) }`), and the other two are decided on opposite sides.
- Today: the escalation, in every rotation. Ported: `Across` in every rotation.

## Done when

- Each of the seven has a pin as described, red under the old first-refusal loop, or a stated reason it cannot be reached.

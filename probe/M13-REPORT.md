# M13: is there a record of a build? (BOXFINISH measurement lane)

Branch `boxfinish/probe-build-record` (never merged). Instrument: `crates/editor-core/src/build_record.rs` and `drive.rs`'s `mod probe`, switched by `BUILD_RECORD_PROBE_DIR` (and `BUILD_RECORD_PARITY=1`). Scripts and their outputs: `probe/build-record/`. Every number below is **measured** on today's main (`aebe03a2a`), default eps, CI's opt-1 profile, locally, unless marked *inferred*.

## The answer

**Yes: a record of a build exists, names every cell, and is the same at `f64` and at the box scalar wherever both lanes give every node the same standing. And it does not do what the proposal needs: it does not separate one known flip.**

- The record: per node its standing; per output body its faces, edges and vertices by name, each face's loops as cycles of named edges and vertices, each edge's ends, shells, lumps; the name tables; the node's coincidences (relation, site, origin, cells).
- 11,123 distinct builds (every top-level `f64` evaluation of `editor-core`'s 3,302 tests and the tour's 98, replayed at `Interval` under the same options). Standing equal in 10,872: **0 record differences, 0 name-table differences, 0 unnamed cells in either lane.**
- What breaks sameness is **standing**: 251 builds differ in a node's standing, nearly all a node that builds at `f64` and fails at `Interval` at the nominal point (Boolean 152, Split 59, Revolve 10, Skin 7, Fillet 5, ...); all but 5 carry an escalation.
- **Every one of today's 661 `FlipCrossing` leaves has the witness's record** (and the witness's carrier kinds and face senses). Under "a decided leaf certifies when it builds what the witness builds" they all certify, the tree's own planted flip among them.
- No leaf today is decided AND builds another record: over 38,119 leaf replays the record differs from the witness's only where a standing differs, and every such leaf is undecided or holds a node that failed with no escalation logged.

## 1. Does it exist, and is it scalar-independent?

| | `editor-core` | tour |
|---|---|---|
| distinct builds | 11,028 | 95 |
| standing equal in both lanes | 10,782 | 90 |
| ... record / names / carrier kinds and senses differ | 0 / 0 / 0 | 0 / 0 / 0 |
| unnamed cells, either lane | 0 | 0 |
| box lane holds an escalation at the nominal | 2,394 (22%) | 51 (54%) |
| box lane fully decided, standing equal | 8,629 | 44 |
| ... whole log differs (record equal in all) | **2,301 (27%)** | 10 |

- **The whole logs are not the same.** The parity instrument is red on 2,311 fully decided builds. By `first_turn`: 2,106 part at a step or an end (`Parting::Step`/`Ended`), 203 agree in choices and part in check sequences, 2 turn a choice (`pcurve_chart_placement` Positive at `f64`, Negative at `Interval`, the tour's bracket). None parts at a mark.
- First differences (`f64`'s entry, the box lane's): `pose_unit_norm` vs `session_box_reach` 1,348, vs `mate_chart_slides_span` 625; `props_circle_axis_class` (check) runs the box lane lacks, 195 (the plate's class; also the tour's cut plate); `ssi_hull_sup` vs `pcurve_projected_lever` 51; `bool_point_in_solid_advance` vs `..._denom` 31; `path_leg_length` vs `trace_plane_*` 21. Not traced to source.
- On the rows named `*corpus*` (370 builds): record equal in all 368 of equal standing; 166 hold a box-lane escalation at the nominal; 17 of 204 decided builds differ in log.
- At the drives' own witnesses, zero-width box (67 distinct, both `Interval` and `Sym<Interval>`): record equal in all 63 of equal standing; 4 differ in standing (a loft's `Skin` fails at the box scalar with no escalation, 3; my cone cut, 1); logs differ in 1 (the tour's cut plate). At `Sym<Interval>` one coincidence's discharge reads `Proven` where `f64` reads `Numeric`.
- The box lane's first escalation at the nominal: `curve_pair_pose_parallel` 1,314, `germ_side` 409, `chart_singularity_nearer` 258, `curve_carrier_gap` 191. Such a document has no leaf that decides, at any width, under either gate.

## 2. Does it separate what must be separated? No.

- Leaf replays: 35,683 in `editor-core` (305 drives, 34 range searches), 2,436 in the tour. Decided leaves: 2,017 and 488.
- **Known flips are not separated.** 660 flip leaves turn a choice (`segment_gap` 300, `path_junction_turn` 220 with `side_planes_cosurface` 220, `extrusion_normal_component` 122, `carrier_line_circle` 18); all 660 hold the witness's record. The `docm9_range` rows hold 219 of them, the `m10_3` and `m10_6` driver rows (the planted-flip fixture `notch` among them) 319, my crease document 12. A flip leaf's record dumped beside its witness's (the crease document, 95 lines) is equal line for line.
- Inspected: the pairs are the same cells, same incidence, same carrier kinds and senses. They differ in geometry the cells do not state: a crease's convexity (`notch`: a convex point below zero, a notch above), which side of the sketch plane an extrusion lies on.
- Converse hunt: 0 groups of decided leaves with equal choices and different records; 0 with equal name tables and a different record (incidence added nothing to the names on today's documents).
- The instrument does see other cells: two blocks slid apart (`probe_build_record.rs`), 12 leaves past the parting hold 5 lumps against the witness's 4 and other edge names. All 12 are undecided (`side_at_infinity_edge`).

## 3. What the record leaves out that a reader uses

| left out | differs with the record equal? | a reader that would see it |
|---|---|---|
| surface and curve carrier kinds, closed-form or general spelling (an edge's stored and described kind) | never (10,872 builds, 2,505 decided leaves) | the measure table (a closed form per carrier pair), STEP export |
| face senses | never | `gap`'s outward normal, mass properties |
| a crease's convexity, positions, the side an extrusion lies on | **on all 660 turned flips** | a blend of that edge; every measure value |
| a coincidence's discharge | once (`Sym<Interval>`: `Proven` for `Numeric`) | the coincidence report |
| a coincidence's margin, a placement's motion, a datum's frame | numbers at the run's scalar; not compared | positional readers |
| an assertion's verdict | 82 certified leaves read `Violated` for `Holds` | reports only (E10 leaves it out by rule) |
| which configuration a measure read (`carrier_pose_*` choices) | not seen: no decided leaf turns one | the measure's value |
| `Carry` (contacts, pieces), fragment groups | not in the instrument | picks, contact reports |

## 4. How much mass moves

Final leaves, summed over the suite's drives (a fixture counts once per row that drives it; my crease rows add 2.67 certified and 0.33 `FlipCrossing` to these sums, and 12 of the 661 flip leaves):

| today | mass | by the record |
|---|---|---|
| certified | 245.39 (82.1%) | same build |
| `FlipCrossing` | 14.92 (5.0%) | **same build: all of it would certify** |
| `Budget` | 30.90 (10.3%) | undecided (an escalation) |
| `SliverTerminal` | 4.39 (1.5%) | undecided |
| `Budget` + `ReadsNoVariable` | 0.45 + 2.75 | a node fails, no escalation logged |

- Tour: the plate study certifies 0.9491 and refuses 0.0455 as `Budget`, all undecided; the cut plate's one leaf (0.9946) and the chain's refusals (1.28 of 2.98) are `Budget`, undecided. **No `FlipCrossing`, no parting, no mass moved by the record anywhere on the tour.**
- Today's 661 `FlipCrossing` leaves by their logs: a turned choice 660; a turned check 0; a parting at a pruning query 0; a parting elsewhere among choices 0; evidence naming nothing 1 (the plate under the range search: choices agree, check sequences part at `props_circle_axis_class`).
- **Partings barely occur on decided leaves today** (1 in 2,505). They are common between the two lanes at one point (section 1).
- **"Builds another" has no decided instance.** 27,781 leaves hold a node that fails where the witness builds with no escalation on any log; 27,667 are one sentence, `Extrude`: "no definite length across the sketch plane". 13,818 of them split today and 13 such boxes hold certified mass below them (2.02 summed). Read as "builds another", that mass would be refused. *Inferred*: the gate needs a failure proven definite, which no log carries today.

## 5. Three small facts

- **A lane choice turning (plane cutting a cone, tilt 0 at the nominal): not observable.** Main's drive over tilt 0 ± 1e-6 certifies nothing (`Budget` 1.0). The `Split` fails at the box scalar on the zero-width box (`chart_singularity_nearer` at `Interval`, `split_join_frame_axis` at `Sym<Interval>`) and every leaf is undecided (`enters_material`). A leaf costs 8 to 196 s. At tilt 0.05 the zero-width replay holds the witness's record and 10 escalations.
- **A crease turning convex to concave** (extruded pentagon, h = 1.1 ± 0.3): main reports 12 certified (0.667), 12 `FlipCrossing` (0.333) naming `path_junction_turn` Positive to Negative and `side_planes_cosurface`, 1 `Budget` (6e-8). The record: all 12 flip leaves are the witness's build. At h = 1.0 the witness does not build.
- **Cost.** Per leaf, taking the record: median 9 us, p90 205 us, max 37 ms; comparing: median 0.4 us. Over 37,092 leaves 4.7 s against 1,152 s of leaf evaluation (0.40%). On the larger parity builds: median 1.4 ms against 31 ms (3.2% in total). The instrument prints names into strings, so this is an upper bound.

## Not measured, and why

- A lane choice or a measure's configuration turning on a decided leaf: no document that reaches one decides at the box scalar.
- `Sym<Interval>` across the 11,123 builds: only at the 67 drive witnesses.
- Other eps rows; hosted CI (local runs only, as the brief says).
- 24 `editor-core` rows are red under the probe (rows that count evaluations or finishes; one killed for memory; my cone row, stopped). Their lines up to the failure are in the data. Three of them (`docm9_range`'s plate row, `m10_9_what_the_registry_buys`, `m10_p_lift::at_sym_a_fixed_box_is_the_build`) are red with the probe switched off too; I did not build clean main to confirm.
- One tour nightly row (`the_cut_plates_first_subtract_...`) is red with the probe's leaf lines on; not chased.

# R1 probes, PR 7745 at f1ff060a

`mutants.patch` applies to f1ff060a. Every mutant is selected by an env var, so one
build serves them all. Each row was run as
`cargo nextest run -p topo -p sweep --profile ci` (7193 tests):

| env | mutant | red |
|---|---|---|
| PROBE_ANYREM | consume on any removal (the old rule) | 2 |
| PROBE_IGNREM | ignore removed members | 29 |
| PROBE_ANYINC | structure on any incident cell counts (no minted restriction) | **0** |
| PROBE_NOSECT | section cells not named | 14 |
| PROBE_SECTV | section vertices named on the carried path too | 1 (the filed P2 panic) |
| PROBE_JOINTBAD | B's clone cells read in the joint body without the graft map | 26 |
| PROBE_GAP=face/edge/recut (+PROBE_PANIC) | lineage gap: a division omits its first live piece; panics where a gapped record is consumed | 3 / 12 / 56 |

The silent drop: `PROBE_GAP=face` on
`sweep face_face_touch::a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it`
(without PROBE_PANIC). The bridge union returns `Ok`. Both cap circles' edge–face
records on the plate face are consumed through `on_structure` (an image of the
circle is incident to a section cell minted in the plate face), and the face–face
record is gone too.

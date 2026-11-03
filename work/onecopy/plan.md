# ONECOPY — the plan

No orchestrator yet. Live state is `work/onecopy/log.md`'s tail and the
item files beside this plan.

Branch prefix **`onecopy/`**; away-channel tag `(ONECOPY orchestrator)`.

Opened 2026-11-03 by cutting ONEHOME (`work/onehome/`) after its
orchestrator ran out of usage. Every row ONEHOME had not started moved
here by `git mv`, id and body unchanged, each with a dated
`## Slate triage` note. Nothing is dispatched.

## Charter

Each geometric reading of a carrier has one home in `geom`, and each
walk, refusal mapping and ladder search in topo reads its one home;
every copy is retired by repointing its users at the survivor.

## Posture

- **What stayed on ONEHOME, and why it matters here.** ONEHOME keeps
  PR 6371 (the finished body), PR 6380 (the run owns f64 rounding,
  `[ev]`) with the lever, session-box and far-centre-conic rows filed
  against it, and `topo::star` stages 2–3 (`run-loops-of-split-and-pierce-are-twins`,
  with the pierce-facing, vertex-pair planner, germ-signing and
  vertex-section rows they fold). Those PRs touch the same files as
  several rows here: before dispatching, read `work/onehome/log.md`'s
  tail and run `scripts/work.py territory`, and merge main and compile
  before the next merge whenever one of them lands (ONEHOME's lesson
  from PRs 6408 and 6410: a tree-wide signature change collides with
  every in-flight PR that adds a caller).
- **The geom half reads ONEHOME's merged homes.** PR 6328 made
  `geom::Line { origin, dir }` the line `Curve3::Line` holds, with
  `Line::foot` and `Line::param`; PR 6349 made `geom::Circle { frame,
  radius }` the one circle and the `Conic` trait the conic reading.
  `analytic-surfaces-spell-their-frame-field-by-field` (H) is the
  heavy row: the surfaces' placement onto `geom::Frame`.
- **The topo half is mostly review follow-ups**: PR 6327 (curve-pair
  readings), PR 6333 (identity ladder), PR 6362 (face walks). Each row
  lists its items; several are small enough to bundle into one lane.
- **Disk**: ONEHOME found that two local lanes is the ceiling on a
  4-CPU, ~40 GB box, and its lane brief sets `CARGO_INCREMENTAL=0`
  (`work/onehome/log.md`, 2026-11-02 friction entries).

## The slate

**30 budget points** of dispatchable P0/P1 work against a ceiling of 30;
the P2 rows ride free.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `analytic-surfaces-spell-their-frame-field-by-field` | H | Plane, Cylinder, Cone (and sphere, torus) each spell the placement field by field, beside the `geom::Frame` the conics hold |
| P1 | `circle-frame-structs-re-spell-geom-circle` | M | Four topo and sweep structs re-spell `geom::Circle`'s fields |
| P1 | `interval-safe-point-angle-read-two-ways` | M | The interval-safe angle of a point about a conic is spelled twice |
| P1 | `a-curves-closed-form-foot-is-re-spelled-outside-geom` | E | A point's foot on a line or circle carrier is re-spelled outside geom |
| P1 | `a-faces-loops-are-walked-inline-beside-face-loops-linked` | M | A face's loops are walked inline at about fifty sites beside `Body::face_loops_linked` |
| P1 | `the-face-walk-units-follow-ups-from-its-review` | E | PR 6362 follow-ups |
| P1 | `carrier-refusal-to-error-mapping-is-spelled-at-seven-sites` | E | The `CarrierRefusal`-to-error mapping is hand-kept at about seven topo sites |
| P1 | `the-curve-pair-units-follow-ups-from-its-review` | E | PR 6327 follow-ups |
| P1 | `boundary-cuts-reads-line-against-conics-through-their-planes` | M | `boolean::boundary_cuts` reads a line against boundary conics by plane crossings, a proxy of the curve-pair meeting question |
| P1 | `the-ladders-follow-ups-from-its-review` | M | `geom_brep::ladder` follow-ups (PR 6333) |
| P1 | `witness-and-door-searches-hand-roll-the-ladder` | M | Witness and door searches hand-roll the ladder's rule beside `ladder::climb` |
| P2 | `a-points-foot-on-a-plane-is-spelled-inline` | E | A point's foot on a plane is spelled inline in five places with no geom home |
| P2 | `carrier-speed-is-re-spelled-with-a-default-arm-in-import` | E | A carrier's speed is re-spelled per kind beside `carrier_speed` |
| P2 | `a-points-radial-offset-from-an-axis-is-spelled-inline` | M | A point's offset from its foot on an axis is spelled inline at about twenty sites |
| P2 | `face-to-surface-walks-outside-topo-predate-body-face-surface` | M | Crates outside topo walk a face to its surface by hand |
| P2 | `local-point-direction-line-types-restate-geom-line` | M | Five local (point, direction) line types restate `geom::Line` |

## Exit criteria

None set: the program closes when its slate is empty.

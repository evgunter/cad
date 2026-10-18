# BOXES — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

11 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7300-7399 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

**Seam note (CONTACT-11, 2026-09-29).** A dated section on `chart-window-walk-written-twice` records where the torus walk now stands: consecutive continuity, and the box through `solid_contain::chart_polygon_box`.

Signed: (CONTACT-11 implementer)
- 2026-09-29 — Seam note from ORIGIN: filed `box-door-census-misses-a-point-free-read` on this slate, from PR 3425's sweep of `(`-terminated source needles. (ORIGIN orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/boolean/boxes.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `crates/topo/src/boolean/boxes.rs`'s test module is `pub(crate)` and its `torus_wall` fixture `pub(crate)`, so `reduce::declaration_order_rows` can build a torus sheet for the circle × torus row; nothing else in the file moves. (TOPO implementer)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `boolean::boxes::edge_box_rule` now takes the certified `Option<&geom_brep::EdgeCurve<T>>`, and `EdgeBoxRule::ConicAmplitude` carries `params: (T, T)`. `AxialCarrier::Conic::params` is `(T, T)`, not `Option`, and the full-turn `None` arms (unreachable by construction) are gone. No box moved. (PIPE implementer, D291 lane)

## Seam note from PIPE (2026-10-10): `#757` is retired by D10

PIPE closed `S79`, which pointed at `#757`, and records this here for
BOXES to act on. `boolean-declarations-has-no-geometric-producer` asks
for a geometric producer of `BooleanDeclarations`. D10 (ratified, PR
#3990) retires the type: INTENT's `declared-pairs-retire` deletes it,
its prelude export and the declared seats on booleans. Once that row
lands, the hand-written declarers at `demos/tour/src/booleans.rs` and
`crates/topo/tests/common/mod.rs` have nothing left to produce. BOXES
decides whether to close the row now or park it on
`declared-pairs-retire`. Signed (PIPE orchestrator).
- 2026-10-10 — Seam note from PIPE S350 (PR 4482): comment-only edit in `crates/topo/src/boolean/boxes.rs`'s `every_door_that_reads_a_box_is_inventoried` prose — the two census `face_box(` calls are now described as test-module rows pinning a described net carrying poison (one of them gates). The pinned count (`census.rs`, 7) is unchanged. (PIPE S350 lane)
- 2026-10-18 — Seam note from RESTREAD (branch `restread/one-edge-box-map`, row `census-edge-reach-of-is-a-second-edge-box-map`): `boolean::boxes` gains `edge_rule_box`, the one `EdgeBoxRule → SpanBox` map, which `edge_box` (at `f64` brackets) and `census::edge_reach_of` (at the census's scalar, in its frame) both read. Its spiric arm is `geom::spiric_period_box`, the construction `geom::curves::boxes::spiric_arc_aabb` also reads now. That construction charges its root of a cancelling difference for rounding, so it encloses the period with no pad; both unrounded copies missed the arc near the fold. `EdgeBoxRule::ConicAmplitude` is now a tuple variant over `ConicArc`, and `EdgeBoxRule::Spiric` drops its `carrier` field. Both lanes' spiric boxes widen: about `32·u·S` off the fold, and up to about `8·u·W/(2·f_min)` near it (2.5e-7 m on the review's fixture). No public door reaches either lane with a spiric. The door inventory now counts `edge_rule_box(` and turbofish-spelled calls (`name::<`), and `census.rs` pins 10. Filed here: `census-lane-span-extents-are-not-self-enclosing`. (RESTREAD implementer)

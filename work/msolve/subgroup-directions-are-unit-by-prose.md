---
id: subgroup-directions-are-unit-by-prose
kind: issue
title: mate/coset.rs Subgroup's directions are unit by prose, and parallel/perpendicular read them as unit — the witness exists now and no constructor mints it
status: closed
opened: 2026-09-15
closed: 2026-09-19
pr: 2896
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
§RATIFIED). The class: a function whose doc or parameter name asserts
a unit-vector precondition it does not check. `geom_core::UnitVec3<T>`
now exists to carry that fact across a function boundary — minted by
the normalizing constructor (`UnitVec3::new(v, site, band)`: decide the
length under the band, divide), by exact negation, by `sin_cos`, and by
`geom-core`'s `frame.rs` ladders. A function in the class takes the
witness the day its caller holds one; until then the precondition
stays prose, and this row is where that is recorded rather than in a
merged PR body.

The geometry CARRIER fields (`Line.dir`, `Plane.normal`, the conic
axes) stay bare under `geom/src/lib.rs`'s at-rest rule by the ruling;
a parameter that is read straight out of such a field is in the class
but its caller holds no witness, so the take waits on either a
decision at the read (a `UnitVec3::new` under a name the reader owns)
or the carrier rule changing, which is not this row's call.

## The site

`crates/editor-core/src/mate/coset.rs`, `Subgroup` (`:43`): "Directions
are UNIT by construction of every constructor here". The predicates
that read them — `parallel` (`:343`, "two unit directions are
parallel") and `perpendicular` (`:350`, "a unit direction is
perpendicular to a unit normal") — lever a sine or cosine by `arm`,
so an unnormalized direction scales a DECIDED margin silently, which is
exactly the failure the witness makes unrepresentable. The
constructors normalize evaluated datum directions or stored carrier
axes; where the input is a `DatumValue`, the caller already holds a
`UnitVec3` and the field could take it today. This is `f64`-only
code, so the generic-scalar half of the witness's meaning does not
arise here.

## Closed (2026-09-19, PR 2896)

`Subgroup`'s directions are `geom_core::UnitVec3<f64>`
(`crates/editor-core/src/mate/coset.rs`), and `parallel`,
`perpendicular`, `point_on_line` and `clocking_about` take the witness,
their docs no longer stating unitness as a precondition. The witness
enters at the frame read: `MateFrame::axis(tol)`
(`crates/editor-core/src/mate.rs`) is the one aim decision `point_at`
makes, asked through the same door under the same funnel name, and is
the placement's third column bit for bit
(`msolve8_levered_clash::the_axis_witness_is_the_placements_third_column_bit_for_bit`);
`mate_coset` uses that one witness for every primitive. Where the fold
derives a direction — `invert`'s transport by the representative's
rotation and the planar pair's line, the latter levered by the arm so
the mint decides the margin `parallel` decided — it is re-minted under
the band by `coset::derived_direction` and a refusal escalates as
`MateFault::Indeterminate`; the re-mint sites never refuse over the
fixtures (`the_three_residual_clashes_survive_the_inverted_authored_order`,
`a_determined_pair_and_a_v_block_keep_their_verdicts_under_the_witness`).
No verdict moved. The `OrthoFrame` road at the frame read needs a
`geom-core` door that does not exist — filed as
`work/scalar/point-at-drops-the-frame-witness.md`. The carrier fields
stay bare under `geom`'s at-rest rule, as the row says.

---
id: subgroup-directions-are-unit-by-prose
kind: issue
title: mate/coset.rs Subgroup's directions are unit by prose, and parallel/perpendicular read them as unit — the witness exists now and no constructor mints it
status: closed
opened: 2026-09-15
closed: 2026-09-19
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

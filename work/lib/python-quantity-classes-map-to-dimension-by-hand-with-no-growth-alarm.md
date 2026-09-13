---
id: python-quantity-classes-map-to-dimension-by-hand-with-no-growth-alarm
kind: issue
title: dimension_of maps the Python quantity classes onto Dimension in an unfenced if-chain
status: open
opened: 2026-09-12
---



Found by DOOR's `dimension-all-has-readers-outside-the-viewer` sweep
over every complete hand-written enumeration of `Dimension`'s variants.
That row projected the sites that had a list to read; this one has
none, so it is filed rather than converted.

## The finding

`crates/pncad-py/src/py/quantity.rs`'s `dimension_of` (`:75`) answers
the dimension of a Python object with a four-branch `if`/`else if`
chain — `Length`, `Angle`, `Count`, then `f64` as `Scalar`, else `None`
— and it is the input to `mismatch`, which decides whether the boundary
raises a typed `QuantityOpMismatch` or a plain `TypeError`.

Nothing fences the chain. Every other spelling of the dimension list in
this crate is an exhaustive `match` (`errors::dimension_tag`,
`errors::canonical_unit`, `py::value::dimension_name`,
`py::analysis::quantity`) and stops compiling on a dimension added to
the lattice; this one compiles unchanged and silently answers `None`
for the new dimension, so a Python quantity class added beside it would
report every operator mismatch as an untyped `TypeError`.

This is the shape `dimension-all-has-readers-outside-the-viewer` set
aside for `u8a_parse::rt_params`: **not a projection**, because what is
enumerated is this crate's own Python classes and not the kernel's
variants, so there is nothing to read `Dimension::ALL` into. What it
wants is a growth alarm — a roster over an exhaustive match on
`Dimension` asserting each variant either has a Python quantity class
here or is deliberately without one, so the next dimension fails until
someone decides which it is.

**That instrument already exists**: `partial_mirror!`
(`crates/viewer/src/forms.rs`), written for `MATE_PRIMITIVES` when
DOOR's `mate-primitives-is-a-partial-mirror-with-no-growth-alarm`
closed. It lives in the viewer today, so taking this row means deciding
whether it moves somewhere both crates can read it or whether
`pncad-py` grows its own;
`work/view/two-partial-mirrors-in-the-viewer-have-no-growth-alarm` is
asking a version of the same question for two more sites.

Also in this crate and NOT this row: `py::value::Measurement`'s
`dimension` field doc spells the four identifiers in prose (`:732`).
A doc mirror no gate reads is a different class, as
`crates/viewer/README.md`'s GQ5 recap is.

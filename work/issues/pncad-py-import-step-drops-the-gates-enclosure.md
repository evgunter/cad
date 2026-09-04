---
id: pncad-py-import-step-drops-the-gates-enclosure
kind: issue
title: pncad-py's import_step drops the gate's enclosure, so a Python import-then-measure pays two certified quadratures
status: open
opened: 2026-09-03
---


The Python binding's importer adopts the imported solid and DROPS the
field beside it:

```rust
Ok(pncad::step_import::StepImport::Solid { body, .. }) => Ok(Body::plain(Arc::new(body))),
```

`crates/pncad-py/src/py/value.rs:1324`. The `..` now covers
`enclosure` — the aggregate tier-3′ gate's own `MassProperties`,
documented at `crates/step-import/src/lib.rs`'s `StepImport::Solid` as
*not a second computation* — and the same module exposes a
`mass_properties` door on the handle it just built
(`crates/pncad-py/src/py/value.rs:237`, which calls
`topo::mass_properties` at `:239`).

So the natural Python journey — import a file, then ask the body for
its volume — runs the certified quadrature TWICE over one body at one
band, which is exactly the redundancy TCOST-K3 removed from the Rust
side. Measured on the rational-walled fixtures TCOST-K3 reports, the
second quadrature is 25-50 % of the row's wall at the default ε.

**Shape of the fix**, so it is not re-derived: `Body` would have to
carry the enclosure the importer already holds (an `Option`, `None`
for a body that did not arrive through a gate), and `mass_properties`
would answer from it when present. That is a change to the handle's
representation and to what `mass_properties` means on it — a public
API question about the binding, not a mechanical follow-on — which is
why this is filed rather than taken in passing.

**Not a soundness defect**: the second quadrature is the same
computation and produces the same four fields bit for bit. It is
cost, and the honesty of one field's documentation: the field says
"not a second computation" and this consumer makes one anyway.

---
id: python-evaluation-has-no-edge-carrier-kind-twin
kind: issue
title: pncad-py's Evaluation has no edge twin of face_carrier_kind
status: open
opened: 2026-09-14
priority: P3
cost: D
---


## What

`Evaluation.face_carrier_kind` (`crates/pncad-py/src/py/value.rs`,
beside `face_frame`/`edge_frame`/`vertex_position`) hands a Python
caller a named face's stored `SurfaceKind` tag. As of TOPO's
`edge-carrier-kind-has-no-readback-door` the edge side has the same
read at every Rust layer — `topo::readback::edge_carrier_kind`, the
`query::edge_carrier_kind` flattening, the document twin
`editor_core::names::interrogate::edge_carrier_kind`, and the
`pncad::select` / `pncad::prelude` re-exports — and the chain stops at
the binding: Python can ask an edge for its FRAME (`edge_frame`) but
not for its KIND, so "is this edge straight" has no Python spelling
while "is this face planar" does.

## What the twin would be

One method beside `face_carrier_kind`, the same four lines:

```rust
fn edge_carrier_kind(&self, py, node: &NodeId, name: &str) -> PyResult<CurveKind> {
    let name = super::doc::name_from_text(name)?;
    pncad::select::edge_carrier_kind(&self.inner, node.0, &name)
        .map(super::select::curve_kind)
        .map_err(|err| super::readback::readback_err(py, &err))
}
```

The Python-side `CurveKind` mirror and its `to_kernel` already exist
(`crates/pncad-py/src/py/select.rs`, the selector's `CurveKinds`
comparand), so what is missing is the kernel→Python direction of that
mirror — the twin of `super::select::surface_kind` — plus the method,
its docstring in the house style (a tag READ, never a verdict; the
`wrong_kind` refusal for a face or vertex name), a `.pyi` entry and a
Python row beside the `face_carrier_kind` one.

Not built by the TOPO unit that found it: `crates/pncad-py/*` is LIB's
territory and the binding's shape — whether the kind comes back as the
`CurveKind` mirror or as its `.name`, and whether it lands with the
other DOCM-era read doors or with the selector vocabulary — is LIB's
call. Announced on `work/lib/log.md` in the same PR (TOPO branch
`topo/edge-carrier-kind-readback-door`).

The binding census carries the debt meanwhile: `edge_carrier_kind` is a
`gap:` entry under the family `B-EDGE-KIND` in
`crates/pncad-py/tests/test_binding_census.py`, whose charter is the
delivery list above. Closing this row moves that entry off the roster
(into `BOUND_AS`, or off it entirely if Python spells the name
identically) and takes the `FAMILIES` charter with it — the census
fails on a chartered id no entry cites.

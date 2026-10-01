---
id: step-export-carrier-kind-duplicates-curve-kind
kind: issue
title: step-export's writer names curve and surface kinds with its own functions beside geom_brep's CurveKind and SurfaceKind
status: open
opened: 2026-10-01
---


Found by D36's kind-namer sweep (`pcert/d36-unsupported-carrier-split`,
which added `geom_brep::CurveKind` beside `SurfaceKind` and retired
`geom_brep`'s own second namers).

`crates/step-export/src/writer.rs` has `carrier_kind` (`Curve3` →
`"line"`, …, `"nurbs curve"`) and `surface_kind` (`Surface` → names,
with the placeholder NURBS told apart from a described one). The first
is `geom_brep::CurveKind::name` with one spelling changed; the second
carries a distinction `SurfaceKind` does not (placeholder vs described),
so it may want to stay, but its kind half could read `SurfaceKind`.
Not a defect today; it is the shape that let `chart_name` and
`SurfaceKind::name` drift apart (`"Nurbs"` vs `"nurbs"`). Decide whether
these refusal texts should share the kernel's spellings.

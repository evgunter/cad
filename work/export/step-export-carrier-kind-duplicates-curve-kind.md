---
id: step-export-carrier-kind-duplicates-curve-kind
kind: issue
title: step-export's writer names curve and surface kinds with its own functions beside geom_brep's CurveKind and SurfaceKind
status: closed
opened: 2026-10-01
closed: 2026-10-02
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

## Closed by `one-kind-mirror-per-geometry-enum` (TQUERY)

The kinds moved down to `geom` (`geom::CurveKind`, `geom::SurfaceKind`,
each with one `name()`). `carrier_kind` is deleted: the volume
classifier's refusal names a carrier by `carrier.kind().name()`.
`surface_kind` keeps only the distinction the kind does not carry — the
NURBS placeholder is named `"nurbs placeholder"` — and names every other
surface by `surface.kind().name()`. The refusal texts now print the
kernel's spellings: `"nurbs"` for a described NURBS surface (was
`"nurbs surface"`), `"approx"` for an approximating one (was
`"approximating surface"`).

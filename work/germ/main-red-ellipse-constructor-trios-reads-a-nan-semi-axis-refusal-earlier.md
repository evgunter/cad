---
id: main-red-ellipse-constructor-trios-reads-a-nan-semi-axis-refusal-earlier
kind: issue
title: Main red: ellipse_constructor_trios expects Escalated for a NaN semi-axis, refused earlier as SemiAxisNotFinite since 53967a14
status: closed
closed: 2026-10-20
opened: 2026-10-20
---


**Main red.** Filed by the NURBS orchestrator from NURBS build C's bisect.

`geom` `curves::tests::ellipse_constructor_trios` fails deterministically on main `43b23583` (3 of 3 runs): it panics at `crates/geom/src/curves.rs:2254` with `SemiAxisNotFinite`.

The row's `mk(2.0, f64::NAN)` case (a NaN minor semi-axis) expects `EllipseInvalid::Escalated(_)`. Since `53967a14d2` (2026-10-20 01:47Z, "germ: build the near-circular ellipse's frame orthonormal; read the bisector's half-angle without cancellation", merged via PR #5156, `d5163286c8`), the constructor refuses it earlier, as `SemiAxisNotFinite`. That commit is red on its own; its parent `5ada12a584` is green.

It reads red on every PR whose CI runs geom's lib tests (first seen on NURBS's #5446, whose diff is geom-core `tensor.rs` tests only).

**Done when** GERM rules which refusal is right and the row and constructor agree:
- either the row expects the earlier typed refusal (the poison is now caught by a finiteness check before the band test);
- or the constructor's order is restored.

**Closed (2026-10-20).** Fixed on main by `2ee8a2ea45` ("geom: ellipse_constructor_trios expects SemiAxisNotFinite for a NaN minor (since 53967a14d2)"): the row now expects the earlier typed refusal.

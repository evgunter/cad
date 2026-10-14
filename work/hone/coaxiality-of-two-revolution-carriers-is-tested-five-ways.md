---
id: coaxiality-of-two-revolution-carriers-is-tested-five-ways
kind: issue
title: Coaxiality of two revolution carriers is tested five ways, and glue's coaxial rows skip every pair but cylinder-sphere silently
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

Coaxiality of two revolution carriers is decided separately in:
- `carrier_eq`'s `cylinder_data`/`torus_data` (`crates/topo/src/boolean/carrier_eq.rs`);
- intersect's `cc_coaxial` and the cone×cylinder coaxial test (`crates/geom-brep/src/intersect.rs`);
- `glue::coaxial_rows` (`crates/topo/src/boolean/glue.rs:205`);
- `blend/battery::support_coaxiality` (`crates/sweep/src/blend/battery.rs:1146`);
- `offset_axial`'s frame (`crates/topo/src/offset_axial.rs`).

`glue::coaxial_rows` matches only cylinder×sphere and has `_ => continue` (`glue.rs`, ~l.230-238). It also drops an `Err` from `cylinder_sphere_section` with no row and no refusal (`if let Ok(..)`, ~l.240).

## Owed

One coaxiality reading, one arm per revolution-kind pair, with each consumer reading it. `coaxial_rows`' skip and dropped `Err` should each be either a table arm or a typed refusal.

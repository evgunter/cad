---
id: identity-and-fixture-names-outside-topos-roster
kind: issue
title: geom-brep's read_identity callers and topo's test_support_samples fixture strings spell predicate names no roster reads
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by the review of the topo roster (#6679, S7).

1. **geom-brep's `read_identity` callers** decide identity data under names no roster reads. They are `variety.rs` (`read_identity(residuals, band)`), `pair_section/torus_coincidence.rs` (`read_identity(d.data.map(|(datum, m)| (datum.row(), m)), band)`) and `pcurve_cache.rs` (two calls, over `gates` and `data`). geom-brep runs `predicate_census` only over `curve_face`, in `curve_face::rows::every_door_name_carries_a_listed_prefix`, which does not walk these files. So their datum names are read only at runtime by the K-telemetry census, where a site is reached at all. The census now reports every `read_identity` call as an indirect site, so a geom-brep roster over its whole `src` would list each and declare where its data's names are spelled.
2. **`crates/topo/src/test_support_samples.rs` spells boolean predicate names as fixture strings.** `"carrier_sphere_radius"` is the witness predicate a sample asserts, and `"carrier_cyl_radius"` is a contradiction's margin name. Both names are now `const`s in `boolean::carrier_eq`, private to the boolean, so the fixture is a second spelling that does not follow a rename. Routing it through the const means giving `test_support_samples` a path to it (`pub(crate)` on the const and the module path), or having the fixture take its name from a decision the boolean raises.

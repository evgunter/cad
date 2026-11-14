---
id: identity-and-fixture-names-outside-topos-roster
kind: issue
title: geom-brep's read_identity callers and topo's test_support_samples fixture strings spell predicate names no roster reads
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-10
pr: 6715
---


Found by the review of the topo roster (#6679, S7).

1. **geom-brep's `read_identity` callers** decide identity data under names no roster reads. They are `variety.rs` (`read_identity(residuals, band)`), `pair_section/torus_coincidence.rs` (`read_identity(d.data.map(|(datum, m)| (datum.row(), m)), band)`) and `pcurve_cache.rs` (two calls, over `gates` and `data`). geom-brep runs `predicate_census` only over `curve_face`, in `curve_face::rows::every_door_name_carries_a_listed_prefix`, which does not walk these files. So their datum names are read only at runtime by the K-telemetry census, where a site is reached at all. The census now reports every `read_identity` call as an indirect site, so a geom-brep roster over its whole `src` would list each and declare where its data's names are spelled.
2. **`crates/topo/src/test_support_samples.rs` spells boolean predicate names as fixture strings.** `"carrier_sphere_radius"` is the witness predicate a sample asserts, and `"carrier_cyl_radius"` is a contradiction's margin name. Both names are now `const`s in `boolean::carrier_eq`, private to the boolean, so the fixture is a second spelling that does not follow a rename. Routing it through the const means giving `test_support_samples` a path to it (`pub(crate)` on the const and the module path), or having the fixture take its name from a decision the boolean raises.

**Closed** (2026-11-10, ONECOPY roster-reach). The home is one harness, keyed the way topo's roster is, with no second list:

1. **A whole-src roster for geom-brep.** `crates/geom-brep/tests/recourse_roster.rs` walks all of `src` on `test_utils::source::roster_violations`, so every `read_identity` call is a listed indirect site (`IDENTITY`), keyed by its place in its file. geom-brep's forwarders take the `_as` suffix and are declared carriers. Its door table is `geom-brep-predicate-names-carry-no-door`. `ladder.rs` itself has since moved to geom-core.
2. **A tree-wide coverage row.** `crates/test-utils/tests/predicate_name_coverage.rs` walks every crate's `src`. A crate that decides a name and runs no roster is red unless `KNOWN_MISSING` lists it with its row, and the list is a two-way ratchet. geom and geom-core run rosters in this PR too. That leaves editor-core listed, as `Roster`, alongside the `ClosedTable` entries for profile's `Gate` table, sweep's blend decisions and geom-brep's pose and pair decisions.
3. **References.** A literal a `predicate` field or comparison names must resolve against the union of every crate's census, each roster crate's built with its own carriers. Otherwise it is marked `// UNDECIDED-NAME: <why>` at the site as a deliberate negative, in test code only, or held by a listed crate at a decide-shaped spelling: a `const`, or a closed table's arm. Since the review, references are also read from every crate's `tests/`. A marked name that resolves is red, and so is a marker in production code. A fixture that outlives a rename is red there. Item 2 here (`test_support_samples`' `carrier_sphere_radius` / `carrier_cyl_radius`) was already gone from main.

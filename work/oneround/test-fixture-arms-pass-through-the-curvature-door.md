---
id: test-fixture-arms-pass-through-the-curvature-door
kind: issue
title: Three test fixtures arm a unit length through Lever::curvature_radius, which neither lever gate counts
status: open
opened: 2026-11-21
---


Found by `oneround/unsourced-stand-ins`'s sweep for a stored coefficient handed to a named lever door (2026-11-21). Three test sites build a fixture's unit arm through the curvature home's bare door, `geom_core::Lever::curvature_radius(1.0)`, where the fixture door belongs (`geom_core::test_support::fixture_lever`):

- `topo::pcurves::decision::decide_plain_joint` (`#[cfg(test)]`): `let reach = || Some(geom_core::Lever::curvature_radius(T::one()));`
- `geom_brep::pair_section::ruling` tests, `a_discriminant_in_the_band_over_the_whole_turn_escalates`'s `Flat::lever`;
- `topo::chord_join::wall_ring_rows`: `lever: geom_core::Lever::curvature_radius(1.0)`.

Neither gate sees them: `lever-doors.sh` counts production references only (`lever_doors.py` reads `gate_rust_code --skip-cfg-test`), and `lever-sources.sh` counts only the fixture doors by name, so a test's stated scale entering through a curvature door is counted nowhere. The fix is the fixture door at each site (raising `lever-fixtures.counts` by three), and a question for the gate's owner (ciw) whether `lever-doors.sh` should refuse its bare curvature doors in test code too.

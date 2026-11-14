---
id: a-solids-own-pierce-blocking-its-pair-has-no-fixture-tier-3-passes
kind: issue
title: The census arm where one solid's own edge pierces block its pair has no fixture tier 3 passes: its only one, a cavity crossing its wall, is now check 10's
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Filed by `onecopy/shell-winding`.

## What

`topo::all contact5_gate_and_beam::a_solid_whose_cavity_crosses_its_wall_blocks_its_pair` was the only fixture for the census arm in which one solid's own edge pierces (`UnrecordedContact` `EdgeFacePierce`) block its pair with another solid. It made the census refuse that pair as `CROSSING` ("another finding reports one part's boundary crossing itself") and read no decided overlap beside the refusal. The fixture is a plate with a cavity poking out through its wall.

Check 10 used to read a shell at its first decisive witness. It now reads every witness (`stands::witness_insides_everywhere`), and refuses that body at tier 3 with `ShellWinding`, so `validate_pseudomanifold` never reaches the census on it. The test is re-pointed to the tier-3 refusal (`a_solid_whose_cavity_crosses_its_wall_is_refused_by_check_10_in_either_order`).

**Since that change, any crossing between two shells of one solid trips check 10.** Each crossing shell has a witness off the other shell, and that witness winds wrong. So no multi-shell fixture can reach this census arm.

The `CROSSING` refusal itself is still asserted elsewhere, by `a_doubled_line_does_not_revert`'s coincident-wedges host (`crates/topo/tests/a_doubled_line_does_not_revert.rs`). What has no fixture is the pierce shape: one solid's own edge-face pierces, no placement read beside them.

## Owed

A fixture that tier 3 passes but whose census sees the solid's boundary crossing itself by an edge-face pierce. The likeliest is a single shell whose boundary crosses itself, which no tier-3 check reads (the embedding certificate is not built). It rests against a second solid, and the census should refuse the pair as `CROSSING` with no decided overlap.

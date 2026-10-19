---
id: a-python-junction-test-builds-a-junction-the-door-now-refuses
kind: issue
title: test_checks.py's TestAProfileJunctionDecidedTangent is red on main since #5377: its same-carrier leg refuses at line_to, and the sqrt(eps) line-arc junction publishes no row
status: open
opened: 2026-10-19
priority: P1
cost: E
---


Found by PR 5407's CI (run 39338440174, python suite, on the
`refs/pull/5407/merge` ref). 5407 touches neither `crates/profile` nor
this test, and its python suite was green at 2369d63 before #5377
(`26e571fc5b`, `paths/joint-reads-its-turn`) reached main.

`crates/pncad-py/tests/test_checks.py`:
- `test_a_junction_on_one_carrier_is_same_oriented` (line 581): the
  `.line_to((10.001 * m, 1e-10 * m))` leg now raises `pncad.PathError`:
  "this junction is tangent at any precision you could care about (turn
  margin 1e-10 m on a 0.001 m arm). Recourse: …". This is #5377's
  same-carrier junction refusing at the door, so the test's premise (a
  corner to the lattice, one carrier continuing to validation) no
  longer builds.
- `test_the_junction_is_one_row_on_its_profile` (line 566):
  `len(rows)` is 0, not 1. The `sqrt(1e-9)` line–arc junction no longer
  publishes its `profile_junction` tangent row.

Needed: decide whether each test's premise is still reachable after
the door change. If it is, re-spell the fixture so it reaches it; if
not, re-pin what the door now does. Then say whether the profile's
junction row is still published where validation decides a tangency.

---
id: a-python-junction-test-builds-a-junction-the-door-now-refuses
kind: issue
title: test_checks.py's TestAProfileJunctionDecidedTangent is red on main since #5377: its same-carrier leg refuses at line_to, and the sqrt(eps) line-arc junction publishes no row
status: closed
opened: 2026-10-19
closed: 2026-10-20
branch: paths/python-junction-main-red
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

## Outcome (`paths/python-junction-main-red`)

Neither premise is reachable from a lattice program, so both rows are
re-pinned to what the door does, as `editor-core`'s
`coincidence_door.rs` twins were in #5377. The class is now
`TestAProfileJunctionTheDoorDecides`:

- `test_a_line_into_an_arc_turning_sqrt_eps_is_a_corner_and_no_row`:
  the `sqrt(1e-9)` line–arc junction builds and validates as a corner,
  with no coincidence row.
- `test_a_leg_on_one_carrier_refuses_at_the_verb`: the 1 mm leg rising
  `1e-10` m off the 10 m line raises `PathError` `junction_tangent` at
  its `line_to`.

**Why no fixture reaches a decided junction.** The door's
`junction_check` and validation's `seg::joint_tangency` read one
`seg::junction_turn` at one arm, `min` of the two legs' arms, on the
band `(ε, Kε)`. The door builds only a turn `≥ Kε`. Validation's two
identity reads run ahead of the turn and are bounded below by it:
`chord_side` is `sin φ` times the leaving line's length, and
`carrier_circles_identity` is `d + |Δr|`, at least the centre offset
the turn levers. So a junction the door builds reads Positive in all
three, unless the stored headings drift from the authored ones by
about `(K − 1)ε`. That drift is a few ulps of the coordinates, which
is `ε` only near `10^7` m at `ε = 1e-9`.

**Probed** at `ε = 1e-9`, by a throwaway `profile` test (`replay` at
`f64` and `Interval`, then `ConstructedProfile::validate`) and through
a local wheel:
- line → arc at the unit arm, turn `f·ε` with `f` from 0.5 to 1000, at
  `x0` = 0, 1e3, 1e6 (and 3e6 to 1e9 in Python);
- line → line rising `f·ε` over 1 mm and 1 m off a 10 m line (in
  Python also at `x0` = 1e6 to 1e9);
- arc → arc on carriers `f·ε` apart;
- a departing leg trimmed by a later `.fillet(ρ)` to 1 mm to 0.1 m,
  turned `0.2ε` to `100ε` at its stored length.

Every case either refuses at the door (`junction_tangent` at or below
`ε`, undecided in the band) or builds with `decided_joints()` empty
and no row. `Interval` agrees, except that it escalates at the band's
edge (`f = 10`) where `f64` builds. The large-coordinate cases only
moved escalations: one validation escalation at `x0 = 3e6`, two at
`1e7`, none decided. The fillet-trimmed leg is read at its trimmed
length.

**`profile_junction` rows are dead from documents.** No document
publishes one: `ValidatedLoop::decided_joints` is reached only by a
raw `Profile` (the `profile` and `sweep` crates' table fixtures).
Recorded on `work/coincide/a-profile-junction-row-has-no-reaching-document.md`,
whose open design question (keep the arm for a future raw door, or
retire it) this does not settle.


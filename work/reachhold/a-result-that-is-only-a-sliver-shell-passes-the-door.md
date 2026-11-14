---
id: a-result-that-is-only-a-sliver-shell-passes-the-door
kind: issue
title: A result that is only a sliver shell passes the door: check 7 exempts its in-band volume sign and check 10 never reads a one-shell solid
status: parked
opened: 2026-10-09
priority: P0
cost: M
refs: [4415]
branch: flush/sliver-only-result-refuses
pr: 4636
blocked_on: [boolean-door-runs-the-census-over-its-result]
---


## What

Found by the door-typing unit (branch `join/door-types-in-band-results`).

D10 (Booleans): a result holds no in-band pair or shell. The gate reads a
shell's role in two places. Check 10 reads it only in a solid of two or
more shells. Check 7 reads a solid's volume sign, and it exempts a sign
in band (`validate.rs` `plus_v_at_target`, the `Certified::Open` arm).
So a result whose only lump is a sliver ships with no reading refusing
it.

**Repro.** Take the census witness pose (`notch307 nt e0 a3 d1e-8`), but
replace the notch prism with its convex piece beyond the notch,
`prism([(2,0), (4,0), (4,2), (2,1)], 1)`. Then `intersect_with(piece,
cube)` at ε = 1e-9 builds one shell. `classify_shells` refuses that
shell `Escalated`, and its certified `V/A` is [3.28872e-9, 3.28872e-9],
wholly in band: the verdict check 10's `ValidationError::ShellRoleUndecided`
carries as `sliver: Some(..)`, but only in a solid of two or more shells.
The intersection still returns `Ok`. At d = 3e-8 the same holds with `V/A` 9.866e-9. At d = −1e-8 the
intersection is empty.

## The shape to give

The door reads check 7's in-band arm (`Certified::Open` with a certified
reading wholly in band) and types it `Escalated { ShellRole }`, the way
`finding_arm` types check 10's. Check 7's at-rest exemption is not
touched. A body at rest that is wholly a sliver is a separate question
from what a boolean may return.

**From PR 4415's first review (NOTE-6).** Keep the in-band judgement in
one home. `RoleUnread::certified_by` is the only reader of the
certified reading's `terminal_sliver`, and `finding_arm` only types
what it found. Typing check 7's `Certified::Open` arm must go through
that same `certified_by` (a `CertifiedSliver`), not through a second
reading of the record.

**From PR 4415's second review (NOTE-2).** The class is "an in-band
shell that is the only shell of its solid", not only a one-shell result.
Check 10 reads roles only within a solid of two or more shells.

The near-tangent witness is refused because its sliver shares the main
lump's solid (`1v1` at d = 1e-8). At d = 1e-7 the result is two solids,
`[1, 1]`, but there the lump's role reads definite, so nothing is in
band.

The pieces sort cannot give an in-band shell a solid of its own: an
unread role blocks the move (`PieceSortError::RoleUnread`). So the class
is reached where the build itself leaves the sliver alone in a solid,
as in the repro above.

## Measured

On main at 19b704d92, ε = 1e-9, the piece
`prism([(2,0),(4,0),(4,2),(2,1)], 1)` ∩ the `notch307 nt e0 a3` cube,
through the plain `topo::intersect` (the row's `intersect_with` was not
re-run: a declaration moves no body).
- d = 1e-8: `Ok`, one solid of one shell. `classify_shells` refuses it
  `Escalated`, walk `V/A` 3.050e-9. The certified enclosure (the
  row's, and the one check 10 quotes in the two-solid case below) is
  [3.28872e-9, 3.28872e-9], wholly in band.
- d = 3e-8: `Ok`, one solid of one shell, walk `V/A` 9.822e-9.
- d = −1e-8: `Ok`, empty.

The same pose with a 0.4 m block added to the piece as a second solid,
standing apart inside the cube, refuses `Escalated { ShellRole }` at
d = 1e-8 on main already. The pieces sort leaves the wedge, whose role
does not read, in the block's solid, so check 10 reads it. A brick slab
beside a taller brick lands the same way. No boolean construction found
here gives a sliver a solid of its own beside a lump.

## Fix

Check 7 (`validate.rs` `plus_v_by_sign`) keeps the certified sliver of
its last open round (`OpenRound::AboutTheBody`). That sliver comes from
`RoleUnread::certified_by`, read through `RoleUnread::sliver`.
- At the target the solid passes as `PlusVVerdict::Sliver`, so the
  at-rest exemption is unchanged.
- A solid's only shell that passed that way is listed on the kept
  `AtRestBody` (`AtRestBody::slivers`). It is listed as an
  `InBandShell`, the one type that check 10's findings also map to in
  `boolean/ops.rs` `finding_arm`.
- The boolean door's `gate` refuses a result holding any such shell
  `Escalated { ShellRole }`, through `escalated`, which it shares with
  `finished_body_refusal`.

**What newly refuses.** The measure of a sole shell is its certified
V/A, the reading check 10 already applies to each shell of a
multi-shell solid. So any boolean result whose only lump is under
about 2Kε thick now refuses instead of shipping. At ε = 1e-9:
- **The wedge.** d = 1e-8 and 3e-8 refuse; d = 3.05e-8 builds.
- **A planar slab** (a unit brick ∩ a brick from z = 1 − t).
  - t = 1e-8 ships `Ok` on main and now refuses `ShellRole`, with
    certified V/A 5e-9.
  - t = 1.9e-8 also refuses.
  - t = 2.1e-8 builds on both.
  - t ≤ 5e-9 refuses earlier as a coincidence, on both.

A thin-walled hollow is a solid of several shells whose total is in
band while each shell decides. It still ships:
`a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses`.

**Tests** (`topo/tests/sliver_shell_role.rs`):
- `a_result_that_is_only_a_sliver_refuses_in_band` and
  `a_slab_thinner_than_two_bands_refuses_and_a_thicker_one_builds` go
  red on main.
- `a_wedge_sharing_a_lumps_solid_refuses_through_check_10` guards check
  10's existing path for the one landing found. It holds on main too.
- `a_thin_walled_hollow_ships_while_a_slab_as_thin_refuses` pins
  today's hollow.

## Closed

PR 4636. The boolean door now refuses a solid whose only shell check 7
certified wholly in band, `Escalated { ShellRole }`, through the same
`escalated` helper as check 10's findings. Its tests:
- `a_result_that_is_only_a_sliver_refuses_in_band`, the row's repro;
- `a_slab_thinner_than_two_bands_refuses_and_a_thicker_one_builds`.

Both are red on main. The review's MINOR-2 is now its own row,
`a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses` (design).
The dual scalar, which runs no at-rest gate, reads no slivers.

## Re-opened: the gate half, under pairs only (PR 4783, 2026-10-12)

Ev's ruling on PR 4690 (fork log row 113) retired the fix above: a
boolean judges how thin its result is by its pairs of cells alone, so
PR 4783 removes the sole-shell `V/A` gate this row's PR built. A result
that is only a sliver must now refuse on an in-band pair. The door
refuses the pairs its own build decides, but not the pairs only the
census reads, and it runs no census. So this row's residue is the door
census: `work/reachhold/boolean-door-runs-the-census-over-its-result.md`,
which it is parked on. It keeps P0 so the inversion stays visible:
results this row's PR refused now ship holding an in-band pair.

Evidence (the full list is on the REACHHOLD row):
- **11** notch307 needles at ε 1e-9 now ship with `pm_census_ee_span`
  in band (R2's `crates/topo/tests/review_4783_r2_probes.rs`,
  `flush/thinness-by-pairs-review-r2` @ ef12bed5), among them this row's
  own repro, the piece ∩ cube at d = 1e-8, in both orders;
- **34** `rc_wide_battery` lines at ε 1e-6 move from `ShellRole` to
  built (R1's `review-sliver-probe`, `flush/thinness-by-pairs-review` @
  a79ba445).

`topo/tests/sliver_shell_role.rs`
`a_needle_ships_an_in_band_edge_pair_until_the_door_runs_the_census`
pins the repro's rows; it goes red when the door runs the census.

## 2026-10-17: the needle's in-band edge pair was the census's arithmetic (RESTREAD)

The `pm_census_ee_span` escalations on the `notch307` needle
(`topo/tests/sliver_shell_role.rs`) were two edges leaving the corner's
one point key whose crossing the closed form, read from the edges'
starts, put 1.4e-9 to 2.8e-9 off that point. Their far ends part by
8.4e-8 to 2.5e-7: a definite number under DESIGN.md tier 3′. RESTREAD's
crossing lane reads the crossing from the shared point
(`the-census-crossing-lane-misplaces-a-shared-points-crossing-on-a-near-collinear-pair`),
and every run of the needle now passes the census at rest. The test is
renamed `a_needle_reads_every_pair_of_its_cells_definite` and pins that.
So a door census leaves these runs green. R2's other notch307 needles
carry the same span escalation and are likely the same shape; they were
not re-measured.

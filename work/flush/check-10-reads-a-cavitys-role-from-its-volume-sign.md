---
id: check-10-reads-a-cavitys-role-from-its-volume-sign
kind: issue
title: Check 10 reads a cavity's role from its V/A, so a definite thin cavity (brick minus slab) refuses ShellRole though its faces are a definite pair
status: closed
closed: 2026-10-19
opened: 2026-10-11
priority: P2
cost: M
branch: flush/shell-role-reader
pr: 5119
---


Found by FLUSH's thin-hollow design fork (fork log row 113). Measured
on main at ε = 1e-9: `brick (−1,2)³ ∖ slab(1.5e-8)` refuses
`Escalated { ShellRole }` on the slab-shaped cavity (V/A −7.5e-9).
The cavity's faces are a definite pair 1.5e-8 apart, and the same slab
alone builds.

Both designers keep check 10's refusal for now, but one of them leans
towards reading a shell's role off a crossing ray rather than V/A. With
that, the "shell" half of D10 would reduce to pairs. That is a
follow-up question after the row-113 ruling, not part of it. It
depends on the containment fix in
`work/cleave/a-missed-ray-reads-the-solids-volume-sign-for-its-side`.

## Measured (CLEAVE `cleave/missed-ray-side`, 2026-10-11)

The containment fix this row depends on is on that branch: a missed
ray's side is read off a ray aimed at a planar face wherever the
volume does not decide it. With it, `brick (−1,2)³ ∖ slab(t)` still
refuses `Escalated { ShellRole }` on the cavity for every
`t ∈ {10, 12, 15, 20}ε` at ε 1e-9. That is this row's reader.

The same reader, a shell's role off its `V/A`, also serves the public
`topo::classify_shells`. On a lone at-rest slab `t` thick it refuses
`Escalated { chk_shell_volume_sign }` with margin `t/2`, for every `t`
above at ε 1e-9 and 1e-6. A one-shell solid's shell is its outer shell,
so this is not a question about the slab. `editor-core`'s expected-
components check (`checks.rs`, `classify_shells`) therefore refuses a
thin body it could count. The readers that share it are
`props::shell_role_read` (`stands::ShellRead`), the result sort
(`pieces.rs`), `shell.rs`'s void read, and the census's `gate_role`.

## Measured (FLUSH `flush/thinness-by-pairs`, 2026-10-12)

With PR 4636's sole-shell gate retired (Ev, PR 4690), this row's reader
is the one thing that still refuses a slab whose pairs are all
definite, at ε 1e-9 for every `t ∈ {10, 12, 15, 19}ε`:
- `brick (−1,2)³ ∖ slab(t)` refuses `Escalated { ShellRole }` on the
  cavity, as before.
- `slab(t) ∪ brick (5,6)³`, in either order, refuses
  `Escalated { ShellRole }` on the slab. The union assembles both
  shells in one solid. The pieces sort (`pieces.rs` `pieces_of`) moves
  a shell out only on a decided `Outer`, and the slab's role reads off
  its in-band `V/A`. So the slab stays in the brick's solid, and check 10
  refuses it there. `topo/tests/missed_ray_side.rs`
  `a_thin_slab_is_a_legal_operand_against_a_brick_far_away_and_its_union_with_it_refuses`
  pins it.

At 21ε all three build. A role read off a crossing ray would let both
build.

## Ruling

Ev approved the shell-role fork on PR 4990 (2026-10-14: "the recommendation sounds good"), including check 7 on the same reader ("three readers of one fact become one … an extremely good reason"). The design is in DESIGN D10 and D1, and fork-log row 138 records the answer.

The implementation is this row's remaining work:
- one role reader: aimed rays first, the certified volume only where no face offers an aim;
- it serves check 10, the pieces sort, `shell`'s cavity read, the census's `gate_role`, `classify_shells`/`classify_shells_of`, `solid_contain`'s missed-ray side and check 7;
- curved aim points as a second step.

The aim's incidence is the implementer's, with a probe: fixed ~60°, or along the normal with advances measured from the aim point. See the 2026-10-13 log entry and `analysis/flush-shell-role-reports`.

## Measured (FLUSH `flush/shell-role-reader`, main `64752132d`, 2026-10-14)

On main, at ε 1e-9 and 1e-6, for every `t ∈ {10, 10.5, 12, 15, 19}ε`:
- `brick (−1,2)³ ∖ slab(t)` refuses `Escalated { ShellRole }` on the
  cavity's certified `V/A`, `−t/2` (e.g. −5.25e-9 at 10.5ε, ε 1e-9);
- `slab(t) ∪ brick (5,6)³` and `brick ∪ slab` refuse the same on the
  slab, `+t/2`;
- `classify_shells` on a lone slab refuses
  `Escalated { chk_shell_volume_sign }`, margin `t/2`;
- the inside-out slab finishes at rest (`Validated`).
At 9.9ε the slab does not build (the prism door's `ParamSpan`).

On the branch every one of those builds or reads: the cavity is a
`Void` of a one-solid, two-shell result, the union is two solids, the
lone slab classifies `Outer`, and the inside-out slab refuses at rest
(`NegativeVolume`).

**The aim.** A probe on the branch (not committed; its numbers are in
the PR body) read a slab in the plane `x = y` and an axis slab at
`t = 10ε = Kε` exactly, 10.5ε and 12ε, under four schemes:
- the operand gate's far point (its diagonal lies in the `x = y`
  slab's plane): in band at every `t` on the `x = y` slab;
- along the normal from `2·reach + 1`, advances from the start: in band
  at `Kε` on the axis slab at ε 1e-9 (9.999999939ε), the probe's known
  reading;
- along the normal, advances from the aim point: in band at `Kε` on the
  `x = y` slab beside a far brick at ε 1e-9 (9.999999939ε);
- at 60° to the face, from twice the reach: decides everywhere, its
  gap `2t` (19.9999994ε at `Kε`), a full `Kε` of slack.
The reader aims at 60°.

## Fix (`flush/shell-role-reader`)

- **One reader.** `boolean/solid_contain.rs` `side_at_infinity`: aimed
  rays at 60° from twice the selection's reach, turned off the
  coordinate planes by the golden angle, one per face that offers an aim
  (a planar face's certified interior point, a cylinder's, cone's or
  torus's window middle, a closed sphere's chart point), in face order;
  the first that decides answers. Its planar rows log as
  `side_at_infinity_*`, apart from point containment's. A pair in band on two faces that do not meet
  refuses (`SideUnread::InBand`, naming the aimed face, both faces and
  their gap along the ray, a magnitude). Two faces that meet are a
  sharp corner, not a pair: such a ray is set aside, and its pair is
  the refusal only where the volume does not read either. The volume
  reads only where no aim does.
- **Its readers.** `props::shell_role` (a solid's only shell `Outer` by
  arity, else the reader; `shell_volume_role` its fallback) serves
  check 10 (`validate.rs` `shell_winding_errors`), the pieces sort
  (`pieces.rs` through `stands::ShellRead`), `shell.rs`'s cavity read,
  the census's `gate_role`, and `classify_shells`/`classify_shells_of`.
  `solid_contain`'s missed-ray side (`missed_side`) reads the same
  reader, with the closed-form volume (`at_infinity_side`) as its
  fallback. Check 7 (`validate.rs` `plus_v_errors`) reads each solid's
  faces with it, the volume walk kept for the certificate and as the
  fallback; `OrientationUndecided` is a solid no reading decides.
- **Retired.** `AtRestBody::slivers`, `PlusVVerdict::Sliver`, the
  operand gate `gate_sliver_orientation`, `InBandShell`,
  `CertifiedSliver`, `ShellRoleUndecided`'s `sliver`, `RoleRefusal`,
  `role_at_target`, `SHELL_ROLE`'s "thickness" size.
- **Pins.** `crates/topo/tests/shell_role_reader.rs`.

Filed: `a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass`
(a wrong body main ships, which the reader now refuses) and
`the-side-at-infinity-reader-offers-no-aim-on-a-trimmed-sphere-or-a-spline`
(the curved step's residue; cylinders, cones, tori and closed spheres
are aimed at in this PR).

## Fix pass (PR 5119's reviews, 2026-10-14)

- The reader moved to `crates/topo/src/side_reader.rs`; every ray it sets
  aside is recorded (`SetAside`), a sharp corner is never a pair, and a
  face no ray can cross (a spline) casts no ray (`ray_crosses`, the one
  list `face_geo` also refuses through).
- One home for the certified volume sign (`props::role_walk`), check 7's
  included.
- The door escalates every reading in band: an aimed pair
  (`ShellRole { near, far, others }`, one lever `AIMED_PAIR`) and a solid
  no ray read whose volume is in band (`SolidSide`).
- Filed the needle class (P1, needs Ev) and the volume backstop's `+V`
  posture (P3); re-aimed the fin row at the at-rest crown.

## Resolution

Closed by PR 5119 (2026-10-19): one reader of a shell's role and a
solid's side, `side_at_infinity` in `crates/topo/src/side_reader.rs`.
- **The reader:** aimed rays, the first that decides answering. Where
  no ray reads, the volume's sign is read from its certified enclosure
  alone, through `props::role_walk`, the one home. Check 7, check 10,
  `props::shell_role`, the pieces sort and the census all read through
  it. The sliver hand-offs are retired.
- **Ev's needle ruling** (fork-log row 176): the volume's sign is never
  held against the band.
- **Thin cavities:** the brick-minus-slab cavity, and needle cavities
  down to the join's own limit, read `Void`.
- **Residues:**
  - `the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite`
    (closed with this PR);
  - `a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face`
    (next unit);
  - `the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`;
  - `revalue-pairs-the-ray-casters-decisions-only-to-their-first-difference`;
  - `a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read`.

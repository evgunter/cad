# CONTACT-6: a split's section faces take their sense from their winding

This file is the PR body for the landing. The orchestrator folds it into
that PR and deletes it at merge.

Carries `work/contact/point-in-solid-reads-out-inside-a-tilted-cut-cylinder-cavity`
(P0). It includes the single full review's fix pass (MAJOR 1, S1–S6).

## What was wrong

`point_in_solid` answered `Out` inside the lower half of a brick with a
rod subtracted, split by a plane at tilt 1.0. The cause is not in the
ray lane. It is in the body the split hands back: one of the lower
half's two section faces carried `sense: false` on a chart whose normal
already points out of material.

### The trace

The fixture is built through public doors:
- `brick([−2, 2]² × [0, 2.5])`;
- minus a unit rod about `z` (`prism_at`, `z ∈ [−0.5, 3]`) through
  `topo::subtract`;
- split by `split` through `(0, 0, 1.25)` with normal
  `(sin 1, 0, cos 1)`.

The plane crosses the bore's wall along a line, so each half's section
is two planar faces, one on each side of the bore in `y`. Each is
bounded by three lines and an ellipse arc.

Probe `(−16/9, 0.9467, 1.8778)`, identity pose, lower half:
1. The pre-pass places it off every face.
2. Ray `+x` (the schedule's first):
   - it meets side face 6v1 at `t = −0.22` (behind the point, dropped);
   - section face 15v1 (`y < 0` side): the hit is outside the face;
   - section face 16v1 (`y > 0` side) at `t = 1.3747`: the hit is
     inside the face.
3. `face_geo` hands the ray lane 16v1's outward normal as
   `−(sin 1, 0, cos 1)`, because the face's sense is `false`.
   `d·n < 0` reads as an entry, and the closest-hit rule answers `Out`.

16v1's outer loop winds counter-clockwise about `+n` (area·n = +4.1698,
the same as 15v1's). Only its sense says otherwise. Tier 3 agrees:
`validate_geometric` returns `LoopRoleInverted { face: 16v1 }` on the
lower half, at the base and at CONTACT-4's head. The row's premise that
it passes does not hold for this fixture. The volume is exactly half
because the props lane integrates the loop windings, not the bit.

### The defective site

`splitting::finish::split_finish`, the promotion loop:
- `mfkrh(ring, New(plane))` promotes the ring;
- `set_face_surface(section.face, New(plane))` re-charts the null face.

The null face was minted by the join's `mef` on an operand face's
surface, and it inherited that face's sense
(`mint_face_surface_and_sense`). `set_face_surface` onto a `New`
surface keeps the bit it finds. So a null face carved from the reversed
cavity wall carried `false` onto the section plane, whatever the
section's winding.

That is two rules for one fact:
- the mint stamps a bit (`mint_face_surface_and_sense`);
- a re-chart keeps one (`set_face_surface`).

Every re-chart caller has to remember to reset the bit.

## The fix

**One door states the bit with the chart.**
`Body::set_face_surface_and_sense(face, surface, sense)` re-charts and
writes the sense in one call. `set_face_surface`'s doc now says it keeps
the bit, and names the new door for a caller that decides the new
chart's orientation. The re-chart callers that already paired the two
calls now use it:
- `sweep::blend::surgery`: blend, corner-patch and band faces (three
  sites);
- `step_import::adopt`: every adopted face;
- `topo::splitting::finish`: both section faces.

**Each section face's sense is its loop's winding about its chart
normal.** This is the reading tier 3's check 6 falsifies the bit with,
through the same function, `Body::planar_loop_winding`. The new helper
`finish::section_sense`:
- takes the reading before the re-chart;
- maps `Positive` to `true` and `Negative` to `false`;
- refuses `SplitFinishError::SectionWindingUndecided { face, diag }`
  where the winding has no sign: in the band, zero, or a loop on a
  spiric or NURBS carrier.

The promoted face goes through `mfkrh(ring, Inherit)` and then the same
door, so both faces take the one reading.

What the reading gives:
- **Cut cavity at tilt 1.0:** both section faces on either side of the
  bore wind counter-clockwise, so both are `true`.
- **Hole class** (a cut that crosses the bore all round): the section
  is a face over the whole outline, with no ring, plus a separate disc
  over the bore that cancels it. The disc winds clockwise about its
  side's outward normal, so it is `false`.

The first version of this fix stamped `true` on both faces. That was
wrong on the hole class's disc: it made both halves fail check 6 where
the base failed one (MAJOR 1). **The square-plus-disc encoding of a
section with a hole stands.** A single annular face with a ring is
REACH's defect, filed by the orchestrator, and this PR does not touch
it.

### The new refusal

`SectionWindingUndecided` is reached only where check 6 itself could
not read the face:
- a winding in the band, or zero;
- an edge with no certified curve.

None of the rows reach it. The split's operand gate
(`classify.rs`) already refuses a spiric or NURBS edge, and a cone,
sphere or torus face. So a section loop rides only line, circle and
ellipse edges, which the winding reads exactly.

The variant is `SplitError::Finish`, so every consumer already matches
it as a split refusal. Its `Display` follows `DescribeEscalated`: the
escalated form quotes the payload and the shared split-plane recourse.

## Measurement

Base is main before CONTACT-4 (`a8f006ab5`). Head is this branch.
Counts are summed over the six poses of `pis_arc_capped_poses::poses`,
on the 9³ + 11³ grid of points at least 1e3·ε from every boundary
surface. They are identical at ε = 1e-9, 1e-6 and 1e-12, except where
noted.

### `point_in_solid`: answered / wrong / refused

| case | half | base | head |
|---|---|---|---|
| cavity cut at tilt 1.0 | below | 3741 / **1274** / 7345 | 5015 / **0** / 7345 |
| | above | 11428 / 0 / 932 | same |
| cavity uncut | | 12360 / 0 / 0 | same |
| cavity cut flat | both | 12360 / 0 / 0 | same |
| cavity cut at tilt 0.3 | below / above | 5825 / 0 / 6535; 8990 / 0 / 3370 | same |
| off-centre cavity (rod at `(0.8, 0)`) cut flat | both | 12360 / 0 / 0 | same |
| off-centre cavity, tilt 1.4, flipped | below / above | 10032 / 0 / 2328; 6120 / 0 / 6240 | same |
| bored cylinder cut flat | both | 12360 / 0 / 0 | same |
| bored cylinder, tilt 0.3 | below / above | 5299 / 0 / 7061; 8605 / 0 / 3755 | same |
| bored cylinder, tilt −1, flipped | below / above | 6071 / 0 / 6289; 8766 / 0 / 3594 | same |

- Every refusal is `VolumeUncertified` on a point outside the body.
- The one exception is the bored cylinder at tilt 0.3 at ε = 1e-6: 3
  in-band `bool_wall_trim` escalations per half (answered 5297 and
  8603), at base and head alike.
- The only change is the cavity at tilt 1.0, below: 1274 false `Out`
  become right. CONTACT-4's head measures exactly as the base on it.

### `validate_geometric`, and section-face senses

| case | half | base: tier 3 / section senses | head: tier 3 / section senses |
|---|---|---|---|
| cavity cut at tilt 1.0 | below | **`LoopRoleInverted`** (section) / true, false | clean / true, true |
| | above | clean / true, true | clean / true, true |
| cavity cut flat | below | clean / false, true | clean / false, true |
| | above | **`LoopRoleInverted`** (section: the disc) / true, true | clean / false, true |
| cavity cut at tilt 0.3 | below | clean / false, true | clean / false, true |
| | above | **`LoopRoleInverted`** (disc) / true, true | clean / false, true |
| off-centre cavity cut flat | below | clean / false, true | clean / false, true |
| | above | **`LoopRoleInverted`** (disc) / true, true | clean / false, true |
| off-centre cavity, tilt 1.4, flipped | below | `RingMeetsOuter` (cap) / true, false | `RingMeetsOuter` (cap) / true, false |
| | above | **`LoopRoleInverted` ×2** (section + cap) / true, true | `LoopRoleInverted` (cap) / true, false |
| bored cylinder cut flat | below | clean / false, true | clean / false, true |
| | above | **`LoopRoleInverted`** (disc) / true, true | clean / false, true |
| bored cylinder, tilt 0.3 | below | clean / false, true | clean / false, true |
| | above | **`LoopRoleInverted`** (disc) / true, true | clean / false, true |
| bored cylinder, tilt −1, flipped | below | `RingMeetsOuter` (cap) / true, false | `RingMeetsOuter` (cap) / true, false |
| | above | **`LoopRoleInverted` ×2** (section + cap) / true, true | `LoopRoleInverted` (cap) / true, false |

The head is no worse than the base in any row, and **no section face
fails check 6 anywhere**.

The findings left over are all on OPERAND cap faces, under the two
steep flipped cuts, and are the same at base:
- below: the cap's ring touches its outer loop (`RingMeetsOuter`);
- above: a cap fragment is inverted (`LoopRoleInverted`).

They are filed on REACH's slate as
`work/reach/split-leaves-a-ringed-cap-fragment-invalid-under-a-steep-cut`.
That file also records that nearby rod positions refuse the split
(`RingHomingAmbiguous`, `TornComponent`) at base and head alike.

## Rows

`crates/sweep/tests/pis_cut_cavity.rs`: nine cases, each case's floor,
escalation cap and tier-3 residue in its own record.
- `every_cut_through_a_bore_reads_its_truth_at_every_pose`: every
  answer is the truth. The refusals admitted are:
  - `VolumeUncertified` on a truth-`Out` point;
  - in-band escalations, up to the measured cap.

  The answered counts are floored at the measured minima.
- `every_section_face_passes_check_6`: no section face is
  `LoopRoleInverted` in any case or half, and tier 3 finds nothing
  beyond the stated residue (`STEEP_RESIDUE` on the two steep cuts).
- `the_traced_probe_reads_in`: the traced probe and the row's example
  column `(−1.422, 1.113, z)` read `In`.

Red without the reading:
- **Leave the inherited bit** (the base's behaviour): all three rows
  red. That is 1275 problems on the first, the traced probe reads
  `Out`, and check 6 fails on the discs and on 16v1.
- **Stamp both `true`** (this PR's first version): the check-6 row is
  red on every hole-class case.

`pis_arc_capped_poses::poses` is `pub(crate)` so the suite shares the
six poses. `sweep`'s `test_support` has no pose home (it holds
fixtures, not rigid maps), so they stay in the test tree (S5).

## Docs corrected (S2)

These said check 6 does not examine loops riding an `Ellipse`. It does:
the arm reads `Line`, `Circle` and `Ellipse` loops.
- `set_face_sense`'s doc (`attach.rs`);
- `ValidationError::LoopRoleInverted`'s doc (`validate.rs`).

`set_face_sense`'s "live case of the mint" line now names the re-chart
door and the winding reading.

## Class sweep

The shape: **a face whose sense came from an inheriting mint, re-charted
onto a new surface without the bit being set again.**

Pass 1, `set_face_surface(` over every crate, outside test modules and
probes:

| site | disposition |
|---|---|
| `topo/splitting/finish.rs` | **fixed**: `set_face_surface_and_sense`, from the winding |
| `step-import/adopt.rs` | moved onto `set_face_surface_and_sense` (it set `spec.sense` right after) |
| `sweep/blend/surgery.rs` (three) | moved onto `set_face_surface_and_sense` (each set the sense right after) |
| `sweep/extrude.rs`, `loft.rs`, `revolve/partial.rs` (cap) | the swept seed face is minted `true`, and the plane is fitted outward; never inherited |
| `sweep/revolve/full.rs` (`Shared(wall0)`) | takes wall 0's sense explicitly |
| `topo/offset_axial.rs`, `offset_together.rs`, `replace_face.rs` | move a face onto a moved copy of its own chart (same normal direction), so the kept bit is still honest |
| `topo/readback.rs` | doc examples on a fresh `mvfs` face |

Pass 2, direct `.surface =` writes: `boolean/combine.rs` re-keys during
a copy (same surface, same bit). The rest are in `#[cfg(test)]`
modules.

Pass 3, the other null-face promotions (`mfkrh(`,
`clear_null_face_pair`):
- `boolean/finish.rs` and `boolean/rest.rs` promote with `Inherit` and
  stay on the parent's surface, so the inherited bit is right;
- `shell.rs` sets `host_sense` explicitly;
- `splitting/reassembly.rs` is a test oracle.

Blind spot: a door that changes a face's surface without either
spelling. Only `combine.rs` does, and it keeps each surface.

## Filed

`work/reach/split-leaves-a-ringed-cap-fragment-invalid-under-a-steep-cut`,
the operand cap findings above.

The row file's body is corrected to the traced cause, and its status is
left to the orchestrator (S6).

## Local results

At the fix-pass head. CI is the verification of record.
- `cargo test -p topo -p sweep` at ε default, 1e-6 and 1e-12: all
  green (topo lib 879, topo `all` 685, sweep `all` 1714 with 7
  ignored). The first battery caught the new door missing from the two
  mutation-door tables (`pcurves::staleness_posture`,
  `review_m1_pr5_internal`); topo lib was rerun green at all three rows
  after the fix.
- `pis_cut_cavity` at all three ε rows: 3 of 3.
- `cargo test -p step-import`: 201 passed, 1 ignored, plus 77.
- `editor-core` concision rows: 7 of 7. `cargo test -p test-utils`:
  green.
- clippy `-D warnings` on topo, sweep and step-import, all targets;
  rustdoc `-D warnings --document-private-items --all-features` on the
  same three; `cargo fmt --check`; every `scripts/gates/*.sh`: clean.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL

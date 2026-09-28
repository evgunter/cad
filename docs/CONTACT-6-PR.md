# CONTACT-6: a split's section faces face out

This file is the PR body for the landing. The orchestrator folds it into
that PR and deletes it at merge.

Carries `work/contact/point-in-solid-reads-out-inside-a-tilted-cut-cylinder-cavity`
(P0).

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

The plane crosses the bore, so each half's section is two planar
faces, one on each side of the bore in `y`. Each is bounded by three
lines and an ellipse arc.

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

Face 16v1's outer loop winds counter-clockwise about `+n`: its area
vector dotted with `+n` is `+4.1698`, the same as 15v1's. Its sense is
the only thing that says otherwise. Tier 3 says the same:
`validate_geometric` on the lower half returns
`LoopRoleInverted { face: 16v1 }`, at the base and at CONTACT-4's head.
The row's premise that it passes does not hold for this fixture. The
volume is exactly half because the props lane integrates the loop
windings, not the bit.

### The defective site

`splitting::finish::split_finish`, the promotion loop (`finish.rs:320`):

```rust
let promoted = body.mfkrh(ring, FaceSurface::New(plane_for(ring_side)))?;
body.set_face_surface(section.face, FaceSurface::New(plane_for(other_side)))?;
```

`plane_for` charts each section face with its OUTWARD normal
(module docs: above `m = −n_SP`, below `m = +n_SP`). So the invariant
`Face::sense` states, "`true` iff the material side agrees with the
chart normal", demands `true` on both.
- The promoted face gets `true` from `mfkrh`'s mint onto a `New`
  surface.
- `section.face` is the null face. The join's `mef` minted it on an
  operand face's surface and so inherited that face's sense
  (`mint_face_surface_and_sense`). `set_face_surface` re-charts a face
  and keeps its bit.

A null face carved from the reversed cavity wall (`sense: false`)
therefore kept `false` on a chart that is already outward. The upper
half's two section faces, and every uncut body, came out right only
because their null faces were carved from `sense: true` faces.

## The fix

The promotion loop now sets both section faces to sense `true`, with
the argument at the site:
- the chart is outward by `plane_for`'s derivation;
- the null face's bit is its parent's, and a reversed wall's is
  `false`.

The fix is at the writer, so every reader of the bit is fixed with it:
- the ray lane's crossing sign;
- `face_outward_normal` and the other `face_normal` doors;
- tier 3's check 6.

Nothing answers where it used to refuse, and nothing refuses where it
used to answer: the refusal counts are unchanged.

## Measurement

The grid is 9³ plus 11³ over the brick, keeping only points whose
analytic distance from every boundary surface (six brick planes, the
rod, the cut plane) is at least 1e3·ε. That leaves 2060 points per
pose, at the six poses of `pis_arc_capped_poses::poses`.

Base is main before CONTACT-4 (`a8f006ab5`, also measured at
`94d5b265f`: identical). Head is CONTACT-4's head without this fix.
Every count is identical at ε = 1e-9, 1e-6 and 1e-12.

| lower half, per pose | base right / wrong / refused | CONTACT-4 head | this branch |
|---|---|---|---|
| identity | 551 / 272 / 1237 | 551 / 272 / 1237 | 823 / 0 / 1237 |
| 0.7 about x | 551 / 272 / 1237 | same | 823 / 0 / 1237 |
| 0.7 about z | 766 / 103 / 1191 | same | 869 / 0 / 1191 |
| 0.3 about y | 560 / 260 / 1240 | same | 820 / 0 / 1240 |
| 1.1 about (1,2,3) | 762 / 95 / 1203 | same | 857 / 0 / 1203 |
| π/2 about x | 551 / 272 / 1237 | same | 823 / 0 / 1237 |
| total | 3741 / **1274** / 7345 | 3741 / **1274** / 7345 | 5015 / **0** / 7345 |

- **CONTACT-4 did not fix it.** It changed nothing here, as expected:
  the in-face walk was right on both faces, and the wrong sign came
  from the sense bit.
- Every wrong answer was a false `Out`.
- **Every refusal is `VolumeUncertified` on a point outside the body.**
  Its first ray meets nothing, and the cut's elliptic wall has no
  closed-form volume (the props lane's, and unchanged).

Controls, identical at base, CONTACT-4's head and this branch, at every
ε row:
- the upper half: 11428 right, 0 wrong, 932 `VolumeUncertified`;
- the uncut cavity: 12360 right, 0 wrong, 0 refused.

## Rows

`crates/sweep/tests/pis_cut_cavity.rs`:
- `the_cut_cavity_reads_its_truth_at_every_pose`:
  - the lower half, the upper half and the uncut cavity, at six poses;
  - every answer is the truth;
  - the only refusal admitted is `VolumeUncertified` on a truth-`Out`
    point;
  - the answered counts are floored at the measured 5015, 11428 and
    12360.
- `the_traced_probe_reads_in_and_every_section_face_faces_out`:
  - the traced probe, and the row's example column
    `(−1.422, 1.113, z)` for `z ∈ {0.128, 0.628, 1.128, 1.628}`, read
    `In`;
  - both halves pass `validate_geometric`;
  - each half has exactly two section faces, both sense `true`.

With the fix reverted, both rows are red: 1275 problems (1274 wrong
plus the floor), and the traced probe reads `Out`.

`pis_arc_capped_poses::poses` becomes `pub(crate)` so the new suite
shares the six poses rather than copying them.

## Class sweep

The shape: **a face whose sense came from an inheriting mint (`mef`,
`mfkrh(Inherit)`), re-charted onto a new surface without the bit being
set again.**

Pass 1 grepped `set_face_surface(` over every crate. The hits outside
test modules and probes:

| site | disposition |
|---|---|
| `topo/splitting/finish.rs` (promotion loop) | **fixed** |
| `step-import/adopt.rs` | sets `spec.sense` right after: not this class |
| `sweep/blend/surgery.rs` (three sites) | sets the band's sense right after (`set_face_sense`): not this class |
| `sweep/extrude.rs`, `loft.rs`, `revolve/partial.rs` (top/end cap) | the swept seed face is minted `true` by the constructor and the plane is fitted to be outward: never inherited |
| `sweep/revolve/full.rs` (`Shared(wall0)`) | takes wall 0's sense explicitly: not this class |
| `topo/offset_axial.rs`, `offset_together.rs`, `replace_face.rs` | re-chart the same face onto a moved copy of its own chart (same normal direction), so the kept bit stays honest |
| `topo/readback.rs` | doc examples on a fresh `mvfs` face |

Pass 1 could not see a surface written without that door. Pass 2
grepped `.surface =` for direct writes. The hits:
- `boolean/combine.rs` re-keys surfaces during a copy (same surface,
  same bit);
- the rest (`merge_faces.rs`, `query.rs`, `solid_contain.rs`,
  `review_m1_pr3.rs`) are in `#[cfg(test)]` modules.

Pass 3 covered the other null-face promotions, grepping `mfkrh(` and
`clear_null_face_pair`:
- `boolean/finish.rs` and `boolean/rest.rs` promote with
  `FaceSurface::Inherit`. The face stays on its parent's surface, so
  the inherited bit is the right one.
- `shell.rs`'s rim promotion sets `host_sense` explicitly.
- `splitting/reassembly.rs` is a test oracle.

The blind spot is a surface change by a door that neither writes
`.surface` nor calls `set_face_surface`, such as a whole-body copy that
maps faces. Only `combine.rs` does that, and it keeps each surface.

## Nothing filed

The row's claim that `validate_geometric` passes is answered above: it
fails at the base on this fixture. The split verb runs no closing tier-3
check, so the defect shipped silently, but that is the verb's documented
contract (callers validate), not a new finding.

## Local results

- at the fix commit `d3d9c57c0` (before merging main):
  - `cargo test -p sweep --test all`: 1703 passed, 7 ignored (ε
    default);
  - the `pis_` rows green at 1e-6 and 1e-12;
  - `cargo test -p topo --lib split` (52) and `--test all split` (36)
    green;
  - `cargo clippy -p topo -p sweep --all-targets -- -D warnings` clean;
  - `cargo fmt --check` clean;
- after merging main (CONTACT-4 landed): the `pis_` rows (11) and
  `topo --lib split` (52) green again.

CI is the verification of record.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL

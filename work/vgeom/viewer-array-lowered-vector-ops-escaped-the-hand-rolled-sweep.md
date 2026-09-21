---
id: viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep
kind: issue
title: the hand-rolled vector-op sweep missed its own dot half, and one datum row asserts something no value can break
status: review
opened: 2026-09-16
priority: P3
cost: E
branch: vgeom/deletions
pr: 3027
---


Found by the VIEW review of #2783, which is also the unit that ran the
sweep this row is about.

## The sweep named three patterns and the diff deleted four things

#2783's class sweep stated its patterns as *"a local normalize, a
hand-rolled cross, a least-aligned-axis branch"*. The diff deleted
**four** helpers from `crates/viewer/src/datums.rs` — `unit`, `cross`,
`dot` and the seed rule — so `dot` was a member of the class the sweep
was sweeping for and was never swept for.

The hit that escaped: `crates/viewer/src/display.rs`, inside
`is_rigid` —

```
let dot = |a: [f64; 3], b: [f64; 3]| a[0]*b[0] + a[1]*b[1] + a[2]*b[2];
```

the same shape as the deleted `datums::dot`, one file over. Its likely
disposition is the one `scene.rs`'s array cross already got — the data
are `[f64; 3]` and not `Vec3`, so there is no door to call without a
lowering — but **a hit with no line is a claim, not a receipt**, and
the sweep's table is the receipt.

`crates/viewer/src/sketch.rs:1038` (`tip_mark`'s `diagonal`) is a
second miss, from the follow-up `.hypot(` pass #2783 ran to close its
own stated `sqrt()` blind spot. Its disposition is clean — it is
`is_finite() && > 0.0` guarded and it multiplies rather than divides —
so this one costs only the receipt.

## The class underneath, which is not this crate's to fix alone

Both misses are array-lowered: `[f64; 3]` values that cannot reach a
`Vec3` door without a conversion. `work/props/geom-core-linalg-has-no-
array-doors.md:73-76` already lists the lowering sites, and
`crates/viewer/src/scene.rs:1423`'s array cross is already filed to
MESH. So the disposition of the `display.rs` hit probably belongs with
that row rather than here; what belongs here is that VIEW's sweep did
not list it.

## An assertion no runtime value can break

Separately, `crates/viewer/tests/datum_draw.rs:1327-1332` does

```
if along == axis_index { continue; }
assert_ne!(along, axis_index);
```

The `continue` makes the `assert_ne!` unreachable-false by
construction, so the row's only live check is the `unwrap_or_else`
panic on `position(…)`. `docs/prompts/implementer-discipline.md` §2:
an assertion no runtime value can break is documentation, and deleting
it is the repair.

**Where**: `crates/viewer/src/display.rs` (`is_rigid`'s `dot`),
`crates/viewer/src/sketch.rs:1038`, `crates/viewer/tests/datum_draw.rs:1327-1332`.

**Confidence**: sure on all three sites; unsure whether the
`display.rs` hit should be fixed in this crate or wait on the array-door
question PROPS holds.

## Closed

Taken by `vgeom/deletions`. **Two of the three sites were repaired
where they stand; the third is not this crate's to fix and its
evidence went to the row that holds the class.** The sweep the row
says was never run was run, and its table is below.

### The `display.rs` hit: named, not fixed

`is_rigid`'s local
`|a: [f64; 3], b: [f64; 3]| a[0]*b[0] + a[1]*b[1] + a[2]*b[2]` is
where the row says it is, re-read at the line. Its disposition is the
one the row guessed and for a stronger reason than the row gives:
`Frame::columns` is `[[f64; 3]; 3]`, and the lift the viewer needs
already EXISTS one crate over as `Frame::linear_f64`
(`crates/editor-core/src/placement.rs`) — a private `fn`. So the
viewer re-spells by hand a conversion that is written twenty lines
from the data, which is `work/linalg/geom-core-linalg-has-no-array-
doors`'s own argument in its sharpest form. Evidence added to that
row rather than opened as a second one, per the row's instruction;
`crates/geom-core` and `crates/editor-core` are out of this
program's fence either way.

The linalg row's own `crates/viewer/src/datums.rs:550` entry is
**stale** — #2783 deleted the lowering it names and `:550` is a doc
comment today. Disclosed on that row, not repointed.

### The second miss: the row's citation names no subject

`crates/viewer/src/sketch.rs:1038`, *"`tip_mark`'s `diagonal`"*, does
not resolve and never did. There is no `tip_mark` and no `diagonal`
anywhere in `sketch.rs` at any commit (`git log --all -S`, both
tokens, over that path: nil), the file's only two `.hypot(` sites are
the arc block's `half` and `heading`'s `length`, and both DIVIDE. The
description — *guarded `is_finite() && > 0.0`, and it multiplies
rather than divides* — fits `View::screen_metres_at` in `datums.rs`
(`let span = self.metres_per_pixel_at(point)? * px`), which is
#2783's own file and is clean exactly as the row says.

What is NOT clean there is the function above it:
`View::metres_per_pixel_at` spells `(point - eye).norm()` by hand as
a `powi(2)` sum under a `sqrt()`, four lines from `View::basis`'s
three real `.norm()` calls. That is #2783's stated `sqrt()` blind
spot's one surviving member in the file the sweep was run on. Filed
on CHROME, which holds `datums.rs` under its 2026-09-15 carve-out:
`work/chrome/metres-per-pixel-at-hand-rolls-a-norm-the-file-already-
calls`.

### The dead assertion: deleted

`crates/viewer/tests/datum_draw.rs`,
`a_world_axis_planes_ruling_stays_on_the_other_two_world_axes`. The
`continue` made `assert_ne!(along, axis_index)` unreachable-false, and
with it gone `axis_index` and the `.enumerate()` were dead too. The
live check — every drawn direction is parallel to some world axis —
stays, as the `assert!` it always was behind the `unwrap_or_else`.

**It is not enough for the claim the row's NAME makes, and that was
measured rather than argued.** With `datums.rs`'s ruling call mutated
to `rule_patch(&mut out, origin, normal, v, bounds, pitch)` — a plane
ruled along its own normal, the exact defect the name forbids — this
row is **green** while seven sibling `datum_draw` rows red
(`cargo nextest run -p viewer --features app --no-fail-fast`: 798
run, 790 passed, 8 failed, the eighth being the standing no-Vulkan
`gpu` row). So the suite is not blind to it and the deliverable is a
citation rather than an assertion, which is the register's own rule
on `a-supersession-outlives-its-own-frame`. Filed on VDOC, which
holds `crates/viewer/tests/*`:
`work/vdoc/a-world-axis-ruling-row-is-green-over-a-ruling-along-the-
normal`.

### The sweep, re-run

Property: **a vector operation — dot, cross, norm, normalize, a
matrix product — spelled component by component over data the kernel
has a type for**, anywhere under `crates/viewer/src`. Four patterns,
because no one of them sees the class: `[f64|f32; 2|3|4]` type
mentions; a literal bracket index adjacent to an arithmetic operator;
`.hypot(` / `.sqrt()` / `.powi(2)`; and `.x`/`.y`/`.z` adjacent to an
arithmetic operator. A fifth pass over iterator-shaped sums
(`zip().map()`, `.sum()`) and variable-indexed arithmetic closed two
blind spots of the first four and found one member they missed.

| site | shape | disposition |
|---|---|---|
| `display.rs` `is_rigid` | `dot` over `[f64; 3]` | not fixed — no door; evidence to `work/linalg/geom-core-linalg-has-no-array-doors` |
| `sketch.rs` `heading` | 2-D normalize over `[f64; 2]` | in fence; header corrected under `headings-unit-vector-is-not-unit-at-the-bottom-of-the-range` |
| `sketch.rs` the arc block | chord half-length, left normal, centre, over `Point2` fields | in fence, NOT a member — it is a construction, not an op with a door; guards rewritten by #2967 |
| `datums.rs` `metres_per_pixel_at` | 3-D norm by hand over `Point3` fields | filed on CHROME (above) |
| `datums.rs` `screen_metres_at` | `mpp * px`, guarded | clean, as the row says |
| `datums.rs` `span`, the tick and arrow offsets | squared distance and point ± dir·scale over `[f64; 3]` | CHROME's; array-lowered, waits on the linalg door |
| `scene.rs` `triangle_normal` | cross + norm + normalize over `[f64; 3]` | already filed — `work/chord/degenerate-triangle-normal-is-substituted`; lowering half named on the linalg row |
| `scene.rs` `extent` | `Vec3::new(...).norm()` | NOT a member — it calls the door |
| `camera.rs` `view_matrix` | `dot` closure over `Vec3`/`Point3` | `vgeom/f32-seam`'s file; named on the linalg row |
| `camera.rs` `project`, `mul` (`(0..4).map(...).sum()`), `cursor_projection` | 4x4 matvec, 4x4 product, perspective divide over `[[f64; 4]; 4]` | `f32-seam`'s file; **no door exists at all** — `geom-core/src/linalg/mat.rs` declares `Mat3` and no `Mat4`. Named on the linalg row |
| `camera.rs` scene-bounds `centre`/`half`/`radius` | midpoint, half-extent, norm over `[f64; 3]` | `f32-seam`'s file; the `inf` half is `finite-bounds-yield-an-infinite-scene-radius` |
| `camera.rs` `ray_through` | norm over `Vec3` fields | `f32-seam`'s file; the `powi(2)` spelling is the ratified interval-square gate, argued at the site |
| `pickindex.rs` closest-point-on-segment | subtract, dot, length over `[f64; 2]` | `vgeom/pick-distance`'s file; array-lowered, named on the linalg row |
| `input.rs` `ndc_of` and its inverse | 2-D affine over `[f64; 2]` | NOT a member — a viewport mapping, not a vector op |
| `pane/viewport.rs` `at_offset` | point + dir·scale over `[f64; 2]` | `f32-seam`'s/`render-spelling`'s file; not a member for the same reason |

**What the patterns could not match**, stated because a sweep whose
blind spot is unstated is a claim:

- **A lowering split across lines.** Every pattern is line-shaped, so
  `let a0 = v[0];` followed by arithmetic on `a0` is invisible. This
  is the register's *a grep over a signature is a grep over one line
  of it* in its data form, and nothing here closes it.
- **A vector op behind a helper this sweep read as ordinary code.** A
  `fn len3(v: &[f64; 3]) -> f64` would be matched at its body and not
  at its twenty callers, so the sweep counts sites and not uses.
- **`[f32; N]` lanes in `gpu.rs`.** The type pass lists the file; the
  arithmetic pass finds no hand-rolled op there because the shader
  does it, and the WGSL itself was not swept. `gpu.rs` is
  `vgeom/f32-seam`'s.
- **Everything outside `crates/viewer/src`.** The brief scoped the
  re-run there. `crates/viewer/src/bin/` was checked for the
  arithmetic patterns and holds none. `crates/viewer/tests/` was NOT
  swept and has hits — ten files match the index-arithmetic pattern
  alone — and that tree is VDOC's, S-TCOST's and Track W's, so a
  sweep there is its own unit. The rest of the repository is the
  linalg row's population, not this one's.

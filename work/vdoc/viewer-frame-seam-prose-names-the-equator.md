---
id: viewer-frame-seam-prose-names-the-equator
kind: issue
title: the viewer's two statements of where the kernel's frame seam is name the equator, which PROPS's sign-hull unit retires
status: open
opened: 2026-09-19
---



## What was measured

PROPS's sign-hull unit (PR #2468, branch `props/sign-hull`) replaces
`Vec3::orthonormal_basis`'s branchless Duff construction — whose one
sign decision is `copysign(1, n.z)`, so whose seam is the whole equator
`n.z == 0` — with one that crosses the normal with a world axis chosen
by `|n.z| ≤ max(|n.x|, |n.y|)/2` and normalises. Ev ruled that shape at
#1944. Three sentences the viewer added while that unit was held read
the retired construction, and none of them is on a test's assertion
path, so they go stale silently:

- `crates/viewer/src/datums.rs:1032-1039` (`fn basis`) — "`n` is unit as
  a property of its type, and `UnitVec3::orthonormal_basis` completes it
  to a right-handed frame **with no length to divide by** … The frame it
  picks is discontinuous across **the equator `n.z == 0`**". Both halves
  move: the shipped construction normalises a cross product (there IS a
  length, divided under a conditioning floor), and its seam is the
  elevation band `|n.z| = max(|n.x|, |n.y|)/2`, running from
  `atan(1/(2√2)) ≈ 19.47°` on the diagonal meridians to
  `atan(1/2) ≈ 26.57°` on `n.x = 0` and `n.y = 0`.
- `crates/viewer/tests/datum_draw.rs:1607-1618` (the `NORMALS` header) —
  "the equator (`n.z == 0`) is the seam the kernel's door documents,
  taken from both sides of the signed zero and from just off it … the
  near-pole pair is where the naive `1/(1 + n.z)` spelling the kernel's
  door replaced would have cancelled". After the unit there is no
  `1/(1 + n.z)` and no signed-zero class at all, and the equator is
  ordinary ground rather than a seam — so the stated REASON each of
  `[1, 1, 0]`, `[1, 1, -0.0]`, `[0.6, 0.8, ±1e-12]` and
  `[1e-9, -1e-9, -1]` is in the fixture no longer holds. The fixture
  itself is still worth keeping; what needs re-deriving is which
  member is now the seam witness (none of the current thirteen sits on
  the new seam band).
- `crates/viewer/tests/datum_draw.rs:1685-1688` — the falsifiability
  clause of `a_planes_ruling_runs_along_the_kernels_orthonormal_basis`:
  "at `(1, 1, 0)` the local least-aligned-axis seed gives a pair turned
  45° from this one about the normal". At `n = (1, 1, 0)` the shipped
  construction takes the `e_z` arm and returns `normalize(e_z × n) =
  (−1, 1, 0)/√2`, which is exactly what a least-aligned-axis seed
  returns there (the smallest-magnitude component is `z`), so the two
  constructions AGREE at that point and the clause names no difference.
  A witness that still separates them is any normal whose smallest
  component is not `z` while `|n.z| > max(|n.x|, |n.y|)/2` — e.g.
  `(0.3, 0.4, 0.866)`, where the shipped rule takes the `e_y` arm and
  the seed rule takes `e_x`.

## Why it matters

None of the three is an assertion: both rows compare the viewer's
drawing against `kernel_basis`, which calls the same door, so they stay
green under either construction — which is the point of the rows and
also why nothing goes red when the prose stops being true. What is lost
is the fixture's own argument: the `NORMALS` header is the only record
of why those thirteen normals and not others, and a later reader who
trusts it will think the seam is covered when it is not.

## What a fix would look like

Re-word the three sentences against the shipped construction and add a
normal on the new seam band to `NORMALS` (the unit's own seam rows use
the `n.x = 0` meridian, where the two candidates are ANTIPARALLEL, as
the sharpest case). The `datums.rs` half is VGEOM's ground and CHROME's
by the 2026-09-15 carve-out, so it is an announce from here rather than
an edit — see this program's keep_out.

## Home

`crates/viewer/tests/datum_draw.rs` (the `NORMALS` header and the two
ruling rows) and `crates/viewer/src/datums.rs`'s `basis`. Found by
PROPS's sign-hull unit at its merge with `main` of 2026-09-19, where
main's viewer rows and the unit's construction met for the first time;
filed rather than fixed because the sentences are this program's and
the fixture's re-derivation is a judgement about what the rows measure.

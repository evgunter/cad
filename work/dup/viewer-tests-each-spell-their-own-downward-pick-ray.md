---
id: viewer-tests-each-spell-their-own-downward-pick-ray
kind: issue
title: Eight private spellings of the axis-aligned pick ray in crates/viewer/tests
status: closed
opened: 2026-09-20
closed: 2026-09-20
branch: dup/viewer-shared-doors
pr: 2929
---


## Finding

- **Where**: `crates/viewer/tests/`, eight private helpers in six
  files. `down_at(x, y)` at origin z = 1.0, byte-identical in
  `common/asm.rs`, `frame_policy.rs` and `select_pick.rs`; the same
  function at z = 0.5 as `down` in `review_gui2_r1.rs` and at z = 5.0
  as `down_at` in `review_gui2_r2.rs`; `up_at(x, y)` at z = −1.0,
  byte-identical in `common/asm.rs` and `review_gui4_r1.rs`; plus
  `select_pick::across_the_rim`, a fixed oblique ray and not a member.
- **What makes it a judgement**: the origin's height. Each spelling
  says "above anything THESE fixtures build", which is a claim about
  that suite's geometry, so the three z values are not a drift to
  reconcile — they are three fixtures. A shared door takes the height,
  or derives it from the document's bounds; deciding which is the work.
  The `up_at` pair is the easy half: two byte-identical copies of one
  ray, one of them already in `tests/common`.
- **Importance**: low-medium. No oracle: a wrong ray misses the body
  and reds the row that aimed it.
- **Instrument, and its blind spot**: a whole-function scan over every
  tracked `crates/viewer/tests/*.rs` for a body containing `Ray {`.
  It misses a ray built by a method or a `From` impl, one whose
  literal is written inline at a row's own site rather than in a
  helper, and any ray in another crate's suites.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner,
and one construction spelled eight times is S-DUP's charter. Any of the
five may claim it by `git mv`.

## Closed 2026-09-20 — re-taken at `cd9fdfd6b`, folded onto `common::{down_from, down_at, up_at}`

### The census, re-taken

The row's instrument was a whole-function scan for a body containing
`Ray {`. Re-run as `git grep -n 'Ray {' -- crates/viewer/tests/` over
every tracked file with no path argument, then a read of each hit:
**eight helpers in six files, exactly as the row said**, and the three z
values (1.0, 0.5, 5.0) and the two senses are as described. **Seven of
the eight are members**: the row itself calls `across_the_rim` a fixed
oblique ray and not one, so a class count that includes it is counting
a declared non-member. The class is **7 helpers + 6 inline = 13**, and
the "8 → 14" first published here was wrong at both ends by the same
one.

**The row's own blind spot — "a ray whose literal is written inline at
a row's own site rather than in a helper" — is where the class actually
was.** Running it found **six more members** the helper scan could not
see, four of them for `up_at` alone:

| inline site | what it is | disposition |
| --- | --- | --- |
| `edge_pick.rs`, twice | `down_at(0.005, 0.005)` written out, path-qualified, in a file with no `down_at` of its own | folded |
| `assembly_walk.rs` | `asm::up_at` over the shelf centre, written out, in a file that already mounts `asm` | folded |
| `review_gui4_r2.rs`, three times | the same, twice byte-identical to `assembly_walk`'s and once over a part centre | folded |

The home: `common::down_from(x, y, z)` takes the height, because the
height IS each suite's claim about its own fixture and the row was
right that the three values are three fixtures rather than a drift.
`common::down_at(x, y)` is `down_from` at one metre — the plate- and
assembly-scale suites' shared reading, stated as an invariant at the
door. `common::up_at(x, y)` is at −1 m; **no `up_from` was minted**,
because every up-ray member in the crate starts at −1.0 and a door
nothing calls is not a fold. `review_gui2_r1` (0.5 m) and
`review_gui2_r2` (5.0 m) keep a one-line binding of `down_from` with
their own height and a sentence saying what it clears.
`common/asm.rs` re-exports `down_at`/`up_at` rather than restating them,
so the ~40 `asm::down_at` call sites are untouched.

`select_pick::across_the_rim` is not a member, as the row said — a
fixed oblique ray, kept.

### What this turned up that is not this class

`select_pick` held **two byte-identical inline `wall` rays**, +x from
x = −1 at the plate's mid-depth and mid-thickness. Folded in-file to
`at_the_wall()`, which made a seventh spelling of a HORIZONTAL
axis-aligned pick ray visible across six files; filed as
`viewer-tests-each-spell-their-own-horizontal-pick-ray`.
`index_memo`'s three longhand aimed-vertex rays are filed as
`viewer-index-memo-aims-three-rays-at-one-vertex-longhand`.

The row's OTHER stated blind spot — *"any ray in another crate's
suites"* — was published as a caveat in the first pass while four
narrower residues were filed. Run: five **byte-identical** copies of
one array-to-`Ray` adapter across `bvh` and `editor-core`, and one
member of this class (`edit_pair_apply_names::down`, a downward ray at
z = 5.0) one crate over. Filed as
`cross-crate-pick-ray-constructions-outside-the-viewer-suites`, with
the denominator so the next lane does not re-derive it.

### The proof, with each plant's direction argued first

Baseline **626 passed / 0 failed / 1 ignored**.

| plant | direction | total | reds |
| --- | --- | --- | --- |
| `down_from` starts at `-z` instead of `z` | **harder**: a downward ray from UNDER the body reaches nothing, so every "this ray meets face F" row fails and no row is made easier | 574 / 52 | `mate_tool_flow` 10, `select_pick` 10, `review_gui2_r2` 9, `review_gui4_r2` 7, `frame_policy` 3, `review_gui4_r1` 3, `assembly_display` 2, `edge_pick` 2, `gesture_table` 2, `assembly_walk` 1, `review_gui2_r1` 1, `rv_matehead_probes` 1, `story_assembly` 1 |
| `down_from` starts ten metres higher | **relaxing**: an axis-aligned downward ray still meets the same face; only a row reading the hit's `t` could see it | 626 / 0 |
| `up_at` starts at +1 m instead of −1 m | **harder**: an upward ray from above the body reaches nothing | 606 / 20 | `mate_tool_flow` 10, `review_gui4_r2` 5, `review_gui4_r1` 2, `assembly_walk` 1, `rv_matehead_probes` 1, `story_assembly` 1 |

Every row sums to 626. **The second direction is the one that earns
method item 16 its keep here**: raising the origin of an axis-aligned
ray is a move that cannot make any predicate in this crate harder, and
its 626 / 0 is a null result about the plant, not about the fixture —
which is exactly what the 574 / 52 beside it shows.

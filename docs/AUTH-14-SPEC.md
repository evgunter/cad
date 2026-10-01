# AUTH-14 — a blend target whose edges cannot be named says so

**Row**: `work/author/blend-swallows-the-edge-name-fault-the-index-calls-loud`
(P2). Read it in full, including its "Moved (AUTH-10)" section. The
face-side sibling is VGEOM's
`work/vgeom/marks-focus-drops-an-unnamed-patch-from-attribution` (P3).

**Branch** `author/edge-name-fault`. **Never merge; I merge.**

## What a person gets

`PickIndex::edge_name_of` fails three ways, and its header says they
are not the same news:
- `NotDrawn` is ordinary;
- `OutOfRange` is an address minted by hand;
- `Unnamed` is the naming-emission bug arm.

Two readers collapse all three:
- `BlendTool::load_all_edges` (`blend.rs` ~:488, `.filter_map(..ok())`).
  When every name refuses, the author is told
  `BlendEvent::NoEdgesOnTarget`, that the body has no edges, when in
  fact the index could not name the edges it drew.
- `marks::HeldEdges::segments` (`marks.rs` ~:228, `is_ok_and`), which
  moved there with AUTH-10. The held-edge mark silently loses edges.

After this unit, the loud arms reach the author as the index's own
typed refusal and are never read as an empty set. The ordinary arm
stays quiet.

## Check these first

- **Every reader of `edge_name_of`.** `app.rs` ~:4651 is one more; say
  whether it is test-only. The class is "a typed name fault collapsed
  to an `Option`/`bool`". Sweep for it by callee, and for its face-side
  twin (`name_of` on patches, the VGEOM row). Say what you find.
- **Where a per-frame fault surfaces.** A mark is a value recomputed
  every frame, never retained (`marks.rs` header). A loud fault met
  while composing marks needs a surface: a frame notice, a badge, or a
  count carried on the composed value. Find the one the crate already
  uses for a per-frame fault (`crate::frame`'s notices, the
  `LegLane::undrawn` precedent in `pane/viewport.rs`). Don't mint a
  new one.

## Design calls: decide and say

1. **`load_all_edges`:**
   - a set some of whose names refuse loudly;
   - a set all of whose names refuse (today, `NoEdgesOnTarget`).

   Say what each becomes. The refusal should be the index's own
   `EdgeNameFault`, carried typed and rendered through its `Display`.
2. **The mark path:** what a loud arm does to the held-edge mark, and
   where it is said.
3. **The face-side twin:** fold it in if it is the same mechanism and
   cheap (post a seam note on VGEOM's log), or leave it on VGEOM's row
   with a note. Say which.

## Traps

- Thirteen of thirteen AUTHOR units have minted a fresh duplication
  while closing one. The likely culprits here: a second rendering of
  `EdgeNameFault`, or a second per-frame-fault surface.
- A loud arm is hard to reach from a real document. Build it the way
  the index's own tests do (a hand-minted address for `OutOfRange`, and
  whatever fixture `pickindex`'s tests use for `Unnamed`), and say how
  you reached each arm.

## Scope, verification, deliverable

In: `blend.rs`, `marks.rs`, `pane/viewport.rs`, `frame.rs` (only if the
surface needs it), and the blend panel in `pane/create.rs`. Post seam
notes for VGEOM, CHROME and VNEWS as `work.py territory` says.

- Rows that go red:
  - a blend target whose names refuse loudly says so and does not say
    "no edges";
  - an ordinary `NotDrawn` stays quiet;
  - the mark path's loud arm is said where you decided.

  Name the mutations, restore each from a byte copy, and **touch**
  afterwards.
- Local:
  - fmt;
  - clippy, both feature sets, `-D warnings`;
  - `doc-gate.sh`;
  - every `scripts/gates/*.sh`;
  - `--lib` and `--test all` as separate runs;
  - `work.py lint`;
  - the `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-14-target` on every invocation,
  including excluded roots. Scratch in `/root/auth-14-scratch/`. Wrap
  `cargo` in `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-14: a blend target whose edges cannot be named says
so`. Report to me; don't merge.

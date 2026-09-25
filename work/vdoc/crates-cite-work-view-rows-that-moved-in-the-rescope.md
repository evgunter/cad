---
id: crates-cite-work-view-rows-that-moved-in-the-rescope
kind: issue
title: Citations under crates/ name work/view/ rows that the 2026-09-17 re-scope moved
status: open
opened: 2026-09-19
refs: [stale-file-citations-after-the-split, renamed-module-leaves-citations-in-two-other-programs]
priority: P4
cost: E
---

Found by a VNEWS census lane
(`work/vnews/a-disabled-control-says-why-in-four-shapes`) while
checking one citation the review flagged, at merge base
`2654cc111417da806d9786c40136106469096fec`. The one turned out to be a
class. Filed here because both VNEWS's and VGEOM's `keep_out` say a
citation defect found on viewer ground is filed on VDOC, and because
the class reaches four further crates.

## The sweep and its result

`grep -rno 'work/view/[a-z0-9-]*' crates/`, each id then checked for
`work/view/<id>.md` on this tree. **At that base, 36 citation sites:
10 resolved and 26 did not.** Those figures are a receipt of that one
run and are not kept current; the table below is the live population —
a line is struck from it when its sites are repaired, and the repair is
recorded in a dated section at the end. VIEW's re-scope of 2026-09-17 moved rows to `vnews`, `vgeom`,
`vseam` and `vdoc` by `git mv`, and the `props` cut moved others to
`ciw`, `guard` and `dup`; nothing repointed the in-tree citations.

| citing file | id cited | now at |
|---|---|---|
| `crates/viewer/src/session/op.rs:1083` | `environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere` | `work/vnews/` |
| `crates/viewer/tests/gesture_table.rs:908` | the same | `work/vnews/` |
| `crates/viewer/src/app.rs:1986` | `projection-fault-has-no-sweeper` | `work/vseam/` |
| `crates/viewer/src/frame.rs:1895` | the same | `work/vseam/` |
| `crates/viewer/src/pane/viewport.rs:390`, `:597` | the same | `work/vseam/` |
| `crates/viewer/README.md:378` | `adjacent-same-typed-arguments-are-the-same-swap` | `work/vseam/` |
| `crates/viewer/src/camera.rs:41` | `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64` | `work/vgeom/` |
| `crates/viewer/src/pane/viewport.rs:297`, `:1591` | `ctrl-wheel-reaches-no-zoom` | `work/vgeom/` |
| `crates/viewer/src/scene.rs:966` | `the-budgets-predicted-count-is-not-always-an-over-count` | `work/vgeom/` |
| `crates/viewer/src/scene.rs:1098` | `a-flat-rung-pair-is-read-as-flat-below-it` | `work/vgeom/` |
| `crates/pncad-py/src/py/pick.rs:393` | `pickindex-merges-parts-on-a-rounded-t-it-never-converts` | `work/vgeom/` |
| `crates/viewer/src/pickcache.rs:36` | `a-module-named-for-its-spine-type-is-unfalsifiable` | `work/vdoc/` |
| `crates/viewer/src/pane/viewport.rs:1488` | `viewer-suites-hold-hand-written-complete-variant-lists` | `work/vdoc/` |
| `crates/viewer/README.md:1754` | the same | `work/vdoc/` |
| `crates/viewer/README.md:656` | `viewer-readme-multi-field-write-sweep-count-does-not-reproduce` | `work/vdoc/` |
| `crates/viewer/src/vocab.rs:92` | `vocabulary-macro-bodies-are-outside-rustfmt` | `work/guard/` |
| `crates/viewer/README.md:1645` | the same | `work/guard/` |
| `crates/viewer/README.md:1860` | `renderer-free-cross-crate-links-are-ungated-off-the-seed-set` | `work/ciw/` |
| `crates/viewer/README.md:1915` | `wasm-only-doc-comments-are-checked-by-nothing` | `work/ciw/` |
| `crates/test-utils/src/f6.rs:6` | `f6-display-predicate-is-spelled-three-times-with-no-home` | `work/dup/` |
| `crates/editor-core/tests/display_contract.rs:45` | the same | `work/dup/` |
| `crates/editor-core/tests/m4_pr4_hit.rs:338` | the same | `work/dup/` |

Still resolving, and untouched by this: `face-selection-carries-a-bare-
stable-name`, `frame-module-has-eight-concerns-and-no-holds-row`,
`free-move-in-flight-refusal-has-no-reachable-producer`,
`joined-notices-nest-their-own-separator`, `seam-split-leaves-a-cycle-
through-the-session`, `startup-notices-need-holding-to-badge`,
`the-dying-seam-fakes-mirror-a-machine-they-do-not-share`,
`the-quiet-seam-half-of-pickcache-indexing-has-no-shipped-producer` —
eight ids in `work/view/` at that base, at ten citation sites
(`startup-notices-need-holding-to-badge` has moved since; see the end).

## What the sweep could not match

- A citation written as a bare item id with no directory. Nothing in
  the text distinguishes one from prose.
- A comment that names a row by its TITLE rather than its path.
- **A path wrapped across two comment lines.** Three of the hits above
  are wrapped, and the raw grep reports their ids truncated
  (`…-with-`, `…-a-mixed-`); they were recovered by reading the lines.
  A gate written from this sweep has to join continuation lines first,
  which is the defect that makes the whole class recur.

## Why this is worth a row rather than a repointing PR

`work/view/` has not closed. Its plan says so in as many words —
*"this program did not close and does not dispatch"*, with the exit
walk a separate ratified step — so every path above is a *live*
directory holding a *different* set of rows than it did, and a reader
following one lands somewhere plausible and wrong rather than on a
404. That is the expensive kind.

The repair is one mechanical pass (each id has exactly one home), but
it touches four crates on five programs' ground, so it is a VDOC unit
with announced crossings rather than a drive-by. **And the standing
half is the more valuable one**: nothing stops the next `git mv`
repeating this, and `work/guard/` is where a gate asserting *"every
`work/<program>/<id>` cited under `crates/` resolves"* would live. That
gate is cheap — the tracker is files — and would have caught every one
the day it broke.

## Relation to the two rows this refs

`stale-file-citations-after-the-split` is the mirror image (tracker
rows citing moved CODE); this is code citing moved TRACKER rows.
`renamed-module-leaves-citations-in-two-other-programs` is the same
direction as that one at a smaller scale. Neither ranges over `work/`
paths written inside `crates/`.

## Crossing from VNEWS, 2026-09-24 (`vnews/frame-rs-prose-pass`)

VNEWS's `frame.rs` prose pass rewrote the sentence at `frame.rs:657`
(`frame_status`'s doc) under
`work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev`,
whose own text gave it the choice of repairing the path or leaving it.
It repaired it, at both `frame.rs` sites of
`one-line-one-subject-loses-a-mixed-frames-expiry` (`frame_status`'s
and `joined_subject`'s docs), and struck that line from the table
above, so this row's pass does not repoint them a second time.

**One more, since this row's sweep.** `startup-notices-need-holding-to-badge`,
listed above as still resolving in `work/view/`, has since moved to
`work/vseam/`. Its `frame.rs` citation (the module header's *"The line:
news, ranked"* section) was repointed by the same pass; its
`crates/viewer/README.md` citation (the status-line section, ~`:859`)
is this row's and is not fixed.

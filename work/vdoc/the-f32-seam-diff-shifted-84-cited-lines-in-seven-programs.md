---
id: the-f32-seam-diff-shifted-84-cited-lines-in-seven-programs
kind: issue
title: vgeom/f32-seam shifted the bands 84 substantive citations point into, across 39 rows in seven programs
status: open
opened: 2026-09-21
priority: P4
cost: D
---



Filed by `vgeom/f32-seam`, which added `crates/viewer/src/narrowing.rs`
and edited `camera.rs`, `scene.rs`, `marks.rs`, `pane/viewport.rs`,
`app.rs` and `lib.rs`. `work/view/plan.md`'s rule register (deleted
2026-09-21 by Ev's ruling; recoverable at `66d7357417`): **a diff that
shifts a file owns the census of the bands it moved, not a sample**;
`docs/prompts/implementer-discipline.md` §6: a lane REPORTS another
program's shifted citations rather than editing them. This is that
census, filed rather than left in a PR body because the same report
has reached `work/` through §6 twice with nothing a later reader could
find (the same register, *"A §6 report is not a durable artifact"*).

## The method, as the thing that produces the number

Reproducible from the branch's merge base. Read every `work/**/*.md`
owned by a viewer program (`vgeom`, `vdoc`, `vnews`, `vseam`, `view`,
`chrome`), excluding `log.md` and `STATUS.md`, plus
`crates/viewer/README.md`; extract every `<basename>.rs:<line>` for
the six files above; and for each, compare the text at that line
number in `git show origin/main:<path>` against the text at the same
number in the branch. A citation whose line text is unchanged was not
moved by this diff. One whose text changed is a candidate. One whose
number is past the end of the file at `origin/main` was already wrong
before the diff existed.

## The numbers

| | |
|---|---|
| citations examined | 283, across 78 rows |
| already past the end of the file at `origin/main` | 26 |
| sitting on a line whose text this diff changed | 113 |
| of those, whose BASE line was substantive text | **84**, across 39 rows |

The last row is the population that matters: a citation whose base
line was `}` or `);` was not naming its subject before the diff
either, so this diff did not break it.

## Where the 84 are

Re-derived on the merged tree, after `origin/main`
came in: **12** of them are on rows this branch itself closes or files,
whose bodies describe the tree BEFORE the fix, leaving **72** out of
fence.

- work/chrome/display-budget-rows-restate-three-private-constants.md &mdash; **2**
- work/vgeom/viewer-substituted-value-class-is-crate-wide.md &mdash; **1**
- work/vdoc/cfg-test-bare-spans-have-no-stated-disposition.md &mdash; **4**
- work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope.md &mdash; **6**
- work/vdoc/every-crate-root-reexport-is-a-second-path-not-the-only-one.md &mdash; **1**
- work/vdoc/session-shims-and-test-imports.md &mdash; **1**
- work/vdoc/stale-file-citations-after-the-split.md &mdash; **1**
- work/vdoc/the-citation-receipts-summary-numbers-are-not-re-derivable.md &mdash; **1**
- work/vdoc/the-f32-seam-diff-shifted-84-cited-lines-in-seven-programs.md &mdash; **1**  (this branch's own)
- work/vdoc/viewer-readme-cross-crate-link-list-cites-display-rs-at-a-line-that-never-held-it.md &mdash; **1**
- work/vgeom/cursor-projection-is-f32-in-a-module-whose-matrices-are-f64.md &mdash; **8**  (this branch's own)
- work/vgeom/the-one-free-transform-is-the-only-total-door-in-camera.md &mdash; **1**  (this branch's own)
- work/vgeom/the-point3-to-gpu-corner-cast-is-at-three-sites.md &mdash; **2**  (this branch's own)
- work/vgeom/viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep.md &mdash; **1**
- work/view/citation-repoint-shifted-a-number-the-lane-knew-was-wrong.md &mdash; **1**
- work/view/cursor-projection-landed-in-marks-for-want-of-a-home.md &mdash; **3**
- work/view/delta-field-renders-a-sub-micrometre-delta-as-zero.md &mdash; **1**
- work/view/doc-comments-name-symbols-that-do-not-exist.md &mdash; **1**
- work/view/field-censuses-inside-view-survived-the-debug-sweep.md &mdash; **7**
- work/view/fixed-precision-length-renders-can-read-as-a-value-they-cannot-be.md &mdash; **3**
- work/view/highlight-and-edge-overlay-disagree-on-hover-equals-selected.md &mdash; **1**
- work/view/loud-skip-marker-says-two-modules-and-there-are-six.md &mdash; **3**
- work/view/marks-header-asserts-universals-its-own-module-breaks.md &mdash; **1**
- work/view/named-not-linked-is-a-silent-disposition-at-eleven-of-thirteen-sites.md &mdash; **1**
- work/view/pick-index-built-on-ui-thread.md &mdash; **4**
- work/view/possessive-code-spans-are-invisible-to-the-path-shaped-sweep-rule.md &mdash; **1**
- work/view/rustdoc-posture-test-names-one-axis-of-three.md &mdash; **1**
- work/view/seeded-draft-is-the-commit-path-and-does-not-round-trip.md &mdash; **1**
- work/view/status-line-writers-bypass-the-ranking.md &mdash; **1**
- work/view/viewer-docs-do-not-build-at-wasm32.md &mdash; **1**
- work/view/viewport-adapter-drops-part-of-two-toolkit-values.md &mdash; **2**
- work/view/was-the-status-route-supposed-to-fire-for-an-absent-chooser.md &mdash; **1**
- work/vnews/a-fold-row-composes-a-producer-with-a-dead-door.md &mdash; **3**
- work/vnews/folded-moved-true-arm-covers-a-fold-that-did-not-move.md &mdash; **3**
- work/vnews/plan.md &mdash; **1**
- work/vnews/ranked-and-unranked-verdicts-are-one-type.md &mdash; **8**
- work/vseam/projection-fault-has-no-sweeper.md &mdash; **1**
- work/vseam/ui-thread-work-after-the-index-seam.md &mdash; **3**

## What this row does NOT claim, and the proxy inside it

**The file token is a BASENAME and the property is a PATH.** This repo
has more than one `lib.rs`, `app.rs` and `scene.rs`, and the census
resolves `scene.rs:857` to `crates/viewer/src/scene.rs` because the
owning row belongs to a viewer program — a classifier standing in for
the thing, which is that register's own proxy class. Restricting to
viewer-program rows bounds it; it does not eliminate it, and a row
citing another crate's `lib.rs` by basename is counted here wrongly.

**No entry has been checked for its SUBJECT.** The register's measured
result on the last census of this shape is that of 31 out-of-fence
citations exactly ONE was a true shift — fifteen cited past the end of
the file, seven were in range with the subject elsewhere, and eight
were quotations. So 84 is an upper bound on the damage and not a count
of it, and **placing it means re-deriving at placing time, by
subject**. The value here is the POPULATION, never the numbers beside
it.

**Nothing was repointed.** The four rows `vgeom/f32-seam` closes are
its own, and their bodies describe the tree BEFORE the fix, which is
what an item's body is for; their `## Closed` sections carry the new
positions by subject.

## Neighbours

`stale-file-citations-after-the-split` is this program's standing
general row for the class and should absorb this one if a sweep takes
both. `crates-cite-work-view-rows-that-moved-in-the-rescope` holds six
of the 84 and is about the same lines from the other direction.

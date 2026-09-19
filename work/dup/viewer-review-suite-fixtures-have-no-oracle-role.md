---
id: viewer-review-suite-fixtures-have-no-oracle-role
kind: issue
title: Three viewer review suites keep fixtures no oracle reads; a mutation measured which
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/viewer/tests/review_gui2_r1.rs`,
  `review_gui2_r2.rs`, `review_gui3_r1.rs` and `review_gui3_r2.rs`.
  PR #2886 folded what it could prove live — `xy_frame`, `len`, `scl`,
  `tempdir`, `GALLERY_RING` and the three `gallery_ring_at`
  re-spellings. **What is left** is the sugar whose fold is a
  judgement rather than a substitution: private `insert`/`inserted`
  wrappers in all four, `length`/`scalar` in `review_gui2_r2`, and
  `rectangle`/`rect`/`triangle` against `common::square`.
- **Importance**: medium. The blanket ground that used to protect them
  ("a promoted review suite's value is that it is an INDEPENDENT
  derivation") was withdrawn by Ev on 2026-09-04 and was removed from
  these three headers by PR carrying
  `viewer-review-suites-cite-the-withdrawn-independence-reading`. What
  replaced it is the surviving per-row test, and applying that test is
  what this row is.
- **Confidence**: sure about the duplicates; the oracle question is
  **measured for one file and open for the other two**.
- **Raised by**: the S-DUP lane for the citation census, 2026-09-19.

## The measurement, and what it decided

At merge base `5b4979ef2`, with `common::xy_frame` folded into all
three suites, two planted mutations in the shared helper:

| mutation | binary | `gui2_r1` | `gui2_r2` | `gui3_r1` |
| --- | --- | --- | --- | --- |
| none | 626 pass / 0 fail | — | — | — |
| `common::xy_frame` v axis y → z | 581 / 45 | **3 red** | **9 red** | **0 red** |
| `common::scl(v)` → `v + 1.0` | 561 / 65 | 1 red | 0 red | 1 red |

So the frame datum is live for the two GUI-2 suites and **inert for
`review_gui3_r1`**: none of its rows can see the world frame move.
That is consistent with what it asserts — history structure, refusals,
landing generations, file bytes — and it is the evidence for the
sentence now in that file's header saying its triangle is a
readability choice, not an independence claim.

## What is left to decide, per file

The surviving clause's test is: name the constant or helper the row
would otherwise read, and say whether a bug in it would be invisible
to the row if the row read it.

- **`review_gui3_r1`** — answered: no oracle reads the fixture's shape,
  so nothing here needs its own derivation. `common::framed_square`,
  `common::inserted` and `common::tempdir` are the candidates, and the
  triangle and `r1_depth` are worth keeping only if a reader values
  telling R1's document apart in the aggregated binary's output.
- **`review_gui2_r1` and `review_gui2_r2`** — their geometric oracles
  (cursor position → resolved face) genuinely need their own
  derivation, but **not for the reason first written here**: neither
  suite builds the plate, so `common`'s plate helpers were never a
  candidate. Both call `Camera::framing` directly, and what keeps them
  honest is that their expectations are hand-derived from their own
  fixtures' coordinates. The sugar that carries no oracle is not
  covered by that and is sharing-eligible. Each remaining one is a
  separate judgement, which is why this is a row and not more edits in
  the census PR.
- **`review_gui3_r2`** — the fifth carrier, found by the class-shaped
  ground sweep rather than the citation census. Its rows assert on
  panels and history; its `rect` fixture is a spelling. Same
  disposition as `review_gui3_r1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one fixture spelled four times, freed
by a withdrawn rule — is S-DUP's charter. Any of the five may claim it
by `git mv`.

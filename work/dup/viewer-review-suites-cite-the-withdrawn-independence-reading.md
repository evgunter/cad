---
id: viewer-review-suites-cite-the-withdrawn-independence-reading
kind: issue
title: Four viewer review suites cite the policy memory for the independence reading it withdrew
status: closed
opened: 2026-09-19
closed: 2026-09-19
pr: 2886
---



## Finding

- **Where**: four files under `crates/viewer/tests/`, each naming
  `memories/review-and-dependency-policy.md` and attributing to it a
  reading that memory withdrew on 2026-09-04:
  - `crates/viewer/tests/common/mod.rs` — *"The two review suites
    (`review_gui0_r1`, `review_gui0_r2`) keep their own fixtures on
    purpose: a promoted review suite's value is that it is an
    INDEPENDENT derivation of what the unit claims (…), and pointing it
    at the implementation's own constants would spend exactly that."*
  - `crates/viewer/tests/review_gui2_r1.rs` — *"deliberately NOT the
    unit's plate helpers, because a promoted review suite's value is
    that it derives the claims independently (…)."*
  - `crates/viewer/tests/review_gui2_r2.rs` — *"Fixtures are authored
    here on purpose … a review suite that read them would be checking
    the implementation against itself (…)."*
  - `crates/viewer/tests/review_gui3_r1.rs` — *"(…: pointing a review
    suite at the implementation's own constants would spend exactly the
    independence that is its value)."*
- **Why it is wrong**: each grounds the keep-your-own-code decision in
  *what the file is* — "a promoted review suite" — which the memory
  names as the withdrawn reading: *"An earlier version of this memory
  made reviewer suites a protected class ('promoted as-is',
  'independence worth keeping', 'never simplify to match shipped
  fixtures'); that reading was withdrawn."* What survives is *"keep
  their own code only where a row's claim needs its own derivation (a
  general test-design question, **not a question of who wrote the
  row**)."*
- **Importance**: medium. Unlike the `crates/*/review_m*` carriers,
  these four state the rule in their own words and cite the memory by
  name, so no grep for the withdrawn sentence reaches them. They are
  the shape that makes the citation, not the sentence, the right
  denominator.
- **Confidence**: sure about the text and the withdrawal. **Not sure
  about the remedy**, which is why this is a row and not an edit.
- **Raised by**: the S-DUP lane for the withdrawn-no-simplify unit,
  2026-09-19, by enumerating every tracked citation of the memory
  rather than every copy of the sentence.

## What is NOT settled, and is the whole reason this is filed

The surviving clause is a **per-file test-design judgement**: does each
of these four rows' claim need its own derivation? Three of them may
well pass it — a suite whose subject is *the viewer's own constants*
arguably cannot read those constants and still be a check. But
answering that requires reading `viewer`'s scene/plate code and each
row's claim, which the lane that found this does not own and did not
do. **Restating the justification on the surviving ground is writing a
standing instruction into a source header**, and that is not a
duplication lane's call.

What would settle it, per file: name the constant or helper the row
would otherwise read, and say whether a bug in it would be invisible to
the row if the row read it. Where the answer is yes, the row's claim
needs its own derivation and the code stays — restated on that ground,
not on "it is a promoted review suite". Where the answer is no, it
shares.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the finding's subject — one withdrawn instruction
spelled in four places — is S-DUP's charter with prose as the artifact.
S-DUP claims no territory and announces by seam (X6, inherited from
SUITE). Any of the five may claim this row by `git mv`.

## Closed 2026-09-19 — the per-file test run, and what it decided

The test this row asked for — *name the constant or helper the row
would otherwise read, and say whether a bug in it would be invisible to
the row if the row read it* — was run on all four, and the answer split
the files rather than the class, which is why the blanket rule had to
go in both directions.

| file | the helper it would read | invisible? | disposition |
| --- | --- | --- | --- |
| `tests/common/mod.rs` (its claim about `review_gui0_r1`/`_r2`) | `common::framed()`, which **is** a call to `Camera::framing` | yes — GUI-0's subject is that door | keeps, restated |
| `review_gui2_r1.rs` | `common::plate_bounds`/`framed`/`corners`, all functions of `viewer::scene::PLATE_EXTENT` | yes — an aim derived from the constants the scene is built from moves with it | geometric oracles keep, restated; the frame datum shares |
| `review_gui2_r2.rs` | same | yes, same reason | same |
| `review_gui3_r1.rs` | `common::square`/`framed_square` | **no oracle reads the shape at all** | restated honestly as a readability choice, not an independence claim; sugar shares |

`review_gui3_r1`'s answer is measured, not argued: with
`common::xy_frame` folded in, mutating that helper's v axis y → z reds
3 rows in `review_gui2_r1` and 9 in `review_gui2_r2` and **0 in
`review_gui3_r1`** (626 pass → 581/45). Mutating `common::scl(v)` to
`v + 1.0` reds 1 row in `review_gui3_r1` (626 → 561/65), so the fold
there is live even though the geometry is inert to its claims.

What this row ruled out for all four: the ground *"it is a promoted
review suite"*. What it did not finish — the remaining shareable sugar
in the three suites, and whether `gui3_r1`'s fixtures should share
outright — is
`work/dup/viewer-review-suite-fixtures-have-no-oracle-role`, filed with
the mutation table.

## Re-measured at the fix pass (2026-09-19), and a fifth carrier

The style review found that two of the four verdicts above rested on a
helper the suites have no use for. `review_gui2_r1` and `_r2` do not
build the plate at all, so they would never read `common::plate_bounds`
/ `framed` / `corners` — they call `Camera::framing` **directly**
(`review_gui2_r1.rs:350`, `review_gui2_r2.rs:189`), the very door the
`gui0` verdict says a framing row must not take from `tests/common`.
**The verdicts stand; the stated reason did not.** What distinguishes
gui2 is that framing is its instrument and not its subject, and its
oracles are cursor positions hand-derived from the fixtures' own
coordinates — a fixture taken from the same place the aim was would
track it silently. Each file now states that in its own header, with no
cross-file pointer.

`review_gui3_r2` is a fifth carrier the citation census could not see
(it states the ground without naming the memory) and is fixed the same
way. `review_gui4_r1` and `_r2` were read and are not carriers.

Mutations re-planted after the folds, same binary, merge base
`5b4979ef2` (baseline **626 pass / 0 fail**):

| planted in | result | `gui2_r1` | `gui2_r2` | `gui3_r1` | `gui3_r2` |
| --- | --- | --- | --- | --- | --- |
| `common::xy_frame` v axis y → z | 581 / 45 | 3 | 9 | 0 | 0 |
| `common::scl(v)` → `v + 1.0` | 561 / 65 | 1 | 0 | 1 | 0 |
| `common::len(m)` → `m + 1.0` | 534 / 92 | 3 | 11 | 0 | 3 |
| `History::undo` does not move the cursor | — | 3 | 3 | **4** | 5 |

**The suite sizes, since a numerator without its denominator says
nothing**: `gui2_r1` 8 rows, `gui2_r2` 28, `gui3_r1` **9**, `gui3_r2`
16 — counted as `#[test]` attributes and confirmed against the built
binary (`--test all review_gui3_r1` selects 9 and filters 618).

**A denominator of 18 stood here and was wrong**, by an instrument
defect worth naming because it is invisible on most inputs: the count
was taken with `grep -c '^fn r1_\|^#\[test\]'`, an alternation that
counts a row TWICE wherever a file's test functions are both
`#[test]`-attributed and named `r1_*` at column 0. `review_gui3_r1` is
the only such file in the crate, so it is the only figure the
instrument doubled — the other three were right, which is exactly what
made the wrong one look sound.

The last table row is the one the style review asked for, and the
question the zeros raise is settled by two mutations with different
figures, each stated with the mutation that produced it:

- **`History::undo` does not move the cursor** — **4 of 9**:
  `r1_an_abandoned_branch_keeps_its_whole_subtree`,
  `r1_redo_walks_the_new_branch_across_two_levels`,
  `r1_open_then_save_reproduces_the_file_bytes_exactly`,
  `r1_a_replayed_history_opens_at_the_tip_with_the_log_undoable`.
- **`History::commit_group` replaces the sibling list**
  (`parent_entry.children = vec![id]` for `.push(id)`) — **1 of 9**,
  and it is `r1_an_abandoned_branch_keeps_its_whole_subtree`, the row
  whose name is the claim.

Either way the conclusion is the same and now rests on named rows
rather than a bare count: this suite's zero on the frame datum is
*it asserts nothing about that helper*, not *it asserts nothing
useful* — the second would have been S-TINT's finding and not this
row's to close.

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

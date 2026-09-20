---
id: viewer-gui2-suites-spell-the-gallery-ring-delta-twice
kind: issue
title: The gallery ring's 2.0e-3 display tolerance is written in both GUI-2 review suites
status: open
opened: 2026-09-20
---


## Finding

- **Where**: `crates/viewer/tests/review_gui2_r1.rs`'s `ring_delta`
  local and `crates/viewer/tests/review_gui2_r2.rs`'s `fn coarse()`.
  Both are `DisplayTolerance::new(2.0e-3)`, both exist so the SAME
  fixture — the gallery ring — tessellates cheaply, and each carries
  its own prose saying so.
- **Why the row that folded the crate's other display tolerances left
  these**: `viewer-review-suite-fixtures-have-no-oracle-role` settled
  `coarse` as **kept**, on the ground that it is `gui2_r2`'s SECOND
  constant and a cost choice rather than a drift against that suite's
  own `delta()`. That verdict is about `coarse` against its neighbour
  in one file; it never looked across at `gui2_r1`, where the same
  value for the same fixture is written again. So the keep is sound and
  the duplicate is real, and they are different questions.
- **What makes the fold a judgement**: both files are promoted review
  suites whose headers enumerate what they take from `common`, and a δ
  chosen for cost is the kind of thing a reviewer may want written
  where the reader is. A shared `common::ring_delta()` is the obvious
  home; whether these two suites should share it is the work.
- **Importance**: low. Measured on 2026-09-20: nothing in the crate
  reds when the sibling constant `common::plate_delta` is moved by a
  factor of 5000 in either direction, so a display tolerance in these
  suites carries no oracle. See
  `work/tint/viewer-plate-suites-index-at-a-display-tolerance-nothing-asserts`.
- **Instrument, and its blind spot**: a value-keyed census —
  `git grep -n 'DisplayTolerance::new(' ` over every tracked file, no
  path argument, bucketed by the literal. It is line-shaped and cannot
  see a call whose argument wraps, nor the same value written `2e-3` or
  `0.002`; and it cannot see a δ reached through `scaled()` from
  another.
- **Raised by**: the S-DUP lane closing the four viewer-suite door rows,
  2026-09-20, measured at `cd9fdfd6b`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled more than once
— is S-DUP's charter. Any of the five may claim it by `git mv`.


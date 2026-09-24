---
id: cross-crate-inline-expr-literal-sites-outside-the-viewer-suites
kind: issue
title: The inline Expr::literal class runs past crates/viewer into sixty-three files
status: open
opened: 2026-09-20
priority: P4
cost: H
---


## Finding

The row `viewer-tests-bypass-the-shared-literal-doors` named *"every
other crate's suites"* as its instrument's blind spot and cited
*"forty-five files"* as evidence that the class runs wider. That
citation was published as a caveat rather than run; this is the row it
owed.

- **The measurement, re-taken at `cd9fdfd6b`** over every tracked file
  with no path argument: `git grep -l 'Dimension::Length).expect\|
  Dimension::Scalar).expect\|Dimension::Angle).expect' -- crates/`,
  minus `crates/viewer/`, names **63 files** — not forty-five. The
  bulk is `crates/editor-core/`, split between `src/` (the kernel's
  own doors) and `tests/corpus/` (the shared document corpus), with
  `crates/pncad/` and `crates/profile/` behind them.
- **What makes this a different question from the viewer's**, and why
  it is a row rather than a bigger sweep: `crates/viewer/tests/`'s
  members bypassed a door that **already existed in the same binary**
  (`common::{len, scl, ang}`), so the fold was a substitution. Outside
  that crate there is no such home, and the sites are split between
  `src/` — where an `Expr::literal` call is the kernel doing its job,
  not a test restating a fixture — and three separate `tests/` trees
  that would each need their own. **Deciding which of the 63 files
  hold members at all is the first instrument**, and a count of
  members should not be published before it runs.
- **Importance**: low. No oracle: a hand-written
  `Expr::literal(v, Dimension::Length).expect(..)` and a door over it
  are the same call.
- **Instrument, and its blind spot**: the `-l` grep above is
  **line-shaped and `.expect`-shaped**. It misses a call whose
  argument list wraps (the viewer unit found one such site, in
  `frame_policy.rs`, written across four lines — so the shape exists
  in this tree and this census certainly undercounts), a site using
  `.ok()`, `?` or a match instead of `.expect`, and
  `Expr::literal_with_unit`, which is a different door (its
  `crates/viewer/tests/` members are folded onto `common::len_mm`).
  It is a FILE count, not a site count, and the two are not
  interchangeable.
- **Raised by**: the S-DUP lane closing the four viewer-suite door
  rows, in its fix pass, 2026-09-20.

## Why this sits on S-DUP's slate

The ground is three crates' `tests/` trees plus one `src/` test module,
each claimed by several programs, so there is no single ground-owner to
file it with, and one construction spelled more than once is S-DUP's
charter. Any claimant may take it by `git mv`.


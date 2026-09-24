---
id: viewer-tests-spell-one-display-tolerance-in-seven-places
kind: issue
title: One display tolerance value written out in seven places in crates/viewer/tests
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## Finding

- **Where**: `DisplayTolerance::new(2.0e-4).expect("a positive delta")`
  — one value, **seven** sites in six files under
  `crates/viewer/tests/`: `blend_authoring.rs:89`, `debug_dumps.rs:28`,
  `edge_pick.rs:50`, `eval_seam.rs:550`, `eval_seam.rs:813`,
  `frame_policy.rs:53`, `select_pick.rs:73`. Six of the seven carry the
  **identical** `expect` string; two spell the type path-qualified as
  `scene::DisplayTolerance`. `eval_seam.rs:813` binds it to a local
  named `coarse`, which is the same number under a name that elsewhere
  in this crate means something ten times coarser.
- **Why it is a duplicate and the per-suite `delta()` helpers are
  not**: these seven are one CONSTANT, agreeing to the digit, in
  suites that nothing distinguishes on this axis. The review suites'
  own tolerances differ on purpose — `review_gui2_r1` 1.5e-4,
  `review_gui2_r2` 3.0e-4 — and stay. The unit that measured this
  looked at the per-suite `delta()` helpers, judged them independent,
  and did not look at the constant inside them.
- **Importance**: medium. A display tolerance decides how finely a
  fixture tessellates, so it is an input to every pick these suites
  make; seven copies agreeing today is seven places to change and six
  chances to miss one.
- **Confidence**: sure about the sites and the value.
- **Instrument, and its blind spot**: `git grep -n
  'DisplayTolerance::new(2.0e-4)'` over every tracked file, no path
  argument. It is literal and line-shaped: it misses the same value
  written `2e-4`, `0.0002` or as an arithmetic expression, and any
  other value shared by two suites. The class is therefore a floor.
- **Raised by**: the S-DUP lane's fix pass on
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `ab086f8c1`. The review that prompted it named five
  sites in five files; re-taking found seven in six.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner,
and one constant spelled seven times is S-DUP's charter. Any of the
five may claim it by `git mv`.

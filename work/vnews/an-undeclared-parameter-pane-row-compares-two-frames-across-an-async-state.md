---
id: an-undeclared-parameter-pane-row-compares-two-frames-across-an-async-state
kind: issue
title: an_undeclared_parameter_is_said_once_in_the_pane compares two painted frames whose evaluation/indexing state differs under load
status: open
opened: 2026-09-28
priority: P3
cost: E
---

Seen 2026-09-28 by `vnews/a-ranked-verdict-is-its-own-type`, in a
full `cargo test -p viewer --all-features` run on a loaded shared box.

`crates/viewer/src/app.rs`, `properties_pane_tests::an_undeclared_parameter_is_said_once_in_the_pane`,
paints two frames through `painted_with` (one with a parameter
selected, one with nothing selected) and asserts their text sets are
equal after the verdict line is swapped in. In the failing run the two
frames differed in text the row is not about:

- the `with` frame showed `indexing…`;
- the `without` frame showed `evaluating…`, a `Cancel` button, and
  three `—` readouts.

So each frame was painted at a different point in the background
evaluation/index work, and the comparison picked that up. Re-run alone
three times, the row passed every time. It is a race on how far the
async work has got by the time each frame is drawn, not a defect in
what the pane says.

**The shape of a fix:** have `painted_with` wait until the evaluation
and index are settled before painting, or have the row compare only the
properties pane's own region. Either way, the row should stop depending
on how far a worker has got. It is a guard that can go red for no
reason, which is P3 under `work/README.md`'s bands.


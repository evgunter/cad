---
id: threaded-seam-wait-loops-are-hand-copied-across-four-test-files
kind: issue
title: four viewer test files each spin on a seam with their own copy of the same bounded nap loop
status: open
opened: 2026-09-15
priority: P4
cost: E
---



## What

Driving a threaded seam from a test means waiting for its worker, and
every row that does it writes its own bounded nap loop: poll, push what
came back, stop on a condition, `sleep(1ms)`, give up after N. Nine
sites in five files carried one on `origin/main` at `d71bb6a785`; the
six in `crates/viewer/tests/eval_seam.rs` are now one `drained` helper
(this row's finder). **Three are left, in files outside that lane's
fence**, and they are not the same loop twice — each picked its own
ceiling and its own stop condition:

- `crates/viewer/tests/review_gui3_r2.rs`, the `count_results` closure
  — `0..10_000`, stops on `!seam.busy()`, over a `&mut dyn
  EvalService`, which is the one site that drives BOTH lanes through
  one loop;
- `crates/viewer/tests/index_memo.rs`, `answer` — `0..100_000`, stops
  at the first answer, and `panic!("the seam never answered")` on
  exhaustion, which is the only one of the three that fails loudly for
  the right reason rather than falling through to a later assertion;
- `crates/viewer/tests/review_gui4_r2.rs` — `0..30_000`, over
  `DocSession::pump` and `DocSession::busy` rather than over a seam
  directly.

## Why it is worth a row

The ceilings differ by a factor of ten and nothing says why. A ceiling
is a claim about how slow a loaded CI box may be, and three different
claims in one crate cannot all be the considered one — the 30 000 row
is the one that actually opens an assembly, which is the slowest
subject of the three, so the spread is not even ordered by cost.

The repair is a placement question rather than a fix: the helper wants
to be in `crates/viewer/tests/common/`, which is shared test MECHANISM
and therefore announced rather than assumed (this program's `keep_out`,
and S-TCOST's and Track W's standing claim on `*/tests/*`). The third
site is not a seam at all, so a seam-shaped helper does not reach it
and the honest answer there may be to leave it.

## Found by

The lane that folded the three threaded seams onto one coalescing
handle (`work/view/the-two-seams-are-hand-maintained-twins`), whose own
item predicted two copies of the harness and found six.

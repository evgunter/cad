---
id: the-cancelling-mid-run-row-checks-its-prefix-less-often-on-a-faster-wheel
kind: issue
title: test_cancellation's mid-run cancel row sleeps 20 ms before cancelling, so on a faster wheel the run finishes first more often and the prefix invariant goes unchecked
status: open
opened: 2026-11-22
priority: P3
cost: E
---


## Found (GREEN, review of PR 7364, 2026-11-22)

`crates/pncad-py/tests/test_cancellation.py`
`TestCancelingARunUnderWay.test_a_thread_cancelling_mid_run_yields_a_legal_partial`:
the helper does `launched.wait(); time.sleep(0.02); token.cancel()`. When the run ends before the 20 ms are up, the row takes its `if not run.canceled: ... return` branch, which never checks the prefix invariant, and it stays green. So the row checks less as the build gets faster. PR 7364 moved the PR gate's wheel from opt-level 0 to 1: the suite went from 1056 s to 344 s, and the `stack(WIDE)` run, ~300 ms at opt 0 per the module docstring, is correspondingly shorter. The nightly still builds at opt 0.

Swapping the sleep for an event alone (cancel the moment the borrow is taken) is not a fix. The stop would land at the first node check almost every time, giving a prefix of 0 or 1, and the row would stop exercising a stop with a non-trivial prefix.

## Fix

Stop the run partway by construction rather than by the clock: for example, cancel from a point the run is known to have passed (a node count observed through a hook the binding already has, if one exists), or size the document so the run outlasts the delay at the fastest profile the gate builds. Or add a floor: count how often the row is canceled across runs, and fail loud if it never is.

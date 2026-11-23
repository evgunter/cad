---
id: the-gil-release-rows-window-narrows-with-the-wheels-speed
kind: issue
title: test_cancellation's GIL-release row counts 1 ms stamps in the middle half of the evaluate window, which a fast enough wheel narrows below one stamp
status: open
opened: 2026-11-22
priority: P3
cost: E
---


## Found (GREEN, review of PR 7364, 2026-11-22)

`crates/pncad-py/tests/test_cancellation.py`
`TestCancelingARunUnderWay.test_the_evaluating_thread_releases_the_gil`: a helper stamps the clock about every 1 ms (`time.sleep(0.001)`), and the row requires at least one stamp in the middle half of `evaluate`'s window. That half-window shrinks in proportion to the evaluation's length. PR 7364 moved the PR gate's wheel from opt-level 0 to 1 (suite 1056 → 344 s). Under about 4 ms of evaluation there is room for no stamp, so a released GIL would read as a held one: a false red. It has not been seen red; it passed on both of PR 7364's runs.

## Fix

Make the window long by construction at the fastest profile the gate builds, e.g. a document sized so the run takes well over a stamp interval at opt-level 1 and up. Or count the stamps over the whole window minus the switch-interval margins at its edges, instead of over the middle half.

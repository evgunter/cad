---
id: main-red-the-teapot-spout-join-refuses-join-on-the-hosted-runner
kind: issue
title: main red (hosted runner only): the Python teapot row's spout join refuses 'join', not 'germ_frame_unsupported', since PR 6872 (likely)
status: open
opened: 2026-11-12
---


**What fails.** `pncad-py` Python suite, `test_north_star.TestTeapot.test_the_handle_join_builds_and_the_spout_join_refuses` (`crates/pncad-py/tests/test_north_star.py:3312`): the spout join refuses with inner kind `join` (`BooleanErrorKind::Join`), where the row pins `germ_frame_unsupported`. Seen on the hosted `python suite (wheel + guide + north-star)` job of PR 6856 at `038d9dd5a9` and again at `fd6289f81d` (both merged with main; 959 tests, 1 failure).

**Hosted runner only.** On a local box the same row passes at the same heads, with the release wheel the job builds (`cargo build --release -p pncad-py --features extension-module`, whole suite 959/959), and so does origin/main `5151c82a9a` (release, the row alone).

**Attribution (likely, not bisected).** The job last passed on PR 6856 at `dc9ba897b4`, before that branch merged main's PR 6872 (`pcert/closed-bound-fine-band`, merge `5151c82a`, which reworks `crates/geom-brep/src/pcurve_cache/closed_image_rows.rs`). PR 6872's own run skipped the Python suite (its change filter), and no main run since has run it. PR 6856's own changes reach no boolean or pcurve code, but this is inference, not a bisection: no hosted run of main alone is available to this lane (`workflow_dispatch` is refused to it).

**Reproduce.** The hosted job, on any head that contains `5151c82a`. Locally it does not reproduce, which is itself the finding: the result moves with the machine, so the refusal is at a threshold the runner's floating point falls on the other side of.

**What closing it takes.** A hosted run of main with the Python suite (`workflow_dispatch`, scope `all`), to confirm main is red; then either PR 6872's closed-image rows made to agree across machines at this pose, or the row re-pinned with the reason.

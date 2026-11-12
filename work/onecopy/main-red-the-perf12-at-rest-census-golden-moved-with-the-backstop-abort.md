---
id: main-red-the-perf12-at-rest-census-golden-moved-with-the-backstop-abort
kind: issue
title: Main red (slow set): perf12's at-rest census verdicts drift from their golden at line 75 since PR 6757's backstop abort
status: open
opened: 2026-11-11
priority: P1
cost: E
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`), at default ε and 1e-6. It was green in the nightly before (42761779270, `50cfcf499d`).

**What fails.** `editor-core::all perf12_census_goldens::the_at_rest_census_verdicts_are_goldened_bit_exact`: "the at-rest census verdicts drifted from their committed golden at line 75 (eps=1e-9): the census is deciding differently".

**Bisected, first-parent, at default ε:**
- green at `2459d705`, `b9b97c8d` and `a4f29540` (PR 6776);
- red at `7099044a`, the merge of PR 6757 (`onecopy/backstop-abort`), at line 75;
- also red at `7e1bb780`, `3e7ce509`, `2e4c44de` and main `019bed88`, at line 75 each time.

Read the diff (`PERF12_BLESS_CENSUS=1` writes the new golden), decide whether the new verdict at line 75 is right, and re-bless deliberately with the argument. The census row names the verdict. Check the 1e-6 golden as well, which is also red.

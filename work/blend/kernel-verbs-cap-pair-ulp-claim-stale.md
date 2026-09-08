---
id: kernel-verbs-cap-pair-ulp-claim-stale
kind: issue
title: docs/KERNEL-VERBS.md:146–150 says cap pairs are pinned equal with the ulp confined to the off-radius boundary; the cap-pair row lands one ulp off
status: open
opened: 2026-09-08
---


## Finding (BLEND unit 5's style review, PR 2155)

`docs/KERNEL-VERBS.md:146–150` states that wall pairs and cap pairs are
"pinned equal" with the one-summation ulp confined to "the off-radius
boundary"; `review_m5_pr10::r2_p3` (a cap pair at the fixture radius)
lands one ulp off on one order — the claim BLEND-5 corrected in
`crates/sweep/src/skin.rs` still stands in the register, in the bare
`sweep/tests/…` path spelling. Prose only: state what the row pins and
cite it in the `<module>::<row>` spelling.

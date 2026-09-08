---
id: sweep-test-suites-cite-eight-deleted-suite-files
kind: issue
title: sweep tests: sixteen tests→tests citations across thirteen suites, eight naming suite files that no longer exist
status: open
opened: 2026-09-08
---


## Finding (BLEND unit 5's style review and fix pass, PR 2155; filed by the BLEND orchestrator)

`crates/sweep/tests/**` carries sixteen `tests/<file>.rs` citations of
OTHER suites across thirteen suite files (e.g. `cert_m2r1_passes.rs:218`,
`common/oracles.rs:21`), eight of which name suite files that no
longer exist. BLEND-5 normalised `crates/sweep/src`'s citations to the
`<module>::<row>` spelling and added a resolver row whose corpus is
`crates/sweep/src` only; the tests→tests class is outside that unit's
fence and this glob's. The fix shape is the same: the module spelling,
and the resolver row's corpus widened to `crates/sweep/tests/**`
(`review_blend5_r5_probes.rs`'s reader, one path added).

## Home

`work/tcost/` — `crates/sweep/tests/*` is S-TCOST's glob.

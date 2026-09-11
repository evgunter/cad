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

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

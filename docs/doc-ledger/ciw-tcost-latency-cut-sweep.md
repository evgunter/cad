# CIW and S-TCOST rows swept after the CI-latency cut — 2026-09-28

The per-PR gate was cut to latency on 2026-09-28 (PRs #3340, #3369;
`work/ciw/latency-cut.md`). That cut deleted the configuration draw and
its un-sampling, the lane axis, the shard matrix and archive hand-off,
the hosted/local mirror and its parity checkers, the local mirror
scripts, the test-cost reporters, the opt-level calibrator, the
demotion selector, per-PR renders, the read reach and the `merge_group`
trigger. The tracker rows about that machinery were deleted in one
sweep rather than left to describe it: 75 files from `work/ciw/` (every
closed row but `latency-cut`, and 18 open rows whose subject is gone)
and 14 from `work/tcost/` (the C1–C4 and B3 build/nightly units, and
the shard, cache, critical-path, eps-battery, tolerance-study and M10-3
chamber rows). `an-unmergeable-pr-is-silently-ungated-not-visibly-red`
was folded into `work/ciw/dirty-pr-gets-no-actions-run.md`, which names
it. The rows still live were kept, with their citations re-pointed.

Sweep SHA `b445eeb5e47372a38deed2399d8e8ed5e6234978`.

    git show b445eeb5e47372a38deed2399d8e8ed5e6234978:work/ciw/<FILE>
    git show b445eeb5e47372a38deed2399d8e8ed5e6234978:work/tcost/<FILE>

A citation of a `work/ciw/…` or `work/tcost/…` row that no longer
resolves is recovered from that SHA.

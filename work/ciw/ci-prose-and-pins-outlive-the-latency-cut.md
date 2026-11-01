---
id: ci-prose-and-pins-outlive-the-latency-cut
kind: issue
title: doc-gate.sh, render.yml and test-fast.sh still argue in billed minutes and archive jobs, and ci.yml pins an sccache nothing installs
status: open
opened: 2026-09-28
priority: P4
cost: E
---

Found by the 2026-09-28 tracker sweep after the CI-latency cut
(`work/ciw/latency-cut.md`). The sweep deleted the rows about billed-minute
arguments and the archive/shard layout because their `ci.yml` sites are
gone; these sites outside `ci.yml` survive, and an agent reading them would
take a billed minute or an archive job for a live consideration.

- `scripts/doc-gate.sh:398-415` and `:482-490` — the rustdoc gate's
  caching and pass-3 costs argued in billed minutes against F6 of
  `docs/CI-MINUTES-2026-08.md`, in "the same `fmt` job", which no longer
  exists; the gate runs nightly (`nightly.yml`, `rustdoc-roots`).
- `.github/workflows/render.yml:474-477` — the merged renderer-free
  lanes justified as "worth two billed minutes".
- `local-scripts/test-fast.sh:22-29` — *"This IS the CI configuration for
  the two nextest-archive jobs"*, with a billed-minute reading. The profile
  knobs are now `ci.yml`'s `test` job env (opt-level 1) and `nightly.yml`'s
  `full-suite` (opt-level 2); say which the script mirrors, or neither.
- `.github/workflows/ci.yml`'s `SCCACHE_VERSION` pin and
  `.github/actions/install-sccache/` — no job uses the action; the pin is
  read only by `scripts/ci-pin.py`'s selftest fixture. Delete both, or say
  why the pin stays.

The fix is prose in the present tense and the deletion of the unused
action; no measurement is owed.

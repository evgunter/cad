---
id: the-all-features-build-overran-its-hour-on-11-13-for-an-unread-reason
kind: issue
title: The nightly all-features build step overran its 60 minutes on 11-13 against 32 to 44 on other nights, its log is gone, and its sweep test binary's rustc peaks near 6 GB
status: open
opened: 2026-11-14
priority: P3
cost: E
---

Filed by GREEN's `green/nightly-jobs` lane (2026-11-14).

## What

`suite under --all-features (default eps)`'s `build` step took 40, 38,
32, 44 and 32 minutes on the nightlies of 11-07 and 11-09 to 11-12, and
on 11-13 (run 43371638371, job 131708851478) was still running when its
60-minute step limit stopped it. That job's log returns 404, so what the
step was doing then cannot be read.

Until 2026-11-14 that step was `cargo nextest run --no-run`, which also
runs every test binary's `--list` (strace, nextest 0.9.140), so the
overrun was either the compile or a listing
(`nightly-all-features-build-hung-once-with-a-test-binary-running`,
closed). The step is now `cargo test --no-run` alone, and the listing is
its own bounded step that names a binary that blocks.

The other candidate is memory. Built locally on 4 cores and 15 GB with
the nightly's env and feature list (the tree at `f42cadd6`), `rustc` for
`sweep`'s `tests/all.rs` peaked at 5.9 GB, with 11.9 GB in use on the
box; one attempt that shared the box with other work was killed for
memory on that same unit. `ubuntu-latest` has 16 GB, so a night where
that unit overlaps the other large test crates would swap rather than
fail, which would read as a slow build.

## What would settle it

The next overrun, if there is one, now lands on a step that does one
thing. If it is the build: that night's log, read for which units were
compiling at the limit, and the runner's memory (`free -m` in a step
before the build would record it). If it is the listing step, it names
the binary.

---
id: actions-cache-budget-under-a-hash-key
kind: issue
title: Does the 10 GB Actions cache budget hold a WRITE-ONCE hash-keyed entry long enough to be worth writing?
status: open
opened: 2026-09-12
---


Split out 2026-09-12 from `rust-cache-never-restores-across-branches` at
its close, so the half that row held and did not answer does not die with
it (`work/README.md`: a residue disclosed inside a `## Closed` section
reads as work done and is invisible to the re-homing sweep — give it its
own file at the moment you disclose it).

## What the close DID answer, and why it is not this

Measured at that close: every sampled `build + archive` job restores its
`rust-cache` entry (0 of 16 miss-shaped per lane), and the build jobs sit
at 251 / 279 s against an 820 / 840 s cold figure. TCOST-B3's `push: main`
primer works.

**But it works by REFRESHING, not by surviving.** The primer re-saves
under a stable shared key on every main push, so those entries are
continuously rewritten and an eviction costs at most one push's worth of
staleness. That says nothing about an entry written ONCE under a key
nobody will write again.

## The question this row owns

TCOST-C4 (PR 1648) measured a **~205 MB entry churning out of the budget
inside the hour**: five restore attempts on one branch, hits at 9 and 17
minutes, misses at 38, 60 and 88. The budget is 10 GB for the whole
repository. A cache keyed on an input hash is written once and read
later — exactly the shape C4 measured being evicted, and exactly the
shape the primer's refreshing does not rescue.

So: **does a write-once, hash-keyed entry survive long enough to be worth
writing?** That is one measurement — write an entry of a realistic size
under a hash key, attempt a restore at increasing intervals, and report
where it stops hitting — and it decides at least one live row.

## Who is waiting on it

`work/ciw/cache-rendered-cells-on-input-hash` is parked on exactly this
and says so in its own `Unparked by` clause: *"either the budget stops
being the binding constraint... or it is measured and found to hold a
render-cells entry for long enough."* It was parked on the rust-cache row
because that row carried the budget question; it is re-parked here at the
same commit that closes that one.

Its non-goal survives the repository going public untouched, by its own
words — it never rested on minutes — so this is a live question and not
a re-costing casualty.

## Keep-out

Not a licence to build the render-cells cache; that design is CIW's and
is already written. This row answers one empirical question about the
budget and hands the answer back.

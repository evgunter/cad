---
id: two-anchored-pin-readers-two-homes
kind: issue
title: the tree now has two anchored pin readers, in two languages, and only one can go red
status: open
opened: 2026-09-11
refs: [seal-oracle-toolchain-read-first-match, ci-pin-quoted-branch-skips-the-bare-branch-rejection, 2327]
---

Disclosed by the style review of PR 2327, which is the PR that created the
second reader.

Both do one job: **read one declared pin out of an anchored region of a
declaration file, and refuse rather than choose when the region does not have
exactly one answer.**

- `scripts/ci-pin.py` — Python, standalone, importable by two checkers,
  `--selftest` with fixtures, invoked by a per-PR gate row
  (`ci.yml`'s `pin reader selftest`), refusals that name every line they saw.
- `local-scripts/seal-oracle.sh`'s `toolchain_pin()` — four lines of `sed`
  plus a count, inside a hand-run investigation script that every hosted job
  deletes. No selftest, no caller but its own file, and a refusal vocabulary
  that resembles the other one without matching it.

**Why it was written that way, and why that is not the end of it.** The
argument in PR 2327 stands on its own terms: `ci-pin.py` is anchored to a
workflow's `env:` block and says in its header that it will not be taught to
rank candidates, a TOML table is a different question, `scripts/ci-pin.py` is
META's file, and four lines of `sed` is proportionate to a script whose wrong
answers are visible in its own output. Each step is right. The result is still
one concept with two homes, and the cost is the ordinary one: the day the
refusal contract changes — and
`ci-pin-quoted-branch-skips-the-bare-branch-rejection` is exactly such a day —
one home is fixed, gated and selftested, and the other is four lines nobody
runs.

**What would settle it**, cheapest first: (a) nothing, with this file as the
record that the duplication was seen and priced; (b) give `toolchain_pin()` a
`--selftest` and a caller that a gate reaches, which lands it in
`criterion-selftest-nightly-only`'s class rather than out of it; (c) a
`ci-pin.py --toml <file> <table> <key>` mode, which is the widening its header
refuses and would have to be re-argued with META rather than assumed.

Not urgent: today both readers agree, and `seal-oracle.sh` reports its
toolchain on stdout before it probes anything.

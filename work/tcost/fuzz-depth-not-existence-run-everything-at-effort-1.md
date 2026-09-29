---
id: fuzz-depth-not-existence-run-everything-at-effort-1
kind: unit
title: Wire the EFFORT policy: ci-filter.py selects a raised EFFORT instead of excluding suites
status: open
opened: 2026-09-11
priority: P4
cost: E
---

Opened 2026-09-11 on Ev's proposal, in chat: *"everything is run at
EFFORT=1 always ... stuff that otherwise would've passed the gate gets a
higher EFFORT run"*. Ratified (PR #2363); the rule now stands in
`docs/prompts/implementer-discipline.md` §8: a fuzzer *"runs at
EFFORT = 1 in the default suite; its `gated_to!` marker names the paths
whose change raises it."* **None of it is wired.** Rewritten 2026-09-28
against the latency-cut CI; the argument that led here is in git
history at this file.

## What the tree does today

- `fuzz::effort()` defaults to 1 (`crates/test-utils/src/fuzz.rs`), and
  `scaled(n) = n * effort()`, never below `n`.
- `scripts/ci-filter.py` emits `TEST_FILTER` as an EXCLUSION: `ci.yml`'s
  `test` job skips every gated suite whose marker paths the diff did not
  touch. So the per-PR gate still decides a fuzzer's EXISTENCE.
- `nightly.yml`'s `full-suite` job runs everything at every eps row,
  gated suites included, at EFFORT = 1.
- Nothing in the kernel runs above EFFORT = 1. The one lane that does is
  the interval oracle (`CAD_FUZZ_EFFORT: "8"`, `interval.yml`), a
  separate cargo root gated at the job level by `ORACLE_PATHS`, and it
  stays job-gated: that root compiles ~234 s to buy ~7 s of cases, so
  running it on every PR spends the expensive half to buy the cheap one.

## The work

1. **Invert what `TEST_FILTER` means.** The default-eps step runs every
   gated suite at EFFORT = 1; the marker-derived set becomes the
   SELECTION for a raised-EFFORT step in the same `test` job (a step,
   not a job: a job pays a second build). The EFFORT value is measured,
   not guessed — the oracle's note in `interval.yml` is the precedent.
   Gated suites that are in the slow set stay out of the per-PR default
   step by that profile, not by the marker.
2. **Depth fails CLOSED** (Ev, 2026-09-11, *"depth fails closed is
   good"*). An unresolvable marker, an unreadable diff and a tier-`all`
   run get the smoke level and no raise. Existence failing open was the
   safe direction; for depth it would raise EFFORT on most merges.
3. **Depth keys on the marker's named paths against the diff**
   (`GatedSuite.selected_by`), not on the crate closure; nothing there
   changes.
4. **`fuzz.rs`'s doc** — *"in the seconds a gated CI job should cost"*
   assumes the gate decides existence; one sentence.
5. **The marker guards stay** (`--gated-check`'s resolution, the
   helper-import arm, the `#[path]`-mount arm). Under the rule a broken
   marker costs the raise, not the run.

## What closes with it

`proptest-modules-in-src-ungated`: its question (split a file whose
deterministic pins and property rows share one marker) stops existing
when nothing is gated for existence.

## Known inert entry

`crates/sweep/tests/tcost_k3_certificate.rs`'s marker names
`crates/step-import/src/lib.rs`, a crate `sweep` does not depend on, so
that diff never builds `sweep`'s tests. Harmless while the sibling
`crates/step-import/tests/tcost_k3_import_certificate.rs` names the same
path from inside the closure. If a second such entry appears without a
covering sibling, it is a row.

## Not taken: a timeout

A wall-clock cutoff makes what the test explored depend on the machine,
so it differs per leg, cannot be reproduced from the logged seed, and
manufactures apparent eps-sensitivity. A ceiling over the whole
EFFORT = 1 population is a tripwire, not the dial.

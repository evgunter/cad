---
id: pinned-version-named-in-present-tense-prose
kind: issue
title: prose across both halves asserts what is pinned NOW by restating the value, and goes false on a bump
status: open
opened: 2026-09-06
refs: [local-half-restates-ci-pins-as-literals]
---

Turned up by the whole-tree arm of the pin sweep in PR 2070 and deliberately
left out of that unit's reconciler, which reaches `local-scripts/` only. The
style review pushed back on part of the reasoning, correctly, so the residue is
filed rather than argued away.

**Two shapes, and only one of them is fine.**

*A record of what was measured* is correct as it stands and must not track a
bump: rewriting the number would falsify the record.

- `scripts/nightly-only-selection.py:150` — "nextest 0.9.140 does not match the
  quoted form"
- `scripts/pr-added-tests.py:304` — "0.9.140 over a three-test throwaway crate"
- `scripts/interval-only-selection.py:206` — "nextest 0.9.140's quoted form"
- `.github/workflows/nightly.yml:2030` — "Verified end-to-end on rustc 1.97.0 /
  cargo-nextest 0.9.140"
- `docs/perf-data/**`, `docs/CI-MINUTES-2026-08.md` — captured tool output and
  measurements, generated or historical

*An assertion about what is pinned NOW* is a different sentence wearing the
same clothes, and it silently becomes false the day the pin moves:

- `scripts/nightly-only-selection.py:23` — "verified against **the pinned**
  0.9.140"
- `.github/workflows/nightly.yml:1548-1549` and
  `.github/workflows/ci.yml:2751` — "measured against **the pinned** 0.9.140"
- `scripts/slowest-tests.py:109,121` — same present-tense spelling (`:316`
  is a captured verbatim record, the other shape).
  **S-TCOST's file** (`work/ciw/program.md`'s `keep_out`): reported here, not
  to be edited from CIW.

**The fix is a tense, not a mechanism.** "measured against the pinned 0.9.140"
wants to be "measured against cargo-nextest 0.9.140", which is a record and
stays true forever; the reader who wants to know what is pinned today reads
`ci.yml`'s `env:` block, which is the point of there being one. No checker is
needed and none is proposed: a reconciler over `scripts/` and the workflows
would have to tell a record from an assertion, and it cannot.

**Not urgent, and cheap.** Nothing installs from these sentences. What they
cost is a reader who believes one of them on the day after a bump — the same
currency as the `local-scripts/` copies, minus the executable command.

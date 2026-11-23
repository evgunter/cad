---
id: main-red-tier-gate-halfcap-eps6-refuses-at-the-last-bit-of-its-recorded-margin
kind: issue
title: Main red: step-import tier_gate's halfcap_eps6 refuses with a margin one bit off the one its table records
status: open
opened: 2026-11-23
priority: P1
---


Seen by `oneround/reach-from-the-read-point` on its slow-set run, and then on a clean `origin/main` at `fce2fbd91` (debug, the default ε). It fails identically there, so it is not that branch's. No open row was found naming it: `grep -rl halfcap_eps6 work` finds none.

- **The row:** step-import `tier_gate::every_corpus_import_passes_the_shared_gate` (slow set).
- **The fixture:** `tests/fixtures/halfcap/halfcap_eps6.step` at its file's ε is "refused for a DIFFERENT reason than the table records".
  - The table wants the message containing `… margin 9.999999999789241e-9 lies inside the ambiguity band (1e-9, 1e-8) …` (face #56, the sphere pole / cone apex / minted meridian meeting the face's boundary).
  - The import now prints `margin 9.99999999978924e-9`. That is the same refusal, the margin one ulp lower.
- **Not settled:** whether the table should match the margin's digits at all, and which merge moved it. It was not bisected.

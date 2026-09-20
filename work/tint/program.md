---
id: tint
kind: program
title: S-TINT — test-suite integrity
status: open
opened: 2026-09-11
area: infra
prefix: tint/
tag: (S-TINT orchestrator)
ab_band: 3500-3599
paths: [crates/*/tests/*, crates/test-utils/*]
keep_out: [cost is S-TCOST's question and no row here is justified by a cpu-second or a wall-second — a row that turns out to be a cost lever goes back by git mv, scripts/ci-filter.py and slowest-tests.py and base-test-listing.sh are S-TCOST's and so is the per-file gate mechanism they carry, .github/workflows/* is CIW's, scripts/gates/* is code-quality Track K's and tools/* is INSTR's, a fix that has to land in crates/*/src (a doctest moved out of tests/, a helper that must be pub) is announced to the program that owns that crate before it lands, no test is deleted for being slow and no fixed seed and no #[ignore] on a row that gates — S-TCOST's keep-outs bind here unchanged, changes to memories/ are Ev's call and a rule this program finds wrong is an [ev] PR and never a lane's edit]
priority: P3
---

**The instruments that cannot go red, after the 2026-09-20 cut**:
censuses, markers, rosters and hand-kept enumerations that pass on a
substring, on a term that matches nothing, or on a list nobody updated.

The source-scanning censuses hand-parse Rust and fail loud, so an
ordinary construct reds the gate rather than being read; `mesh7r1_probes`'
`R1-DOOR-ONLY` markers are grep-only, sentinels with no reader; the
hand-written-impl census's only per-impl sight anchor is a suppression
list that SHRINKS; a `Shared` ledger row is checked by one substring,
so a site that keeps the substring and loses the behaviour still
passes. And `test-utils` documents itself as dev-only while being
production source to every narrowing gate — the fact several of those
gates' own arguments rest on.

TINT measured **207.5 budget points unpriced and 97.5 once its rows
carried a cost** — the clearest reading yet of what the 2.5-point
default does to an unscored slate. It was cut on 2026-09-20 (Ev, in
chat) into four tracks: VACUITY (rows that cannot fail), FIXTURE (one
shape spelled n times), HELPER (one helper with several homes) and this
remainder. TINT keeps its band 3400-3499.

Charter and order: `work/tint/plan.md`; narrative in `work/tint/log.md`.

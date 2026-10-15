---
id: revalue-rows-exhaust-memory-with-profiles-elaborated-at-sym
kind: issue
title: With the pinned lift retired, revalue_corpus's boss_union row exhausts a 16 GB runner at Sym, and five other rows hold several GB each
status: open
opened: 2026-10-15
priority: P1
cost: M
refs: [the-pinned-profile-lift-launders-at-sym]
---


Found applying Ev's ruling A on #5068 (PATHS' `the-sketch-plane-is-its-frame`):
`ProfileLift::Pinned` retired, so `crates/editor-core/tests/revalue_corpus.rs`
now replays each corpus document at `Sym<f64>` with its profiles elaborated
at the lane (the guided second pass), as this program's
`the-pinned-profile-lift-launders-at-sym` owed.

Measured on #5068's head (dev profile, the hosted 4-vCPU / 16 GB runner and
a shared 4-core / 15 GB box):

- `revalue_boss_union` alone ran 3409 s on the hosted runner and the runner
  died (shutdown signal, then SIGKILL): run 38647943553, job
  116139800609, step "test (the slow set of the crates this diff seeds)".
  Four rows at once killed the runner after about 600 s on the previous head
  (run 38635923344).
- Locally the cgroup OOM-killed `revalue_boss_union` at about 5 GB RSS, and
  `revalue_cut_cylinder`, `revalue_die_chamfer`, `revalue_declared_tangency`,
  `revalue_plate_param` and `revalue_heat_sink_fins` with it, each holding
  3.5 to 6 GB.
- `revalue_loft_prism` (646 s), `revalue_face_sketch` (1053 s) and
  `revalue_measured_web` (2353 s) passed, on the shared box.

`.config/nextest.toml` now runs the re-valuation rows one at a time; that
was not enough for `boss_union`. Until this is answered the PR gate's slow
step cannot finish for any diff that seeds editor-core's slow set.

Owed: find where the symbolic replay's memory goes once profile geometry is
elaborated at `Sym` (the loops' arcs and the guided validation's
predicates are now symbolic expressions, where the pinned lift made them
literals), bound it, and re-pin each row's move and floor as
`the-pinned-profile-lift-launders-at-sym` asks. A row whose result changes
is a finding.

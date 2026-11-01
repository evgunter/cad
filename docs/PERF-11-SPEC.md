# PERF-11 — the remaining serial face walks in `props`: the shell census maps, the continuation is measured

**Status: ratified at dispatch (PERF orchestrator, 2026-09-13).** Binds
the implementer of unit `PERF-11`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/face-walks-outside-mass-properties-are-still-serial.md`;
`work/perf/quadrature-setup-is-re-derived-per-round-window.md` is
the item this unit measures against and may execute.

## 0. The finding this executes

PERF-8 made `mass_properties_impl` and `sign_certified` D9 idiom 1
(`decide_faces` + `splice_in_arena_order` + `fold_runs`) with the
K-funnel composed through `k_stats::detached`/`splice`. Two per-face
walks in `crates/topo/src/props.rs` stayed serial:

- `classify_shells_of`'s per-shell face loop — the same `face_flux`
  through the same `reporting_hook`, restricted to a shell's faces,
  with the same arena-order sum; nothing structural blocks the map.
  It is the door `assemble`'s tier-3′ census runs per solid, so on a
  many-solid document it is paid many times (the assemble census's
  own item is PERF-12's).
- `SignCertificate::refine_to_target` — NOT the same shape: it
  resumes each open face in arena order and must return at the first
  refusal it reaches, resumed or outstanding (the reporting walk's
  rule, so a later face's hard refusal cannot pre-empt an earlier
  face's budget one). A map would decide faces the serial
  continuation never reaches. What it costs today is the round-
  window setup re-entry the setup item measures (gate + continue
  1.3–1.8× one measurement on rational walls; the round spout's
  measurement flat at 15 s while its gate fell to 2.5 s).

## 1. What this unit delivers

1. **`classify_shells_of` as idiom 1 + idiom 2**: `decide_faces` over
   the shell's faces (one call site more of the PERF-8 shape — share
   it, no third spelling), recordings spliced in arena order up to
   the first refusal, the per-shell sum as the sequential fold it
   already is. Verdict logs and the probe population byte-identical
   at any width (the PERF-8 goldens' shape, cut on the merge base for
   the shell census's bodies: a multi-shell corpus body, a hollow
   body, an inside-out shell that refuses).
2. **`refine_to_target`, measured first, then decided.** Instrument
   (release, under the slot) how many faces a continuation resumes on
   the bodies that reach it — the tour's 78 gated bodies after PR 2440,
   `loft_prism`, the arc lofts, the round spout — and how much of the
   continuation's time is the per-face setup re-entry named in the
   setup item versus the rounds themselves. Then ONE of:
   - the continuation resumes its open faces as an indexed parallel
     map, decides every open face, and reports the first refusal in
     arena order — accepting the failure-path cost with the numbers
     that show it is bounded on the bodies measured, and keeping the
     refusal-order rule (the first refusal in ARENA order, resumed or
     outstanding — pinned by the row PERF-6 has: refusal parity with
     `mass_properties`); or
   - the continuation stays serial and the setup item is executed
     instead: the lane's round-independent setup (derivative grids,
     hulls, interior-knot lists, `last_round_width_lo`) is computed
     once per face and carried in the `FaceRun` across windows, so a
     resumed window pays rounds only. Bit identity of every round's
     pieces is the pin (PERF-6's `gate + refine == one` and the
     reporting-door digest); the setup value must be a pure function
     of the face and ε, stated at the type.
   Choose by the measurement: if the resumed faces are few and the
   setup dominates, the second; if many faces resume and rounds
   dominate, the first; if both, both are in scope. Say which and why.
3. `refine_to_target`'s doc states the true cost after this unit.

## 2. The pin

- PERF-8's thread-count goldens and PERF-6's digest, `sign_certified_
  plus_v`, `tcost_k3_certificate` unchanged; new goldens for the
  shell census at 1 and 4 threads cut on the merge base.
- `assemble`'s aggregate census verdicts on the corpus unchanged
  (its own suite; `crates/editor-core` product rows).
- Whatever shape §1.2 takes: refusal parity between `refine_to_
  target` and `mass_properties` on every roster body (same face,
  same class) at 1 and 4 threads; piece-evaluation identity.

## 3. Measurement to report

Release, 4 vCPU under the slot, medians of 3, at 1 and 4 threads:
the shell census on a many-solid document (the heat sink driven to
40 and 160 fins as PERF-12's item does — coordinate with PERF-12,
which runs beside you: it owns the census's pair sweeps, you own the
per-solid shell walk; merge `origin/main` before opening the PR);
`refine_to_target` on the bodies of §1.2 before and after; the round
spout's measurement through the tour.

## 4. Out of fence

`fold_runs`; the quadrature schedule and target; the funnel door (it
exists); `classify_shells_of`'s decision rule; the census's pair
sweeps (PERF-12). PROPS/TOPO territory, announced in `work/perf/log.md`.

## 5. Report

≤120 lines: the shell walk's shape, the continuation's measurement
and the choice it forced, the pins, the measurements of §3,
deviations, findings outside the fence.

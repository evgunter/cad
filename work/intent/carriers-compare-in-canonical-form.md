---
id: carriers-compare-in-canonical-form
kind: issue
title: D10 stage 4 PR C: rung 2 at the door: CarrierForm per kind, LinForm over VarId with exact rational coefficients, each verb's CarrierFlow, and the lint's recourse
status: parked
opened: 2026-10-08
priority: P0
cost: H
design: true
blocked_on: [coincidences-are-recorded-at-one-door, operands-are-reads, select-defines-face-and-edge-variables]
needs_ev: true
---

INTENT stage 4, PR C. Spec: `docs/INTENT-STAGE4-SPEC.md` §4. Design open: FORK-S4-1 (provenance read from names vs `GeomSource`; N6) and FORK-S4-2 (a stated `CarrierFlow` with a witness vs derived).

`editor-core/src/canon.rs`: `LinForm` over `VarId` with exact rational coefficients (definitions expanded, nonlinear subterms opaque atoms), `PoseForm` modulo the kind's `Subgroup`, `CarrierForm` per kind. Each verb's `CarrierFlow` beside `ParamFlow`, with a corpus witness. The door gains `Rung::CanonicalForm` and the residual-driven recourse. A placement chain is one opaque pose atom until H. Needs stage 2 B (a frame is a read) and E (a face-read frame reduces). Releases 2 rows (spec §12).

FORK-S4-2 was weighed as FORK-S4F (fork log row 93) and went to Ev in an
`[ev]` PR with FORK-S4P; this unit builds on the answer provisionally,
and it reshapes the unit. No construction describes its outputs: the
hand-written `CarrierFlow`, `LinForm`/`PoseForm` builders and the f64
corpus witness are not built. The door replays the document at `Sym`
with every variable a symbol and proves a row by two rungs: the
kernel's own carrier-pair verdict (`oriented_plane_eq`, `carrier_eq`)
run on the two named cells' symbolic carriers, then the identity of the
deciding margin. Units C and D merge, and need nothing from stage 2 (slots
and placement steps are `VarId`s since stage 1), so this unit's
`blocked_on` drops `operands-are-reads` and
`select-defines-face-and-edge-variables` when Ev rules. The guard is
one re-valuation test (re-value each corpus document's variables, compare
the derived forms with the f64 build), which needs
`a-computed-value-re-enters-as-a-constant`. H becomes a measurement after
stage 3 (does a mate-placed contact discharge as a theorem?) and a build
only if not; until stage 3 a solved pose is one opaque symbol per solve.

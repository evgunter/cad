---
id: EXCH-E1
kind: unit
title: D343 — typed payloads stop rendering through Debug in the two STEP crates, with its two riders
status: dispatched
opened: 2026-09-04
branch: exch/d343-typed-payloads
refs: [D343, 1490, 1481]
---

Executes code-quality row `D343` (Track U, claimed by EXCH):
`step-import`'s `error.rs` `Display` renders `Placement`/`Instance`
kernel sources `{source:?}` while sibling arms forward `{source}`
(the `TierInvalid` `{e:?} — {e}` spelling is argued at the site —
cite it before converting; the `{at:?}` coordinate triple is
location, not payload); `step-export`'s `lib.rs` has eight `{key:?}`
arena spellings reaching Python through `pncad`'s export door,
against `pncad-py`'s never-an-arena-key posture. Two riders on the
same lane: `writer.rs::closed_shell` emits unconditionally under a
prose-only "currently unreachable" note (the guard is one
`is_empty()` refusal), and
`writer.rs::tests::the_carrier_placeholder_refuses_typed` never
drives the `UnsupportedCurve` refusal it is named for. E build,
single style review, outside the A/B row protocol (test/error-surface
unit; FILLET E1–E3 the precedent). Runs in parallel with EXCH-H1 on
disjoint files; the row `D343` closes at this unit's merge.

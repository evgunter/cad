---
id: mates-declare-no-contact
kind: issue
title: D10 stage 4 PR I: ContactClass on mates, class_admission and MintedDeclaration retire; at rest every contact is recorded at the door and A5's hard error becomes an unproven-coincidence finding
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [placed-carriers-compare-through-their-frames, declared-pairs-retire]
---

INTENT stage 4, PR I. Spec: `docs/INTENT-STAGE4-SPEC.md` §9.

`Node::Mate.class`, `ContactClass`, `class_admission`, `MintedDeclaration`, `NoAtRestRecord` and `FIT_DEFERRAL` go. The at-rest census records every contact between copies at the door; A5's hard error on an unattributed contact becomes an `unproven-coincidence` finding. Pierces and same-side crossings stay interference, as stage 5 B (`interference-at-rest-is-a-finding`) leaves them; stage 5 C reports these rows. The perf12 census goldens are re-blessed as rows.

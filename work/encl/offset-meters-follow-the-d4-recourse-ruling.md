---
id: offset-meters-follow-the-d4-recourse-ruling
kind: issue
title: the offset meters fold Zero into Negative and say 'lower the tolerance' where D4 now says 'tighten, if this size is intended'
status: closed
pr: 3382
opened: 2026-09-28
priority: P3
cost: M
closed: 2026-09-28
---


## What

The D4 ¶1 ruling (Ev, `[ev]` PR 3352, 2026-09-28) derives a refusal's ending from its decision and verdict. PR 3347 routed the offset meters by predicate name, before the ruling. Three things now disagree with the rule.

- **Zero and Negative are folded together.** `geom_brep::offset_meters::MeterError::NormalFloor` and `CurvatureHeadroom` (`crates/geom-brep/src/offset_meters.rs`) fold `Sign::Zero | Sign::Negative` into one variant.
  - These are clearance-type decisions, which pass on Positive.
  - Their Zero-classified arm is band-decided, so it should carry "…, or, if this size is intended, tighten the tolerance below m". The certainly-Negative arm should carry no ε advice.
  - The fold means today's text cannot tell the two apart. For `NormalFloor`, whose margin is ≥ 0, the definite arm is always Zero, so it always lacks the tighten clause.
- **"Lower" should be "tighten, if intended".** The in-band escalation (`escalation_recourse`, with `NORMAL_FLOOR_RECOURSE` and `CURVATURE_HEADROOM_RECOURSE`) says "or lower the tolerance". Under the rule it is the conditional, valued "if this size is intended, tighten the tolerance below m/K".
- **Routing is by name, not by type.** It goes through predicate-name constants, where the rule asks for the decision's closed type: the meter kind, with its pass set.

## Repair shape

- Carry the verdict (Zero vs Negative) on the variant, or as a field.
- Derive the ending from (meter kind, verdict).
- Re-pin the offset-fit concision rows and `validate`'s `classify_offset_fit` meter rows.
- Follow the reshape of PR 3351 (certify), which sets the pattern.

---
id: census-span-clear-readers-push-their-refusal-as-they-read
kind: issue
title: census::span_clear's readers push their refusal into the error list as they read, so its first-refusal loop cannot keep a reading in band
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by QML T4 (`onecopy/refusal-standing`), on the callee register's `census::span_clear` site.

## What

`topo::census::span_clear` asks whether anything of a face's boundary meets an edge's span: anything meeting answers "not clear", a theorem of the rule, so the ladder's `Any`. Its three reads (`on_edge_interior_reading(..)?` at a lone vertex and at a side's end, and `interiors_meet(..)?` along a side) are by two readers, which return `Option` and **push their refusal into the census's `errors` list as they read**. `None` means "refused, already reported".

So a reading in band cannot be kept and superseded by a later member that meets: by the time the ladder would know an answer came, the refusal is already in `errors`, and the record already rests on it.

## Done when

The two readers return their cause (`Result<_, Cause>`) instead of pushing it. `span_clear` then climbs the boundary's members under `Any` and pushes only the ranked refusal where no member answers. Its line leaves `scripts/gates/escalation-question-marks-callee.counts`. The census has other readers of the same push-as-you-read shape: sweep them in the same pass.

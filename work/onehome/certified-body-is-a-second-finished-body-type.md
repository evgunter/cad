---
id: certified-body-is-a-second-finished-body-type
kind: issue
title: topo::CertifiedBody (body + the gate's sign certificate) is a second finished-body type beside AtRestBody
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Found by the gate-typing fork's designer weighing (PR 6326, 2026-11-02).

`topo::CertifiedBody` (f64 only: a body plus the sign certificate check 7 derived, built by `step-import`'s `gate3` and read by the export prelude) and `topo::AtRestBody` both mean "a body that passed the gate, with what the gate derived on these bits". Both designers place folding them into the one finished type (holding what the gate derived through `T::Carries`) in the final state of PR 6326, as its own unit sequenced after it. Putting the Boolean's contact records into the finished type (DESIGN's tier 3′) is a further step that waits on `work/reachhold/boolean-door-runs-the-census-over-its-result.md`.

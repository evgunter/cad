---
id: the-conic-test-oracle-states-an-arc-held-ellipse-through-its-frame
kind: issue
title: The double-double conic oracle states an arc-held ellipse through its frame, a different curve from the one the evaluator stores
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [7523]
---

Found by the conic-reader fork's class sweep (`analysis/oneround/conic-harmonic/sweep2-readers.md` on `oneround/conic-harmonic-probe`) and by two of its designers, 2026-11-23; read, not run.

`crates/geom-brep/src/curve_face/conic_oracle/exact.rs` `truth` builds its double-double statement of a conic from the conic's frame. For an ellipse held on its arc the frame's origin is the derived centre, rounded at the semi-axes' scale, so the oracle states a curve that stands off the stored one by about u·a, and a row judged against it on such an ellipse is judged against the wrong curve. Every arc-held ellipse today is `plane_cone_section`'s, at sizes where that offset is far under ε, so no row is known to pass or fail wrongly.

Owed: the oracle reads what the evaluator reads (the stored point, the phase, the semi-axis vectors), as the suspects lane's 60-digit oracle does (`analysis/oneround/conic-suspects/oracle_t1.py` on `oneround/conic-suspects-probe`). It is under PR 7510's gate (`truth`, in the certified-harmonics row's list), so it goes when `Conic::frame` does at the latest.


---
id: a-seat-held-apart-across-two-shells-of-one-solid-passes-the-gate
kind: issue
title: A seat whose two faces end in different shells of one solid passes the held-apart gate, which compares shells
status: open
opened: 2026-11-10
priority: P3
cost: E
refs: [a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record]
---


Found by SEAL's lane on `a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record` (2026-11-10), from a run probe.

The result gate `crates/topo/src/boolean/ops.rs` `Held::apart` (~5630) refuses where one **shell** holds both faces of a carried area record. Take the flush coaxial cut: a shaft `z 0..0.5` in a bored plate, the seat recorded by hand, cut by a block covering it. It builds one solid in two shells, 18 faces, volume 0.576, where the right body is two solids. Each record's two faces lie in different shells of that one solid, so the gate passes it.

Today the curved seat is read by no gate (the referenced row). No planar flush analog was testable: the census does not record that seat (five findings), and the cut refuses `LineWedges`.

Done when the gate compares solids, or a reason is recorded why shells suffice, with a pinned planar or curved flush row.

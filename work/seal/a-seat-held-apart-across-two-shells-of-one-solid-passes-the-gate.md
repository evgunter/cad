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

## Narrowed (2026-11-12, corrected on PR 6860's fix pass): the flush cut no longer reaches the gate

The flush coaxial cut this row measured is a cut by a tool whose solids rest on each other, which the cut now reads as material (`crates/topo/README.md` `[topo.cut-material]`). The tool's curved record between its two solids refuses `ToolSeatUnread` at the cut's door (`crates/sweep/tests/a_cut_by_a_curved_seat_refuses_its_material.rs`, the `flush` case). A planar record between two solids is fused before the cut.

A face–face record inside one solid, this row's two-shells shape, is handled at the door as follows:
- **Planar**: it refuses `TouchesItself` naming the record, whatever the cut reaches (`a_forged_record_of_the_seat_refuses_loudly`). On main, a cut refused only where a vertex crossed both faces or a result shell held both.
- **Curved**: it rides into the plain cut as on main.

Not measured: whether a finished body can hold a planar one. No fixture was built, since an operator whose result holds an area self-touch inside one shell refuses `TouchesItself`. The refusal is pinned with a caller's forged record.

The row narrows to the ops that keep the pieces: a `∪` that bridges a seat and leaves its two faces in two shells of one solid, and a curved record inside one solid, which no gate reads.

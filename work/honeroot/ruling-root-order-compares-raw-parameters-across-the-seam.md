---
id: ruling-root-order-compares-raw-parameters-across-the-seam
kind: issue
title: the section ruling's root order compares raw parameters, so two roots ε apart across its ±π seam read a turn apart
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [periodic-branch-floor-remains-at-the-raw-picks]
---


Found by PCERT's `periodic-branch-floor-remains-at-the-raw-picks` (PR
4941's review), by reading the readers of `circle_roots`' re-centred
roots. Not measured.

`crates/topo/src/boolean/section_cert/ruling.rs` reads
`certified_subdivision`'s roots on the fixed window `(−π, π)`, and
re-centres a seam root onto `−π`. It then sorts them with `before`,
which decides `(y − x)·speed` less both roots' slacks as raw arc length,
not modulo τ.
- Two roots within the slack of each other ACROSS the `±π` seam differ
  by about τ there. So `before`'s refusal of an ε-close pair (the
  in-band arm) never fires for them, while the same pair inside the
  window would refuse.
- The assembly reads folds cyclically (`folds[0] + τ` for the wrap
  arc), so the wrap arc's asymptote test can place an asymptote between
  two such roots where an in-window pair would refuse.

What would close it: decide `before` on the centred difference (or ask
the seam pair separately), with a row that puts two roots ε apart
across `±π`.

---
id: a-path-held-at-a-lane-scalar-owes-a-roll-that-follows-it
kind: issue
title: A path held at a lane scalar owes a station roll that follows its variables; the chained carry is two orders too wide at 1e-9 and freezes at Sym
status: open
opened: 2026-11-13
priority: P2
cost: H
---


Filed by the station-frames fork (DESIGN-FORK-LOG row 266, `[ev]` PR on `onehome/ev-station-roll`).

**Today.** A skin's path is `NurbsCurve3<f64>`, a constant of the model, and no document's Sweep reaches `sweep_body` yet (`SWEEP_FRONTIER` in editor-core). So the roll is selected at `f64`: one reference vector per station, with each frame minted at the lane scalar (`crates/sweep/README.md` `[sweep.frame]`). The day the path is held at `T`, its roll depends on the variables. A frozen roll would then certify a family whose roll differs at first order from the body built at each point of the box.

**What a design for that day must solve**, measured on the probe commit `c9ea79bf` (`onehome/skin-turn`; a 48-station ribbon and a 65-station helix):
- **Interval width.** A carry chained at `T` wraps: about ×1.2 per step, reaching 8.9e-7 at the helix's last station. A Lohner-style (wrap-free) carry grows additively, at about 2e-12 per step (4.6e-10 and 4.7e-10 at the last stations). That is still about two orders too wide at ε = 1e-9, where four of the five injectivity rows refuse, and all five refuse at 1e-12.
- **The per-step cost.** A step minted at `Interval` costs about 2e-12, roughly 20× a frame minted alone (1.1e-13). Find where that factor comes from first.
- **Sym.** The minted turns are large forms over the path, and they freeze. The wide-start-frame row discharges less (663 → 517), and it refuses at 1e-12 where main passes. The five rows run out of memory at Sym<Interval> on main as well; that is a separate defect.
- **Must hold:** width that does not grow with station count when the path is a point; identities along the stations that discharge at Sym without expanding the carry; and a typed refusal naming the station frame, not a downstream certificate's text.

**Acceptance.** The sweep rows main certifies at the gated ε, including the wide-start-frame row at 1e-12, pass at `Interval` and at `Sym<Interval>`.

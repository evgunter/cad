---
id: touch-point-spread
kind: issue
title: A touch's reported point carries no certified spread
status: open
opened: 2026-11-10
priority: P1
---

Found answering Ev on PR 6570 (comment 6762758008), traced on main `8be19d2f88`.

A touch's meeting is reported at one point of its in-band arc, and nothing bounds how far that point can move between two readings of the same contact (two spans, two operand orders, two callers). Readers that match the point by distance against the band (census pass 5′ `Declared::confine`, the boolean's curved pierce `held_on`) therefore rest on a reproducibility nothing certifies. Under C5 option A and option C alike, that is a canonical point picked with no certified bound, which is a fail-loud gap.

- **Coplanar subdivision touch**:
  - `crates/geom-brep/src/half_angle.rs:972-997`. The least `G″` on the piece (`curve`) is proved positive (`touch.convex`), and `F′`'s rounding `e₁` is charged.
  - `θ*` is the midpoint of a bisection bracket on `F′` (`:980-981`), reported at `:995`. Its spread `(|G′(θ*)| + e₁)/c` is bounded by terms already read at `:990-992`, but it is never computed, returned or compared.
  - Only the fall, a distance, is read against the band.
- **Coplanar closed form**: `crates/geom-brep/src/curve_pair/pose.rs:182-188`. The touch is `C(θ_hi)`, the first harmonic's angle. No `c` is certified and no spread is carried.
- **Across planes**: not affected. The polish's reach `m₋` (`pose.rs:258-268`) is a certified spread, and at a true touch the basin fails and the reading refuses or escalates (`pose.rs:169-176, 313-316`).

Not in scope: whether one point may stand for the whole in-band arc where a reader's question (In/Out of a face, on an edge or a vertex) straddles the arc (`crates/topo/src/census.rs:1302-1313`, `crates/topo/src/boolean/carrier_cross.rs:245`). That waits on Ev's ruling on PR 6570.

**Done when**:
- Both coplanar touch paths compute the reported point's spread and carry it with the meeting. The subdivision's is `(|G′(θ*)| + e₁)/c`. The closed form's is a certified bound on `θ_hi`'s rounding over the harmonic's amplitude.
- A reading escalates (typed, `MeetFault::Escalated`) where that spread is not below the band its matchers use.
- Pins show both: a near-degenerate touch whose spread exceeds the band escalates, and a well-conditioned touch reads as before.

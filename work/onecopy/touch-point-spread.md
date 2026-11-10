---
id: touch-point-spread
kind: issue
title: A touch's reported point carries no certified spread
status: closed
opened: 2026-11-10
priority: P1
closed: 2026-11-10
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

## Closed (2026-11-10, `onecopy/touch-point-spread`)

**Measured first.** A temporary log of every coplanar touch's spread, over geom-brep, topo, sweep and editor-core's ci profile, read about 610 touches at each ε: 48 placed by the subdivision, about 280 conic beside a line, and about 287 circle pair. None read a spread that the band does not hold.
- The worst subdivision touch reads `3·10⁻⁴` of the band's zero.
- A first circle-pair bound that charged the centres' size, not their difference's, would have escalated 17 touches of `meet_oracle`'s pinned draw. That would have taken it under its seven-in-ten floor, so it was the wrong bound: a correctly rounded difference of two near centres errs by `u` of its own size.

**The subdivision** (`crates/geom-brep/src/half_angle.rs`). `certified_subdivision_touching` now returns each touch as a `Touch { at, spread }`. The spread is `speed_hi·(|G′(θ*)| + e₁)/c`, from the terms already read there.

**The closed forms** (`crates/geom-brep/src/curve_pair/pose.rs`, `angle_spread`). The touch is `C(θ_hi)`, `θ_hi` the angle of the first harmonic's coefficients. That angle is a well-conditioned reading of them: its rounding is relative to their length, however near the centres or however close the conic to the line.
- So at a point scalar it is treated as exact, and the spread is zero. This applies ratified DESIGN `[simple.conditioned]`, under which no site charges rounding; it is not a departure from the row's "Done when". The row's "a certified bound on `θ_hi`'s rounding" is, under that rule, the angle's own enclosure. (A `geom_core::running` charge, tried first, is what `scripts/gates/bare-rounding-allowlist.sh` refuses.)
- At `Interval` the spread is the angle's own enclosure, `r·(θ − θ)`, which is certified and charges nothing.

**Held to the band** (`spread_held`, `curve_pair_pose_touch_spread`). The most two readings can lie apart is read: twice the subdivision's spread, or the angle enclosure's width.
- Zero keeps the touch.
- In the gap it escalates (`MeetFault::Escalated`).
- Decided past the band it refuses on the row's meter (`Unsettled::Meter`), as a root's slack past the band does.

**Pinned** by `curve_pair::touch_spread_rows`:
- `a_well_conditioned_touch_reads_one_meeting_at_the_touch`: the three paths at unit scale, at ε = 1e-6, 1e-9 and 1e-12.
- `the_subdivisions_touch_whose_spread_exceeds_the_band_escalates`: two touching ellipses scaled out to `1e4` escalate, and to `1e5` refuse on the meter, at ε = 1e-12.
- `a_closed_forms_touch_is_held_to_its_angles_enclosure`:
  - at `f64`, far out, one meeting;
  - at `Interval` widened by `w`, two nearly concentric circles escalate on the spread;
  - the circle beside a line escalates on its extreme first, since it has no cancellation to widen its angle past it.

The two escalation rows are red with the spread unread.

The layer-2 question (whether one point stands for the in-band arc) stays with Ev's answer on PR 6570.

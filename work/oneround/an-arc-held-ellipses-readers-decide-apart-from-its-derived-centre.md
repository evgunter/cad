---
id: an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre
kind: unit
title: curve_pair::meet answers no meeting for a line or circle through a point of an arc-held ellipse
status: dispatched
opened: 2026-11-23
priority: P0
cost: H
branch: oneround/conic-pair-readers
parent: the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre
refs: [7203, 7510]
---


**A wrong answer on main, not a refusal.** Found 2026-11-23 by the conic-harmonic measurement lane (`oneround/conic-harmonic-probe`), reproduced on `origin/main` at `2130aa45f` through the public API.

`geom_brep::curve_pair::meet` answers `Ok` with zero meetings for a line that passes through a point of an ellipse held on its arc (in the ellipse's plane, square to the arc, 0.3 m of arc from the anchor):

| ε | semi-axis `a` | answer |
|---|---|---|
| 1e-9 | 1e4, 1e5, 1e6 | one meeting |
| 1e-9 | 1e7, 1e8 | refused (`curve_pair_walk_pair_gap`, margins 2.6e-9, 9.2e-9) |
| 1e-9 | 1e9 | **`Ok`, no meeting** |
| 1e-12 | 1e4, 1e5 | refused (the same row) |
| 1e-12 | 1e6 to 1e9 | **`Ok`, no meeting** |

**Judged against** the curve the stored numbers state (anchor, axis, seam, semi-axes, phase), evaluated at 60 digits by the lane's oracle: the line is built through `e.point_at(θ)` itself and the true gap of these poses is 4.7e-17 m. `plane_cone_section` (cone half-angle 1.3) mints such ellipses up to `a` = 1e9 at both ε, none refused.

## The reproduction

```rust
use geom::{Conic, Curve3, Ellipse};
use geom_brep::curve_pair::{Span, meet};
use geom_core::{Band, Point3, Tol, Vec3};
#[test]
fn a_line_through_a_point_of_an_arc_held_ellipse_meets_it() {
    let band = Band::linear(Tol::witness()).unwrap();
    let a = 1e6; // 1e9 at eps 1e-9
    // the stored numbers plane_cone_section mints on the probe's ladder
    let e = Ellipse::anchored(
        Point3::new(1.3, -0.7, 0.4),
        Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528),
        Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582),
        a, 0.3 * a);
    let theta = 0.3 / (0.3 * a);
    let p = e.point_at(theta);
    let tan = e.derivative_at(theta).normalize();
    let d = e.frame().axis().cross(tan).normalize();
    let line = Curve3::line(p - d * 2.0, d);
    let carrier = Curve3::Ellipse(e);
    let half = 10.0 / (0.3 * a);
    let got = meet(&Span::new(&line, (0.0, 4.0)), &Span::new(&carrier, (theta - half, theta + half)), band).unwrap();
    assert!(!got.points.is_empty()); // fails on main: Ok, 0 meetings
}
```

Run with `CAD_TOLERANCE_EPS=1e-12`, or 1e-9 for the `a` = 1e9 row.

## Where it goes wrong (the lane's trace, on its branch; main gives the same verdicts)

Not the root's posing: taking the harmonic's constant from the stored point, or posing the root about it, leaves every miss in place. Three other readers of the derived centre decide "apart", none of them metered for the centre's rounding:

1. **`line_conic`'s router** (`curve_pair/pose.rs`): the line's height above the ellipse's plane is read `(o + u·mid − k.frame().origin())·n`, from the centre. It reads −1.8e-11 m at `a` = 1e5 and −6e-8 m at `a` = 1e9 for a line that lies in the plane, and `route` answers `Parallel`. 61 of the 79 line misses at 1e-12, 20 of 25 at 1e-9.
2. **The in-plane `Slant`'s `H`**: the conic's offset along the line's other normal, its constant read from the centre; floor 1.3e-11 to 1.5e-8 m, so the placed candidate is read off both spans (`place`, `clear_by`).
3. **Ellipse against a circle across planes** (a unit circle through the same point): `window.holds_only` in `transverse` certifies the window empty; 66 misses at 1e-12 (`a` ≥ 1e5), 28 at 1e-9 (`a` ≥ 1e8). Not traced inside `window`.

Counts on the lane's branch near the anchor (within 1 km of arc), of 324 poses: the line door 79 certified misses at 1e-12 and 25 at 1e-9; the circle door 67 and 29. None at `a` ≤ 1e7 at 1e-9; none at `a` = 1e4 at 1e-12.

## What it is

PR 7203 made the cone's section an ellipse held on its arc; the readers outside `conic.rs` still read its derived centre, which is rounded at the semi-axes' scale. The row cut for those readers said they lose digits; it did not say that an unmetered one answers wrongly. **Class: a reader that decides from a derived datum without a charge for the datum's own rounding certifies what it cannot see.** The measurement lane is sweeping the other 62 functions the gate pins for the same shape; each further wrong answer is added here or gets its own row.

Whether main before PR 7203 answered these poses rightly is being measured; the line there has to be built through the true section point, not through `point_at`.

## The unit

`oneround/conic-pair-readers`, stacked on `oneround/conic-readers` (PR 7510). **Letter H; tier single, a FULL review**: the fix is in the certified lane of `curve_pair`, where a reading moved to another datum changes what each margin is measured against, and believing it takes more than reading it.

Every reading of where a conic is that `curve_pair` makes (`pose.rs`, `pose/window.rs`, `walk.rs`, `on_surface.rs`; the gate's list for those files) is read from the point the conic stores, through the conic: a point's height over the conic's plane; a line's or a plane's height along the conic as a first harmonic about the stored point; a foreign point's parameter and level. Nothing outside `conic.rs` learns the placement. Where a reading stays on the derived centre, its verdict is metered for the centre's rounding, so it refuses and cannot certify. The direction is the one three designers signed for the whole family (`analysis/design-fork/conic-reader-interface`): the readings this unit adds to `Conic` are what that design's position would later answer, and no more.

Acceptance: the reproduction above as a row, and the lane's ladder (`a` from 1e4 to 1e9, fat and thin, line and circle doors, near the anchor and far from it) at 1e-9 and 1e-12 against the 60-digit oracle: no pose certified wrongly, and every refusal that remains named and metered. A differential against the base: every verdict that moves, judged against the oracle.

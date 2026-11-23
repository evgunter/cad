---
id: an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre
kind: unit
title: Readers of an arc-held ellipse decide from its derived centre with no charge: curve_pair::meet misses, the plane door says Miss for the ellipse's own plane, on_carrier says Off inside the gap
status: dispatched
opened: 2026-11-23
priority: P1
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

**Not a regression of PR 7203: the same verdicts, miss for miss, on the tree before it** (measured 2026-11-23 by the same lane on `194131e40`, the first parent of `2b9cd8b24c`, with the section held at its centre and the line built through the point where a cone generator meets the plane, judged against the cone and the plane themselves at 60 digits):

| `a` | ε 1e-9, before and after | ε 1e-12, before and after |
|---|---|---|
| 1e4 | 3 meet | 2 meet and 1 escalates before; 3 meet after |
| 1e5 | 3 meet | 3 escalate |
| 1e6 | 3 meet | **3 miss** |
| 1e7 | 2 meet, 1 escalates | **3 miss** |
| 1e8 | 3 escalate | **3 miss** |
| 4.8e8 (1e-9) / 1e9 (1e-12) | **3 miss** | **3 miss** |

What PR 7203 changed is where the stored curve stands, not what `meet` answers. Before it the stored section itself stood off the true section by the centre's rounding (1e-10 m at 1e6, 3e-8 m at 1e8), so each miss was wrong of the cone and the plane but not provably wrong of the stored numbers. Since it the stored curve passes within 1e-15 m of the line, and the same answer is wrong of the stored numbers too. The readers outside `conic.rs` read a conic's centre and decide from it with no charge for its rounding at the semi-axes' scale; the row cut for them said they lose digits, not that an unmetered one certifies wrongly. **Class: a reader that decides from a datum rounded at a scale larger than the tolerance, with no charge for it, certifies what it cannot see.**

## How far it reaches: no built body, on main as it stands (measured 2026-11-23)

Through the public door `curve_pair::meet`, the plane door and `Curve3::on_carrier` these are wrong answers. **No modelling door builds a body on a section large enough for any of them to go wrong**, by two measurement lanes (`analysis/oneround/conic-harmonic/sweep.md` on `oneround/conic-harmonic-probe`; `analysis/oneround/conic-suspects/task3.md` on `oneround/conic-suspects-probe`, `93d18a6ac`, on clean main `dc263f855`, f64):

- **The largest cone-section semi-axis any door builds is 1.0e6 m at ε 1e-9 and 1.0e3 m at ε 1e-12**; the measured readers go wrong from 1e8 and 1e5, two decades above at both ε.
- **The boolean's own mint is no wider than `split`'s**: `union`, `intersect`, `subtract` and `split` build and refuse at exactly the same sizes over 12 poses and 60 tilts (half-angles 0.46, 1.0, 1.3; as built and under a generic rigid motion).
- **Every body built is right**: 832 bodies, each section within 4.8e-16 m of the operands' cone and 3.6e-16 m of the plane as its stored numbers state it, each boolean's volume within 8.1e-16 relative of the closed form, judged at 60 digits.
- **What stops it is about the carrier, not the pose**: past the largest built size, `VolumeUncomputable` on `props_cone_ellipse_on_cone` (escalating, one or two tilts wide), then for every larger section the edge certificate's `KernelLimit`, whose bound is 5e-16 to 1e-15 times the semi-axis and so refuses any cone-section edge past about ε/6e-16 (1.7e6 m at 1e-9, 1.7e3 m at 1e-12). It runs on every minted edge, at `topo::finish` and at STEP adoption. A body cannot be made larger to dodge it: `finish` refuses anything past the session box.
- **Four of the readers ARE asked about the section in every body that builds** (`plane_conic` through `height`; `Foot::on_carrier` through `clearance`; `solid_contain::curved_edges::passes`; `props::curved::require_cone_ellipse`), at sizes below where they go wrong; where `KernelLimit` refuses, the edge dies at its mint before any of them reads it.
- **Not shown closed:** a hand-written STEP file with a larger `ELLIPSE` (files at semi-axes of 6e4 and up were refused on `SingularitySideUndecided`, not traced, which is not the carrier's limit); `factored_crossing` on a far section's edge inside a boolean.

**So this is a wrong answer at the curve-level doors that reaches no built body today, kept so by one certificate's limit with two decades of margin, not by a re-read of any answer.** A change that loosens `KernelLimit`, or mints a cone-section edge without it, exposes four readers at once. **Re-banded from P0 to P1 on this measurement**: it is the readers' architecture (the band of the row it was cut from) guarding a latent wrong answer, not a verb broken on geometry a person can author. The fix unit goes on as dispatched.

## Two more of the class (2026-11-23, the lane's sweep; `origin/main` at `2130aa45f`, public API)

**A. The plane door answers `Miss` for the ellipse's own plane.** `geom_brep::curve_face::plane_crossing::plane_crossing_lane(&carrier, t0, t1, e.point_at(0.0), e.frame().axis(), band)`, the plane through the ellipse's stored point with its stored axis:

| ε | answer by semi-axis |
|---|---|
| 1e-9 | 1e4 to 1e7 `InPlane`; 1e8 `Roots(Err(PlaneParallel))` (escalates); 1e9 **`Miss`** |
| 1e-12 | 1e4 to 1e6 `InPlane`; 1e7, 1e8, 1e9 **`Miss`** |

`Miss` is documented "the carrier definitely never meets the plane", and the plane's origin is a point of the carrier (the anchor's own bits). Cause, by the lane's reading: `curve_face::pose::plane_conic` decides `curve_face_pose_belly_graze` on `r − |d0|` bare, with `d0 = plane_offset(origin, normal, k.frame().origin())` from the derived centre (`curve_pair::pose::height`); with the plane parallel, `r` ≈ 0 and `d0` is the centre's rounding along the normal, about 1e-16·a, read as a definite gap. Callers that act on it: `topo::boolean::slit` maps everything but `InPlane`/`PlaneParallel` to `Placed::Off`; `boolean::reduce` and `factored_crossing` read `Miss` as no crossing.

**B. `Curve3::on_carrier` answers `Off` inside the band's gap.** `q = e.point_at(θ) + axis·δ`, `carrier.on_carrier(q, θ, band)`: at δ = 0 and 0.5 ε it answers `On` and at 30 ε `Off`, both right at every size. At δ = 5 ε, inside the gap where neither verdict is decidable:

| ε | answer by semi-axis |
|---|---|
| 1e-9 | 1e4 to 1e7 refused (right); 1e8, 1e9 **`Off`** |
| 1e-12 | 1e4 refused; 1e5 to 1e9 **`Off`** |

`OnCarrier::Off` is "definitely past the band from the whole carrier". Cause, by the lane's reading: `Curve3::clearance`'s ellipse arm (`geom/src/curves/on_carrier.rs`) reads `e.frame().in_plane(q)` and `e.frame().height(q)` about the derived centre, and `Foot::on_carrier` decides `curve_carrier_clear` on it bare, so the lower bound on the distance carries the centre's rounding as if it were distance.

```rust
use geom::{Conic, Curve3, Ellipse};
use geom_brep::curve_face::plane_crossing::plane_crossing_lane;
use geom_core::{Band, Point3, Tol, Vec3};
#[test]
fn the_centre_readers_decide_on_the_centres_rounding() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (eps, a) = (band.zero(), 1e7); // a = 1e9 at eps 1e-9
    let e = Ellipse::anchored(
        Point3::new(1.3, -0.7, 0.4),
        Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528),
        Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582),
        a, 0.3 * a);
    let theta = 0.3 / (0.3 * a);
    let (p, n) = (e.point_at(theta), e.frame().axis());
    let carrier = Curve3::Ellipse(e);
    let half = 10.0 / (0.3 * a);
    // A: prints Conic(Miss); the plane holds the carrier's own stored point
    println!("{:?}", plane_crossing_lane(&carrier, theta - half, theta + half, e.point_at(0.0), n, band));
    // B: Ok(Off) for a point 5 eps from the arc, inside the gap
    assert!(!matches!(carrier.on_carrier(p + n * (5.0 * eps), theta, band), Ok(geom::OnCarrier::Off)));
}
```

Not traced: which built results these two reach. Through `split` a far-section body builds only to a semi-axis of about 6e5 m at 1e-9, below where either goes wrong.

## Six more, from running the sweep's suspects (2026-11-23; clean main `dc263f855`, public API, f64)

The suspects lane ran the readers the sweep had marked by reading. All seven reproductions (these six and the first) are one file, `analysis/oneround/conic-suspects/src/suspects_repro.rs` on `oneround/conic-suspects-probe` (`236dcf122`), run on clean main as its own test target: 6 fail at ε 1e-9 and 7 at 1e-12. Each is judged against the curve the stored numbers state at 60 digits; the smallest semi-axis at which each answers wrongly is given as ε 1e-9 / ε 1e-12. The reach measured above stands: no body is built at these sizes.

1. **`meet`, a line that pierces the ellipse's plane through a point of the arc: no meeting** (1e8 / 1e5; 17 of 24 poses at 1e9). The decision log ends on `curve_pair_pose_clear` in 94 of 100 wrong answers: `Polish::closest`'s `decide(row::CLEAR, Margin::of(self.clear − self.reach))`, `clear` being `Curve3::clearance` of the seed about the derived centre and `reach` carrying `height` from it; the other 6 on `curve_carrier_piece_clear_by`. One-sided: the same lines 30 ε off the arc never read a meeting.
2. **`meet`, a line along the major axis out through the anchor vertex: no meeting** (1e9 / 1e5). `curve_pair_pose_gap` (`route`'s GAP row: the conic's extent on the line, its constant from the derived centre, no charge) in 4, the router's `curve_pair_pose_parallel` first in the other 28.
3. **`meet`, a unit circle in the ellipse's plane crossing the arc: no meeting** (1e9 / 1e6; 18 of 18). Every one ends on `conic_conic`'s `constant_as(row::PARALLEL, …)` → `Pose::Apart`. Stating each plane through a point of the arc instead leaves 16 of 18 wrong: the ellipse's own constant, read from its centre against the circle's plane, certifies the same.
4. **`meet`, a small centre-held ellipse in the plane crossing the arc: no meeting** (1e8 / 1e5). `conic_conic`'s constant in 98 of 108; **`curve_pair_coplanar_sub_clear` in 4** (`coplanar_ellipses`' subdivision, its residual about the other's centre with no noise); `Curve3::clearance`'s deciders in 6.
5. **`meet`, two arc-held ellipses through one point: no meeting** (1e8 / 1e5), in one plane and across planes. In one plane `conic_conic`'s constant, a derived centre on both sides; across planes `curve_pair_window_off` (22) and **`curve_pair_pose_root_gap` (14), a decision in neither the sweep's suspects nor its untraced eight, not read.**
6. **`OnSurface::under` (`curve_pair/on_surface.rs`) answers wrongly in BOTH directions.** `Roots([])` for a line that lies in the edge's plane (1e9 / 1e6); **`InPlane` for a line 30 ε above it** (at 1e-12 from 1e7: the centre's rounding along the normal cancels the lift); a definite `Roots([])` for a line 5 ε above, in the gap; and for a line piercing at 30° a root 1.2e-7 m from the true crossing at 1e9 (past K ε in 3 of 6 at 1e8). One caller, `topo/src/region.rs`, not read.

The first reproduction's third cause is corrected by the same lane (`task1.md`, `b35f1f349`): the circle-across miss is not `Window::height_jet` alone; four readers certify it in layers. The window handed to `holds_only` is displaced by the centre's rounding along the normal (each plane stated through the derived centre); with the planes stated through a point of the arc, `height_jet` reads 5.96e-8 m at a point 1.7e-16 m from the plane and `off_by`'s clearance 4.48e-7 m; with those re-posed too the miss is still certified, on `curve_pair_window_recedes` then `curve_pair_window_off`, or on `curve_pair_pose_clear`: 48 of 288 truly-meeting poses still wrong at ε 1e-9 and 170 at 1e-12 with all three re-posed. What is left is the level `F = X²/A² + Y²/B² − 1` (`level_on`, `Level::jet_at`), which loses about u·a metres by its own form whatever its centre, so it is owed a re-posing from the stored point, not a constant.

Over all the lane's runs: nothing answers wrongly, and nothing answers definitely in the gap, below a semi-axis of 1e8 at ε 1e-9 (1,908 runs) or 1e5 at 1e-12 (954 runs); `meet` is wrong in one direction only (663 "no meeting" answers for curves that meet; no meeting reported for curves 30 ε or more apart in 800 runs per ε); `under` is wrong in both. The apart twins are the rows that guard a fix against buying meetings by loosening.

## The unit

`oneround/conic-pair-readers`, stacked on `oneround/conic-readers` (PR 7510). **Letter H; tier single, a FULL review**: the fix is in the certified lane of `curve_pair`, where a reading moved to another datum changes what each margin is measured against, and believing it takes more than reading it.

Every reading of where a conic is that `curve_pair` makes (`pose.rs`, `pose/window.rs`, `walk.rs`, `on_surface.rs`; the gate's list for those files) is read from the point the conic stores, through the conic: a point's height over the conic's plane; a line's or a plane's height along the conic as a first harmonic about the stored point; a foreign point's parameter and level. Nothing outside `conic.rs` learns the placement. Where a reading stays on the derived centre, its verdict is metered for the centre's rounding, so it refuses and cannot certify. The direction is the one three designers signed for the whole family (`analysis/design-fork/conic-reader-interface`): the readings this unit adds to `Conic` are what that design's position would later answer, and no more.

**All of `curve_pair/` was in scope from the start; the six above are in it and each reproduction becomes a row.** **Extended on the 23rd to the two readers above** (`curve_face::pose::plane_conic`'s belly graze, which reads `curve_pair::pose::height`; `Curve3::clearance`'s ellipse arm in `geom/src/curves/on_carrier.rs`) **and to any further reader the lane's sweep table marks as answering wrongly**: each re-posed from the stored point through the conic, or metered so that it refuses.

Acceptance: the reproductions above as rows, and the lane's ladder (`a` from 1e4 to 1e9, fat and thin, line and circle doors, near the anchor and far from it) at 1e-9 and 1e-12 against the 60-digit oracle: no pose certified wrongly, and every refusal that remains named and metered. A differential against the base: every verdict that moves, judged against the oracle.

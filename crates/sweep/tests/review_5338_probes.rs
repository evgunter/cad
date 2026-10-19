//! Review probes for PR 5338 (a cup on a support's strip goes with the
//! band). Each row states what the head does and what a sound carve
//! must do.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Vec3};
use sweep::test_support::{prism_on, sketch_from_axes};
use topo::Body;

use crate::band_concave_bar::{blend, boolean};
use crate::band_planar_cut_off::{Verb, edge, tol, volume};

const S3: f64 = 1.732_050_807_568_877_2;

/// The equilateral wedge: section `(0, 0), (−3, 0), (−1.5, −1.5·√3)`
/// in `(y, z)`, run `len` along `x`. Its apex edge `(0, 0, 0) → (len, 0, 0)`
/// is convex with a 60° dihedral between the top `z = 0` and the
/// slant `z = √3·y`.
fn wedge(len: f64) -> Body<f64> {
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let verts = [(0.0, 0.0), (-3.0, 0.0), (-1.5, -1.5 * S3)];
    prism_on(
        plane,
        verts
            .iter()
            .map(|&(y, z)| (Point2::new(y, z), 0.0))
            .collect(),
        len,
        tol(),
    )
}

/// A square post `[x0, x1] × [y0, y1] × [z0, z1]`.
fn post(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, tol())
}

/// **The ν-convexity of `S⁺` fails at an acute dihedral.** A fillet of
/// radius 1 on the 60° wedge's apex: the ball's centre is `(−√3, −1)`,
/// its arc runs from the top's foot `(−√3, 0)` round to the slant's foot
/// `(−√3/2, −3/2)`, below the centre's level. A line along the top's
/// normal at `y ≈ −0.764` meets the band's section twice: under the top
/// down to the arc (`z ≈ −0.75`), and again in the sliver between the
/// arc's underside (`z ≈ −1.25`) and the slant (`z ≈ −1.32`). A square
/// dimple from the top's strip whose floor sits in that lower sliver
/// has every boundary point inside `M` (its mouth in `M'`, its floor
/// in the core), but its walls cross the disk the fillet keeps: the
/// dimple is NOT wholly inside `R`, and `S − R` keeps the part of it
/// in the disk, a pocket through the fillet's face. A sound carve
/// refuses it as a crossing.
#[test]
fn a_dimple_whose_walls_cross_the_disk_at_an_acute_dihedral_must_refuse() {
    let len = 1.0;
    let (y0, y1) = (-0.775, -0.753);
    let (x0, x1) = (0.5 - 0.011, 0.5 + 0.011);
    for zf in [-0.2, -1.3] {
        let body = boolean(false, wedge(len), post((x0, x1), (y0, y1), (zf, 0.2)));
        let apex = edge(&body, [0.0, 0.0, 0.0], [len, 0.0, 0.0]);
        let plain = wedge(len);
        let plain_apex = edge(&plain, [0.0, 0.0, 0.0], [len, 0.0, 0.0]);
        let filleted = blend(Verb::Fillet, &plain, &[plain_apex], 1.0).expect("the plain wedge");
        let v_plain = volume(&filleted.body);
        let got = blend(Verb::Fillet, &body, &[apex], 1.0);
        // The true `S − R`: the filleted wedge less the post.
        let truth = boolean(
            false,
            filleted.body.clone(),
            post((x0, x1), (y0, y1), (zf, 0.2)),
        );
        let v_true = volume(&truth);
        eprintln!(
            "floor z = {zf}: blend = {:?}; V(fillet(wedge)) = {v_plain:.9}, \
             V(S − R) = {v_true:.9}",
            got.as_ref()
                .map(|b| volume(&b.body))
                .map_err(|e| format!("{e}"))
        );
        match got {
            Ok(out) => {
                let v = volume(&out.body);
                assert!(
                    (v - v_true).abs() < 1e-9,
                    "floor z = {zf}: the carve built V = {v:.9}, but S − R is \
                     V = {v_true:.9} (Δ = {:.3e}): the dimple's walls cross the disk",
                    v - v_true
                );
            }
            Err(e) => assert!(
                zf < -1.0,
                "floor z = {zf}: the shallow dimple lies in R and should go: {e}"
            ),
        }
    }
}

/// **The same, with the crossing inside a face, off every edge.** A
/// round dimple of radius `0.025` centred at `y = −0.725`: its seams
/// (at `y = −0.725`) stand clear of the ball (`|y + √3| > 1`), its floor
/// circle at `z = −1.2` lies in the lower sliver, its mouth on the top's
/// strip; but its wall near `y = −0.75` passes through the ball for
/// `z ∈ [−1.19, −0.81]`. Every boundary sample lies in `M`, and the
/// wall's rulings run along `ν`, so the `ν` stray charges nothing. The
/// point `(0.5, −0.745, −1)` is air in `S` (inside the dimple) and lies
/// in the ball (`|(−0.745 + √3, 0)| = 0.987 < 1`), so not in `R`: it is
/// air in `S − R`. A carve that builds the plain filleted wedge has
/// material there.
#[test]
fn a_round_dimple_whose_wall_crosses_the_disk_between_its_seams_must_refuse() {
    let len = 1.0;
    let (xc, yc, a) = (0.5, -0.725, 0.025);
    let w = (-0.745f64 + S3).hypot(0.0);
    assert!(
        w < 1.0 && (-0.745f64 - yc).abs() < a,
        "the witness is in the ball and the dimple"
    );
    let plain = wedge(len);
    let plain_apex = edge(&plain, [0.0, 0.0, 0.0], [len, 0.0, 0.0]);
    let v_plain = volume(
        &blend(Verb::Fillet, &plain, &[plain_apex], 1.0)
            .expect("the plain wedge")
            .body,
    );
    for zf in [-0.2, -1.2] {
        let cup = crate::band_strip_mouth_cups::post::<f64>(
            Point3::new(xc, yc, 0.2),
            Vec3::new(0.0, 0.0, -1.0),
            0.2 - zf,
            a,
        );
        let body = boolean(false, wedge(len), cup);
        let apex = edge(&body, [0.0, 0.0, 0.0], [len, 0.0, 0.0]);
        let got = blend(Verb::Fillet, &body, &[apex], 1.0);
        eprintln!(
            "round, floor z = {zf}: blend = {:?}; V(fillet(plain wedge)) = {v_plain:.12}",
            got.as_ref()
                .map(|b| volume(&b.body))
                .map_err(|e| format!("{e}")),
        );
        if zf < -1.0 {
            assert!(
                got.is_err(),
                "round, floor z = {zf}: the carve built V = {:.12} (the plain fillet's \
                 {v_plain:.12}), but the dimple's wall crosses the ball the fillet keeps, \
                 so S − R has a pocket there",
                volume(&got.unwrap().body)
            );
        } else {
            got.expect("the shallow round dimple lies in R and goes");
        }
    }
}

/// A 60° groove pocket with end walls: a brick
/// `[−1, len + 1] × [−5, 2] × [−2, 2.5]` less the prism over
/// `(0, 0), (−√3, 3), (−4, 3), (−4, 0)` run `x ∈ [0, len]`, open at the
/// top. Its crease `(0, 0, 0) → (len, 0, 0)` is concave, its void
/// opening 60° between the floor `z = 0` and the wall `z = −√3·y`.
fn groove(len: f64) -> Body<f64> {
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let verts = [(0.0, 0.0), (-S3, 3.0), (-4.0, 3.0), (-4.0, 0.0)];
    let void = prism_on(
        plane,
        verts
            .iter()
            .map(|&(y, z)| (Point2::new(y, z), 0.0))
            .collect(),
        len,
        tol(),
    );
    boolean(
        false,
        post((-1.0, len + 1.0), (-5.0, 2.0), (-2.0, 2.5)),
        void,
    )
}

/// **The concave mirror**: a round boss from the groove's floor up into
/// the void, through the ball the fill leaves void (centre `(−√3, 1)`),
/// its end in the fill's upper sliver by the wall. `S ∪ F` keeps the
/// boss's part in the ball, e.g. `(0.5, −0.745, 1)`; a carve that builds
/// the plain filled groove does not.
#[test]
fn a_round_boss_whose_wall_crosses_the_void_disk_at_an_acute_groove_must_refuse() {
    let len = 1.0;
    let (xc, yc, a) = (0.5, -0.725, 0.025);
    let plain = groove(len);
    let plain_crease = edge(&plain, [0.0, 0.0, 0.0], [len, 0.0, 0.0]);
    let v_plain = volume(
        &blend(Verb::Fillet, &plain, &[plain_crease], 1.0)
            .expect("the plain groove")
            .body,
    );
    for zt in [0.2, 1.2] {
        let cup = crate::band_strip_mouth_cups::post::<f64>(
            Point3::new(xc, yc, -0.2),
            Vec3::new(0.0, 0.0, 1.0),
            0.2 + zt,
            a,
        );
        let body = boolean(true, groove(len), cup);
        let crease = edge(&body, [0.0, 0.0, 0.0], [len, 0.0, 0.0]);
        let got = blend(Verb::Fillet, &body, &[crease], 1.0);
        eprintln!(
            "boss, top z = {zt}: blend = {:?}; V(fillet(plain groove)) = {v_plain:.12}",
            got.as_ref()
                .map(|b| volume(&b.body))
                .map_err(|e| format!("{e}")),
        );
        if zt > 1.0 {
            assert!(
                got.is_err(),
                "boss, top z = {zt}: the carve built V = {:.12} (the plain fill's \
                 {v_plain:.12}), but the boss crosses the ball the fill leaves void",
                volume(&got.unwrap().body)
            );
        } else {
            got.expect("the short boss lies in F and goes");
        }
    }
}

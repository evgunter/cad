//! **An aimed ray beside a cylinder's rim that moves with a box is set
//! aside at the rim, and reads no side a member lacks** (`topo`'s side
//! reader, `side_reader::aimed_ray`; the curved edge's reading,
//! `boolean::solid_contain::curved_edges`).
//!
//! A cylinder about `z` built through the public Euler doors at the
//! symbolic box scalar: two caps and one wall on one seam, its height or
//! its radius a box, so its top rim (a circle, the edge between the cap
//! and the wall) moves with the box. Every member is a solid whose side
//! at infinity is `Out`, and an aimed ray starts beyond every face
//! whatever it aims at, so a reading of `Side(In)` is a wrong side.
//!
//! No sweep door builds a curved body over a box today
//! (`sweep`'s `an_aimed_ray_over_boxed_door_bodies`), so the body is
//! built by hand.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Interval, ParamSymbol, Point3, Real, Sym, SymBudget, Tol, Vec3};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{AtRestPolicy, Body, FaceKey};

type Boxed = Sym<Interval>;

/// The cylinder of `radius` about `z` over `[0, top]`, and its wall:
/// the wall seeded at `(radius, 0, 0)`, the bottom rim closed there, one
/// seam up to `(radius, 0, top)`, and the top rim closed at its end.
/// Each rim runs against its cap's outward normal's turn, as the loop of
/// a face the edge's minus half bounds does.
fn cylinder<T: Decide + AtRestPolicy>(radius: T, top: T, tol: Tol) -> (Body<T>, FaceKey) {
    let (zero, one) = (T::zero(), T::one());
    let (x, z) = (Vec3::new(one, zero, zero), Vec3::new(zero, zero, one));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(Point3::new(radius, zero, zero), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Cylinder {
                origin: Point3::origin(),
                axis: z,
                radius,
                u_ref: x,
            },
            sense: true,
        },
    )
    .unwrap();
    let rim = |at: T, axis: Vec3<T>| {
        let circle = Curve3::circle(Point3::new(zero, zero, at), axis, radius, x);
        EdgeCurveSpec::arc_of_circle(circle, T::zero(), T::tau()).unwrap()
    };
    let cap = |at: T, normal: Vec3<T>| FaceSurface::New {
        surface: Surface::Plane {
            origin: Point3::new(zero, zero, at),
            normal,
            u_ref: x,
        },
        sense: true,
    };
    let bottom = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            rim(zero, z),
            cap(zero, z * -one),
            tol,
        )
        .unwrap();
    let seam = body
        .mev_line(
            MevSite::Fan {
                he1: bottom.he_plus,
                he2: bottom.he_plus,
            },
            Point3::new(radius, zero, top),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: seam.he_minus,
            he2: seam.he_minus,
        },
        rim(top, z * -one),
        cap(top, z),
        tol,
    )
    .unwrap();
    (body, seed.face)
}

/// Runs `read` under a symbolic session on the cylinder's height and
/// radius, each over `(lo, hi)`, a point where the two are equal.
fn over<R>(
    top: (f64, f64),
    radius: (f64, f64),
    read: impl FnOnce(Boxed, Boxed, Band, Tol) -> R,
) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let axis = |name: &str, (lo, hi): (f64, f64)| {
        if lo == hi {
            Boxed::from_f64(lo)
        } else {
            Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id(name)),
                Interval::from_bounds(lo, hi),
                lo,
                hi,
            )
        }
    };
    geom_core::sym::with_session(budget, || {
        read(
            axis("boxed_cylinder_top", top),
            axis("boxed_cylinder_radius", radius),
            band,
            tol,
        )
    })
    .0
}

/// One aimed ray at `aim` about `normal`, over the box.
fn boxed_ray(top: (f64, f64), radius: (f64, f64), aim: [f64; 3], normal: [f64; 3]) -> String {
    over(top, radius, |t, r, band, tol| {
        let (body, wall) = cylinder::<Boxed>(r, t, tol);
        let iv = Boxed::from_f64;
        topo::test_support::ray_at(
            &body,
            wall,
            Point3::new(iv(aim[0]), iv(aim[1]), iv(aim[2])),
            Vec3::new(iv(normal[0]), iv(normal[1]), iv(normal[2])),
            band,
        )
    })
}

/// The same ray on the member of height `top` and radius `radius`.
fn member_ray(top: f64, radius: f64, aim: [f64; 3], normal: [f64; 3]) -> String {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let (body, wall) = cylinder::<f64>(radius, top, tol);
    topo::test_support::ray_at(
        &body,
        wall,
        Point3::new(aim[0], aim[1], aim[2]),
        Vec3::new(normal[0], normal[1], normal[2]),
        band,
    )
}

/// **A fixed ray beside a rim that moves with the box is set aside on
/// the rim's own reading.** The cylinder's height lies anywhere in
/// `[0.9, 1.1]`, or its radius does. Each ray is aimed down at a fixed
/// point where the middle member's top cap is, a hundredth to six
/// hundredths inside its rim, at four places round it clear of the seam.
/// Every member reads `Out` on it (five are read at `f64`, as the
/// witness): the ray enters the cap or, where the box has carried the
/// rim past it, the wall. Over the box the ray's distance from the rim
/// is an enclosure from zero up to past the band, which no reading
/// places: the ray is set aside there, on the carrier's gap
/// (`curve_carrier_gap`), and reads no side.
///
/// **Red on each of the four rays under either of two mutants.** With
/// the curved arm alone reading a carrier's escalation as the edge met
/// (`curved_edges::passes` returning `Passes::Met` where it returns
/// `Passes::Unread`), the rim stands in for its faces' crossings and
/// the ray is set aside further on, where the wall's root is read
/// against the passage (`side_at_infinity_met_edge_reaches`); none
/// reads a wrong side. With `Gap::escalated` passing every escalation
/// as in band (`solid_contain::edge_gap`, the door every arm reads
/// through), each ray reads `Side(Out)`: the side every member has,
/// read through a rim no reading placed.
#[test]
fn a_ray_beside_a_moving_rim_is_set_aside_at_the_rim() {
    let fixed = (1.0, 1.0);
    let moving = (0.9, 1.1);
    let mut elsewhere = Vec::new();
    for (what, top, radius, insets) in [
        ("a boxed height", moving, fixed, [0.01, 0.01]),
        ("a boxed radius", fixed, moving, [0.01, 0.06]),
    ] {
        for (a, inset) in [(1.9_f64, insets[0]), (3.1, insets[1])] {
            let aim = [(1.0 - inset) * a.cos(), (1.0 - inset) * a.sin(), 1.0];
            let normal = [0.0, 0.0, 1.0];
            let at = format!("{what}, aimed {inset} inside the rim at angle {a}");
            for u in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let (t, r) = (
                    top.0 + u * (top.1 - top.0),
                    radius.0 + u * (radius.1 - radius.0),
                );
                assert_eq!(
                    member_ray(t, r, aim, normal),
                    "Ok(Side(Out))",
                    "{at}: the member of height {t} and radius {r}"
                );
            }
            let boxed = boxed_ray(top, radius, aim, normal);
            assert!(
                !boxed.contains("Side(In)"),
                "{at}: the side no member has: {boxed}"
            );
            if !(boxed.starts_with("Ok(SetAside(Crossing(") && boxed.contains("curve_carrier_gap"))
            {
                elsewhere.push(format!("{at}: {boxed}"));
            }
        }
    }
    assert!(
        elsewhere.is_empty(),
        "not set aside on the rim's carrier reading: {elsewhere:#?}"
    );
}

/// **Rays aimed about a boxed cylinder's moving rim, from any normal,
/// read no side no member has** (counterexample search). Boxes on the
/// height, the radius or both, from a few bands wide to nine tenths of
/// the dimension; each aim lies on or beside the top rim of a member of
/// the box, on it or off it at four scales, and half the normals are
/// the cap's, the wall's and their negatives.
///
/// **What this can go red on:** a reading of the side no member has, and
/// nothing else. A reading lost is not a wrong one, so a change that
/// only sets more rays aside leaves it green; and it stays green with a
/// carrier's escalation read as the edge met
/// (`Gap::escalated` passing every escalation), where
/// [`a_ray_beside_a_moving_rim_is_set_aside_at_the_rim`] is red: over a
/// box a met rim stands for a crossing only where the wall's root is
/// read near the passage or reached from it in band, and that reading
/// refuses on an enclosure past the band. So it is evidence that these
/// draws find no wrong side, not that the rim's reading is sound.
#[test]
fn no_ray_about_a_boxed_cylinder_reads_the_side_no_member_has() {
    let mut rng = test_utils::fuzz::start("boxed_cylinder_rays");
    let replay = test_utils::fuzz::replay();
    let eps = Tol::witness().eps();
    let boxes = test_utils::fuzz::scaled(6);
    let rays = test_utils::fuzz::scaled(24);
    let mut tally = std::collections::BTreeMap::<&str, usize>::new();
    for k in 0..boxes {
        let mid_h = rng.log_uniform(0.05, 3.0);
        let mid_r = rng.log_uniform(0.05, 3.0);
        let half = |rng: &mut test_utils::fuzz::Rng, mid: f64| rng.log_uniform(20.0 * eps, 0.9) * mid;
        let half_h = if k % 3 == 1 { 0.0 } else { half(&mut rng, mid_h) };
        let half_r = if k % 3 == 0 { 0.0 } else { half(&mut rng, mid_r) };
        let top = (mid_h - half_h, mid_h + half_h);
        let radius = (mid_r - half_r, mid_r + half_r);
        let draws: Vec<([f64; 3], [f64; 3])> = (0..rays)
            .map(|_| {
                let pick = |rng: &mut test_utils::fuzz::Rng, (lo, hi): (f64, f64)| match rng.below(4)
                {
                    0 => lo,
                    1 => hi,
                    2 => 0.5 * (lo + hi),
                    _ => rng.range(lo, hi),
                };
                let (h, r) = (pick(&mut rng, top), pick(&mut rng, radius));
                let a = rng.range(0.0, core::f64::consts::TAU);
                let off = match rng.below(5) {
                    0 => 0.0,
                    1 => rng.range(-1e-2, 1e-2),
                    2 => rng.range(-0.2, 0.2) * r,
                    3 => rng.range(-1e-6, 1e-6),
                    _ => rng.range(-0.05, 0.05),
                };
                let dz = match rng.below(4) {
                    0 => rng.range(-0.3, 0.3) * h,
                    1 => rng.range(-1e-2, 1e-2),
                    _ => 0.0,
                };
                let aim = [(r + off) * a.cos(), (r + off) * a.sin(), h + dz];
                let normal = match rng.below(8) {
                    0 => [0.0, 0.0, 1.0],
                    1 => [a.cos(), a.sin(), 0.0],
                    2 => [0.0, 0.0, -1.0],
                    3 => [-a.cos(), -a.sin(), 0.0],
                    _ => {
                        let v = [
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                        ];
                        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                        [v[0] / n, v[1] / n, v[2] / n]
                    }
                };
                (aim, normal)
            })
            .collect();
        let readings = over(top, radius, |t, r, band, tol| {
            let (body, wall) = cylinder::<Boxed>(r, t, tol);
            let iv = Boxed::from_f64;
            draws
                .iter()
                .map(|&(a, n)| {
                    topo::test_support::ray_at(
                        &body,
                        wall,
                        Point3::new(iv(a[0]), iv(a[1]), iv(a[2])),
                        Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])),
                        band,
                    )
                })
                .collect::<Vec<_>>()
        });
        for (r, (aim, normal)) in readings.iter().zip(&draws) {
            assert!(
                !r.contains("Side(In)"),
                "top {top:?}, radius {radius:?}, ε {eps:e}, aim {aim:?} normal {normal:?}: a \
                 wrong side: {r} ({replay})"
            );
            let kind = if r.contains("Side(Out)") {
                "out"
            } else if r.contains("curve_carrier_gap") {
                "aside at the rim"
            } else {
                "aside elsewhere"
            };
            *tally.entry(kind).or_default() += 1;
        }
    }
    println!("ε {eps:e} boxed cylinder rays: {tally:?} ({replay})");
}

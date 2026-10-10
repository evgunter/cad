//! **Review probes for the rim-free spherical wedge** (issue 542), now
//! measured by the sphere's Gauss–Bonnet arm: the lune is attacked from
//! the inputs it is allowed to read and from the ones it must not, the
//! band ladder around a hairline wedge is walked, and the coincident
//! pair is pinned as the cusp it is.
//!
//! Every row states its own closed form. The sphere is `R = 10 mm`
//! about `+Z` at the origin unless the row builds its own.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

use crate::shared::surf;
use crate::shared::tol::{band, eps};
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face};
use geom_core::{Point3, Vec3};

const RS: f64 = 0.010;

fn sphere() -> Surface<f64> {
    surf::sphere(RS)
}

fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_great(RS, u, t0, t1, a, b)
}

/// The wedge between azimuth `0` and `theta`, the unit's own loop.
fn wedge(theta: f64) -> Vec<LoopEdge<f64>> {
    vec![
        great(0.0, FRAC_PI_2, -FRAC_PI_2, 0, 1),
        great(theta, -FRAC_PI_2, FRAC_PI_2, 1, 0),
    ]
}

/// The azimuthal width the arm answered, read back from the area
/// (`area = 2R²·Δu` for a pole-to-pole lune).
fn du_of(edges: &[LoopEdge<f64>], sense: bool) -> f64 {
    curved_face(&sphere(), edges, sense, band())
        .expect("measured")
        .area
        / (2.0 * RS * RS)
}

/// **The loop is a cycle, and the arm must read it as one.** The arm
/// reads its half-plane frame off `edges[0]` alone and argues that the
/// read at the other arc is its negation. Listing the same cycle from
/// the other meridian is therefore a no-op, and this row is what goes
/// red if the "one arc's read is the loop's" claim is false.
#[test]
fn the_width_does_not_depend_on_which_meridian_the_loop_is_listed_from() {
    for theta in [PI / 6.0, FRAC_PI_2, 2.0, 1.5 * PI, 1.75 * PI] {
        for sense in [true, false] {
            let fwd = wedge(theta);
            let rot = vec![fwd[1].clone(), fwd[0].clone()];
            let (a, b) = (du_of(&fwd, sense), du_of(&rot, sense));
            assert!(
                (a - b).abs() < 1e-14,
                "theta = {theta}, sense = {sense}: Δu {a} from edge 0, {b} from edge 1"
            );
        }
    }
}

/// **No value may ride in that is not the sense bit, the forward bit,
/// the carrier axis and the stored endpoints.** The second meridian is
/// re-authored on the SAME half-plane with its carrier axis negated
/// and its parameter running the other way — so its stored `forward`
/// bit flips, its carrier axis flips, and its `t0` endpoint moves to
/// the other pole, while the face it bounds is the same face. The
/// answer must not move.
#[test]
fn the_width_does_not_depend_on_the_stored_direction_of_the_second_meridian() {
    for theta in [PI / 6.0, FRAC_PI_2, 2.0, 1.5 * PI] {
        for sense in [true, false] {
            let plain = wedge(theta);
            // The u = theta meridian, carrier reversed: axis negated,
            // u_ref kept, so its parameter is MINUS the latitude and
            // the south-to-north traversal is stored backwards.
            let flipped = Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::new(-theta.sin(), theta.cos(), 0.0),
                radius: RS,
                u_ref: Vec3::new(theta.cos(), theta.sin(), 0.0),
            };
            let recut = vec![
                plain[0].clone(),
                topo::edge(flipped, FRAC_PI_2, -FRAC_PI_2, 1, 0),
            ];
            assert_ne!(recut[1].forward, plain[1].forward, "the bit must differ");
            let (a, b) = (du_of(&plain, sense), du_of(&recut, sense));
            assert!(
                (a - b).abs() < 1e-14,
                "theta = {theta}, sense = {sense}: Δu {a} vs {b} for the same face"
            );
        }
    }
}

/// **Past π in both senses.** A wedge whose SHORT azimuthal arc is on
/// the wrong side is the case the rule exists for; `3π/2` is the
/// unit's, `7π/4` and `2π − 0.5` are not.
#[test]
fn wedges_past_pi_measure_their_own_arc() {
    for theta in [1.5 * PI, 1.75 * PI, TAU - 0.5] {
        for sense in [true, false] {
            let want = if sense { theta } else { TAU - theta };
            let got = du_of(&wedge(theta), sense);
            assert!(
                (got - want).abs() / want < 1e-12,
                "theta = {theta}, sense = {sense}: Δu {got} != {want}"
            );
        }
    }
}

/// **The two lunes of a split ball are complementary and exhaust it.**
/// The same two meridian planes bound two faces — the loop and its
/// reversal — and the two widths must sum to `2π` and the two band
/// faces' fluxes to the whole ball's volume, `(4/3)πR³`.
#[test]
fn the_two_lunes_of_a_split_ball_sum_to_the_whole_ball() {
    for theta in [PI / 6.0, FRAC_PI_2, 2.0, PI, 1.5 * PI] {
        let fwd = wedge(theta);
        let rev = vec![
            great(theta, FRAC_PI_2, -FRAC_PI_2, 0, 1),
            great(0.0, -FRAC_PI_2, FRAC_PI_2, 1, 0),
        ];
        let (a, b) = (du_of(&fwd, true), du_of(&rev, true));
        assert!(
            (a + b - TAU).abs() < 1e-12,
            "theta = {theta}: {a} + {b} != 2π"
        );
        let v: f64 = [&fwd, &rev]
            .into_iter()
            .map(|e| curved_face(&sphere(), e, true, band()).unwrap().flux / 3.0)
            .sum();
        let exact = 4.0 / 3.0 * PI * RS.powi(3);
        assert!(
            (v - exact).abs() / exact < 1e-12,
            "theta = {theta}: the two lunes measure {v}, the ball is {exact}"
        );
    }
}

/// The meridian great circle of an arbitrary sphere: the arc in the
/// half-plane at azimuth `u` of the frame `(axis, u_ref)`, parameter
/// the latitude, exactly as [`crate::shared::topo::sphere_great`]
/// builds it for the polar one.
#[allow(clippy::too_many_arguments)] // a fixture's frame, spelled out: centre, radius, axis, u_ref, azimuth, span, tags
fn great_on(
    center: Point3<f64>,
    radius: f64,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    u: f64,
    t0: f64,
    t1: f64,
    a: u32,
    b: u32,
) -> LoopEdge<f64> {
    let e2 = axis.cross(u_ref);
    let h = u_ref * u.cos() + e2 * u.sin();
    topo::edge(
        Curve3::Circle {
            center,
            axis: h.cross(axis),
            radius,
            u_ref: h,
        },
        t0,
        t1,
        a,
        b,
    )
}

/// **A ball of another radius, off the origin, about a tilted axis.**
/// The lever in `Margin::levered(φ, R)` and the `1/R` that normalises
/// the half-plane frame are the only places the radius enters; the
/// width is a pure angle and must not move with either.
#[test]
fn an_off_origin_tilted_ball_of_another_radius_reads_the_same_width() {
    let r = 0.37;
    let c = Point3::new(-1.25, 4.0, 0.5);
    let axis = Vec3::new(1.0, 1.0, 1.0) * (1.0 / 3.0_f64.sqrt());
    let u_ref = Vec3::new(1.0, -1.0, 0.0) * (1.0 / 2.0_f64.sqrt());
    let s: Surface<f64> = Surface::Sphere {
        center: c,
        radius: r,
        axis,
        u_ref,
    };
    for theta in [PI / 6.0, FRAC_PI_2, 2.0, 1.5 * PI] {
        for sense in [true, false] {
            let edges = vec![
                great_on(c, r, axis, u_ref, 0.0, FRAC_PI_2, -FRAC_PI_2, 0, 1),
                great_on(c, r, axis, u_ref, theta, -FRAC_PI_2, FRAC_PI_2, 1, 0),
            ];
            let want = if sense { theta } else { TAU - theta };
            let got = curved_face(&s, &edges, sense, band())
                .expect("measured")
                .area
                / (2.0 * r * r);
            assert!(
                (got - want).abs() / want < 1e-12,
                "theta = {theta}, sense = {sense}: Δu {got} != {want}"
            );
        }
    }
}

/// **The band ladder across a hairline wedge.** Below the coincidence
/// threshold the two meridians' departures at each pole are antiparallel
/// within the band, a cusp refused typed (`props_sphere_loop_cusp`);
/// through the ladder's middle the cusp decide escalates; above the
/// escalate threshold the arm answers the wedge's own width. No rung
/// answers a degenerate face or another width.
#[test]
fn the_hairline_ladder_is_decided_by_the_cusp_alone() {
    let z = eps() / RS; // the azimuth whose lever puts it AT `zero`
    let mut seen = Vec::new();
    for f in [0.25_f64, 0.5, 1.0, 2.0, 5.0, 9.0, 10.0, 11.0, 20.0, 1e3] {
        let theta = f * z;
        let got = curved_face(&sphere(), &wedge(theta), true, band());
        let tag = match &got {
            Ok(fc) => {
                let du = fc.area / (2.0 * RS * RS);
                if (du - PI).abs() < 1e-12 {
                    "hemisphere"
                } else if (du - theta).abs() / theta < 1e-6 {
                    "own width"
                } else {
                    "other"
                }
            }
            Err(PropsError::Escalated { .. }) => "escalated",
            Err(PropsError::DegenerateFace) => "degenerate",
            Err(PropsError::SphereLoop {
                what: "props_sphere_loop_cusp",
            }) => "slit",
            Err(_) => "other refusal",
        };
        seen.push((f, tag));
        assert_ne!(
            tag, "degenerate",
            "f = {f}: a hairline wedge reached a degenerate-face refusal"
        );
        assert_ne!(tag, "other", "f = {f}: unexplained width");
    }
    // `f = 10` is the cusp decide's escalate threshold itself: the rung
    // reads `escalated` at some ε rows and `own width` at others, and
    // which way it falls is not the subject.
    let pinned: Vec<(f64, &str)> = seen.iter().copied().filter(|&(f, _)| f != 10.0).collect();
    assert_eq!(
        pinned,
        vec![
            (0.25, "slit"),
            (0.5, "slit"),
            (1.0, "slit"),
            (2.0, "escalated"),
            (5.0, "escalated"),
            (9.0, "escalated"),
            (11.0, "own width"),
            (20.0, "own width"),
            (1e3, "own width"),
        ],
        "the ladder is decided by props_sphere_loop_cusp alone"
    );
}

/// **The coincident pair, refused in both readings.** Two meridian
/// half-planes a hair apart turn back on themselves at each pole, so
/// both the hairline WEDGE and the hairline SLIT (a ball missing a
/// sliver) refuse as the cusp they are; nothing is measured.
#[test]
fn a_hairline_wedge_and_a_slit_refuse_as_cusps() {
    let delta = 0.5 * eps() / RS;
    for theta in [delta, TAU - delta] {
        assert_eq!(
            curved_face(&sphere(), &wedge(theta), true, band()).map(|_| ()),
            Err(PropsError::SphereLoop {
                what: "props_sphere_loop_cusp"
            }),
            "theta = {theta}"
        );
    }
}

/// **Two meridian arcs that stop short of both poles do not close.**
/// Tagged as a closed loop, they leave a gap at each end, and the arm
/// refuses them on its own closure decide (`props_loop_closed`)
/// rather than trusting the tags.
#[test]
fn meridian_arcs_that_never_reach_the_poles_refuse_as_an_open_loop() {
    let edges = vec![
        great(0.0, FRAC_PI_4, -FRAC_PI_4, 0, 1),
        great(FRAC_PI_2, -FRAC_PI_4, FRAC_PI_4, 1, 0),
    ];
    assert_eq!(
        curved_face(&sphere(), &edges, true, band()).map(|_| ()),
        Err(PropsError::SphereLoop {
            what: "props_loop_closed"
        })
    );
}

/// **A wedge whose meridians are SPLIT measures the wedge.** The same
/// lune with both meridians cut at the equator — four arcs on two great
/// circles — turns by zero at each joint and measures the quarter lune
/// `2R²·π/2`.
#[test]
fn a_wedge_whose_meridians_are_split_measures_the_lune() {
    let four = vec![
        great(0.0, FRAC_PI_2, 0.0, 0, 1),
        great(0.0, 0.0, -FRAC_PI_2, 1, 2),
        great(FRAC_PI_2, -FRAC_PI_2, 0.0, 2, 3),
        great(FRAC_PI_2, 0.0, FRAC_PI_2, 3, 0),
    ];
    let area = curved_face(&sphere(), &four, true, band())
        .expect("the quarter lune")
        .area;
    let exact = 2.0 * RS * RS * FRAC_PI_2;
    assert!((area - exact).abs() / exact < 1e-12, "{area} vs {exact}");
}

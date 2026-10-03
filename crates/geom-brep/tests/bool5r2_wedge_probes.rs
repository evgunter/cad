//! **Review probes for the rim-free spherical-wedge arm** (issue 542):
//! the structural rule is attacked from the inputs it is allowed to
//! read and from the ones it must not, the band ladder around the
//! coplanar gate is walked, and the coincident-pair blind spot the
//! review executed is pinned as the typed refusal the fix pass gave it
//! (`props_band_opposite`), with the wedge arm's pole-to-pole premise
//! decided rather than inherited. Adopted by the unit; the blind-spot
//! rows are re-aimed, the rest stand as written.
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

/// **The band ladder across the coplanar gate.** `props_band_coplanar`
/// decides `R·|sin φ|` and `props_wedge_azimuth` decides `R·φ` against
/// the SAME band, and `|φ| ≥ |sin φ|`, so the first decide is the only
/// one that ever moves: below the coincidence threshold the pair is
/// coplanar and `props_band_opposite` refuses the hairline wedge as the
/// SLIT it is, through the ladder's middle the coplanar decide
/// escalates, and above the escalate threshold the wedge arm answers
/// its own width. The `Zero` arm of `props_wedge_azimuth` — the arm's
/// declared `DegenerateFace` floor — is never reached from `sphere()`
/// (by the factor K, not by rounding), and this row is what goes red
/// if it ever is.
#[test]
fn the_wedge_arms_zero_floor_is_never_reached_from_the_sphere_branch() {
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
            Err(PropsError::NotIsoRectangle { what })
                if what.starts_with("a rimless sphere face whose coplanar meridians") =>
            {
                "slit"
            }
            Err(_) => "other refusal",
        };
        seen.push((f, tag));
        assert_ne!(
            tag, "degenerate",
            "f = {f}: props_wedge_azimuth reached its Zero floor from sphere()"
        );
        assert_ne!(tag, "other", "f = {f}: unexplained width");
    }
    // `f = 10` is `props_band_coplanar`'s escalate threshold itself,
    // where `R·sin θ` sits a rounding BELOW `R·θ`: the rung reads
    // `escalated` at some ε rows and `own width` at others, and which
    // way it falls is not the subject. Every other rung is pinned, and
    // the direction of the disagreement is: the coplanar decide is the
    // indefinite one, which is what keeps the wedge arm's `Zero` floor
    // out of reach.
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
        "the ladder is decided by props_band_coplanar alone"
    );
}

/// **The coincident pair, refused in both readings.** Two meridian
/// half-planes a hair apart are coplanar to `props_band_coplanar`,
/// which cannot tell them from OPPOSITE half-planes; what can is the
/// loop's traversal, which REVERSES at each pole here and continues on
/// the two-band face — `props_band_opposite`. Both the hairline WEDGE
/// and the hairline SLIT (a ball missing a sliver) refuse under that
/// name; nothing is measured at `Δu = π`.
#[test]
fn the_coplanar_arm_refuses_a_hairline_wedge_and_a_slit_typed() {
    let delta = 0.5 * eps() / RS;
    for theta in [delta, TAU - delta] {
        assert_eq!(
            curved_face(&sphere(), &wedge(theta), true, band()).map(|_| ()),
            Err(PropsError::NotIsoRectangle {
                what: "a rimless sphere face whose coplanar meridians share one half-plane — a \
                       slit the flux lane does not measure"
            }),
            "theta = {theta}"
        );
    }
}

/// **The wedge arm's own premise — that the two arcs run pole to pole
/// — is decided, not argued from loop closure.** Two non-coplanar
/// meridian arcs that stop short of both poles, tagged as a closed
/// loop, are a boundary only tier 1 could otherwise have refused; the
/// arm refuses them under the premise's name, on the pole helper's own
/// margins (`props_meridian_pole` definite Negative at both poles).
#[test]
fn meridian_arcs_that_never_reach_the_poles_refuse_the_premise() {
    let edges = vec![
        great(0.0, FRAC_PI_4, -FRAC_PI_4, 0, 1),
        great(FRAC_PI_2, -FRAC_PI_4, FRAC_PI_4, 1, 0),
    ];
    assert_eq!(
        curved_face(&sphere(), &edges, true, band()).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "a rimless wedge's meridians run pole to pole"
        })
    );
}

/// **A wedge whose meridians are SPLIT is refused as unfolded.** The
/// arm reads a two-edge boundary only, so the same lune with both
/// meridians cut at the equator — four arcs on two great circles,
/// which is exactly a wedge — refuses under a `what` that says that
/// and nothing more (the torus arm folds pieces by lineage; the sphere
/// arm does not). The row pins the scope and the wording.
#[test]
fn a_wedge_whose_meridians_are_split_refuses_as_unfolded() {
    let four = vec![
        great(0.0, FRAC_PI_2, 0.0, 0, 1),
        great(0.0, 0.0, -FRAC_PI_2, 1, 2),
        great(FRAC_PI_2, -FRAC_PI_2, 0.0, 2, 3),
        great(FRAC_PI_2, 0.0, FRAC_PI_2, 3, 0),
    ];
    assert_eq!(
        curved_face(&sphere(), &four, true, band()).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "the wedge arm reads a two-edge boundary; a meridian in pieces is not folded \
                   on the sphere"
        })
    );
}

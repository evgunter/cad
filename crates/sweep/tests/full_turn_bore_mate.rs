//! **A shaft set into a full-turn bore, resting on its cylindrical
//! wall, unions.**
//!
//! The collar is the rectangle `ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved
//! a full turn about `y`: its bore is ONE face with a self-mated seam
//! ruling at azimuth 0, bounded by two rim circles that each carry one
//! vertex. The shaft is the three-arc peg of radius 0.5 (three wall
//! thirds, three seam rulings) turned so its axis is `y`, its first
//! ruling at azimuth `a`. Every bore × peg-wall pair is one carrier
//! with opposed senses, a `Rest`.
//!
//! What the mate makes the kernel do, by the shaft's azimuth and span:
//!
//! - a peg ruling passing a rim away from the rim's vertex has both its
//!   ends past the bore while the rim crosses it — recorded where the
//!   ruling crosses the collar's flat cap, and read as no event against
//!   the bore once its interior is certified clear of the bore's
//!   boundary;
//! - the bore is one face where the peg is three, so the two solids
//!   divide the contact band differently, and the seam runs where only
//!   one of them has an edge;
//! - at an azimuth off the collar's seam, the collar's seam vertices sit
//!   inside a peg wall third, with the collar's flat caps leaving them
//!   radially;
//! - with the bore split by a circle on its own carrier, the rulings
//!   cross that circle where no other face meets it, and only the
//!   crossing layer records them.
//!
//! Each op runs in both operand orders.
//! Every pose of [`crate::common::poses::poses`] moves both operands.
//! The oracle is closed form: the interiors are disjoint, so the union
//! is the collar's annulus volume plus the shaft's disc volume, the
//! intersection is empty, and each difference is its minuend whole.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{full_turn_collar, onto_y, peg_at, walls_at};
use core::f64::consts::PI;
use geom_core::{Affine3, Point3, Tol};
use sweep::test_support::finished;
use topo::construction::Construct;
use topo::{AtRestBody, Body, BooleanOp, BooleanResult};

/// The collar's bore and outer radii and its span in `y`.
const BORE: f64 = 0.5;
const OUTER: f64 = 1.5;
const COLLAR: (f64, f64) = (1.0, 2.0);

fn collar() -> Body<f64> {
    full_turn_collar(BORE, OUTER, COLLAR)
}

/// The three-arc peg, first ruling at azimuth `deg` (from `+x`, the
/// collar's seam), spanning `y ∈ [y0, y0 + h]`.
fn shaft(deg: f64, y0: f64, h: f64) -> Body<f64> {
    onto_y(&peg_at(deg, y0, h))
}

/// `b` moved by `pose`, finished as an operand.
fn placed(b: &Body<f64>, pose: &Affine3<f64>) -> AtRestBody<f64> {
    let moved = topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(pose, Tol::witness()),
        Tol::witness(),
    )
    .unwrap();
    finished("the placed operand", moved, Tol::witness())
}

fn volume(b: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(b), Tol::witness()),
        Tol::witness(),
    )
    .volume
}

/// The collar's closed-form volume, `π(R² − r²)·h`.
fn collar_volume() -> f64 {
    PI * (OUTER * OUTER - BORE * BORE) * (COLLAR.1 - COLLAR.0)
}

/// The shaft's closed-form volume, `π r² h`: its three arcs close one
/// circle.
fn shaft_volume(h: f64) -> f64 {
    PI * BORE * BORE * h
}

/// The shaft spans, by name: through both rims; exactly the bore; and
/// flush at one rim, proud of the other.
const SPANS: [(&str, f64, f64); 4] = [
    ("through", 0.5, 2.0),
    ("flush", 1.0, 1.0),
    ("proud above", 1.0, 1.5),
    ("proud below", 0.5, 1.5),
];

/// The blind spans: the shaft enters through one rim and its cap floats
/// inside the bore; or both its caps float inside it.
const BLIND: [(&str, f64, f64); 3] = [
    ("blind from below", 0.5, 1.0),
    ("blind from above", 1.5, 1.0),
    ("wholly inside", 1.2, 0.6),
];

/// `got` against the closed form, relative to the larger operand.
fn agrees(got: f64, want: f64) -> bool {
    (got - want).abs() <= 1e-12 * collar_volume()
}

/// `c ∪ p` and `p ∪ c`: a body, its volume the closed form, one shell, tier 3 and the
/// pseudomanifold census clean.
fn unions_both_ways(c: &AtRestBody<f64>, p: &AtRestBody<f64>, h: f64, tag: &str) {
    let tol = Tol::witness();
    for (order, a, b) in [("collar ∪ shaft", c, p), ("shaft ∪ collar", p, c)] {
        let tag = format!("{tag}, {order}");
        let bb = match topo::union(a, b, tol) {
            Ok(BooleanResult::Body(bb)) => bb,
            other => panic!("{tag}: the mate does not union: {:?}", other.err()),
        };
        let (got, want) = (volume(&bb.body), collar_volume() + shaft_volume(h));
        assert!(agrees(got, want), "{tag}: union volume {got} vs {want}");
        assert_eq!(bb.body.shells().count(), 1, "{tag}: one shell");
        assert_eq!(
            bb.body.outcome(),
            topo::AtRestOutcome::Validated,
            "{tag}: the site that built it gated it at tier 3"
        );
        assert_eq!(
            topo::validate_geometric(&bb.body, tol),
            Ok(()),
            "{tag}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
            Ok(()),
            "{tag}: the census"
        );
    }
}

/// **The union** at every span and pose, in both operand orders.
fn unions_at(collar: &Body<f64>, deg: f64, what: &str) {
    unions_over(collar, deg, what, &SPANS);
}

/// [`unions_at`] over the spans named.
fn unions_over(collar: &Body<f64>, deg: f64, what: &str, spans: &[(&str, f64, f64)]) {
    for (pose_name, pose) in crate::common::poses::poses() {
        let c = placed(collar, &pose);
        for &(span, y0, h) in spans {
            let p = placed(&shaft(deg, y0, h), &pose);
            let tag = format!("{what}, azimuth {deg}, {span}, pose {pose_name}");
            unions_both_ways(&c, &p, h, &tag);
        }
    }
}

#[test]
fn a_shaft_on_the_bores_seam_unions() {
    unions_at(&collar(), 0.0, "one-face bore");
}

#[test]
fn a_shaft_off_the_bores_seam_unions() {
    unions_at(&collar(), 60.0, "one-face bore");
}

#[test]
fn a_shaft_a_quarter_turn_off_the_bores_seam_unions() {
    unions_at(&collar(), 90.0, "one-face bore");
}

/// **A blind shaft unions** on the bore's seam, off it and a quarter
/// turn off it, at every pose and in both operand orders: its floating
/// cap rim crosses the bore's seam ruling, and the section joins it.
/// Wholly inside, the contact is a band round the whole bore, and the
/// run it walls off reads its rings on one full period of the chart.
#[test]
fn a_blind_shaft_unions_on_and_off_the_bores_seam() {
    for deg in [0.0, 60.0, 90.0] {
        unions_over(&collar(), deg, "one-face bore", &BLIND);
    }
}

/// **Off the seam, the join builds the mate**: the peg's rulings put
/// the join's run vertices at exactly the pierce rings' azimuth, and the
/// chart reads each ring off those rows.
#[test]
fn a_shaft_off_the_bores_seam_is_built_by_the_join() {
    let (c, (_, y0, h)) = (collar(), SPANS[0]);
    let p = shaft(60.0, y0, h);
    let join = topo::test_support::boolean_join_refusal(BooleanOp::Union, &c, &p, Tol::witness());
    assert!(
        join.as_ref().is_ok_and(Option::is_none),
        "the join connects the mate, got {join:?}"
    );
    let tol = Tol::witness();
    let (c, p) = (
        sweep::test_support::finished("the collar", c, tol),
        sweep::test_support::finished("the shaft", p, tol),
    );
    unions_both_ways(&c, &p, h, "the join's row");
}

/// **The join's union carries the carrier ladder's rows**: the mate off
/// the seam records each bore × peg-wall `Rest` once, as `SameOpposite`
/// at the carrier ladder, its cells the two faces of their operands. In
/// both operand orders.
#[test]
fn the_joins_union_records_each_rest_at_the_carrier_ladder() {
    let (c, (_, y0, h)) = (collar(), SPANS[0]);
    let p = shaft(60.0, y0, h);
    let tol = Tol::witness();
    let (c, p) = (
        sweep::test_support::finished("the collar", c, tol),
        sweep::test_support::finished("the shaft", p, tol),
    );
    for (order, a, b) in [("collar ∪ shaft", &c, &p), ("shaft ∪ collar", &p, &c)] {
        let mut rests: Vec<_> = walls_at(a, BORE)
            .into_iter()
            .flat_map(|fa| walls_at(b, BORE).into_iter().map(move |fb| (fa, fb)))
            .map(|(fa, fb)| {
                [
                    topo::RowCell::face(topo::Operand::A, fa),
                    topo::RowCell::face(topo::Operand::B, fb),
                ]
            })
            .collect();
        rests.sort();
        assert_eq!(
            rests.len(),
            3,
            "{order}: the bore against three wall thirds"
        );
        let bb = match topo::union(a, b, tol) {
            Ok(BooleanResult::Body(bb)) => bb,
            other => panic!("{order}: the mate does not union: {:?}", other.err()),
        };
        let curved: Vec<_> = bb
            .coincidences
            .iter()
            .filter(|r| r.site == topo::DecisionSite::CarrierLadder)
            .collect();
        let mut cells: Vec<_> = curved.iter().map(|r| r.cells).collect();
        cells.sort();
        assert_eq!(cells, rests, "{order}: one row per Rest");
        assert!(
            curved
                .iter()
                .all(|r| r.relation == topo::Relation::SameOpposite),
            "{order}: {curved:?}"
        );
    }
}

/// The collar with its bore split into two full-turn faces by the
/// circle at `y = 1.5`: the circle is an edge between two faces of ONE
/// carrier, so no other face of the collar meets it.
fn split_collar() -> Body<f64> {
    let tol = Tol::witness();
    let mut c = collar();
    let bore = crate::mate2_common::walls_at(&c, BORE)[0];
    let skey = c.get_face(bore).unwrap().surface;
    let carrier = |c: &Body<f64>, e: &topo::Edge| {
        c.get_curve_geom(e.curve)
            .and_then(topo::null::CurveGeom::certified)
            .map(|g| g.carrier().clone())
    };
    // The bore's self-mated seam ruling, split at the circle's height.
    let (seam, origin, dir) = c
        .edges()
        .find_map(|(k, e)| match carrier(&c, e) {
            Some(geom::Curve3::Line { origin, dir })
                if c.face_of_half_edge(e.he_plus) == Some(bore)
                    && c.face_of_half_edge(e.he_minus) == Some(bore) =>
            {
                Some((k, origin, dir))
            }
            _ => None,
        })
        .expect("the bore's seam ruling");
    let w = c
        .split_edge(seam, (1.5 - origin.y) / dir.y, tol)
        .unwrap()
        .vertex;
    // The circle: the lower rim's carrier, lifted to `y = 1.5`.
    let rim = c
        .edges()
        .find_map(|(_, e)| match carrier(&c, e) {
            Some(geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            }) if (radius - BORE).abs() < 1e-12 && (center.y - COLLAR.0).abs() < 1e-12 => {
                Some(geom::Curve3::Circle {
                    center: Point3::new(center.x, 1.5, center.z),
                    axis,
                    radius,
                    u_ref,
                })
            }
            _ => None,
        })
        .expect("the bore's lower rim");
    let pw = *c.get_point(c.get_vertex(w).unwrap().point).unwrap();
    let t0 = rim.param_near(pw, 0.0).unwrap();
    let spec = geom_brep::EdgeCurveSpec::arc_of_circle(rim, t0, t0 + core::f64::consts::TAU)
        .unwrap()
        .at_rest_in_chart(skey, false);
    // `w`'s two visits in the bore's loop, one per side of the seam.
    let visits: Vec<_> = c
        .half_edges()
        .filter(|&(k, h)| h.start == w && c.face_of_half_edge(k) == Some(bore))
        .map(|(k, _)| k)
        .collect();
    let [down, up] = visits[..] else {
        panic!("the seam vertex is visited twice: {visits:?}");
    };
    c.mef(
        topo::construction::MefSite::Chords { he1: up, he2: down },
        spec,
        topo::construction::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    assert_eq!(
        topo::validate_geometric(&c, tol),
        Ok(()),
        "the split collar"
    );
    assert_eq!(
        crate::mate2_common::walls_at(&c, BORE).len(),
        2,
        "two bore faces"
    );
    c
}

/// **Only the crossing layer sees the shaft's rulings cross the bore's
/// split circle.** The circle's two faces are both on the shared
/// carrier, so no transverse face meets the rulings there — the
/// one-face bore's rims have its flat caps, which do. At the bore's
/// seam azimuth every shaft span unions in both orders and at every
/// pose. Off the seam, the circle's own vertex sits inside a shaft wall
/// third
/// ([`a_bore_split_on_its_own_carrier_unions_off_the_seam_at_every_span`]).
#[test]
fn a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth() {
    unions_at(&split_collar(), 0.0, "split bore");
}

/// **Off the seam, the split bore unions at every span**: through,
/// flush, proud above and proud below, at every pose and in both operand
/// orders, though the circle's vertex sits inside a shaft wall third.
#[test]
fn a_bore_split_on_its_own_carrier_unions_off_the_seam_at_every_span() {
    unions_over(&split_collar(), 60.0, "split bore", &SPANS);
}

/// **The other three ops answer the closed form**: `∩` empty in both
/// operand orders, and each difference its minuend whole, at azimuths
/// 0°, 60° and 90°, every span and every pose, on the one-face bore and
/// (at the seam azimuth) the split bore.
#[test]
fn intersect_and_differences_answer_the_closed_form() {
    let tol = Tol::witness();
    for (pose_name, pose) in crate::common::poses::poses() {
        for (bore, collar, degs) in [
            ("one-face bore", collar(), &[0.0, 60.0, 90.0][..]),
            ("split bore", split_collar(), &[0.0][..]),
        ] {
            let c = placed(&collar, &pose);
            for &deg in degs {
                for (span, y0, h) in SPANS {
                    let p = placed(&shaft(deg, y0, h), &pose);
                    let tag = format!("{bore}, azimuth {deg}, {span}, pose {pose_name}");
                    let run = |op, a, b| {
                        topo::boolean_op_with(
                            op,
                            a,
                            b,
                            &topo::BooleanInputs::default(),
                            topo::SweepStrategy::Realized,
                            tol,
                        )
                    };
                    for (order, a, b) in [("c ∩ p", &c, &p), ("p ∩ c", &p, &c)] {
                        assert!(
                            matches!(run(BooleanOp::Intersect, a, b), Ok(BooleanResult::Empty)),
                            "{tag}: {order} is empty"
                        );
                    }
                    for (order, a, b, want) in [
                        ("c ∖ p", &c, &p, collar_volume()),
                        ("p ∖ c", &p, &c, shaft_volume(h)),
                    ] {
                        let bb = match run(BooleanOp::Subtract, a, b) {
                            Ok(BooleanResult::Body(bb)) => bb,
                            other => panic!("{tag}: {order}: {other:?}"),
                        };
                        let got = volume(&bb.body);
                        assert!(agrees(got, want), "{tag}: {order}: volume {got} vs {want}");
                        assert_eq!(bb.body.shells().count(), 1, "{tag}: {order}: one shell");
                        assert_eq!(
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
                            Ok(()),
                            "{tag}: {order}: the census"
                        );
                    }
                }
            }
        }
    }
}

/// **The mate's vertices on the bore's rims are read no partner of,
/// and say so**: a shaft ruling on the bore's seam meets the collar at
/// a rim vertex, where neither the bore nor the end face holds a
/// subdivision bisector to read their wedge by
/// (`UnreadNeighbourhood::NoBisector`), so the boolean classes none of
/// the shaft's edges there; the record lists each such shaft vertex and
/// why (`BooleanNaming::unread_partners`), which naming refuses by
/// (`NamingError::PartnerUnread`) rather than reading a missing row,
/// and no listed vertex holds a class.
#[test]
fn the_mate_records_its_unread_partners() {
    let tol = Tol::witness();
    let c = finished("the collar", collar(), tol);
    for &(span, y0, h) in &SPANS {
        let p = finished("the shaft", shaft(0.0, y0, h), tol);
        for (order, a, b) in [("collar ∪ shaft", &c, &p), ("shaft ∪ collar", &p, &c)] {
            let bb = match topo::union(a, b, tol) {
                Ok(BooleanResult::Body(bb)) => bb,
                other => panic!(
                    "{span}, {order}: the mate does not union: {:?}",
                    other.err()
                ),
            };
            let unread = &bb.naming.unread_partners;
            assert!(
                !unread.is_empty(),
                "{span}, {order}: no partner went unread"
            );
            for u in unread {
                assert_eq!(
                    (u.operand == topo::Operand::A, u.neighbourhood),
                    (
                        order.starts_with("shaft"),
                        topo::UnreadNeighbourhood::NoBisector
                    ),
                    "{span}, {order}: the shaft's vertex at the rim, no bisector: {u:?}"
                );
                assert!(
                    !bb.naming
                        .edge_classes
                        .iter()
                        .any(|c| (c.operand, c.vertex) == (u.operand, u.vertex)),
                    "{span}, {order}: an unread vertex holds a class: {u:?}"
                );
            }
        }
    }
}

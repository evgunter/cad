//! **The plane–plane band's mitre** (`work/band/a-plane-plane-blend-
//! cannot-end-at-an-unrequested-corner.md`, step 4): where two requested
//! edges turn at a trivalent vertex whose third edge is unrequested,
//! each band is cut off by the other's support and the two meet along
//! their intersection — a line for a chamfer, a planar ellipse for a
//! fillet. Where the trihedron is isosceles about the third edge the
//! mitre runs from the trimlines' crossing on the shared face down to
//! that edge, which ends there at a valence-4 vertex.
//!
//! The box rows' closed forms: each band removes its section along its
//! whole edge, and the two prisms overlap at the corner in
//! `∫₀ᵈ s(w)² dw`, `s(w)` the section's width at depth `w` — `d³/3` for
//! the chamfer and `(5/3 − π/2)·r³` for the fillet; the mitre is that
//! overlap's boundary, so `ΔV = section·(a + b) − overlap`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::k_stats::Bracket;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Sign, Vec3};
use sweep::blend::battery::TURN_NOT_ISOSCELES;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::blend::{
    BlendDecision, BlendError, CornerConfig, DecidedCoincidence, FILLET3_CORNER_RECOURSE,
    run_battery_for,
};
use sweep::test_support::{
    assert_naming_totality, block, pocket_die, prism, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, mass_properties, validate_geometric};

use crate::band_planar_cut_off::{
    D, Verb, carve, edge, midpoint_tol, pad_ceiling, the_box, tol, volume, volume_enclosure,
};
use crate::common::cavity::{brick, cavity_corner, cut, edges_with_corners, rod, vented_cavity};
use crate::common::operands::{leaning_turn, parallelepiped};

/// What two band prisms meeting at a right trihedron overlap in.
fn overlap(verb: Verb) -> f64 {
    match verb {
        Verb::Chamfer => D.powi(3) / 3.0,
        Verb::Fillet => (5.0 / 3.0 - PI / 2.0) * D.powi(3),
    }
}

/// **Two adjacent top edges of a box**, both verbs: one turn, each band
/// cut off at its far end.
#[test]
fn two_adjacent_top_edges_meet_at_a_mitre() {
    let body = the_box();
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let left = edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &[front, left],
            verb,
            verb.section() * 3.5 - overlap(verb),
            "two adjacent top edges",
        );
        assert_eq!(out.blend_faces.len(), 2, "two bands");
        let rec = out.naming.as_ref().expect("births");
        assert_eq!(rec.mitres.len(), 1, "one mitre");
        assert_eq!(rec.turn_feet.len(), 1, "one turn foot");
    }
}

/// **A box's whole top rim**, both verbs: four turns, every band ending
/// in a mitre at both ends.
#[test]
fn a_box_top_rim_turns_four_mitres() {
    let body = the_box();
    let rim = [
        edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge(&body, [0.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &rim,
            verb,
            verb.section() * 7.0 - 4.0 * overlap(verb),
            "the top rim",
        );
        assert_eq!(out.blend_faces.len(), 4, "four bands");
        let rec = out.naming.as_ref().expect("births");
        assert_eq!(rec.mitres.len(), 4, "four mitres");
    }
}

/// **The concave side**: the pocketed die's floor rim, four concave
/// turns, each band adding its section and the four overlaps counted
/// once.
#[test]
fn a_pocket_floor_rim_turns_four_concave_mitres() {
    let body = pocket_die(0.0, 0.0, 0.0, tol());
    let rim = [
        edge(&body, [0.25, 0.25, 0.5], [0.75, 0.25, 0.5]),
        edge(&body, [0.75, 0.25, 0.5], [0.75, 0.75, 0.5]),
        edge(&body, [0.75, 0.75, 0.5], [0.25, 0.75, 0.5]),
        edge(&body, [0.25, 0.75, 0.5], [0.25, 0.25, 0.5]),
    ];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &rim,
            verb,
            -(verb.section() * 2.0 - 4.0 * overlap(verb)),
            "the pocket floor rim",
        );
        assert_eq!(
            out.naming.as_ref().expect("births").mitres.len(),
            4,
            "four mitres"
        );
    }
}

/// The convex pentagon a cap-rim row extrudes, its interior angles all
/// different.
const PENTAGON: [(f64, f64); 5] = [(0.0, 0.0), (2.0, 0.0), (2.5, 1.0), (1.0, 2.0), (-0.5, 1.0)];

/// **An extruded cap's rim**, a convex pentagon: every turn is isosceles
/// by construction, both walls square to the cap. The band removes
/// `∫ a(s(w)) dw`, `a(t) = P·t − K·t²` the area within `t` of the
/// rim, `P` the perimeter and `K = Σ cot(θᵢ/2)` over the interior
/// angles: `P·d²/2 − K·d³/3` chamfered and
/// `P·(1 − π/4)·r² − K·(5/3 − π/2)·r³` filleted. Each fillet mitre is
/// the ellipse of minor semi-axis `r` and major `r / sin(θ/2)`.
#[test]
fn an_extruded_pentagon_cap_rim_turns_at_every_vertex() {
    let n = PENTAGON.len();
    let corner = |i: usize| Vec3::new(PENTAGON[i].0, PENTAGON[i].1, 0.0);
    let (mut perimeter, mut k) = (0.0, 0.0);
    let mut half_sines = Vec::with_capacity(n);
    for i in 0..n {
        let (prev, here, next) = (corner((i + n - 1) % n), corner(i), corner((i + 1) % n));
        perimeter += (next - here).norm();
        let (a, b) = ((prev - here).normalize(), (next - here).normalize());
        let theta = a.dot(b).acos();
        k += 1.0 / (theta / 2.0).tan();
        half_sines.push((theta / 2.0).sin());
    }
    let body = prism(
        PENTAGON
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        1.0,
        tol(),
    );
    let rim: Vec<EdgeKey> = (0..n)
        .map(|i| {
            let (a, b) = (PENTAGON[i], PENTAGON[(i + 1) % n]);
            edge(&body, [a.0, a.1, 1.0], [b.0, b.1, 1.0])
        })
        .collect();
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let removed = match verb {
            Verb::Chamfer => perimeter * D * D / 2.0 - k * D.powi(3) / 3.0,
            Verb::Fillet => perimeter * verb.section() - k * (5.0 / 3.0 - PI / 2.0) * D.powi(3),
        };
        let out = carve(&body, &rim, verb, removed, "a pentagon's cap rim");
        let rec = out.naming.as_ref().expect("births");
        assert_eq!(rec.mitres.len(), n, "a mitre per vertex");
        if let Verb::Fillet = verb {
            let mut majors: Vec<f64> = rec
                .mitres
                .iter()
                .map(|(m, _)| {
                    let c = out
                        .body
                        .get_curve_geom(out.body.get_edge(*m).unwrap().curve)
                        .and_then(|g| g.certified())
                        .expect("a certified mitre");
                    match *c.carrier() {
                        geom::Curve3::Ellipse { major, minor, .. } => {
                            assert!((minor - D).abs() < 1e-12, "the minor semi-axis is r");
                            major
                        }
                        ref other => panic!("a fillet mitre is an ellipse, got {other:?}"),
                    }
                })
                .collect();
            let mut want: Vec<f64> = half_sines.iter().map(|s| D / s).collect();
            majors.sort_by(f64::total_cmp);
            want.sort_by(f64::total_cmp);
            for (got, want) in majors.iter().zip(&want) {
                assert!(
                    (got - want).abs() < 1e-12,
                    "a mitre's major semi-axis is r / sin(θ/2): {got} vs {want}"
                );
            }
        }
    }
}

/// **A turn that is not isosceles refuses typed** (`leaning_turn`): the
/// leaning wall's dihedral is not the end face's, so one band reaches
/// past the mitre — the overrun, which is not built. Near equality the
/// verdict is in band and escalates on `fillet3_turn_isosceles`, whose
/// margin is the face angles' cosines apart, `s/√(1 + s²)`, levered at
/// the longer edge, `2 − s`; the lean is read off the run's band.
#[test]
fn a_turn_whose_faces_are_not_symmetric_refuses_typed() {
    let (body, turn) = leaning_turn(0.5);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        match verb.run(&body, &turn) {
            Err(BlendError::UnsupportedRunOut { detail, .. }) => {
                assert_eq!(detail, TURN_NOT_ISOSCELES, "{verb:?}");
            }
            other => panic!("{verb:?}: the overrun refuses as a run-out, got {other:?}"),
        }
    }
    let band = Band::linear(tol()).expect("the run's band");
    let s = (band.zero() * band.escalate()).sqrt() / 2.0;
    let (body, turn) = leaning_turn(s);
    match Verb::Chamfer.run(&body, &turn) {
        Err(BlendError::Escalated {
            decision, source, ..
        }) => {
            assert_eq!(decision, BlendDecision::TurnIsosceles);
            assert_eq!(source.predicate, Some("fillet3_turn_isosceles"));
        }
        other => panic!("s = {s}: the sliver band escalates, got {other:?}"),
    }
}

/// **The four-edge turn foot**: a mitre leaves the third edge ending at
/// a valence-4 vertex, so blending that edge in a later call refuses,
/// and the recourse says to request it in the same call — where the
/// three edges of the corner build the patch instead.
#[test]
fn a_turn_foot_refuses_a_later_blend_of_its_edge_and_names_the_same_call() {
    let body = the_box();
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let left = edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    let up = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = verb.run(&body, &[front, left]).expect("the mitre builds");
        let rec = out.naming.as_ref().expect("births");
        let remnant = rec
            .meridian_remnants
            .iter()
            .find(|(_, source)| *source == up)
            .map(|(e, _)| *e)
            .expect("the third edge's surviving piece");
        let err = verb
            .run(&out.body, &[remnant])
            .expect_err("the third edge ends at the four-edge turn foot");
        assert!(
            matches!(
                err,
                BlendError::UnsupportedCorner {
                    corner: CornerConfig::NEdgeVertex { valence: 4 },
                    ..
                }
            ),
            "{verb:?}: the turn foot refuses as a valence-4 vertex, got {err:?}"
        );
        assert!(
            err.to_string().contains(FILLET3_CORNER_RECOURSE)
                && FILLET3_CORNER_RECOURSE.ends_with("in one call"),
            "{verb:?}: the recourse names the same call: {err}"
        );
        carve(
            &body,
            &[front, left, up],
            verb,
            verb.section() * 4.5 - verb.corner(),
            "the corner's three edges in one call",
        );
    }
}

/// **The two adjacent top edges, by another door**: the box less the
/// two prisms beyond the chamfer planes, each running past the box at
/// both ends, reaches the mitred solid at the same volume.
#[test]
fn a_mitred_chamfer_matches_the_boolean_less_its_two_prisms() {
    let body = the_box();
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let left = edge(&body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    // The prism beyond the plane `u + (1 − z) = d`, `u` the distance
    // into the box from the wall along `across`, extruded along
    // `across × z` from `origin`.
    let beyond = |origin: Point3<f64>, across: Vec3<f64>| {
        let plane = sketch_from_axes(origin, across, Vec3::new(0.0, 0.0, 1.0), tol());
        let verts = vec![
            (Point2::new(-0.5, 0.5 - D), 0.0),
            (Point2::new(D + 0.5, 1.5), 0.0),
            (Point2::new(-0.5, 1.5), 0.0),
        ];
        prism_on(plane, verts, 3.0, tol())
    };
    let cut_front = beyond(Point3::new(-0.5, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let cut_left = beyond(Point3::new(0.0, 2.5, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let by_boolean = realized(
        BooleanOp::Subtract,
        &realized(BooleanOp::Subtract, &body, &cut_front, tol()),
        &cut_left,
        tol(),
    );
    let out = carve(
        &body,
        &[front, left],
        Verb::Chamfer,
        Verb::Chamfer.section() * 3.5 - overlap(Verb::Chamfer),
        "two adjacent top edges",
    );
    let (by_carve, by_door) = (volume(&out.body), volume(&by_boolean));
    assert!(
        (by_carve - by_door).abs() < 1e-12,
        "the carve and the boolean agree: {by_carve} vs {by_door}"
    );
}

/// **The verdict records the coincidence** it decided (D10): each
/// isosceles turn once, with the reading decided Zero — exactly zero on
/// the box, whose walls are square to its top by construction.
#[test]
fn an_isosceles_turn_is_recorded_as_a_value_decided_coincidence() {
    let body = the_box();
    let rim = [
        edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge(&body, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
        edge(&body, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge(&body, [0.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    for kind in [
        sweep::blend::BlendKind::Chamfer,
        sweep::blend::BlendKind::Fillet,
    ] {
        let request = sweep::blend::BlendRequest {
            body: &body,
            edges: rim.to_vec(),
            size: D,
        };
        let band = Band::linear(tol()).expect("the run's band");
        let verdict = run_battery_for(&request, band, kind).expect("the battery admits");
        assert_eq!(verdict.turns.len(), 4, "{kind:?}: four turns");
        assert_eq!(
            verdict.coincidences().count(),
            4,
            "{kind:?}: one record per turn"
        );
        for turn in &verdict.turns {
            let DecidedCoincidence::IsoscelesTurn { vertex, reading } = &turn.coincidence;
            assert_eq!(*vertex, turn.vertex, "{kind:?}: recorded at the turn");
            assert!(
                reading.abs() < 1e-15,
                "{kind:?}: the reading decided Zero, got {reading}"
            );
        }
    }
}

/// **The `Interval` replay**: two adjacent top edges of the box at the
/// certified scalar, both verbs — carved, tier-3 valid, naming total,
/// the turn decided Zero on the funnel's log, and the volume enclosure
/// bracketing the closed form.
#[test]
fn the_mitre_carves_at_the_certified_scalar() {
    use crate::common::interval::iv;
    let tol = tol();
    let body = block::<Interval>(2.0, 1.5, 1.0, tol);
    let near = |p: &Point3<Interval>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    let between = |a: [f64; 3], b: [f64; 3]| {
        topo::query::all_edges(&body)
            .into_iter()
            .find(|&e| {
                let he = body.get_edge(e).unwrap().he_plus;
                let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (s, t) = (
                    p(body.get_half_edge(he).unwrap().start),
                    p(body.half_edge_end(he).unwrap()),
                );
                (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
            })
            .expect("a box edge")
    };
    let edges = [
        between([0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        between([0.0, 0.0, 1.0], [0.0, 1.5, 1.0]),
    ];
    let v0 = mass_properties(&body, tol).expect("interval props").volume;
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let bracket = Bracket::open();
        let out = match verb {
            Verb::Chamfer => chamfer_edges(&body, &edges, iv(D), tol),
            Verb::Fillet => fillet_edges(&body, &edges, iv(D), tol),
        }
        .unwrap_or_else(|e| panic!("{verb:?}: carves at Interval, got {e:?}"));
        let turns: Vec<_> = bracket
            .finish()
            .verdicts
            .into_iter()
            .filter(|v| v.predicate == "fillet3_turn_isosceles")
            .collect();
        assert!(
            !turns.is_empty() && turns.iter().all(|v| v.sign == Sign::Zero),
            "{verb:?}: the turn is isosceles at Interval: {turns:?}"
        );
        validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("{verb:?}: tier 3, {e:?}"));
        assert_naming_totality(&body, &out, &edges, "two adjacent top edges at Interval");
        let p = mass_properties(&out.body, tol).expect("interval props");
        let removed = v0 - p.volume;
        let truth = verb.section() * 3.5 - overlap(verb);
        let pad = p.volume_pad.hi();
        assert!(
            removed.lo() - pad <= truth && truth <= removed.hi() + pad,
            "{verb:?}: ΔV {removed:?} ± {pad} brackets {truth}"
        );
        assert!(
            removed.hi() - removed.lo() + 2.0 * pad < 1e-5,
            "{verb:?}: the enclosure is a claim: {removed:?} ± {pad}"
        );
    }
}

/// **An isosceles turn whose dihedrals are not right angles**: the top
/// rim of a square frustum — two trapezoid prisms intersected, each
/// side leaning in by `s` — whose every corner is symmetric about its
/// leaning lateral edge, the two requested dihedrals both `90° + atan s`.
/// Both verbs build four mitres, tier-3 valid with naming total; the
/// fillet's lie on both their cylinders. The frustum is convex, so the
/// chamfered solid is the frustum less each edge's chamfer half-space,
/// and the boolean's volume is the oracle; the filleted solid is the
/// frustum less each edge's band prism, the union the mitres bound,
/// sampled point by point near the rim.
#[test]
fn a_frustum_top_rim_mitres_at_leaning_walls() {
    use topo::boolean::SolidContainment;
    let s = 0.3;
    let body = frustum(s);
    validate_geometric(&body, tol()).expect("the frustum is tier-3 valid");
    let (lo, hi) = (s, 2.0 - s);
    let rim = [
        edge(&body, [lo, -lo, 1.0], [hi, -lo, 1.0]),
        edge(&body, [hi, -lo, 1.0], [hi, -hi, 1.0]),
        edge(&body, [hi, -hi, 1.0], [lo, -hi, 1.0]),
        edge(&body, [lo, -hi, 1.0], [lo, -lo, 1.0]),
    ];
    // Each top edge: a point on it, the top's inward direction across
    // it, and its wall's outward unit normal.
    let k = (1.0 + s * s).sqrt();
    let edges = [
        ([lo, -lo], [0.0, -1.0], [0.0, 1.0]),
        ([hi, -lo], [-1.0, 0.0], [1.0, 0.0]),
        ([hi, -hi], [0.0, 1.0], [0.0, -1.0]),
        ([lo, -hi], [1.0, 0.0], [-1.0, 0.0]),
    ]
    .map(|([x, y], [mx, my], [nx, ny])| {
        (
            Point3::new(x, y, 1.0),
            Vec3::new(mx, my, 0.0),
            Vec3::new(nx / k, ny / k, s / k),
        )
    });
    let top = Vec3::new(0.0, 0.0, 1.0);
    let in_frustum = |p: Point3<f64>| {
        (0.0..=1.0).contains(&p.z)
            && (s * p.z..=2.0 - s * p.z).contains(&p.x)
            && (s * p.z - 2.0..=-s * p.z).contains(&p.y)
    };
    let v0 = volume(&body);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = verb
            .run(&body, &rim)
            .unwrap_or_else(|e| panic!("{verb:?}: the frustum's rim mitres, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{verb:?}: tier 3, {e:?}"));
        assert_naming_totality(&body, &out, &rim, "the frustum's top rim");
        assert_eq!(
            out.naming.as_ref().expect("births").mitres.len(),
            4,
            "{verb:?}: four mitres"
        );
        let (_, stray) = crate::band_planar_cut_off::arc_residual(&out);
        assert!(
            stray < 1e-12,
            "{verb:?}: a mitre strays {stray} from a band"
        );
        match verb {
            Verb::Chamfer => {
                let oracle = edges.iter().fold(body.clone(), |cut, &(p, m, n)| {
                    realized(
                        BooleanOp::Intersect,
                        &cut,
                        &half_space(p + m * D, top + n),
                        tol(),
                    )
                });
                let (removed, want) = (v0 - volume(&out.body), v0 - volume(&oracle));
                assert!(
                    (removed - want).abs() < 1e-9,
                    "chamfer: ΔV {removed} against the half-spaces' {want}"
                );
            }
            Verb::Fillet => {
                // Each band's prism: the wedge between its axis's two
                // radii to the feet, outside its cylinder.
                let prisms = edges.map(|(p, _, n)| {
                    let c = p - (top + n) * (D / (1.0 + top.dot(n)));
                    (c, (top.cross(n)).normalize(), n)
                });
                let removed = |q: Point3<f64>| -> Option<bool> {
                    let mut any = false;
                    for &(c, axis, n) in &prisms {
                        let w = q - c;
                        let w = w - axis * w.dot(axis);
                        let g = top.dot(n);
                        let (a, b) = (
                            (w.dot(top) - g * w.dot(n)) / (1.0 - g * g),
                            (w.dot(n) - g * w.dot(top)) / (1.0 - g * g),
                        );
                        let rim_gap = (w.norm() - D).abs().min(a.abs()).min(b.abs());
                        if rim_gap < 1e-6 {
                            return None;
                        }
                        any |= a > 0.0 && b > 0.0 && w.norm() > D;
                    }
                    Some(any)
                };
                let band = Band::linear(tol()).expect("the run's band");
                let (mut checked, mut wrong) = (0, Vec::new());
                for (i, j, h) in
                    (0..9).flat_map(|i| (0..9).flat_map(move |j| (0..5).map(move |h| (i, j, h))))
                {
                    // Near the corner `(lo, −lo, 1)` and along both its
                    // edges, every corner being the same by symmetry.
                    let q = Point3::new(
                        lo - 0.02 + 0.37 * f64::from(i) / 8.0,
                        -lo + 0.02 - 0.37 * f64::from(j) / 8.0,
                        0.86 + 0.13 * f64::from(h) / 4.0,
                    );
                    let faces = [1.0 - q.z, q.x - s * q.z, -s * q.z - q.y];
                    if faces.iter().any(|f| f.abs() < 1e-6) {
                        continue;
                    }
                    let Some(gone) = removed(q) else { continue };
                    let want = in_frustum(q) && !gone;
                    let got = topo::boolean::point_in_solid(&out.body, q, band, tol())
                        .expect("membership reads");
                    checked += 1;
                    if matches!(got, SolidContainment::In) != want
                        || matches!(got, SolidContainment::OnBoundary)
                    {
                        wrong.push((q, got, want));
                    }
                }
                assert!(checked > 200, "fillet: {checked} points sampled");
                assert!(
                    wrong.is_empty(),
                    "fillet: {} of {checked} points disagree with the prisms' union: {wrong:?}",
                    wrong.len()
                );
            }
        }
    }
}

/// The square frustum of [`a_frustum_top_rim_mitres_at_leaning_walls`]:
/// `x ∈ [s z, 2 − s z]`, `y ∈ [s z − 2, −s z]`, `z ∈ [0, 1]`.
fn frustum(s: f64) -> Body<f64> {
    // The trapezoid between heights `z0` and `z1` whose sides run
    // through `(0, 0)–(s, 1)` and `(2, 0)–(2 − s, 1)`, in the plane
    // through `origin` spanned by `u` and `z`, extruded 3 along `u × z`.
    let trapezoid = |origin: Point3<f64>, u: Vec3<f64>, (z0, z1): (f64, f64)| {
        let plane = sketch_from_axes(origin, u, Vec3::new(0.0, 0.0, 1.0), tol());
        prism_on(
            plane,
            vec![
                (Point2::new(s * z0, z0), 0.0),
                (Point2::new(2.0 - s * z0, z0), 0.0),
                (Point2::new(2.0 - s * z1, z1), 0.0),
                (Point2::new(s * z1, z1), 0.0),
            ],
            3.0,
            tol(),
        )
    };
    // The second reaches past the first's top and bottom, so the two
    // share no face plane.
    let along_x = trapezoid(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        (0.0, 1.0),
    );
    let along_y = trapezoid(
        Point3::new(2.5, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        (-0.5, 1.5),
    );
    realized(BooleanOp::Intersect, &along_x, &along_y, tol())
}

/// The half-space `(x − q)·n ≤ 0` within a slab ten wide: a prism on
/// its plane, extruded along `−n`.
fn half_space(q: Point3<f64>, n: Vec3<f64>) -> Body<f64> {
    let n = n.normalize();
    let seed = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = (seed - n * seed.dot(n)).normalize();
    let v = u.cross(n);
    prism_on(
        sketch_from_axes(q - u * 5.0 - v * 5.0, u, v, tol()),
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(10.0, 0.0), 0.0),
            (Point2::new(10.0, 10.0), 0.0),
            (Point2::new(0.0, 10.0), 0.0),
        ],
        10.0,
        tol(),
    )
}

/// [`carve`]'s checks — tier 3, naming totality, `ΔV` against the
/// closed form — on a body with a feature the bands leave alone, which
/// may be a second shell.
fn carve_beside(body: &Body<f64>, edges: &[EdgeKey], verb: Verb, removed: f64, what: &str) {
    let out = verb
        .run(body, edges)
        .unwrap_or_else(|e| panic!("{what} ({verb:?}): builds, got {e}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what} ({verb:?}): tier 3, got {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    let ((v0, pad0), (v1, pad1)) = (volume_enclosure(body), volume_enclosure(&out.body));
    let (dv, pad) = (v0 - v1, pad0 + pad1);
    assert!(
        pad < pad_ceiling() && (dv - removed).abs() < midpoint_tol() + pad,
        "{what} ({verb:?}): ΔV {dv} ± {pad} vs the closed form {removed}"
    );
}

fn fuse(what: &str, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let a = sweep::test_support::finished(&format!("{what}: the first operand"), a.clone(), tol());
    let b = sweep::test_support::finished(&format!("{what}: the second operand"), b.clone(), tol());
    topo::union(&a, &b, tol())
        .unwrap_or_else(|e| panic!("{what}: the union succeeds: {e:?}"))
        .body()
        .unwrap_or_else(|| panic!("{what}: the union leaves material"))
        .body
        .clone()
        .into_body()
}

/// Whether `verb` refuses `edges` of `body` on predicate 2's reach.
fn refuses_on_reach(body: &Body<f64>, edges: &[EdgeKey], verb: Verb, what: &str) {
    match verb.run(body, edges) {
        Err(BlendError::FaceClearance { bounded: false, .. }) => {}
        Err(e) => panic!("{what} ({verb:?}): refuses FaceClearance, got {e}"),
        Ok(out) => panic!(
            "{what} ({verb:?}): refuses FaceClearance, built (tier 3 {:?})",
            validate_geometric(&out.body, tol())
        ),
    }
}

/// **A convex mitre over a void.** The box with a sealed void
/// `[0.05, 0.3]² × [0.5, top]` under the corner its two top edges turn
/// at: the void's vertical edge `x = y = 0.05` lies in the mitre's
/// plane. The chamfer removes it above `z = 1 − d + 0.05 = 0.95`, the
/// fillet above `1 − r + √(r² − 0.05²) ≈ 0.9866`; each verb refuses a
/// void whose top is `0.02` (chamfer) or `0.0034` (fillet) inside, and
/// builds at the volume the void does not touch where it is
/// `0.02`/`0.0116` clear.
#[test]
fn a_mitre_over_a_void_refuses_where_its_bands_reach_it() {
    let with_void = |top: f64| {
        cut(
            "void",
            &the_box(),
            &brick(Point3::new(0.05, 0.05, 0.5), Point3::new(0.3, 0.3, top)),
        )
    };
    let turn = |body: &Body<f64>| {
        [
            edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
            edge(body, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]),
        ]
    };
    for (verb, inside, clear) in [(Verb::Chamfer, 0.97, 0.93), (Verb::Fillet, 0.99, 0.975)] {
        let body = with_void(inside);
        assert_eq!(body.solids().count(), 1, "the void is a shell of the box");
        refuses_on_reach(&body, &turn(&body), verb, "a void in the mitre's bands");
        let body = with_void(clear);
        carve_beside(
            &body,
            &turn(&body),
            verb,
            verb.section() * 3.5 - overlap(verb),
            "a void clear of the mitre's bands",
        );
    }
}

/// **Concave mitres beside an island.** [`vented_cavity`]'s floor rim,
/// four concave turns, with an island standing `gap` off the floor and
/// every wall on a stem through the floor. The bands add material to
/// the island's bottom edges where the chamfer's `2·gap < d`
/// (`gap < 0.05`) or the fillet's `gap < r(1 − 1/√2) ≈ 0.0293`: each
/// verb refuses about `0.005` inside and builds at the closed form
/// about `0.005` clear.
#[test]
fn concave_mitres_beside_an_island_refuse_where_their_bands_reach_it() {
    let with_island = |gap: f64| {
        let lo = 1.0 + gap;
        let island = brick(
            Point3::new(lo, lo, lo),
            Point3::new(3.0 - gap, 3.0 - gap, 2.4),
        );
        let stem = rod(Point2::new(2.0, 2.0), 0.1, 0.9, lo + 0.05);
        fuse("island", &fuse("stem", &vented_cavity(), &stem), &island)
    };
    let floor = |body: &Body<f64>| {
        edges_with_corners(body, |p| cavity_corner(p) && (p.z - 1.0).abs() < 1e-12)
    };
    for (verb, inside, clear) in [(Verb::Chamfer, 0.045, 0.055), (Verb::Fillet, 0.024, 0.034)] {
        let body = with_island(inside);
        let rim = floor(&body);
        assert_eq!(rim.len(), 4, "the cavity's floor rim");
        refuses_on_reach(&body, &rim, verb, "an island in the floor rim's bands");
        let body = with_island(clear);
        carve_beside(
            &body,
            &floor(&body),
            verb,
            -(verb.section() * 8.0 - 4.0 * overlap(verb)),
            "an island clear of the floor rim's bands",
        );
    }
}

/// **A sheared box's supplementary corners refuse; its isosceles ones
/// build** (`common::operands::parallelepiped`, lean `s = 0.3`). At
/// `(s, s, 1)` the two top edges make one angle with the lateral edge,
/// and the turn mitres: tier 3, naming total, and the chamfer at the
/// volume of the box less its two chamfer half-spaces. At `(2 + s, s,
/// 1)` they make supplementary angles, and both verbs refuse it as not
/// isosceles, which is all the refusal claims: a chamfer's two feet on
/// the lateral edge coincide there, so no band reaches past the other,
/// and that chamfer is the overrun's design, not built here.
#[test]
fn a_sheared_box_mitres_its_isosceles_corners_and_refuses_its_supplementary_ones() {
    let s = 0.3;
    let body = parallelepiped(s);
    validate_geometric(&body, tol()).expect("the sheared box is tier-3 valid");
    let v = [s, s, 1.0];
    let iso = [
        edge(&body, v, [2.0 + s, s, 1.0]),
        edge(&body, v, [s, 1.5 + s, 1.0]),
    ];
    let k = (1.0 + s * s).sqrt();
    let top = Vec3::new(0.0, 0.0, 1.0);
    let p = Point3::new(s, s, 1.0);
    let oracle = [
        (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, s) / k),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, s) / k),
    ]
    .iter()
    .fold(body.clone(), |cut, &(m, n)| {
        realized(
            BooleanOp::Intersect,
            &cut,
            &half_space(p + m * D, top + n),
            tol(),
        )
    });
    let v0 = volume(&body);
    let w = [2.0 + s, s, 1.0];
    let supplementary = [
        edge(&body, w, [s, s, 1.0]),
        edge(&body, w, [2.0 + s, 1.5 + s, 1.0]),
    ];
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = verb
            .run(&body, &iso)
            .unwrap_or_else(|e| panic!("{verb:?}: the isosceles corner mitres, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{verb:?}: tier 3, {e:?}"));
        assert_naming_totality(&body, &out, &iso, "the sheared box's isosceles corner");
        if let Verb::Chamfer = verb {
            let (removed, want) = (v0 - volume(&out.body), v0 - volume(&oracle));
            assert!(
                (removed - want).abs() < 1e-9,
                "chamfer: ΔV {removed} against the half-spaces' {want}"
            );
        }
        match verb.run(&body, &supplementary) {
            Err(BlendError::UnsupportedRunOut { detail, .. }) => {
                assert_eq!(detail, TURN_NOT_ISOSCELES, "{verb:?}");
            }
            other => panic!("{verb:?}: the supplementary corner refuses, got {other:?}"),
        }
    }
}

/// **A sliver void behind an oblique turn's station**: the sheared
/// box's isosceles corner, its cap planes oblique to both edges, so each
/// band's material near the foot on the lateral edge lies at stations
/// behind the vertex along its own edge — inside the reach's window
/// only by the cap's pad. The chamfer reaches the void and refuses it as
/// its material; the fillet, whose section is thinner there, leaves it
/// whole and builds at the void-free carve's volume.
#[test]
fn a_void_behind_an_oblique_turns_station_refuses_where_its_band_reaches_it() {
    let s = 0.3;
    let clean = parallelepiped(s);
    let body = cut(
        "sliver void",
        &clean,
        &brick(
            Point3::new(0.286, 0.279, 0.9),
            Point3::new(0.298, 0.284, 0.925),
        ),
    );
    let v = [s, s, 1.0];
    let turn = |b: &Body<f64>| [edge(b, v, [2.0 + s, s, 1.0]), edge(b, v, [s, 1.5 + s, 1.0])];
    refuses_on_reach(&body, &turn(&body), Verb::Chamfer, "a sliver void behind v");
    let base = Verb::Fillet
        .run(&clean, &turn(&clean))
        .expect("the void-free fillet builds");
    let removed = volume_enclosure(&clean).0 - volume_enclosure(&base.body).0;
    carve_beside(
        &body,
        &turn(&body),
        Verb::Fillet,
        removed,
        "a sliver void behind v",
    );
}

/// **A turn's foot and a cut-off's foot on one third edge, crossing
/// where the support screen passes.** A prism over the quadrilateral
/// `(0, 0), (cos 30°, −sin 30°), (2, h), (0, h)` in `xz`, extruded along
/// `−y`: its top two edges at `(0, 0, h)` turn about the vertical edge
/// L, whose foot stands `d` below the top; the front bottom edge, at
/// 120° to L, is cut off at L's lower end, its foot `d / sin 120°` up
/// L. The two cross at `h ≤ d·(1 + 2/√3) ≈ 0.2155`, while the front
/// face's two blended edges stand `h` apart, which the screen clears
/// for `h > 2d`. Between the two the shared-rim meter alone refuses;
/// above, both verbs build.
#[test]
fn a_turn_foot_and_a_cut_off_foot_cross_where_the_screen_passes() {
    let (c, sn) = (30f64.to_radians().cos(), 30f64.to_radians().sin());
    for (h, crosses) in [(0.22, false), (0.215, true), (0.205, true)] {
        let body = prism_on(
            sketch_from_axes(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                tol(),
            ),
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(c, -sn), 0.0),
                (Point2::new(2.0, h), 0.0),
                (Point2::new(0.0, h), 0.0),
            ],
            1.5,
            tol(),
        );
        let edges = [
            edge(&body, [0.0, 0.0, h], [2.0, 0.0, h]),
            edge(&body, [0.0, 0.0, h], [0.0, -1.5, h]),
            edge(&body, [0.0, 0.0, 0.0], [c, 0.0, -sn]),
        ];
        for verb in [Verb::Chamfer, Verb::Fillet] {
            match (verb.run(&body, &edges), crosses) {
                (Err(BlendError::UnsupportedRunOut { detail, .. }), true) => assert!(
                    detail.contains("feet cross or coincide on the rim they share"),
                    "h = {h} ({verb:?}): the shared-rim meter refuses, got {detail}"
                ),
                (Ok(out), false) => {
                    validate_geometric(&out.body, tol())
                        .unwrap_or_else(|e| panic!("h = {h} ({verb:?}): tier 3, {e:?}"));
                    assert_naming_totality(&body, &out, &edges, "a turn and a cut-off on L");
                }
                (other, _) => panic!(
                    "h = {h} ({verb:?}): {} expected, got {:?}",
                    if crosses {
                        "the feet-cross refusal"
                    } else {
                        "a build"
                    },
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// **The mitre's end lies on both trimlines within the band**, at
/// obtuse and acute face angles: the sheared box's isosceles corner at
/// leans `±0.6` (face angles about 117° and 63°), and the top rims of a
/// frustum and of an inverted one (each corner's face angles about 106°
/// and 74°). The verdict decides Zero only with the two bands' feet on
/// the third edge within the band of each other, so the foot — their
/// midpoint — is within half of it from each band's trimline on its
/// own face of the third edge.
#[test]
fn the_turn_foot_lies_on_both_trimlines_within_the_band() {
    let band = Band::linear(tol()).expect("the run's band");
    let sheared = |s: f64| {
        let body = parallelepiped(s);
        let v = [s, s, 1.0];
        let e = vec![
            edge(&body, v, [2.0 + s, s, 1.0]),
            edge(&body, v, [s, 1.5 + s, 1.0]),
        ];
        (body, e)
    };
    let rim = |s: f64| {
        let body = frustum(s);
        let (lo, hi) = (s, 2.0 - s);
        let e = vec![
            edge(&body, [lo, -lo, 1.0], [hi, -lo, 1.0]),
            edge(&body, [hi, -lo, 1.0], [hi, -hi, 1.0]),
            edge(&body, [hi, -hi, 1.0], [lo, -hi, 1.0]),
            edge(&body, [lo, -hi, 1.0], [lo, -lo, 1.0]),
        ];
        (body, e)
    };
    for (what, (body, edges)) in [
        ("sheared 0.6", sheared(0.6)),
        ("sheared −0.6", sheared(-0.6)),
        ("frustum 0.3", rim(0.3)),
        ("inverted frustum −0.3", rim(-0.3)),
    ] {
        for kind in [
            sweep::blend::BlendKind::Chamfer,
            sweep::blend::BlendKind::Fillet,
        ] {
            let request = sweep::blend::BlendRequest {
                body: &body,
                edges: edges.clone(),
                size: D,
            };
            let verdict = run_battery_for(&request, band, kind)
                .unwrap_or_else(|e| panic!("{what} ({kind:?}): the battery admits, got {e}"));
            assert!(!verdict.turns.is_empty(), "{what} ({kind:?}): a turn");
            for turn in &verdict.turns {
                for (edge, face) in turn.requested.iter().zip(turn.others) {
                    let link = verdict
                        .chains
                        .iter()
                        .flat_map(|c| c.links())
                        .find(|l| l.edge == *edge)
                        .expect("the turn's link");
                    let (line, _) = if link.face_a == face {
                        &link.blend.trim_a
                    } else {
                        &link.blend.trim_b
                    };
                    let geom::Curve3::Line { origin, dir } = line else {
                        panic!("{what} ({kind:?}): a plane band's trimline is a line");
                    };
                    let miss = (turn.foot - *origin).cross(*dir).norm() / dir.norm();
                    assert!(
                        miss <= band.zero() / 2.0,
                        "{what} ({kind:?}): the foot misses a trimline by {miss:e}, \
                         over half the band {:e}",
                        band.zero()
                    );
                }
            }
        }
    }
}

/// **An acute turn whose face angles agree within the band but whose
/// feet do not.** A spike: the box `[−0.2, 0.6] × [−0.5, 0.5] × [0, 0.4]`
/// less two half-spaces through the origin, each through one requested
/// edge in `z = 0` — at `±α`, `α = 15°` — and the third edge, rising
/// along `(cos β, 0, sin β)`, `β = 13.6°`, so both face angles are about
/// 20°. A chamfer's feet on the third edge stand `d / sin φ` up it,
/// which moves as `cos φ / sin³φ` in `cos φ`: turn the second edge on
/// by `ε`, the face angles' cosines then differing by half the zero
/// band over the edge's lever, and the feet stand about twice the band
/// apart. The verdict reads the feet too, so that turn escalates rather
/// than building a mitre whose end misses both trimlines; the symmetric
/// spike builds.
#[test]
fn an_acute_turn_symmetric_only_by_its_angles_escalates() {
    let band = Band::linear(tol()).expect("the run's band");
    let (alpha, beta) = (15f64.to_radians(), 13.6f64.to_radians());
    let spike = |eps: f64| {
        let e1 = Vec3::new(alpha.cos(), alpha.sin(), 0.0);
        let e2 = Vec3::new((alpha + eps).cos(), -(alpha + eps).sin(), 0.0);
        let l = Vec3::new(beta.cos(), 0.0, beta.sin());
        let outward = |n: Vec3<f64>, away: Vec3<f64>| if n.dot(away) > 0.0 { -n } else { n };
        let o = Point3::new(0.0, 0.0, 0.0);
        let body = [outward(e1.cross(l), e2), outward(e2.cross(l), e1)]
            .iter()
            .fold(
                brick(Point3::new(-0.2, -0.5, 0.0), Point3::new(0.6, 0.5, 0.4)),
                |b, &n| realized(BooleanOp::Intersect, &b, &half_space(o, n), tol()),
            );
        let far = |d: Vec3<f64>| [0.6, 0.6 * d.y / d.x, 0.0];
        let edges = [
            edge(&body, [0.0, 0.0, 0.0], far(e1)),
            edge(&body, [0.0, 0.0, 0.0], far(e2)),
        ];
        (body, edges)
    };
    let (body, edges) = spike(0.0);
    let out = Verb::Chamfer
        .run(&body, &edges)
        .unwrap_or_else(|e| panic!("the symmetric spike mitres, got {e}"));
    validate_geometric(&out.body, tol()).expect("the symmetric spike's mitre is tier-3 valid");
    let lever = 0.6 / alpha.cos();
    let eps = band.zero() / (2.0 * lever * alpha.sin() * beta.cos());
    let (body, edges) = spike(eps);
    match Verb::Chamfer.run(&body, &edges) {
        Err(BlendError::Escalated {
            decision, source, ..
        }) => {
            assert_eq!(decision, BlendDecision::TurnIsosceles);
            let reading = source.margin.diagnostic_f64_for_error_text().value();
            assert!(
                reading.is_some_and(|m| m > band.zero()),
                "the feet's gap is what is in band, got {reading:?}"
            );
        }
        other => panic!(
            "ε = {eps:e}: the feet apart by more than the band escalate, got {:?}",
            other.map(|_| ())
        ),
    }
}

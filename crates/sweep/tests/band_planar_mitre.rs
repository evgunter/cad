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
use sweep::blend::battery::TURN_OVERRUN;
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
use crate::common::operands::leaning_turn;

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
                assert_eq!(detail, TURN_OVERRUN, "{verb:?}");
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
            verdict.coincidences.len(),
            4,
            "{kind:?}: one record per turn"
        );
        for (turn, record) in verdict.turns.iter().zip(&verdict.coincidences) {
            let DecidedCoincidence::IsoscelesTurn { vertex, reading } = record;
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
/// Both verbs build four mitres; the fillet's lie on both their
/// cylinders, the chamfer's in both their planes, and every face the
/// carve mints is tier-3 valid with naming total.
#[test]
fn a_frustum_top_rim_mitres_at_leaning_walls() {
    let s = 0.3;
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
    // `x ∈ [0, 2]` across `y ∈ [−2.5, 0.5]`, and `y ∈ [−2, 0]` across
    // `x ∈ [−0.5, 2.5]`.
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
    let body = realized(BooleanOp::Intersect, &along_x, &along_y, tol());
    validate_geometric(&body, tol()).expect("the frustum is tier-3 valid");
    let (lo, hi) = (s, 2.0 - s);
    let rim = [
        edge(&body, [lo, -lo, 1.0], [hi, -lo, 1.0]),
        edge(&body, [hi, -lo, 1.0], [hi, -hi, 1.0]),
        edge(&body, [hi, -hi, 1.0], [lo, -hi, 1.0]),
        edge(&body, [lo, -hi, 1.0], [lo, -lo, 1.0]),
    ];
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
        // Each band removes at most the triangle of its two setbacks
        // along its whole edge.
        let alpha = core::f64::consts::FRAC_PI_2 + s.atan();
        let bound = 4.0 * (hi - lo) * D * D * alpha.sin() / 2.0;
        let removed = v0 - volume_enclosure(&out.body).0;
        assert!(
            removed > 0.0 && removed < bound,
            "{verb:?}: ΔV {removed} is positive and under the bands' prisms, {bound}"
        );
    }
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

//! A solid touching another, or itself, along a tangent line through
//! two faces' interiors: a face–face record of the two faces, with no
//! edge on the line (`crates/topo/README.md` C7).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Band, Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::boolean::BooleanResult;
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

/// A circle of radius `r` about `(cx, cy)` with its two vertices at
/// angles `phase` and `phase + π`.
fn circle(cx: f64, cy: f64, r: f64, phase: f64) -> ProfileLoop<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    bulge_loop(vec![
        (at(phase), 1.0),
        (at(phase + core::f64::consts::PI), 1.0),
    ])
}

fn ext(what: &str, loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> AtRestBody<f64> {
    let profile = Profile::new(sketch_at(z0), loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished(what, body, tol())
}

fn point(body: &Body<f64>, v: topo::VertexKey) -> [f64; 3] {
    let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
    [p.x, p.y, p.z]
}

/// Each record's two cells' carrier kinds, faces only.
fn face_pairs(r: &topo::BooleanBody<f64>) -> Vec<(SurfaceKind, SurfaceKind)> {
    let kind = |f: topo::FaceKey| {
        let face = r.body.get_face(f).unwrap();
        r.body.get_surface(face.surface).unwrap().kind()
    };
    r.contacts
        .face_face()
        .map(|(x, y)| {
            let (p, q) = (kind(x), kind(y));
            if p == SurfaceKind::Plane {
                (p, q)
            } else {
                (q, p)
            }
        })
        .collect()
}

/// A unit disc about (1, 0) inside a disc of radius 2 about the origin,
/// tangent from inside along x = 2, y = 0, each circle's vertices off
/// that line, caps coplanar or the inner disc run past both: every op
/// refuses `CurvedPierceUnsupported`, the outer disc's cap circles
/// touching the inner wall mid-arc. So `A ∖ B`, a solid pinched along
/// the line, is not reached this way.
#[test]
fn a_nested_disc_with_its_vertices_off_the_line_refuses_its_mid_arc_touch() {
    let half_pi = core::f64::consts::FRAC_PI_2;
    let a = ext("fat", vec![circle(0.0, 0.0, 2.0, half_pi)], 0.0, 1.0);
    let b = ext("thin", vec![circle(1.0, 0.0, 1.0, half_pi)], 0.0, 1.0);
    let tall = ext("tall thin", vec![circle(1.0, 0.0, 1.0, half_pi)], -1.0, 3.0);
    for (what, got) in [
        ("A ∖ B", topo::subtract(&a, &b, tol())),
        ("A ∪ B", topo::union(&a, &b, tol())),
        ("B ∖ A", topo::subtract(&b, &a, tol())),
        ("A ∩ B", topo::intersect(&a, &b, tol())),
        ("A ∖ tall B", topo::subtract(&a, &tall, tol())),
    ] {
        assert!(
            matches!(got, Err(topo::BooleanError::CurvedPierceUnsupported { .. })),
            "{what}: {:?}",
            got.map(|_| ())
        );
    }
}

/// A rod resting on a plate's side face along a ruling through both
/// faces' interiors, its vertices off the ruling. The union is two
/// solids carrying the face–face record of the plate's face and the
/// rod's wall. A union with a box bridging the rod's top to the plate's
/// joins them into one solid, and the record, carried as the first
/// union's, lands on the plate's face and the piece of the rod's wall
/// still on it; the tier-3′ pass confirms it. Carried with the first
/// union's edge–face records as well, the bottom cap circle's lands
/// beside it, read as a point touch of the circle and the plate's face,
/// and the top cap's, whose touch point the bridge swallows, is
/// consumed.
#[test]
fn a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it() {
    let pi = core::f64::consts::PI;
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let u = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    assert_eq!(u.body.solids().count(), 2, "plate ∪ rod: two solids");
    let v = topo::mass_properties(&u.body, tol()).unwrap().volume;
    assert!(
        (v - (16.0 + 0.125 * pi)).abs() < 1e-9,
        "plate ∪ rod: volume {v}"
    );
    assert_eq!(
        face_pairs(&u),
        [(SurfaceKind::Plane, SurfaceKind::Cylinder)],
        "plate ∪ rod: the face–face record of the plate's face and the rod's wall"
    );
    let (side, _) = u.contacts.face_face().next().unwrap();
    let plane = |f: topo::FaceKey| {
        let face = u.body.get_face(f).unwrap();
        u.body.get_surface(face.surface).unwrap().kind() == SurfaceKind::Plane
    };
    let side = if plane(side) {
        side
    } else {
        u.contacts.face_face().next().unwrap().1
    };
    // Each cap circle touches the plate's face at one point inside its
    // arc: an edge–face record of the circle and that face.
    let ef: Vec<_> = u.contacts.edge_face().collect();
    let heights: Vec<f64> = ef
        .iter()
        .map(|&(e, f)| {
            assert_eq!(f, side, "plate ∪ rod: a cap record names the plate's face");
            let zs: Vec<f64> = u
                .body
                .half_edges()
                .filter(|(_, h)| h.edge == e)
                .map(|(_, h)| point(&u.body, h.start)[2])
                .collect();
            assert!(
                zs.windows(2).all(|w| w[0] == w[1]),
                "plate ∪ rod: a cap record's edge lies at one height: {zs:?}"
            );
            zs[0]
        })
        .collect();
    assert_eq!(
        heights,
        [0.25, 0.75],
        "plate ∪ rod: the bottom and top cap circles' edge–face records"
    );
    // The face–face record cites the one row the glue door recorded for
    // its tangency.
    let tangent: Vec<usize> = u
        .coincidences
        .iter()
        .enumerate()
        .filter(|(_, c)| c.relation == topo::Relation::SurfaceTangent)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        tangent.len(),
        1,
        "plate ∪ rod: one tangency row: {:?}",
        u.coincidences
    );
    assert_eq!(
        u.coincidences[tangent[0]].site,
        topo::DecisionSite::TangentWitness,
        "plate ∪ rod: the tangency row is the glue door's"
    );
    let cites: Vec<_> = u
        .contacts
        .rows()
        .iter()
        .find(|r| matches!(r.kind(), topo::ContactKind::FaceFace(..)))
        .unwrap()
        .cites
        .iter()
        .collect();
    assert_eq!(
        cites,
        [topo::Backing::Decided(u32::try_from(tangent[0]).unwrap())],
        "plate ∪ rod: the face–face record cites the tangency row"
    );
    let bridge = finished(
        "bridge",
        brick::<f64>((-1.0, 1.0), (1.0, 2.5), (0.6, 1.6), tol()),
        tol(),
    );
    let bridged = |records_a| {
        topo::boolean_op_with(
            topo::BooleanOp::Union,
            &u.body,
            &bridge,
            &topo::BooleanInputs {
                records_a,
                ..topo::BooleanInputs::default()
            },
            topo::SweepStrategy::Realized,
            tol(),
        )
    };
    let ff = topo::OperandRecords {
        ff: u.contacts.carried().ff,
        ..topo::OperandRecords::default()
    };
    match bridged(ff) {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(r.body.solids().count(), 1, "∪ bridge: one solid");
            let v = topo::mass_properties(&r.body, tol()).unwrap().volume;
            let want = 18.2 + 0.85 * pi / 8.0;
            assert!((v - want).abs() < 1e-9, "∪ bridge: volume {v}, want {want}");
            assert_eq!(
                face_pairs(&r),
                [(SurfaceKind::Plane, SurfaceKind::Cylinder)],
                "∪ bridge: the record lands on the plate's face and the rod's wall"
            );
            assert_eq!(
                topo::validate_pseudomanifold(&r.body, &r.contacts, tol()),
                Ok(()),
                "∪ bridge: tier 3′ with the landed record"
            );
        }
        other => panic!("∪ bridge: {:?}", other.map(|_| ())),
    }
    match bridged(u.contacts.carried()) {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(
                r.body.solids().count(),
                1,
                "∪ bridge, every record: one solid"
            );
            let ef: Vec<f64> = r
                .contacts
                .edge_face()
                .map(|(e, _)| edge_ends(&r.body, e)[0][2])
                .collect();
            assert_eq!(
                (face_pairs(&r).len(), ef),
                (1, vec![0.25]),
                "∪ bridge, every record: the face–face record and the bottom cap's land; the \
                 top cap's touch point is inside the bridge"
            );
            assert_eq!(
                topo::validate_pseudomanifold(&r.body, &r.contacts, tol()),
                Ok(()),
                "∪ bridge, every record: tier 3′ confirms the landed records"
            );
        }
        other => panic!("∪ bridge, every record: {:?}", other.map(|_| ())),
    }
}

/// The ends of `e`, from its half-edges' starts.
fn edge_ends(body: &Body<f64>, e: topo::EdgeKey) -> Vec<[f64; 3]> {
    body.half_edges()
        .filter(|(_, h)| h.edge == e)
        .map(|(_, h)| point(body, h.start))
        .collect()
}

/// **A cap circle's point touch is read where it is.** The union of the
/// rod and the plate passes its own 3′ on the cap circles' edge–face
/// records (the cross-solid curved search stays undecided, RESTREAD's
/// row, and is the only finding). A step block glued under the rod's
/// bottom cap, past x = 0.2, divides that cap's circle at (0.4, 2.2):
/// carrying every record, the union of the step lands the bottom cap's
/// record on the piece from (−0.5, 2.5) to (0.4, 2.2), which holds the
/// touch point (0, 2), and not on the piece past it.
#[test]
fn a_cap_circles_point_touch_is_read_and_lands_on_the_piece_holding_it() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let u = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    let caps: Vec<topo::EdgeKey> = u.contacts.edge_face().map(|(e, _)| e).collect();
    let findings = topo::validate_pseudomanifold(&u.body, &u.contacts, tol())
        .err()
        .unwrap_or_default();
    assert!(
        findings
            .iter()
            .all(|f| matches!(f, topo::ValidationError::CensusUndecidable { .. })),
        "plate ∪ rod: 3′ reads both cap records; only the cross-solid search is undecided: \
         {findings:?}"
    );
    assert_eq!(caps.len(), 2, "plate ∪ rod: two cap records");
    let step = finished(
        "step",
        brick::<f64>((0.2, 1.0), (2.2, 3.5), (-0.5, 0.25), tol()),
        tol(),
    );
    let r = match topo::boolean_op_with(
        topo::BooleanOp::Union,
        &u.body,
        &step,
        &topo::BooleanInputs {
            records_a: u.contacts.carried(),
            ..topo::BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("∪ step: {:?}", other.map(|_| ())),
    };
    let landed: Vec<Vec<[f64; 3]>> = r
        .contacts
        .edge_face()
        .map(|(e, _)| edge_ends(&r.body, e))
        .filter(|ends| ends[0][2] == 0.25)
        .collect();
    let near = |p: [f64; 3], q: [f64; 3]| p.iter().zip(q).all(|(a, b)| (a - b).abs() < 1e-9);
    let holds = |ends: &Vec<[f64; 3]>| {
        let want = [[-0.5, 2.5, 0.25], [0.4, 2.2, 0.25]];
        (near(ends[0], want[0]) && near(ends[1], want[1]))
            || (near(ends[0], want[1]) && near(ends[1], want[0]))
    };
    assert!(
        landed.len() == 1 && holds(&landed[0]),
        "∪ step: the bottom cap's record is on the piece holding the touch point: {landed:?}"
    );
    let past = r.body.edges().any(|(e, _)| {
        let ends = edge_ends(&r.body, e);
        ends.iter().any(|&p| near(p, [0.5, 2.5, 0.25]))
            && ends.iter().any(|&p| near(p, [0.4, 2.2, 0.25]))
    });
    assert!(
        past,
        "∪ step: the cap circle's piece past the cut is an edge of the result"
    );
}

/// **A step handed its tangency records the touch as the op that
/// decided it does**: the rod and the plate united with the pair
/// boolean's own tangency given ([`topo::BooleanInputs::given`], a union
/// fold step's channel) carry the same face–face record, citing a row
/// that restates the given decision on its margin.
#[test]
fn a_given_tangency_through_both_interiors_records_the_face_face_touch() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let decided = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    assert_eq!(
        decided.verdicts.len(),
        1,
        "plate ∪ rod: one verdict, the tangency"
    );
    let verdict = &decided.verdicts[0];
    let given: Vec<topo::GluedPair> = vec![verdict.given_at(topo::Operand::A, verdict.a())];
    let decision = *decided
        .coincidences
        .iter()
        .find(|row| row.relation == topo::Relation::SurfaceTangent)
        .unwrap();
    let stepped = match topo::boolean_op_with(
        topo::BooleanOp::Union,
        &plate,
        &rod,
        &topo::BooleanInputs {
            given: Some(given.clone()),
            ..topo::BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod, given: {:?}", other.map(|_| ())),
    };
    assert_eq!(
        stepped.contacts.face_face().collect::<Vec<_>>(),
        decided.contacts.face_face().collect::<Vec<_>>(),
        "plate ∪ rod, given: the face–face record the decided union carries"
    );
    let cites: Vec<_> = stepped
        .contacts
        .rows()
        .iter()
        .find(|r| matches!(r.kind(), topo::ContactKind::FaceFace(..)))
        .unwrap()
        .cites
        .iter()
        .collect();
    let [topo::Backing::Decided(k)] = cites[..] else {
        panic!("plate ∪ rod, given: the face–face record cites {cites:?}");
    };
    let row = stepped.coincidences[k as usize];
    assert_eq!(
        (row.cells, row.relation, row.site, row.margin),
        (
            decision.cells,
            decision.relation,
            decision.site,
            decision.margin
        ),
        "plate ∪ rod, given: the cited row restates the given decision"
    );
}

/// **A carried edge–face or face–face row is validated at the door**,
/// as its siblings are: a key that does not resolve, a face–face row
/// naming one face twice, and an edge–face row whose edge bounds its
/// face (no record: their interiors cannot meet) each refuse there.
#[test]
fn a_carried_curve_touch_row_with_a_bad_key_refuses_at_the_door() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let u = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    let far = finished(
        "far",
        brick::<f64>((10.0, 11.0), (10.0, 11.0), (0.0, 1.0), tol()),
        tol(),
    );
    let (edge, face) = u.contacts.edge_face().next().unwrap();
    let (fa, _) = u.contacts.face_face().next().unwrap();
    let rim = u
        .body
        .half_edges()
        .find(|&(he, _)| u.body.face_of_half_edge(he) == Some(face))
        .map(|(_, h)| h.edge)
        .unwrap();
    let ef = |edge, face| topo::OperandRecords {
        ef: vec![topo::CarriedRecord {
            contact: topo::EfContact { edge, face },
            record: 0,
        }],
        ..topo::OperandRecords::default()
    };
    let ff = |a, b| topo::OperandRecords {
        ff: vec![topo::CarriedRecord {
            contact: topo::FfContact { a, b },
            record: 0,
        }],
        ..topo::OperandRecords::default()
    };
    for (records_a, what) in [
        (
            ef(topo::EdgeKey::default(), face),
            "carried e-f edge key does not resolve",
        ),
        (
            ef(edge, topo::FaceKey::default()),
            "carried e-f face key does not resolve",
        ),
        (ef(rim, face), "carried e-f edge bounds its face"),
        (
            ff(fa, topo::FaceKey::default()),
            "carried f-f face key does not resolve",
        ),
        (ff(fa, fa), "carried f-f pair names one face twice"),
    ] {
        let got = topo::boolean_op_with(
            topo::BooleanOp::Union,
            &u.body,
            &far,
            &topo::BooleanInputs {
                records_a,
                ..topo::BooleanInputs::default()
            },
            topo::SweepStrategy::Realized,
            tol(),
        )
        .map(|_| ());
        assert!(
            matches!(
                got,
                Err(topo::BooleanError::InvalidInput {
                    operand: topo::Operand::A,
                    what: w,
                }) if w == what
            ),
            "{what}: {got:?}"
        );
    }
}
/// **A record on an arc-trimmed face confirms where the line runs
/// through it** (a face's extent along a tangent line is its true
/// extent, arcs included): a half-disc slab (chord on y = 0 from x = −1
/// to 1, arc through +y, z 0..1) and a rod lying on its top cap along
/// x = 0 over y ∈ (`y0`, 0.8), grafted into one body, with a face–face
/// record of the cap and each half of the rod's wall. The line x = 0,
/// z = 1 runs through the cap over y ∈ (0, 1) and through the bottom
/// half's interior over the rod's span: that record confirms, and the
/// top half's is stale, whether the rod's span starts inside the cap,
/// on its chord or past it, where both cap vertices project to y = 0.
#[test]
fn a_record_on_an_arc_trimmed_face_confirms_along_the_line_through_it() {
    let at = |x: f64, y: f64| Point2::new(x, y);
    let y1 = 0.8;
    for y0 in [0.2, 0.0, -0.5] {
        let slab = ext(
            "half disc",
            vec![bulge_loop(vec![(at(1.0, 0.0), 1.0), (at(-1.0, 0.0), 0.0)])],
            0.0,
            1.0,
        );
        let plane = sweep::test_support::sketch_from_axes(
            geom_core::Point3::new(0.0, y1, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            tol(),
        );
        let rod =
            sweep::test_support::extruded(plane, vec![circle(0.0, 1.5, 0.5, 0.0)], y1 - y0, tol());
        let mut body: Body<f64> = (*slab).clone();
        topo::graft_disjoint(&mut body, &rod).unwrap();
        let surface = |f: topo::FaceKey| {
            let face = body.get_face(f).unwrap();
            body.get_surface(face.surface).unwrap().clone()
        };
        let cap = body
            .faces()
            .map(|(k, _)| k)
            .find(|&f| {
                matches!(surface(f), geom::Surface::Plane { origin, normal, .. }
                    if normal.z.abs() > 0.5 && origin.z > 0.5)
            })
            .unwrap();
        let walls: Vec<_> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|&f| {
                matches!(surface(f), geom::Surface::Cylinder { axis, .. } if axis.y.abs() > 0.5)
            })
            .collect();
        assert_eq!(walls.len(), 2, "rod y {y0}..{y1}: two wall halves");
        let read: Vec<Result<(), Vec<topo::ValidationError>>> = walls
            .iter()
            .map(|&w| {
                let rec = topo::test_support::records(
                    &body,
                    [(topo::Cell::Face(cap), topo::Cell::Face(w))],
                );
                // The two solids' other touches are not this row's: the
                // cap circles' points and the curved pair in reach.
                topo::validate_pseudomanifold(&body, &rec, tol()).or_else(|es| {
                    let es: Vec<_> = es
                        .into_iter()
                        .filter(|e| {
                            !matches!(
                                e,
                                topo::ValidationError::CensusUndecidable { .. }
                                    | topo::ValidationError::UnrecordedContact { .. }
                            )
                        })
                        .collect();
                    if es.is_empty() { Ok(()) } else { Err(es) }
                })
            })
            .collect();
        let stale = |r: &Result<(), Vec<topo::ValidationError>>| matches!(r, Err(es) if matches!(es[..], [topo::ValidationError::StaleContactDeclaration { .. }]));
        assert!(
            read.iter().filter(|r| r.is_ok()).count() == 1
                && read.iter().filter(|r| stale(r)).count() == 1,
            "rod y {y0}..{y1}: the bottom half's record confirms and the top half's is stale: {read:?}"
        );
    }
}

/// **A given tangency whose faces' extents along its line are disjoint
/// is no touch**: the rod's verdict against the plate, handed to a step
/// whose plate stands one unit above the rod (the same brick moved up,
/// so the same face keys), places the line in neither face's span and
/// records nothing; the two solids stand apart.
#[test]
fn a_given_tangency_whose_faces_part_along_its_line_records_nothing() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let raised = finished(
        "raised plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (2.0, 3.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let decided = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    let verdict = &decided.verdicts[0];
    let side = raised.get_face(verdict.a()).unwrap();
    assert!(
        matches!(raised.get_surface(side.surface).unwrap(), geom::Surface::Plane { normal, .. } if normal.y > 0.5),
        "the raised plate's face under the verdict's key is its y = 2 side"
    );
    match topo::boolean_op_with(
        topo::BooleanOp::Union,
        &raised,
        &rod,
        &topo::BooleanInputs {
            given: Some(vec![verdict.given_at(topo::Operand::A, verdict.a())]),
            ..topo::BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(u)) => {
            assert_eq!(u.body.solids().count(), 2, "raised ∪ rod: two solids apart");
            assert_eq!(
                u.contacts.face_face().count(),
                0,
                "raised ∪ rod: no face–face record"
            );
        }
        other => panic!("raised ∪ rod, given: {:?}", other.map(|_| ())),
    }
}

/// The points of `f`'s vertices.
fn face_points(body: &Body<f64>, f: topo::FaceKey) -> Vec<[f64; 3]> {
    body.half_edges()
        .filter(|(_, h)| body.get_loop(h.parent_loop).is_some_and(|l| l.face == f))
        .map(|(_, h)| point(body, h.start))
        .collect()
}

/// The x-range of `f`'s vertices.
fn face_x_range(body: &Body<f64>, f: topo::FaceKey) -> (f64, f64) {
    face_points(body, f)
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
            (lo.min(v[0]), hi.max(v[0]))
        })
}

/// Two disjoint parts as one operand.
fn parts(what: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union(x, y, tol()) {
        Ok(BooleanResult::Body(u)) => finished(what, u.body.into_body(), tol()),
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

/// **A face–face record whose two faces the op divides lands on the
/// piece pair the line runs through, and passes every other.** The rod
/// on the plate's y = 2 face along the ruling x = 0, z ∈ [0.25, 0.75];
/// one block crosses the rod's far wall (y ∈ [2.8, 3.2]) and another
/// the plate's face at x ∈ [1, 1.5]. Outside: A = plate + far block,
/// B = rod + face block. Inside: A = a plate slotted through the rod's
/// wall near y = 1, B = a rod tangent to y = 2 from inside + the face
/// block. Each op's found row is confirmed on the result's piece pairs;
/// a pair on the record's carriers that the line does not reach (the
/// face block's cavity face, a piece of the plate's face past x = 1)
/// does not meet, whatever its material sides, and the row lands on
/// the pair that holds the line or nowhere. Red when such a pair reads
/// contradicted: the op then refuses `CarryRefused { Found, Unread }`.
///
/// Volumes are closed form: the far block takes from the rod
/// `0.5·∫₋.₁^.₁ (√(.25 − x²) − .3) dx`, and the slot takes the same
/// from the bore's rod.
#[test]
fn a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line() {
    let pi = core::f64::consts::PI;
    let brick_at = |what, x: (f64, f64), y: (f64, f64), z: (f64, f64)| {
        finished(what, brick::<f64>(x, y, z, tol()), tol())
    };
    let plate = brick_at("plate", (-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0));
    let far_block = brick_at("far block", (-0.1, 0.1), (2.8, 3.2), (0.0, 1.0));
    let face_block = brick_at("face block", (1.0, 1.5), (1.5, 2.5), (-0.5, 1.5));
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let slotted = ext(
        "slotted plate",
        vec![
            bulge_loop(
                [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]
                    .map(|(x, y)| (Point2::new(x, y), 0.0))
                    .to_vec(),
            ),
            bulge_loop(
                [(-0.1, 0.8), (0.1, 0.8), (0.1, 1.2), (-0.1, 1.2)]
                    .map(|(x, y)| (Point2::new(x, y), 0.0))
                    .to_vec(),
            ),
        ],
        0.0,
        1.0,
    );
    let bore_rod = ext("bore rod", vec![circle(0.0, 1.5, 0.5, 0.0)], 0.25, 0.5);
    let notch = {
        let f = |x: f64| x * (0.25 - x * x).sqrt() / 2.0 + 0.125 * (2.0 * x).asin();
        0.5 * (2.0 * f(0.1) - 0.06)
    };
    let (rod_v, block_v, overlap) = (0.125 * pi, 0.5 * 1.0 * 2.0, 0.5 * 0.5 * 1.0);
    let out_a = parts("plate + far block", &plate, &far_block);
    let out_b = parts("rod + face block", &rod, &face_block);
    let in_b = parts("bore rod + face block", &bore_rod, &face_block);
    let (va, vb) = (16.0 + 0.08, rod_v + block_v);
    let (va_in, vb_in) = (16.0 - 0.08, rod_v + block_v);
    let (vi, vi_in) = (notch + overlap, rod_v - notch + overlap);
    // (label, x, y, op, solids, volume, face–face records)
    let rows = {
        use topo::BooleanOp::{Intersect, Subtract, Union};
        [
            ("out A∪B", &out_a, &out_b, Union, 2, va + vb - vi, 1),
            ("out B∪A", &out_b, &out_a, Union, 2, va + vb - vi, 1),
            ("out A∖B", &out_a, &out_b, Subtract, 2, va - vi, 0),
            ("out B∖A", &out_b, &out_a, Subtract, 2, vb - vi, 0),
            ("out A∩B", &out_a, &out_b, Intersect, 2, vi, 0),
            ("out B∩A", &out_b, &out_a, Intersect, 2, vi, 0),
            (
                "in A∪B",
                &slotted,
                &in_b,
                Union,
                1,
                va_in + vb_in - vi_in,
                0,
            ),
            ("in B∖A", &in_b, &slotted, Subtract, 2, vb_in - vi_in, 0),
            ("in A∩B", &slotted, &in_b, Intersect, 2, vi_in, 0),
            ("in B∩A", &in_b, &slotted, Intersect, 2, vi_in, 0),
        ]
    };
    for (label, x, y, op, solids, want_v, ff) in rows {
        let got = match op {
            topo::BooleanOp::Union => topo::union(x, y, tol()),
            topo::BooleanOp::Subtract => topo::subtract(x, y, tol()),
            topo::BooleanOp::Intersect => topo::intersect(x, y, tol()),
        };
        let r = match got {
            Ok(BooleanResult::Body(r)) => r,
            other => panic!("{label}: {:?}", other.map(|_| ())),
        };
        assert_eq!(r.body.solids().count(), solids, "{label}: solids");
        let v = topo::mass_properties(&r.body, tol()).unwrap().volume;
        assert!(
            (v - want_v).abs() < tol().eps(),
            "{label}: volume {v}, want {want_v}"
        );
        assert_eq!(
            r.contacts.face_face().count(),
            ff,
            "{label}: face–face records"
        );
        for (p, q) in r.contacts.face_face() {
            let kind = |f: topo::FaceKey| {
                r.body
                    .get_surface(r.body.get_face(f).unwrap().surface)
                    .unwrap()
                    .kind()
            };
            let (plane, wall) = if kind(p) == SurfaceKind::Plane {
                (p, q)
            } else {
                (q, p)
            };
            let walls: Vec<topo::FaceKey> = r
                .body
                .faces()
                .filter(|(_, f)| {
                    r.body.get_surface(f.surface).unwrap().kind() == SurfaceKind::Cylinder
                })
                .map(|(k, _)| k)
                .collect();
            let on_rod = face_points(&r.body, wall)
                .iter()
                .all(|v| ((v[0]).hypot(v[1] - 2.5) - 0.5).abs() < 1e-9);
            let beside = topo::point_in_solid(
                &r.body,
                geom_core::Point3::new(0.0, 2.01, 0.5),
                geom_core::Band::linear(tol()).unwrap(),
                tol(),
            );
            assert!(
                walls == [wall] && on_rod && beside == Ok(topo::SolidContainment::In),
                "{label}: the record's cylinder face is the rod's one wall piece, which bounds \
                 the rod's material beside the line: walls {walls:?}, record's {wall:?}, \
                 on the rod {on_rod}, (0, 2.01, 0.5) {beside:?}"
            );
            let (lo, hi) = face_x_range(&r.body, plane);
            assert!(
                lo < 0.0 && hi < 1.0 + 1e-9,
                "{label}: the record's plane face is the piece of the plate's face \
                 holding x = 0, not the one past the face block: x in [{lo}, {hi}]"
            );
        }
    }
}

/// The plate and a rod of radius 0.5 along z ∈ [0.25, 0.75] beside its
/// y = 2 face, its axis at `(0, 2.5 + gap)`, united in `rod_first` order,
/// then bridged into one solid by a box over both tops, handing the
/// bridge the first union's face–face records when `carry`.
fn bridged_rod(gap: f64, rod_first: bool, carry: bool) -> topo::BooleanBody<f64> {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5 + gap, 0.5, 0.0)], 0.25, 0.5);
    let (x, y) = if rod_first {
        (&rod, &plate)
    } else {
        (&plate, &rod)
    };
    let u = match topo::union(x, y, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod, gap {gap}: {:?}", other.map(|_| ())),
    };
    assert_eq!(
        u.body.solids().count(),
        2,
        "plate ∪ rod, gap {gap}: two solids"
    );
    let bridge = finished(
        "bridge",
        brick::<f64>((-1.0, 1.0), (1.0, 2.5 + gap), (0.6, 1.6), tol()),
        tol(),
    );
    let records_a = if carry {
        topo::OperandRecords {
            ff: u.contacts.carried().ff,
            ..topo::OperandRecords::default()
        }
    } else {
        topo::OperandRecords::default()
    };
    match topo::boolean_op_with(
        topo::BooleanOp::Union,
        &u.body,
        &bridge,
        &topo::BooleanInputs {
            records_a,
            ..topo::BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(r.body.solids().count(), 1, "∪ bridge, gap {gap}: one solid");
            r
        }
        other => panic!("∪ bridge, gap {gap}: {:?}", other.map(|_| ())),
    }
}

/// The two faces a finding pairs, as their carriers' kinds, plane first.
fn pair_kinds(body: &Body<f64>, a: topo::FaceKey, b: topo::FaceKey) -> (SurfaceKind, SurfaceKind) {
    let kind = |f: topo::FaceKey| {
        body.get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .kind()
    };
    let (p, q) = (kind(a), kind(b));
    if p == SurfaceKind::Plane {
        (p, q)
    } else {
        (q, p)
    }
}

/// One solid pinched along a tangent line through two of its own faces'
/// interiors, the crescent void (the rod bridged onto the plate) and no
/// record handed in: the census discovers the touch and refuses it
/// `UnrecordedContact`, a face–face `Tangent` finding of the plate's
/// face and the rod's wall, in either operand order. Handed the first
/// union's record, the bridge lands it and 3′ passes.
#[test]
fn a_bridged_rods_unrecorded_tangent_line_refuses_and_its_carried_record_passes() {
    for rod_first in [false, true] {
        let label = format!("rod first {rod_first}");
        let r = bridged_rod(0.0, rod_first, false);
        assert_eq!(
            r.contacts.face_face().count(),
            0,
            "{label}: no face–face record"
        );
        let errors = topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).unwrap_err();
        let found: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                topo::ValidationError::UnrecordedContact {
                    contact: topo::CensusContact::FaceFaceTangent { finding },
                    ..
                } => Some(finding),
                _ => None,
            })
            .collect();
        assert_eq!(
            found.len(),
            1,
            "{label}: one tangent-line touch found: {errors:?}"
        );
        assert_eq!(
            pair_kinds(&r.body, found[0].pair.a, found[0].pair.b),
            (SurfaceKind::Plane, SurfaceKind::Cylinder),
            "{label}: the plate's face and the rod's wall"
        );
        assert_eq!(
            found[0].pair.class,
            topo::ContactClass::Tangent,
            "{label}: a Tangent finding"
        );
        let r = bridged_rod(0.0, rod_first, true);
        assert_eq!(
            topo::validate_pseudomanifold(&r.body, &r.contacts, tol()),
            Ok(()),
            "{label}, carried: 3′ confirms the landed record"
        );
    }
}

/// The same bridge with the rod standing off the plate's face by a few
/// escalation bands: the faces are near-tangent but apart, so there is
/// nothing to discover and 3′ passes with no record.
#[test]
fn a_bridged_rod_standing_off_the_plate_discovers_nothing() {
    let gap = 4.0 * Band::linear(tol()).unwrap().escalate();
    for rod_first in [false, true] {
        let r = bridged_rod(gap, rod_first, false);
        assert_eq!(
            topo::validate_pseudomanifold(&r.body, &r.contacts, tol()),
            Ok(()),
            "rod first {rod_first}: gap {gap}, nothing touches"
        );
    }
}

/// The plate minus a rod tangent to its y = 2 face from inside: one
/// solid pinched along x = 0 with its material in the crescent beside
/// the line, which owes two cusp edges (D1 tier 3). Read with no
/// record, the census reports the missing edges as the op's defect,
/// naming the plate's face and the bore's wall, and never a contact.
#[test]
fn a_bore_tangent_from_inside_with_no_edge_on_its_line_is_missing_its_cusp_edges() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 1.5, 0.5, 0.0)], 0.25, 0.5);
    let r = match topo::subtract(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("plate ∖ rod: {:?}", other.map(|_| ())),
    };
    assert_eq!(r.body.solids().count(), 1, "plate ∖ rod: one solid");
    let errors = topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol())
        .unwrap_err();
    let missing: Vec<_> = errors
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::CuspEdgesMissing { a, b, .. } => {
                Some(pair_kinds(&r.body, *a, *b))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        missing,
        [(SurfaceKind::Plane, SurfaceKind::Cylinder)],
        "plate ∖ rod: the plate's face and the bore's wall miss their cusp edges: {errors:?}"
    );
    assert!(
        !errors.iter().any(|e| matches!(
            e,
            topo::ValidationError::UnrecordedContact {
                contact: topo::CensusContact::FaceFaceTangent { .. },
                ..
            } | topo::ValidationError::ContactContradicted { .. }
        )),
        "plate ∖ rod: a material crescent is no contact: {errors:?}"
    );
}

// ---- Review probes for PR 5456 (rev3 lane): not for merge. ----

/// A summary of a census refusal list: each error's variant name, and
/// for the new arm's two findings, the pair's carrier kinds.
fn probe_summary(body: &Body<f64>, errors: &[topo::ValidationError]) -> Vec<String> {
    errors
        .iter()
        .map(|e| match e {
            topo::ValidationError::UnrecordedContact {
                contact: topo::CensusContact::FaceFaceTangent { finding },
                ..
            } => format!(
                "FFT{:?}{:?}",
                pair_kinds(body, finding.pair.a, finding.pair.b),
                finding.verdict
            ),
            topo::ValidationError::CuspEdgesMissing { a, b, .. } => {
                format!("CUSP{:?}", pair_kinds(body, *a, *b))
            }
            other => {
                let s = format!("{other:?}");
                s.chars().take(90).collect()
            }
        })
        .collect()
}

fn probe_validate(body: &Body<f64>, contacts: &topo::ContactRecords) -> Vec<String> {
    match topo::validate_pseudomanifold(body, contacts, tol()) {
        Ok(()) => vec![],
        Err(errors) => probe_summary(body, &errors),
    }
}

/// `bridged_rod` with the scale, the rod's vertex phase, and every
/// carried record kind (not only ff) when `carry_all`.
fn probe_bridged(
    s: f64,
    gap: f64,
    phase: f64,
    rod_first: bool,
    carry: u8,
    rod_z: (f64, f64),
) -> Result<topo::BooleanBody<f64>, String> {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0 * s, 2.0 * s), (-2.0 * s, 2.0 * s), (0.0, s), tol()),
        tol(),
    );
    let rod = ext(
        "rod",
        vec![circle(0.0, 2.5 * s + gap, 0.5 * s, phase)],
        rod_z.0 * s,
        (rod_z.1 - rod_z.0) * s,
    );
    let (x, y) = if rod_first {
        (&rod, &plate)
    } else {
        (&plate, &rod)
    };
    let u = match topo::union(x, y, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => return Err(format!("plate ∪ rod: {:?}", other.map(|_| ()))),
    };
    let bridge = finished(
        "bridge",
        brick::<f64>(
            (-1.0 * s, 1.0 * s),
            (1.0 * s, 2.5 * s + gap),
            (rod_z.1 * s - 0.15 * s, rod_z.1 * s + 0.85 * s),
            tol(),
        ),
        tol(),
    );
    let records_a = match carry {
        0 => topo::OperandRecords::default(),
        1 => topo::OperandRecords {
            ff: u.contacts.carried().ff,
            ..topo::OperandRecords::default()
        },
        _ => u.contacts.carried(),
    };
    match topo::boolean_op_with(
        topo::BooleanOp::Union,
        &u.body,
        &bridge,
        &topo::BooleanInputs {
            records_a,
            ..topo::BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(r)) => Ok(r),
        other => Err(format!("∪ bridge: {:?}", other.map(|_| ()))),
    }
}

fn probe_motions() -> Vec<(&'static str, Option<geom_core::RigidMotion<f64>>)> {
    let band = Band::linear(tol()).unwrap();
    let axis =
        geom_core::UnitVec3::new(geom_core::Vec3::new(1.0, 2.0, 3.0), "probe", band).unwrap();
    vec![
        ("identity", None),
        (
            "turned",
            Some(geom_core::RigidMotion::rotation(axis, 0.7).unwrap()),
        ),
        (
            "far",
            Some(
                geom_core::RigidMotion::rotation_then_translation(
                    axis,
                    0.7,
                    geom_core::Vec3::new(1.0e4, -3.0e3, 7.0e3),
                )
                .unwrap(),
            ),
        ),
    ]
}

fn probe_moved(
    body: &Body<f64>,
    m: &Option<geom_core::RigidMotion<f64>>,
) -> Result<Body<f64>, String> {
    match m {
        None => Ok(body.clone()),
        Some(m) => topo::transform_rigid(body, m, tol()).map_err(|e| format!("transform: {e:?}")),
    }
}

#[test]
fn probe_5456_void_pinch_poses_scales() {
    let mut out = Vec::new();
    for s in [1.0e-2, 1.0, 1.0e2] {
        for rod_first in [false, true] {
            for carry in [0u8, 1, 2] {
                let r = probe_bridged(s, 0.0, 0.0, rod_first, carry, (0.25, 0.75));
                let r = match r {
                    Ok(r) => r,
                    Err(e) => {
                        out.push(format!("s {s} rf {rod_first} carry {carry}: OP {e}"));
                        continue;
                    }
                };
                for (name, m) in probe_motions() {
                    let got = probe_moved(&r.body, &m).map(|b| probe_validate(&b, &r.contacts));
                    out.push(format!(
                        "s {s} rf {rod_first} carry {carry} {name}: {got:?}"
                    ));
                }
            }
        }
    }
    eprintln!("PROBE void poses\n{}", out.join("\n"));
}

#[test]
fn probe_5456_seam_on_the_line() {
    let mut out = Vec::new();
    let half_pi = core::f64::consts::FRAC_PI_2;
    // Rod at (0, 2.5), r 0.5: phase −π/2 puts a vertex, and the seam
    // ruling, at (0, 2): on the tangent line.
    for rod_first in [false, true] {
        for carry in [0u8, 1, 2] {
            let got = probe_bridged(1.0, 0.0, -half_pi, rod_first, carry, (0.25, 0.75)).map(|r| {
                (
                    r.contacts.face_face().count(),
                    probe_validate(&r.body, &r.contacts),
                )
            });
            out.push(format!("void seam rf {rod_first} carry {carry}: {got:?}"));
        }
    }
    // The bore: rod at (0, 1.5), r 0.5, phase π/2 puts the seam at (0, 2).
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    for (label, phase) in [("bore seam on line", half_pi), ("bore seam off", 0.0)] {
        let rod = ext("rod", vec![circle(0.0, 1.5, 0.5, phase)], 0.25, 0.5);
        match topo::subtract(&plate, &rod, tol()) {
            Ok(BooleanResult::Body(r)) => {
                out.push(format!(
                    "{label}: solids {} ff {} ef {}: own records {:?}; none {:?}",
                    r.body.solids().count(),
                    r.contacts.face_face().count(),
                    r.contacts.carried().ef.len(),
                    probe_validate(&r.body, &r.contacts),
                    probe_validate(&r.body, &topo::ContactRecords::default()),
                ));
            }
            other => out.push(format!("{label}: OP {:?}", other.map(|_| ()))),
        }
    }
    eprintln!("PROBE seam\n{}", out.join("\n"));
}

#[test]
fn probe_5456_bore_poses_scales() {
    let mut out = Vec::new();
    for s in [1.0e-2, 1.0, 1.0e2] {
        let plate = finished(
            "plate",
            brick::<f64>((-2.0 * s, 2.0 * s), (-2.0 * s, 2.0 * s), (0.0, s), tol()),
            tol(),
        );
        let rod = ext(
            "rod",
            vec![circle(0.0, 1.5 * s, 0.5 * s, 0.0)],
            0.25 * s,
            0.5 * s,
        );
        let r = match topo::subtract(&plate, &rod, tol()) {
            Ok(BooleanResult::Body(r)) => r,
            other => {
                out.push(format!("s {s}: OP {:?}", other.map(|_| ())));
                continue;
            }
        };
        for (name, m) in probe_motions() {
            let got = probe_moved(&r.body, &m).map(|b| {
                (
                    probe_validate(&b, &topo::ContactRecords::default()),
                    probe_validate(&b, &r.contacts),
                )
            });
            out.push(format!("s {s} {name}: (none, own) {got:?}"));
        }
    }
    eprintln!("PROBE bore poses\n{}", out.join("\n"));
}

#[test]
fn probe_5456_near_band_and_endpoint() {
    let mut out = Vec::new();
    let band = Band::linear(tol()).unwrap();
    for (label, gap) in [
        ("gap 0.5 esc", 0.5 * band.escalate()),
        ("gap 1.5 esc", 1.5 * band.escalate()),
        ("gap 0.5 eps", 0.5 * tol().eps()),
        ("gap 2 eps", 2.0 * tol().eps()),
        ("gap -0.5 esc", -0.5 * band.escalate()),
    ] {
        for carry in [0u8, 2] {
            let got = probe_bridged(1.0, gap, 0.0, false, carry, (0.25, 0.75)).map(|r| {
                (
                    r.contacts.face_face().count(),
                    probe_validate(&r.body, &r.contacts),
                )
            });
            out.push(format!("{label} carry {carry}: {got:?}"));
        }
    }
    // The rod's run along the line ends exactly where the plate's face
    // ends (z = 1): the line touches the face's closed region only at
    // its end, and the rod along z ∈ [1, 1.5] meets it at one point.
    for (label, z) in [("endpoint", (1.0, 1.5)), ("flush top", (0.5, 1.0))] {
        for carry in [0u8, 2] {
            let got = probe_bridged(1.0, 0.0, 0.0, false, carry, z).map(|r| {
                (
                    r.contacts.face_face().count(),
                    probe_validate(&r.body, &r.contacts),
                )
            });
            out.push(format!("{label} carry {carry}: {got:?}"));
        }
    }
    eprintln!("PROBE band/endpoint\n{}", out.join("\n"));
}

/// Two parallel rods touching externally along x = 0.5, y = 2.5,
/// bridged into one solid: a void cylinder × cylinder pinch.
#[test]
fn probe_5456_cylinder_cylinder() {
    let mut out = Vec::new();
    let half_pi = core::f64::consts::FRAC_PI_2;
    for (label, r2, cx2) in [("external", 0.5, 1.0), ("external unequal", 0.3, 0.8)] {
        let r1 = ext("rod1", vec![circle(0.0, 2.5, 0.5, half_pi)], 0.25, 0.5);
        let r2 = ext("rod2", vec![circle(cx2, 2.5, r2, half_pi)], 0.25, 0.5);
        let u = match topo::union(&r1, &r2, tol()) {
            Ok(BooleanResult::Body(u)) => u,
            other => {
                out.push(format!("{label}: rod ∪ rod {:?}", other.map(|_| ())));
                continue;
            }
        };
        out.push(format!(
            "{label}: rods solids {} ff {}",
            u.body.solids().count(),
            u.contacts.face_face().count()
        ));
        let bridge = finished(
            "bridge",
            brick::<f64>((-0.2, cx2 + 0.2), (2.3, 2.7), (0.6, 1.6), tol()),
            tol(),
        );
        for carry in [0u8, 2] {
            let records_a = if carry == 0 {
                topo::OperandRecords::default()
            } else {
                u.contacts.carried()
            };
            let got = match topo::boolean_op_with(
                topo::BooleanOp::Union,
                &u.body,
                &bridge,
                &topo::BooleanInputs {
                    records_a,
                    ..topo::BooleanInputs::default()
                },
                topo::SweepStrategy::Realized,
                tol(),
            ) {
                Ok(BooleanResult::Body(r)) => Ok((
                    r.body.solids().count(),
                    r.contacts.face_face().count(),
                    probe_validate(&r.body, &r.contacts),
                )),
                other => Err(format!("{:?}", other.map(|_| ()))),
            };
            out.push(format!("{label} carry {carry}: {got:?}"));
        }
    }
    eprintln!("PROBE cyl×cyl\n{}", out.join("\n"));
}

/// An independent oracle for the sign: a point in the crescent beside
/// the line, classified by `point_in_solid`, against the arm's finding.
#[test]
fn probe_5456_sign_oracle() {
    let band = Band::linear(tol()).unwrap();
    let mut out = Vec::new();
    // Void: the rod (centre y = 2.5) resting on the plate's y = 2 face.
    let r = probe_bridged(1.0, 0.0, 0.0, false, 0, (0.25, 0.75)).unwrap();
    // Material: the bore (centre y = 1.5) tangent from inside.
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 1.5, 0.5, 0.0)], 0.25, 0.5);
    let Ok(BooleanResult::Body(m)) = topo::subtract(&plate, &rod, tol()) else {
        panic!()
    };
    for (label, body, cy) in [("void", &r.body, 2.5), ("material", &m.body, 1.5)] {
        let x: f64 = 0.05;
        let wall_y = if cy > 2.0 {
            cy - (0.25 - x * x).sqrt()
        } else {
            cy + (0.25 - x * x).sqrt()
        };
        let mid = 0.5 * (2.0 + wall_y);
        let at = topo::point_in_solid(body, geom_core::Point3::new(x, mid, 0.4), band, tol());
        let found = probe_validate(body, &topo::ContactRecords::default());
        out.push(format!(
            "{label}: crescent point y {mid}: {at:?}; arm {found:?}"
        ));
    }
    eprintln!("PROBE oracle\n{}", out.join("\n"));
}

/// Cost probe: a plate extruded with an n × n grid of round holes, each
/// hole's walls a band or two from its neighbours' boxes, timed through
/// the full validate.
#[test]
fn probe_5456_cost_grid_of_holes() {
    let mut out = Vec::new();
    for n in [4usize, 8, 16] {
        let pitch = 1.0;
        let r = 0.45;
        let side = pitch * n as f64;
        let mut loops = vec![bulge_loop(
            [(0.0, 0.0), (side, 0.0), (side, side), (0.0, side)]
                .map(|(x, y)| (Point2::new(x, y), 0.0))
                .to_vec(),
        )];
        for i in 0..n {
            for j in 0..n {
                loops.push(circle(
                    pitch * (i as f64 + 0.5),
                    pitch * (j as f64 + 0.5),
                    r,
                    0.3,
                ));
            }
        }
        let body = ext("grid", loops, 0.0, 1.0).into_body();
        let t = std::time::Instant::now();
        let got = topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol());
        let dt = t.elapsed();
        out.push(format!(
            "n {n}: faces {} validate {:?} in {:?}",
            body.faces().count(),
            got.map_err(|e| e.len()),
            dt
        ));
    }
    eprintln!("PROBE cost\n{}", out.join("\n"));
}

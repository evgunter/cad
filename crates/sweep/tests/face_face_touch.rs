//! A solid touching another, or itself, along a tangent line through
//! two faces' interiors: a face–face record of the two faces, with no
//! edge on the line (`crates/topo/README.md` C7).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Band, Point2, Tol};
use geom_core::{RigidMotion, Turns, UnitVec3};
use profile::SketchPlane;
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
    let v = topo::test_support::pinned_volume("the measured body", &u.body, tol()).volume;
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
            let v = topo::test_support::pinned_volume("the measured body", &r.body, tol()).volume;
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

/// The n-ary union of `bodies`, ranked in list order: its judgements
/// and its steps.
fn nary_union(what: &str, bodies: &[&AtRestBody<f64>]) -> topo::NaryRun<f64> {
    let inputs: Vec<topo::MemberInputs> = (0..bodies.len())
        .map(|rank| topo::MemberInputs {
            rank: u32::try_from(rank).unwrap(),
            ..topo::MemberInputs::default()
        })
        .collect();
    let members: Vec<topo::NaryMember<'_, f64>> = bodies
        .iter()
        .zip(&inputs)
        .map(|(&body, inputs)| topo::NaryMember { body, inputs })
        .collect();
    let run = topo::nary_boolean(
        topo::NaryOp::Union,
        &members,
        topo::SweepStrategy::Realized,
        tol(),
    );
    assert!(
        matches!(run.end, topo::NaryEnd::Built),
        "{what}: {:?}",
        run.end
    );
    run
}

/// **A step handed its tangency records the touch as the op that
/// decided it does**: the rod and the plate, beside a block far from
/// both, united by the n-ary door; the step joining the rod is given
/// the pair's tangency and carries the judgement's face–face record,
/// citing a row that restates the given decision on its margin.
#[test]
fn a_given_tangency_through_both_interiors_records_the_face_face_touch() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let far = finished(
        "far",
        brick::<f64>((10.0, 11.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let built = nary_union("plate ∪ rod ∪ far", &[&plate, &rod, &far]);
    let [judged] = &built.judgements[..] else {
        panic!("plate ∪ rod ∪ far: one judgement, the plate and the rod");
    };
    assert_eq!(
        judged.body.verdicts.len(),
        1,
        "plate ∪ rod: one verdict, the tangency"
    );
    let at = judged
        .body
        .coincidences
        .iter()
        .position(|row| row.relation == topo::Relation::SurfaceTangent)
        .unwrap();
    let decision = judged.body.coincidences[at];
    let stepped = &built.steps[0].body;
    assert_eq!(
        stepped.contacts.face_face().collect::<Vec<_>>(),
        judged.body.contacts.face_face().collect::<Vec<_>>(),
        "plate ∪ rod, given: the face–face record the judged union carries"
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
    assert_eq!(
        built.steps[0].restates(k as usize),
        Some(
            &[topo::JudgedRow {
                judgement: 0,
                row: at
            }][..]
        ),
        "plate ∪ rod, given: the door says the row stands for the judged tangency"
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
        brick::<f64>((-0.1, 0.1), (1.4, 1.6), (0.4, 0.6), tol()),
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
        let plane = SketchPlane::from_frame(RigidMotion::turned(
            geom_core::Point3::new(0.0, y1, 0.0),
            UnitVec3::exact_x(),
            Turns::quarters(1),
        ));
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
/// is no touch**: a block standing across the plate's y = 2 face at
/// x ∈ [1, 1.5] divides it, and the rod tangent to that face along
/// x = 0 is given to both pieces (each descends from the judged face);
/// the line runs through the left piece only, so the right piece's
/// pair places it in neither face's span and records nothing.
#[test]
fn a_given_tangency_whose_faces_part_along_its_line_records_nothing() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let block = finished(
        "block",
        brick::<f64>((1.0, 1.5), (1.5, 2.5), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let built = nary_union("plate ∪ block ∪ rod", &[&plate, &block, &rod]);
    let judged = &built.judgements[1];
    assert_eq!(
        (judged.a, judged.b, judged.body.verdicts.len()),
        (0, 2, 1),
        "plate ∪ rod: one verdict, the tangency"
    );
    let side = judged.body.verdicts[0].a();
    let step = &built.steps[1];
    let acc = &built.steps[0].body.body;
    let held: Vec<topo::FaceKey> = acc
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            built.steps[0]
                .lineage
                .member_faces(f)
                .is_some_and(|m| m.contains(&(0, side)))
        })
        .collect();
    let ranges: Vec<(f64, f64)> = held.iter().map(|&f| face_x_range(acc, f)).collect();
    assert!(
        ranges.len() == 2 && ranges.contains(&(-2.0, 1.0)) && ranges.contains(&(1.5, 2.0)),
        "the block divides the plate's y = 2 face into two pieces, each given the tangency: \
         {ranges:?}"
    );
    let tangent: Vec<&topo::Coincidence> = step
        .body
        .coincidences
        .iter()
        .filter(|row| row.relation == topo::Relation::SurfaceTangent)
        .collect();
    let [row] = tangent[..] else {
        panic!("one restated tangency, the left piece's: {tangent:?}");
    };
    let (f, _) = row.face_pair().unwrap();
    assert_eq!(
        face_x_range(acc, f),
        (-2.0, 1.0),
        "the restated tangency is the left piece's; the right piece's records nothing"
    );
    assert_eq!(
        step.body.contacts.face_face().count(),
        1,
        "plate ∪ block ∪ rod: one face–face record, the left piece's"
    );
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
        let v = topo::test_support::pinned_volume("the measured body", &r.body, tol()).volume;
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
    bridged_rod_posed(1.0, 0.0, gap, rod_first, carry)
}

/// [`bridged_rod`] scaled by `s` about the origin and moved by `d` along
/// each axis.
fn bridged_rod_posed(
    s: f64,
    d: f64,
    gap: f64,
    rod_first: bool,
    carry: bool,
) -> topo::BooleanBody<f64> {
    let at = |lo: f64, hi: f64| (s * lo + d, s * hi + d);
    let plate = finished(
        "plate",
        brick::<f64>(at(-2.0, 2.0), at(-2.0, 2.0), at(0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext(
        "rod",
        vec![circle(d, s * 2.5 + gap + d, s * 0.5, 0.0)],
        s * 0.25 + d,
        s * 0.5,
    );
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
        brick::<f64>(
            at(-1.0, 1.0),
            (s + d, s * 2.5 + gap + d),
            at(0.6, 1.6),
            tol(),
        ),
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
            topo::GlueClass::Tangent,
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

/// Whether `errors` holds a face–face tangent-line touch.
fn found_a_line_touch(errors: &[topo::ValidationError]) -> bool {
    errors.iter().any(|e| {
        matches!(
            e,
            topo::ValidationError::UnrecordedContact {
                contact: topo::CensusContact::FaceFaceTangent { .. },
                ..
            }
        )
    })
}

/// The rod resting on the plate before any bridge: two solids touching
/// along the line, read with no record. The touch is between solids, so
/// the same-solid pass says nothing of it, whatever else refuses. Red
/// when the pass drops its one-solid test. Both operand orders.
#[test]
fn a_rod_on_a_plate_as_two_solids_is_not_a_same_solid_touch() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    for (x, y) in [(&plate, &rod), (&rod, &plate)] {
        let u = match topo::union(x, y, tol()) {
            Ok(BooleanResult::Body(u)) => u,
            other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
        };
        assert_eq!(u.body.solids().count(), 2, "plate ∪ rod: two solids");
        let errors =
            topo::validate_pseudomanifold(&u.body, &topo::ContactRecords::default(), tol())
                .err()
                .unwrap_or_default();
        assert!(
            !found_a_line_touch(&errors),
            "plate ∪ rod, no record: no same-solid finding: {errors:?}"
        );
    }
}

/// The plate with its internally tangent bore, and a small cube floating
/// in the bore as a second solid, inside the plate's reach: the bore's
/// missing cusp edges are the plate's defect alone, and the cube's
/// clearance from the plate is still decided. Red when the missing
/// edges leave every solid pair unexamined.
#[test]
fn a_solid_missing_its_cusp_edges_leaves_a_solid_in_its_bore_cleared() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 1.5, 0.5, 0.0)], 0.25, 0.5);
    let bored = match topo::subtract(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("plate ∖ rod: {:?}", other.map(|_| ())),
    };
    let cube = finished(
        "cube",
        brick::<f64>((-0.1, 0.1), (1.4, 1.6), (0.4, 0.6), tol()),
        tol(),
    );
    let r = match topo::union(&bored, &cube, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("bored ∪ cube: {:?}", other.map(|_| ())),
    };
    assert_eq!(r.body.solids().count(), 2, "bored ∪ cube: two solids");
    let errors = topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol())
        .unwrap_err();
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, topo::ValidationError::CuspEdgesMissing { .. })),
        "bored ∪ cube: only the plate's missing cusp edges: {errors:?}"
    );
}

/// The bridged rod and the bore at other scales and far from the origin
/// (adopted from PR 5456's review probes): the void pinch is the one
/// unrecorded touch and the material one the missing cusp edges, at
/// each pose. Other arms may escalate far off the origin at a fine ε;
/// this row reads only what the same-solid pass says.
#[test]
fn the_void_and_material_pinches_read_alike_at_every_pose() {
    for (s, d) in [(1.0e-2, 0.0), (1.0e2, 0.0), (1.0, 1.0e3)] {
        let r = bridged_rod_posed(s, d, 0.0, false, false);
        let errors = topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).unwrap_err();
        let touches = errors
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    topo::ValidationError::UnrecordedContact {
                        contact: topo::CensusContact::FaceFaceTangent { .. },
                        ..
                    }
                )
            })
            .count();
        assert_eq!(touches, 1, "bridged rod, scale {s}, offset {d}: {errors:?}");
        let at = |lo: f64, hi: f64| (s * lo + d, s * hi + d);
        let plate = finished(
            "plate",
            brick::<f64>(at(-2.0, 2.0), at(-2.0, 2.0), at(0.0, 1.0), tol()),
            tol(),
        );
        let rod = ext(
            "rod",
            vec![circle(d, s * 1.5 + d, s * 0.5, 0.0)],
            s * 0.25 + d,
            s * 0.5,
        );
        let r = match topo::subtract(&plate, &rod, tol()) {
            Ok(BooleanResult::Body(r)) => r,
            other => panic!(
                "plate ∖ rod, scale {s}, offset {d}: {:?}",
                other.map(|_| ())
            ),
        };
        let errors =
            topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default(), tol())
                .unwrap_err();
        let missing = errors
            .iter()
            .filter(|e| matches!(e, topo::ValidationError::CuspEdgesMissing { .. }))
            .count();
        assert!(
            missing == 1 && !found_a_line_touch(&errors),
            "plate ∖ rod, scale {s}, offset {d}: the missing cusp edges and no touch: {errors:?}"
        );
    }
}

/// **A touch on the tangent line outside its overlap is its own pair.**
/// The plate's `y = 2` face is a C: a notch `x < 1`, `z ∈ (0.4, 0.6)`
/// cut into it. The rod lies along the face over `z ∈ [0.25, 0.6]`, so
/// the line contact runs `z ∈ [0.25, 0.4]` and the rod's top circle
/// rests on the notch's upper edge at `(0, 2, 0.6)`, where the line
/// leaves the plate downward and the rod upward: outside the closure of
/// the contact. With the face–face record alone, that touch is the one
/// thing reported, as an edge–edge touch of its own. Red if the line
/// arm lets the pair answer for it.
#[test]
fn a_rod_touching_a_notch_edge_off_its_tangent_run_is_its_own_pair() {
    let unite = |a: &AtRestBody<f64>, b: &AtRestBody<f64>| match topo::union(a, b, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("union: {:?}", other.map(|_| ())),
    };
    let brick_at = |x, z, what| finished(what, brick::<f64>(x, (-2.0, 2.0), z, tol()), tol());
    let low = brick_at((-2.0, 2.0), (0.0, 0.4), "low");
    let high = brick_at((-2.0, 2.0), (0.6, 1.0), "high");
    let web = brick_at((1.0, 2.0), (0.4, 0.6), "web");
    let c = unite(&low, &web);
    let c = finished("c", c.body.into_body(), tol());
    let plate = unite(&c, &high);
    let plate = finished("plate", plate.body.into_body(), tol());
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.35);
    let mut body = plate.into_body();
    topo::graft_disjoint_all_keyed(&mut body, &rod.into_body()).unwrap();
    let kind = |f: topo::FaceKey| {
        let face = body.get_face(f).unwrap();
        body.get_surface(face.surface).unwrap().kind()
    };
    let side = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            kind(f) == SurfaceKind::Plane && face_points(&body, f).iter().all(|p| p[1] == 2.0)
        })
        .expect("the plate's y = 2 face");
    let record = |wall| {
        topo::ContactRecords::from_iter(vec![topo::Cited::new(
            topo::Contact::new(&body, topo::Cell::Face(wall), topo::Cell::Face(side)).unwrap(),
            topo::Cites::decided(0),
        )])
    };
    let read = |records: &topo::ContactRecords| {
        topo::validate_pseudomanifold(&body, records, tol())
            .err()
            .unwrap_or_default()
    };
    // The rod's wall is two half-cylinders; the one on the plate is the
    // one whose record confirms.
    let errors = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| kind(f) == SurfaceKind::Cylinder)
        .map(|wall| read(&record(wall)))
        .find(|errors| {
            !errors
                .iter()
                .any(|e| matches!(e, topo::ValidationError::StaleContactDeclaration { .. }))
        })
        .expect("the wall half on the plate confirms");
    let found: Vec<_> = errors
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::UnrecordedContact {
                contact, witness, ..
            } => Some((*contact, witness.clone())),
            _ => None,
        })
        .collect();
    assert!(
        found.iter().any(
            |(c, w)| matches!(c, topo::CensusContact::EdgeEdgeCross { .. }) && w.contains("0.6")
        ),
        "the notch touch is its own pair: {errors:?}"
    );
    assert!(
        found.iter().all(|(_, w)| w.contains("0.6")),
        "and the only one: the run's own touches are the pair's: {errors:?}"
    );
}

/// A rod along +x from `x0`, of length `len`, radius 0.5, its axis at
/// y = 2.5 and height `z`: tangent to the plane y = 2 along that height.
fn rod_along_x(x0: f64, len: f64, z: f64) -> AtRestBody<f64> {
    // A quarter turn about y maps the sketch's (u, v) to (0, v, -u) and
    // its normal to +x.
    let plane = SketchPlane::from_frame(RigidMotion::turned(
        geom_core::Point3::new(x0, 0.0, 0.0),
        UnitVec3::exact_y(),
        Turns::quarters(1),
    ));
    let body = sweep::test_support::extruded(plane, vec![circle(-z, 2.5, 0.5, 0.0)], len, tol());
    finished("rod", body, tol())
}

/// **CONTACTHOLD's open m2, pinned as it stands**
/// (`work/contacthold/a-fold-steps-restated-tangency-takes-one-of-several-verdicts-margins.md`):
/// two plates overlapping in z whose y = 2 faces are coplanar (the
/// second raised ε/100 off it, inside the band) and merge, and a rod
/// along x tangent to the merged face along z = 0.75, through both
/// faces' interiors. The pass decides the rod's tangency against each
/// plate on its own margin; the step joining the rod is handed one
/// given pair for (merged face, rod wall), under the first verdict, so
/// its restated row carries that verdict's margin alone while it stands
/// for both judged rows. Fixing m2 flips the margin assertion.
#[test]
fn contacthold_m2_a_restated_tangency_carries_the_first_of_two_verdicts_margins() {
    let p1 = finished(
        "p1",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let p2 = finished(
        "p2",
        brick::<f64>(
            (-2.0, 2.0),
            (-2.0, 2.0 + 0.01 * tol().eps()),
            (0.5, 1.5),
            tol(),
        ),
        tol(),
    );
    let rod = rod_along_x(-1.0, 2.0, 0.75);
    let run = nary_union("p1 ∪ p2 ∪ rod", &[&p1, &p2, &rod]);
    let tangency = |j: &topo::NaryJudgement<f64>| {
        let row = j
            .body
            .coincidences
            .iter()
            .position(|r| r.relation == topo::Relation::SurfaceTangent)
            .unwrap_or_else(|| panic!("judgement ({}, {}) decides a tangency", j.a, j.b));
        (row, j.body.coincidences[row].margin)
    };
    let judged: Vec<(usize, usize, geom_core::MarginDiag)> = run
        .judgements
        .iter()
        .enumerate()
        .filter(|(_, j)| j.b == 2)
        .map(|(k, j)| {
            let (row, margin) = tangency(j);
            (k, row, margin)
        })
        .collect();
    let [(j0, r0, m0), (j1, r1, m1)] = judged[..] else {
        panic!("two judgements decide the rod's tangency: {judged:?}");
    };
    assert!(
        m0 != m1,
        "the two verdicts are decided on two margins: {m0:?} and {m1:?}"
    );
    let step = &run.steps[1];
    let restated: Vec<(usize, &[topo::JudgedRow])> = (0..step.body.coincidences.len())
        .filter_map(|k| step.restates(k).map(|r| (k, r)))
        .collect();
    let [(k, rows)] = restated[..] else {
        panic!("the step joining the rod restates one row: {restated:?}");
    };
    assert_eq!(
        rows,
        [
            topo::JudgedRow {
                judgement: j0,
                row: r0
            },
            topo::JudgedRow {
                judgement: j1,
                row: r1
            }
        ],
        "the restated row stands for both judged tangencies"
    );
    assert_eq!(
        step.body.coincidences[k].margin, m0,
        "m2 (open): the restated row carries the first verdict's margin alone"
    );
}

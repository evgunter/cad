//! What a gate does with a record of a kind no census lane reads
//! (`topo::ContactReach::NoLane`): PR 7124's second fix pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{
    Assembled, AtRestBody, Body, Built, Cell, Cited, Cites, Contact, ContactRecords, EdgeKey,
    FaceKey, NotFinished,
};

fn tol() -> Tol {
    Tol::witness()
}

fn rod(cx: f64, cy: f64, r: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    let circle: ProfileLoop<f64> =
        bulge_loop(vec![(at(0.0), 1.0), (at(core::f64::consts::PI), 1.0)]);
    let profile = Profile::new(sketch_at(z0), vec![circle])
        .validate(tol())
        .unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
    .into_body();
    finished("rod", body, tol())
}

/// A prism on the triangle `(1.5, 0), (1.5, 1.5), (0, 1.5)`, from
/// `z0` up by one: its slanted side is the plane `x + y = 1.5`, and
/// that side's box is the square `[0, 1.5]²`.
fn wedge(z0: f64) -> AtRestBody<f64> {
    let corner = |x, y| (Point2::new(x, y), 0.0);
    let body: Body<f64> = sweep::test_support::prism_at(
        vec![corner(1.5, 0.0), corner(1.5, 1.5), corner(0.0, 1.5)],
        z0,
        1.0,
        tol(),
    );
    finished("wedge", body, tol())
}

/// A body's faces and edges.
type Cells = (Vec<FaceKey>, Vec<EdgeKey>);

/// Two bodies joined, and each one's faces and edges in the joined
/// body's keys.
fn joined(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> (Body<f64>, [Cells; 2]) {
    let mut join = Assembled::<f64>::new();
    let ka = join.graft_finished(a, None, tol()).unwrap();
    let kb = join.graft_finished(b, None, tol()).unwrap();
    let done = topo::finish_assembled(join, tol()).unwrap();
    let body = topo::FinishedBody::into_body(done.body);
    let cells = |src: &AtRestBody<f64>, k: &topo::GraftKeys| {
        (
            src.faces().filter_map(|(f, _)| k.face(f)).collect(),
            src.edges().filter_map(|(e, _)| k.edge(e)).collect(),
        )
    };
    (body.clone(), [cells(a, &ka), cells(b, &kb)])
}

fn kind(body: &Body<f64>, f: FaceKey) -> geom::SurfaceKind {
    body.face_surface(f).map(geom::Surface::kind).unwrap()
}

/// The gate's answer to `body` holding the one record `x`–`y`.
fn handed(body: &Body<f64>, x: Cell, y: Cell) -> Result<AtRestBody<f64>, NotFinished> {
    let mut records = ContactRecords::default();
    records.extend([Cited::new(
        Contact::new(body, x, y).unwrap(),
        Cites::decided(0),
    )]);
    topo::finish(Built::of(body.clone()).recording(records), tol())
}

fn names(r: &Result<AtRestBody<f64>, NotFinished>) -> String {
    match r {
        Ok(done) => format!("HELD {:?}", done.bar()),
        Err(e) => format!(
            "refused {:?}",
            e.as_slice()
                .iter()
                .map(|f| format!("{f:?}").chars().take(160).collect::<String>())
                .collect::<Vec<_>>()
        ),
    }
}

fn wall(body: &Body<f64>, faces: &[FaceKey]) -> FaceKey {
    *faces
        .iter()
        .find(|&&f| kind(body, f) == geom::SurfaceKind::Cylinder)
        .expect("a rod has a wall")
}

fn stale(r: &Result<AtRestBody<f64>, NotFinished>) -> bool {
    matches!(r, Err(e) if matches!(
        e.as_slice(),
        [.., topo::ValidationError::StaleContactDeclaration { .. }]
    ))
}

fn contradicted(r: &Result<AtRestBody<f64>, NotFinished>) -> bool {
    matches!(r, Err(e) if matches!(
        e.as_slice(),
        [topo::ValidationError::ContactContradicted { .. }]
    ))
}

/// A rod's wall and each face of a wedge standing 0.56 off it, their
/// world boxes overlapping, recorded as touching. The two caps cross
/// the rod's axis, a pair the tangent certifier has no locus for: it
/// answers "not mine" having read no distance, and the gate reads the
/// two faces apart along the wedge's slanted normal and refuses the
/// record stale. The three sides are parallel to the axis, where the
/// certifier reads the carriers' gap itself and contradicts the
/// record. Red on a gate that holds what a lane declines: the caps'
/// records were held at tier 3.
#[test]
fn a_wall_and_a_plane_that_do_not_touch_refuse_whatever_the_planes_tilt() {
    let (body, [(rod_faces, _), (wedge_faces, _)]) =
        joined(&rod(0.0, 0.0, 0.5, 0.0, 1.0), &wedge(0.0));
    let wall = wall(&body, &rod_faces);
    let (mut caps, mut sides) = (0, 0);
    for &f in &wedge_faces {
        let got = handed(&body, Cell::Face(wall), Cell::Face(f));
        if stale(&got) {
            caps += 1;
        } else if contradicted(&got) {
            sides += 1;
        } else {
            panic!("the wall and wedge face {f:?}: {}", names(&got));
        }
    }
    assert_eq!(
        (caps, sides),
        (2, 3),
        "two caps read apart, three sides contradicted"
    );
}

/// Two parallel rods 0.27 apart, their world boxes overlapping, the
/// walls recorded as touching: the certifier reads the gap between the
/// axes and contradicts the record.
#[test]
fn parallel_walls_that_do_not_touch_refuse() {
    let (body, [(a, _), (b, _)]) =
        joined(&rod(0.0, 0.0, 0.5, 0.0, 1.0), &rod(0.9, 0.9, 0.5, 0.0, 1.0));
    let got = handed(
        &body,
        Cell::Face(wall(&body, &a)),
        Cell::Face(wall(&body, &b)),
    );
    assert!(contradicted(&got), "{}", names(&got));
}

/// A rod's foot circle on the carrier plane of a wedge's top, beside
/// the top, their world boxes overlapping, recorded as touching: no
/// lane is built along an edge's carrier, and the gate reads the two
/// apart along the axis between them and refuses. Red on a gate that
/// holds what a lane declines.
#[test]
fn an_edge_on_a_faces_carrier_beside_the_face_refuses() {
    let (body, [(_, rod_edges), (wedge_faces, _)]) =
        joined(&rod(0.3, 0.3, 0.25, 0.0, 1.0), &wedge(-1.0));
    let mut read = 0;
    for &e in &rod_edges {
        for &f in &wedge_faces {
            let got = handed(&body, Cell::Edge(e), Cell::Face(f));
            assert!(got.is_err(), "edge {e:?} and face {f:?}: {}", names(&got));
            read += 1;
        }
    }
    assert!(read > 0, "pairs were read");
}

/// A true tangency of each shape the certifier holds confirms, and the
/// body names tier 3′: a rod against a block's side along a ruling,
/// and two rods side by side along a generator. Each touches along
/// `(0, 0.5, z)`, inside the walls: a rod's wall is two faces meeting
/// at `y = 0`, and a touch along that seam is the seam edge's record,
/// not the faces'.
#[test]
fn a_true_tangency_of_each_certified_shape_confirms() {
    let block = finished(
        "block",
        sweep::test_support::brick::<f64>((-1.0, 1.0), (0.5, 1.5), (0.0, 1.0), tol()),
        tol(),
    );
    let (body, [(a, _), (b, _)]) = joined(&rod(0.0, 0.0, 0.5, 0.0, 1.0), &block);
    let confirmed = b
        .iter()
        .filter(|&&f| {
            handed(&body, Cell::Face(wall(&body, &a)), Cell::Face(f))
                .is_ok_and(|done| done.bar() == topo::Bar::Tier3Prime)
        })
        .count();
    assert_eq!(
        confirmed, 1,
        "the block's side at y = 0.5, and no other face"
    );
    let (body, [(a, _), (b, _)]) =
        joined(&rod(0.0, 0.0, 0.5, 0.0, 1.0), &rod(0.0, 1.0, 0.5, 0.0, 1.0));
    let walls = |faces: &[FaceKey]| -> Vec<FaceKey> {
        faces
            .iter()
            .copied()
            .filter(|&f| kind(&body, f) == geom::SurfaceKind::Cylinder)
            .collect()
    };
    let confirmed = walls(&a)
        .into_iter()
        .flat_map(|x| walls(&b).into_iter().map(move |y| (x, y)))
        .filter(|&(x, y)| {
            handed(&body, Cell::Face(x), Cell::Face(y))
                .is_ok_and(|done| done.bar() == topo::Bar::Tier3Prime)
        })
        .count();
    assert_eq!(
        confirmed, 1,
        "the two half walls that face each other, and no other pair"
    );
}

/// An L-shaped prism: the square `[0, 2]²` less its corner `[1, 2]²`,
/// from `z = -1` up to `z = 0`.
fn ell() -> AtRestBody<f64> {
    let corner = |x, y| (Point2::new(x, y), 0.0);
    let body: Body<f64> = sweep::test_support::prism_at(
        vec![
            corner(0.0, 0.0),
            corner(2.0, 0.0),
            corner(2.0, 1.0),
            corner(1.0, 1.0),
            corner(1.0, 2.0),
            corner(0.0, 2.0),
        ],
        -1.0,
        1.0,
        tol(),
    );
    finished("ell", body, tol())
}

/// **The residual, demonstrated: a false record that is still held.**
/// A rod stands in the notch of an L-shaped prism, its foot circle on
/// the carrier plane of the prism's top and 0.25 clear of the top
/// itself. The foot's edge and the top, recorded as touching: no lane
/// reads an edge along a face's carrier, and no separating direction
/// the gate reads parts a circle from the L around it, so the record
/// is held on its citation at tier 3. The record is false.
///
/// This row asserts the hold. It goes red when the census gains the
/// lane, which is what
/// `work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`
/// owes; the assertion then turns to a refusal.
#[test]
fn a_false_record_no_lane_reads_and_no_axis_parts_is_still_held() {
    let (body, [(_, rod_edges), (ell_faces, _)]) = joined(&rod(1.5, 1.5, 0.25, 0.0, 1.0), &ell());
    let held: Vec<topo::Bar> = rod_edges
        .iter()
        .flat_map(|&e| ell_faces.iter().map(move |&f| (e, f)))
        .filter_map(|(e, f)| handed(&body, Cell::Edge(e), Cell::Face(f)).ok())
        .map(|done| done.bar())
        .collect();
    assert!(!held.is_empty(), "a false record is held");
    assert!(
        held.iter().all(|&bar| bar == topo::Bar::Tier3),
        "and never at tier 3': {held:?}"
    );
}

/// **The residual's second shape, also still held.** A rod against a
/// block's side touches it along a ruling. The rod's wall and either
/// cap of the block meet only at that ruling's end, a point on both
/// boundaries, so a record of the wall touching a cap is false. The
/// cap crosses the rod's axis, a pair the tangent locus has no line
/// for, and no direction parts two cells that share a boundary point:
/// held on its citation at tier 3.
///
/// As the row above, this asserts the hold and goes red when
/// `work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`
/// is done.
#[test]
fn a_wall_against_the_cap_of_a_block_it_touches_along_a_ruling_is_still_held() {
    let block = finished(
        "block",
        sweep::test_support::brick::<f64>((-1.0, 1.0), (0.5, 1.5), (0.0, 1.0), tol()),
        tol(),
    );
    let (body, [(rod_faces, _), (block_faces, _)]) = joined(&rod(0.0, 0.0, 0.5, 0.0, 1.0), &block);
    let held: Vec<topo::Bar> = rod_faces
        .iter()
        .filter(|&&f| kind(&body, f) == geom::SurfaceKind::Cylinder)
        .flat_map(|&x| block_faces.iter().map(move |&y| (x, y)))
        .filter_map(|(x, y)| handed(&body, Cell::Face(x), Cell::Face(y)).ok())
        .map(|done| done.bar())
        .filter(|&bar| bar == topo::Bar::Tier3)
        .collect();
    assert!(
        !held.is_empty(),
        "a false record of a wall on a cap is held"
    );
}

/// **The residual is wider than a notch: nesting defeats every axis.**
/// A tube, and a block wholly inside its bore, 0.3 clear all round. A
/// cap of the block is a plane across the tube's axis, a pair the
/// tangent locus has no line for, and it lies inside the footprint of
/// each of the tube's walls, so no axis the gate reads parts them (the
/// one that would is the cylinder's radial direction, which is not
/// among them). A record of a wall touching a cap is false and held at
/// tier 3: each of the tube's walls against each of the block's two
/// caps. A pin in a bore is the commonest assembly there is.
///
/// Asserts the hold; red when
/// `work/heldrecords/a-recorded-tangency-past-the-certifiers-set-is-held-on-its-citation.md`
/// lands a lane or an axis.
#[test]
fn a_cap_nested_in_a_bore_is_still_held_against_every_wall() {
    let topo::BooleanResult::Body(tube) = topo::subtract(
        &rod(0.0, 0.0, 1.0, 0.0, 1.0),
        &rod(0.0, 0.0, 0.5, -0.5, 2.0),
        tol(),
    )
    .unwrap() else {
        panic!("a rod less its core is a tube")
    };
    let inner = finished(
        "inner",
        sweep::test_support::brick::<f64>((-0.2, 0.2), (-0.2, 0.2), (0.4, 0.6), tol()),
        tol(),
    );
    let (body, [(tube_faces, _), (inner_faces, _)]) = joined(&tube.body, &inner);
    let walls: Vec<FaceKey> = tube_faces
        .iter()
        .copied()
        .filter(|&f| kind(&body, f) == geom::SurfaceKind::Cylinder)
        .collect();
    let held = walls
        .iter()
        .flat_map(|&w| inner_faces.iter().map(move |&f| (w, f)))
        .filter(|&(w, f)| {
            handed(&body, Cell::Face(w), Cell::Face(f))
                .is_ok_and(|done| done.bar() == topo::Bar::Tier3)
        })
        .count();
    assert!(!walls.is_empty(), "the tube has walls");
    assert_eq!(
        held,
        walls.len() * 2,
        "each wall against each of the block's two caps"
    );
}

/// **The residual's wall in a notch.** The rod standing in the notch of
/// the L-shaped prism again, its foot on the carrier plane of the
/// prism's top: the rod's wall and that top, a plane across the wall's
/// axis that the L wraps round the rod. A record of the wall touching
/// the top is false and held at tier 3.
///
/// Asserts the hold; red with the row above.
#[test]
fn a_wall_in_the_notch_of_a_plane_across_its_axis_is_still_held() {
    let (body, [(rod_faces, _), (ell_faces, _)]) = joined(&rod(1.5, 1.5, 0.25, 0.0, 1.0), &ell());
    let held = rod_faces
        .iter()
        .filter(|&&f| kind(&body, f) == geom::SurfaceKind::Cylinder)
        .flat_map(|&w| ell_faces.iter().map(move |&f| (w, f)))
        .filter(|&(w, f)| {
            handed(&body, Cell::Face(w), Cell::Face(f))
                .is_ok_and(|done| done.bar() == topo::Bar::Tier3)
        })
        .count();
    assert!(held > 0, "a false record of the wall on the top is held");
}

/// A cone of half-angle 30°, its apex at `(0, 1, 0)`.
fn cone() -> AtRestBody<f64> {
    let r = 30.0_f64.to_radians().tan();
    let body = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    finished("cone", body, tol())
}

/// **A reading that ran and did not decide refuses at the gate.** A
/// block stands on a cone's apex, an edge of it running through the
/// apex. That edge and the cone's face, recorded as touching: the
/// curve × face reading runs, finds the edge through the apex, where the
/// cone has no tangent plane, and does not decide
/// (`ContactReach::Unread`), so `finish` refuses the body. Red on a gate
/// that takes every `NotCertifiable` as "no lane": it held the record.
#[test]
fn a_record_a_reading_ran_on_and_did_not_decide_refuses_at_the_gate() {
    let block = finished(
        "block",
        sweep::test_support::brick::<f64>((-1.0, 1.0), (1.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let (body, [(cone_faces, _), (_, block_edges)]) = joined(&cone(), &block);
    let unread = |r: &Result<AtRestBody<f64>, NotFinished>| {
        matches!(r, Err(e) if e.as_slice().iter().any(|f| matches!(
            f,
            topo::ValidationError::CensusUnsupported {
                cause: topo::CensusUnsupportedCause::ContactLane(
                    topo::ContactRefusal::NotCertifiable {
                        reach: topo::ContactReach::Unread,
                        ..
                    }
                ),
                ..
            }
        )))
    };
    let refused = cone_faces
        .iter()
        .filter(|&&f| kind(&body, f) == geom::SurfaceKind::Cone)
        .flat_map(|&f| block_edges.iter().map(move |&e| (e, f)))
        .filter(|&(e, f)| unread(&handed(&body, Cell::Edge(e), Cell::Face(f))))
        .count();
    assert!(
        refused > 0,
        "the edge through the apex refuses on a reading that did not decide"
    );
}

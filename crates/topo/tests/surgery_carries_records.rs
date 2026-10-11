//! **A surgery verb carries its operand's contact records**: each
//! record's cells mapped through the verb's own lineage, each survivor
//! citing the operand's record it is, and a record the verb cannot map
//! refused rather than dropped.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, describe_as_intersections, finished, geometric_cube, line};
use geom_core::{Affine3, Point3, Tol, Vec3};
use topo::{
    AtRestBody, Backing, Body, BooleanBody, BooleanResult, CarryRefusal, Cell, Cited, Cites,
    Contact, ContactRecords, FaceKey, FaceSurface, MefSite, Surface, ValidationError, union,
    validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

/// Two unit cubes kissing at the corner `(1, 1, 1)`: one vertex–vertex
/// record.
fn kiss() -> BooleanBody<f64> {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    match union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

fn face_at_x(body: &Body<f64>, x: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() == 1.0 && origin.x == x
            )
        })
        .map(|(k, _)| k)
        .expect("a face on x")
}

/// `carried` is the kiss's one record carried: it cites the operand's
/// record 0, the at-rest gate passes `body` with it, and withholding it
/// refuses the touch as undeclared — the refusal every verb's result met
/// before it could carry.
fn carries_the_kiss(what: &str, body: &Body<f64>, carried: &ContactRecords) {
    assert_eq!(carried.vertex_vertex().count(), 1, "{what}: {carried:?}");
    let cites: Vec<Backing> = carried.rows()[0].cites.iter().collect();
    assert_eq!(
        cites,
        [Backing::Carried {
            input: 0,
            record: 0
        }],
        "{what}: the record cites the operand's"
    );
    assert_eq!(
        validate_pseudomanifold(body, carried, tol()),
        Ok(()),
        "{what}: the carried record passes at rest"
    );
    let withheld = validate_pseudomanifold(body, &ContactRecords::default(), tol())
        .expect_err("the kiss is a touch");
    assert!(
        withheld
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{what}: withheld, the kiss is undeclared: {withheld:?}"
    );
}

/// **Every topo surgery verb's result carries the kiss and passes at
/// rest.** Red on a verb whose carry drops the record, re-keys it onto a
/// cell the result does not hold, or cites anything but the operand's
/// record.
#[test]
fn every_surgery_verb_carries_a_corner_kiss() {
    let base = kiss();
    assert_eq!(base.contacts.len(), 1, "{:?}", base.contacts);

    // Key-stable: every cell is itself.
    let moved = topo::transform_rigid(
        &base.body,
        &Affine3::translation(Vec3::new(0.3, 0.2, 0.1)),
        tol(),
    )
    .expect("a rigid move");
    let carried = base.contacts.carried_from(0);
    carries_the_kiss("transform_rigid", &moved, &carried);

    let shelled = topo::shell(&base.body, 0.1, tol()).expect("the kiss shells");
    carries_the_kiss(
        "shell",
        &shelled.body,
        &shelled
            .carry_contacts(
                &base.contacts,
                geom_core::Band::linear(tol()).unwrap(),
                Some(topo::RegionLane::certified()),
            )
            .unwrap(),
    );

    let opened = topo::shell_open(&base.body, 0.1, &[face_at_x(&base.body, 0.0)], tol())
        .expect("the kiss shells open");
    carries_the_kiss(
        "shell_open",
        &opened.body,
        &opened
            .carry_contacts(
                &base.contacts,
                geom_core::Band::linear(tol()).unwrap(),
                Some(topo::RegionLane::certified()),
            )
            .unwrap(),
    );

    let mut offset = Body::clone(&base.body);
    let face = face_at_x(&offset, 0.0);
    let outcome = topo::replace_face_offset(&mut offset, face, 0.1, tol()).expect("an offset");
    carries_the_kiss(
        "replace_face_offset",
        &offset,
        &outcome.carry_contacts(&base.contacts, &offset).unwrap(),
    );

    let mut merged = Body::clone(&base.body);
    let outcome = merged.merge_coplanar_faces(tol()).expect("a merge");
    carries_the_kiss(
        "merge_coplanar_faces",
        &merged,
        &outcome.carry_contacts(&base.contacts, &merged).unwrap(),
    );

    // The cut at x = 1/2 leaves the kiss whole on the side above it.
    let plane = common::split_plane(Point3::new(0.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the kiss splits");
    let sides = split
        .carry_contacts(
            &base.contacts,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified()),
        )
        .unwrap();
    let above = split.above.body().expect("material above");
    carries_the_kiss("split above", above, &sides.above);
    let below = split.below.body().expect("material below");
    assert!(sides.below.is_empty(), "{:?}", sides.below);
    assert_eq!(
        validate_pseudomanifold(below, &sides.below, tol()),
        Ok(()),
        "split below: nothing touches"
    );
}

/// **A record whose cells the cut parts is refused**, never dropped: a
/// pair of a cell below the cut and one above it (bookkeeping, not a
/// touch) has no side to land on. Red if the carry drops it or files it
/// on one side.
#[test]
fn a_record_the_cut_parts_is_refused() {
    let base = kiss();
    let below = face_at_x(&base.body, 0.0);
    let above = face_at_x(&base.body, 2.0);
    let mut records = base.contacts.clone();
    records.extend([Cited::new(
        Contact::new(&base.body, Cell::Face(below), Cell::Face(above)).unwrap(),
        Cites::decided(7),
    )]);
    let plane = common::split_plane(Point3::new(0.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the kiss splits");
    assert_eq!(
        split.carry_contacts(
            &records,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified())
        ),
        Err(CarryRefusal::Parted { record: 1 })
    );
}

/// **A merge carries a record through what it kills.** The cube's top is
/// cut on its diagonal and its bottom edge split, so the merge glues the
/// top back (killing the diagonal and absorbing one triangle) and joins
/// the split vertex away. Each record naming a killed cell lands on the
/// cell holding it now: the vertex on the joined edge, the diagonal and
/// the absorbed triangle on the kept top, and two records landing on one
/// pair are one record citing both. Red if any is dropped or kept on its
/// dead key; the identity carry over the same lineage is refused, typed,
/// at the dead vertex.
#[test]
fn a_merge_carries_records_onto_what_it_made_of_their_cells() {
    let mut cube = geometric_cube::<f64>(tol());
    describe_as_intersections(&mut cube.body, tol());
    let bottom = cube.mefs[0].face;
    let split = cube
        .body
        .split_edge(cube.mevs[0].edge, 0.5, tol())
        .expect("the bottom edge splits");
    let (a1, c1) = (cube.mevs[3].vertex, cube.mevs[5].vertex);
    let top = cube.seed.face;
    let f = cube.body.get_face(top).unwrap();
    let topo::LoopBoundary::Cycle { first } = cube.body.get_loop(f.outer).unwrap().boundary else {
        panic!("the top loop is a cycle");
    };
    let starting = |v| {
        cube.body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| cube.body.get_half_edge(he).unwrap().start == v)
            .unwrap()
    };
    let (he1, he2) = (starting(a1), starting(c1));
    let at = |v| {
        *cube
            .body
            .get_point(cube.body.get_vertex(v).unwrap().point)
            .unwrap()
    };
    let diagonal = cube
        .body
        .mef(
            MefSite::Chords { he1, he2 },
            line(at(a1), at(c1)),
            FaceSurface::Inherit,
            tol(),
        )
        .expect("the diagonal cuts the top");
    let (t1, t2) = (top, diagonal.face);

    let pair = |body: &Body<f64>, x, y| Contact::new(body, x, y).unwrap();
    let v = Cell::Vertex(split.vertex);
    let records: ContactRecords = [
        Cited::new(pair(&cube.body, v, Cell::Face(t1)), Cites::decided(0)),
        Cited::new(
            pair(&cube.body, Cell::Edge(diagonal.edge), Cell::Face(bottom)),
            Cites::decided(1),
        ),
        Cited::new(pair(&cube.body, Cell::Face(t2), v), Cites::decided(2)),
    ]
    .into_iter()
    .collect();

    let outcome = cube.body.merge_coplanar_faces(tol()).expect("the merge");
    assert_eq!(outcome.groups.len(), 1, "{outcome:?}");
    assert_eq!(outcome.joins.len(), 1, "{outcome:?}");
    let kept = outcome.groups[0].kept;
    let joined = Cell::Edge(topo::joined_edge(&outcome.joins, outcome.joins[0].kept));

    let carried = outcome.carry_contacts(&records, &cube.body).unwrap();
    let rows: Vec<([Cell; 2], Vec<Backing>)> = carried
        .rows()
        .iter()
        .map(|r| (r.cells(), r.cites.iter().collect()))
        .collect();
    let carried_from = |record| Backing::Carried { input: 0, record };
    assert_eq!(
        rows,
        [
            (
                pair(&cube.body, joined, Cell::Face(kept)).cells(),
                vec![carried_from(0), carried_from(2)]
            ),
            (
                pair(&cube.body, Cell::Face(kept), Cell::Face(bottom)).cells(),
                vec![carried_from(1)]
            ),
        ],
        "each record on the cell that holds its own now"
    );

    assert_eq!(
        records.carried_through(0, &cube.body, Some),
        Err(CarryRefusal::Unmapped { record: 0, cell: v }),
        "the identity carry refuses the joined-away vertex"
    );
}

/// **A touch the verb creates is refused at rest, never blessed.** Two
/// boxes a tenth apart carry no record; moving one box's facing wall
/// across the gap lays it on the other, and the carry has nothing to
/// carry, so the census refuses the new patch as undeclared.
#[test]
fn a_touch_the_verb_creates_is_refused_at_rest() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.1, 2.1), (0.5, 1.5), (0.0, 1.0), "b");
    let apart = match union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    };
    assert!(apart.contacts.is_empty(), "{:?}", apart.contacts);
    let mut moved = Body::clone(&apart.body);
    let wall = face_at_x(&moved, 1.0);
    let outcome = topo::replace_face_offset(&mut moved, wall, 0.1, tol()).expect("an offset");
    let carried = outcome.carry_contacts(&apart.contacts, &moved).unwrap();
    assert!(carried.is_empty(), "{carried:?}");
    let refused = validate_pseudomanifold(&moved, &carried, tol()).expect_err("a new touch");
    assert!(
        refused
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "the created touch is undeclared: {refused:?}"
    );
}

/// **A rim that opens under a resting corner leaves its record stale,
/// loudly.** A cube balanced on one corner on a slab's top carries one
/// vertex–face record; opening that top as a rim keeps the face's key on
/// the annulus, so the record carries by key, and the census refuses it
/// stale, because the corner now points into the opening. Whether such a
/// record is consumed instead is undecided
/// (`work/contacthold/a-blend-or-shell-that-consumes-a-recorded-cell-refuses-the-record.md`).
/// Red if the carry drops the record or the census blesses it.
#[test]
fn a_rim_that_opens_under_a_resting_corner_leaves_its_record_stale() {
    let slab = box_of((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), "slab");
    let tilted = common::mapped_cube(
        |x, y, z| {
            let (e1, e2, e3) = (
                Vec3::new(0.9, 0.1, 0.3),
                Vec3::new(-0.2, 0.8, 0.45),
                Vec3::new(-0.3, -0.4, 0.85),
            );
            Point3::new(
                2.0 + x * e1.x + y * e2.x + z * e3.x,
                2.0 + x * e1.y + y * e2.y + z * e3.y,
                1.0 + x * e1.z + y * e2.z + z * e3.z,
            )
        },
        tol(),
    );
    let tilted = finished("the tilted cube", tilted, tol());
    let base = match union(&tilted, &slab, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    };
    assert_eq!(
        base.contacts.vertex_face().count(),
        1,
        "{:?}",
        base.contacts
    );
    assert_eq!(base.contacts.len(), 1, "{:?}", base.contacts);
    let top = base
        .body
        .faces()
        .find(|(_, f)| {
            matches!(
                base.body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == 1.0
            )
        })
        .map(|(k, _)| k)
        .unwrap();
    let opened = topo::shell_open(&base.body, 0.1, &[top], tol()).expect("the slab opens");
    let carried = opened
        .carry_contacts(
            &base.contacts,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified()),
        )
        .unwrap();
    assert_eq!(
        carried.vertex_face().collect::<Vec<_>>(),
        base.contacts.vertex_face().collect::<Vec<_>>(),
        "the record carries by key"
    );
    let refused = validate_pseudomanifold(&opened.body, &carried, tol()).expect_err("stale");
    assert!(
        refused
            .iter()
            .all(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
        "the corner touches nothing now: {refused:?}"
    );
}

/// One record of `x` and `y` in `body`, citing decision 0.
fn one_record(body: &Body<f64>, x: Cell, y: Cell) -> ContactRecords {
    [Cited::new(
        Contact::new(body, x, y).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect()
}

/// **An offset door's join carries a record onto the edge it makes.** A
/// brick with one edge split: its two faces stay put and every other
/// face moves in, so the door ends by joining the split vertex away.
/// Records on the vertex and on the split-off piece land on the joined
/// edge, one record citing both. Red if the carry skips the join: the
/// vertex and the piece are dead.
#[test]
fn an_offset_door_carries_a_record_through_its_join() {
    let band = geom_core::Band::linear(tol()).unwrap();
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let e = body.edges().next().unwrap().0;
    let still: Vec<FaceKey> = {
        let d = body.get_edge(e).unwrap();
        [d.he_plus, d.he_minus]
            .into_iter()
            .map(|h| body.face_of_half_edge(h).unwrap())
            .collect()
    };
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(e).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let split = body.split_edge(e, 0.5 * (t0 + t1), tol()).unwrap();
    let far = body
        .faces()
        .map(|(f, _)| f)
        .find(|f| !still.contains(f))
        .unwrap();
    let mut records = one_record(&body, Cell::Vertex(split.vertex), Cell::Face(far));
    records.extend(
        one_record(&body, Cell::Edge(split.new_edge), Cell::Face(far))
            .rows()
            .to_vec(),
    );
    let moves: Vec<topo::ChartMove<f64>> = body
        .faces()
        .map(|(f, _)| topo::ChartMove {
            faces: vec![f],
            distance: if still.contains(&f) { 0.0 } else { -0.1 },
        })
        .collect();
    let out = topo::offset_planes_together(&mut body, &moves, band, tol()).unwrap();
    assert_eq!(out.joins.len(), 1, "{:?}", out.joins);
    let joined = Cell::Edge(topo::joined_edge(&out.joins, out.joins[0].kept));
    let carried = out.carry_contacts(&records, &body).unwrap();
    let rows: Vec<([Cell; 2], Vec<Backing>)> = carried
        .rows()
        .iter()
        .map(|r| (r.cells(), r.cites.iter().collect()))
        .collect();
    assert_eq!(
        rows,
        [(
            Contact::new(&body, joined, Cell::Face(far))
                .unwrap()
                .cells(),
            vec![
                Backing::Carried {
                    input: 0,
                    record: 0
                },
                Backing::Carried {
                    input: 0,
                    record: 1
                }
            ]
        )]
    );
}

/// **A record at a conventional vertex is a record on its closed edge.**
/// A disc planted in a brick's top is bounded by one closed circle whose
/// one vertex has no other edge. Red if the carry keeps the vertex.
#[test]
fn a_record_at_a_conventional_vertex_lands_on_its_closed_edge() {
    let mut body = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol());
    let top = face_at_z(&body, 1.0);
    let bottom = face_at_z(&body, 0.0);
    let f = body.get_face(top).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the top loop is a cycle");
    };
    let disc = common::plant_disc_face(&mut body, first, Point3::new(1.0, 1.0, 1.0), 0.5, tol());
    let v = body
        .get_half_edge(body.get_edge(disc.edge).unwrap().he_plus)
        .unwrap()
        .start;
    assert!(topo::is_conventional_vertex(&body, v));
    let records = one_record(&body, Cell::Vertex(v), Cell::Face(bottom));
    let carried = records.carried_through(0, &body, Some).unwrap();
    assert_eq!(
        carried.cell_pairs().collect::<Vec<_>>(),
        [(Cell::Edge(disc.edge), Cell::Face(bottom))]
    );
}

/// **A record whose two vertices map to one is consumed**, as the boolean
/// drops a record whose vertex pair it fused (D1); any other pair mapped
/// to one cell is a lineage bug and refused. Red if the carry refuses the
/// fusion, keeps a one-cell record, or drops the face pair.
#[test]
fn a_record_whose_cells_map_to_one_cell_is_consumed() {
    let base = kiss();
    let [a, b] = base.contacts.rows()[0].cells();
    let carried = base
        .contacts
        .carried_through(0, &base.body, |c| Some(if c == b { a } else { c }))
        .unwrap();
    assert!(carried.is_empty(), "{carried:?}");

    // Two faces mapped to one are no fusion: refused, not dropped.
    let (fa, fb) = (face_at_x(&base.body, 0.0), face_at_x(&base.body, 2.0));
    let faces = one_record(&base.body, Cell::Face(fa), Cell::Face(fb));
    assert_eq!(
        faces.carried_through(0, &base.body, |c| Some(if c == Cell::Face(fb) {
            Cell::Face(fa)
        } else {
            c
        })),
        Err(CarryRefusal::Pair {
            record: 0,
            error: topo::ContactPairError::OneCell(Cell::Face(fa))
        })
    );
}

fn face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == z
            )
        })
        .map(|(k, _)| k)
        .expect("a face on z")
}

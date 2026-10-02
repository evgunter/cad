//! **Swept walls over a run, named** (`names/README.md`, N1 "Swept walls
//! over a run"): an extrude or a revolve builds one wall over a run of
//! collinear profile pieces, and its role-path segment holds the run —
//! its pieces in authored order — while rims and cap vertices stay per
//! piece and a station inside the run mints no `LateralEdge` or
//! `BandRim`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use editor_core::{
    EntityKind, LoopProgram, MeridianEnd, Node, PieceRun, ProfileDoc, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, RoleSeg, StableName,
};
use geom_core::Tol;

use crate::fixture::{
    ang, axis_in_plane, frame, insert, len, len2, minted, piece, run, table, vpiece,
};

fn to(x: f64, y: f64) -> ProgramTarget {
    ProgramTarget::Point(len2([x, y]))
}

/// A rectangle `[x0, x0 + 2] × [0, 2]` with its bottom side drawn as a
/// leg to `(x0 + 1, 0)` and the DECLARED straight continuation on to
/// `(x0 + 2, 0)`: segments 0 and 1 are one run.
fn subdivided(x0: f64) -> Vec<ProgramStep> {
    vec![
        ProgramStep::At(len2([x0, 0.0])),
        ProgramStep::LineTo(to(x0 + 1.0, 0.0)),
        ProgramStep::ContinueTo(to(x0 + 2.0, 0.0)),
        ProgramStep::LineTo(to(x0 + 2.0, 2.0)),
        ProgramStep::LineTo(to(x0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

fn profiled(steps: Vec<ProgramStep>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("band_run_wall_names", Tol::witness());
    let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
        }),
    );
    (doc, plane, p)
}

fn extruded(steps: Vec<ProgramStep>) -> (ProfileDoc, RecipeNodeId) {
    extruded_by(steps, 1.0)
}

fn extruded_by(steps: Vec<ProgramStep>, distance: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, _, p) = profiled(steps);
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(distance),
            side: ExtrudeSide::Along,
        },
    )
}

fn revolved(steps: Vec<ProgramStep>, angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane, p) = profiled(steps);
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(angle),
        },
    )
}

fn run_of(doc: &ProfileDoc, sweep: RecipeNodeId, segments: &[usize]) -> PieceRun {
    PieceRun::new(segments.iter().map(|&k| piece(doc, sweep, 0, k)).collect()).unwrap()
}

fn rows_with(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    pred: impl Fn(&RoleSeg) -> bool,
) -> Vec<StableName> {
    table(ev, node)
        .iter()
        .filter(|(n, _)| n.path.first().is_some_and(&pred))
        .map(|(n, _)| n.clone())
        .collect()
}

/// **Extrude.** One `Lateral` over the run, its pieces in authored
/// order; no `LateralEdge` at the station; a rim per piece at each cap
/// and a cap vertex at the station on each cap.
#[test]
fn an_extruded_run_wall_is_named_by_its_pieces() {
    let (doc, ex) = extruded(subdivided(0.0));
    let ev = run(&doc, &Default::default());
    let t = table(&ev, ex);
    let wall = minted(
        EntityKind::Face,
        ex,
        RoleSeg::Lateral(run_of(&doc, ex, &[0, 1])),
    );
    assert!(t.lookup(&wall).is_some(), "the run wall: {wall:?}");
    for k in [0, 1] {
        let one = minted(
            EntityKind::Face,
            ex,
            RoleSeg::Lateral(run_of(&doc, ex, &[k])),
        );
        assert!(t.lookup(&one).is_none(), "no wall of piece {k} alone");
    }
    assert_eq!(
        rows_with(&ev, ex, |s| matches!(s, RoleSeg::Lateral(_))).len(),
        4,
        "four walls for five pieces"
    );
    let station = vpiece(&doc, ex, 0, 1);
    assert!(
        t.lookup(&minted(EntityKind::Edge, ex, RoleSeg::LateralEdge(station)))
            .is_none(),
        "a station has no strut"
    );
    for end in [editor_core::CapEnd::Start, editor_core::CapEnd::End] {
        for k in [0, 1] {
            let rim = minted(
                EntityKind::Edge,
                ex,
                RoleSeg::RimEdge(end, piece(&doc, ex, 0, k)),
            );
            assert!(t.lookup(&rim).is_some(), "{end:?} rim of piece {k}");
        }
        let v = minted(EntityKind::Vertex, ex, RoleSeg::CapVertex(end, station));
        assert!(t.lookup(&v).is_some(), "{end:?} cap vertex at the station");
    }
}

/// **A run wrapping through the loop's start begins at its first piece
/// after the start vertex**: the square authored from `(1, 0)`, the
/// middle of its bottom side (the seam's straight continuation
/// declared), puts that side's two halves at the END and the START of
/// the loop; the wall lists the piece leaving the start vertex first.
#[test]
fn a_wrapping_run_begins_after_the_start_vertex() {
    let (doc, ex) = extruded(vec![
        ProgramStep::At(len2([1.0, 0.0])),
        ProgramStep::LineTo(to(2.0, 0.0)),
        ProgramStep::LineTo(to(2.0, 2.0)),
        ProgramStep::LineTo(to(0.0, 2.0)),
        ProgramStep::LineTo(to(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::StartArriving),
    ]);
    let ev = run(&doc, &Default::default());
    let wall = minted(
        EntityKind::Face,
        ex,
        RoleSeg::Lateral(run_of(&doc, ex, &[0, 4])),
    );
    assert!(table(&ev, ex).lookup(&wall).is_some(), "{wall:?}");
}

/// **Revolve, partial and full.** The bottom side sweeps one `Band` over
/// the run either way. A partial revolve's wedge caps keep the station:
/// its meridians stay per piece and the station's meridian vertices are
/// named, but no `BandRim` stands there. A full revolve keeps no entity
/// for the station: one seam meridian over the run, no rim, no
/// meridian vertex.
#[test]
fn a_revolved_run_wall_is_named_by_its_pieces() {
    for angle in [std::f64::consts::FRAC_PI_2, std::f64::consts::TAU] {
        let full = angle == std::f64::consts::TAU;
        let (doc, rev) = revolved(subdivided(1.0), angle);
        let ev = run(&doc, &Default::default());
        let t = table(&ev, rev);
        let band = minted(
            EntityKind::Face,
            rev,
            RoleSeg::Band(run_of(&doc, rev, &[0, 1])),
        );
        assert!(
            t.lookup(&band).is_some(),
            "{angle}: the run's band {band:?}"
        );
        let station = vpiece(&doc, rev, 0, 1);
        assert!(
            t.lookup(&minted(EntityKind::Edge, rev, RoleSeg::BandRim(station)))
                .is_none(),
            "{angle}: a station has no rim"
        );
        if full {
            let seam = minted(
                EntityKind::Edge,
                rev,
                RoleSeg::Meridian(MeridianEnd::Seam, run_of(&doc, rev, &[0, 1])),
            );
            assert!(t.lookup(&seam).is_some(), "the run's one seam meridian");
            let v = minted(
                EntityKind::Vertex,
                rev,
                RoleSeg::MeridianVertex(MeridianEnd::Seam, station),
            );
            assert!(t.lookup(&v).is_none(), "the station has no entity");
        } else {
            for end in [MeridianEnd::Start, MeridianEnd::End] {
                for k in [0, 1] {
                    let m = minted(
                        EntityKind::Edge,
                        rev,
                        RoleSeg::Meridian(end, run_of(&doc, rev, &[k])),
                    );
                    assert!(t.lookup(&m).is_some(), "{end:?} meridian of piece {k}");
                }
                let v = minted(
                    EntityKind::Vertex,
                    rev,
                    RoleSeg::MeridianVertex(end, station),
                );
                assert!(
                    t.lookup(&v).is_some(),
                    "{end:?} meridian vertex at the station"
                );
            }
        }
    }
}

/// **A one-piece run is spelled as its one locator**, on the wire and in
/// `Debug`, so a name minted before runs existed reads back unchanged.
#[test]
fn a_one_piece_run_is_spelled_as_its_locator() {
    let (doc, ex) = extruded(subdivided(0.0));
    let p = piece(&doc, ex, 0, 0);
    let run = PieceRun::one(p);
    assert_eq!(
        serde_json::to_string(&run).unwrap(),
        serde_json::to_string(&p).unwrap()
    );
    assert_eq!(format!("{run:?}"), format!("{p:?}"));
    let back: PieceRun = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
    assert_eq!(back, run);
    let two = PieceRun::new(vec![p, piece(&doc, ex, 0, 1)]).unwrap();
    let back: PieceRun = serde_json::from_str(&serde_json::to_string(&two).unwrap()).unwrap();
    assert_eq!(back, two, "a run of two round-trips as its list");
    assert!(PieceRun::new(Vec::new()).is_none(), "a run is never empty");
    assert!(
        serde_json::from_str::<PieceRun>("[]").is_err(),
        "an empty list reads back as no run"
    );
}

/// **A reversed extrusion names each wall by the piece it sweeps.** At a
/// negative distance the sweep traverses the loop backwards; the names
/// are read off canonical positions all the same, so the wall over the
/// bottom side `y = 0` (segments 0 and 1) is the one named for them,
/// and the wall over `x = 2` the one named for segment 2.
#[test]
fn a_reversed_extrusion_names_each_wall_by_its_own_pieces() {
    let (doc, ex) = extruded_by(subdivided(0.0), -1.0);
    let ev = run(&doc, &Default::default());
    let Some(editor_core::NodeResult::Ok(v)) = ev.nodes.get(&ex) else {
        panic!("the extrude evaluated");
    };
    let editor_core::ValuePayload::Body(body) = &v.payload else {
        panic!("a body value");
    };
    let plane_of = |name: &StableName| {
        let Some(editor_core::Entry::Unique(e)) = v.name_table.lookup(name) else {
            panic!("a unique face for {name:?}");
        };
        let editor_core::EntityKey::Face(f) = e.key else {
            panic!("a face key");
        };
        match body.get_surface(body.get_face(f).unwrap().surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => (*origin, *normal),
            other => panic!("a plane wall, got {other:?}"),
        }
    };
    let (o, n) = plane_of(&minted(
        EntityKind::Face,
        ex,
        RoleSeg::Lateral(run_of(&doc, ex, &[0, 1])),
    ));
    assert!(
        o.y.abs() < 1e-12 && n.y.abs() > 0.99,
        "the bottom wall: {o:?} {n:?}"
    );
    let (o, n) = plane_of(&minted(
        EntityKind::Face,
        ex,
        RoleSeg::Lateral(run_of(&doc, ex, &[2])),
    ));
    assert!(
        (o.x - 2.0).abs() < 1e-12 && n.x.abs() > 0.99,
        "the x = 2 wall: {o:?} {n:?}"
    );
}

//! **Swept walls over a run, named** (`names/README.md`, N1 "Swept walls
//! over a run"): an extrude or a revolve builds one wall over a run of
//! collinear or cocircular profile pieces, and its role-path segment
//! holds the run —
//! its pieces in authored order — while rims and cap vertices stay per
//! piece and a station inside the run mints no `LateralEdge` or
//! `BandRim`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    EntityKind, LoopProgram, MeridianEnd, Node, PieceRun, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg, StableName,
};
use geom_core::Tol;

use crate::fixture::{
    ang, axis_in_plane, frame, insert, len, len2, minted, piece, run, scl, table, vpiece,
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

/// A D on `x = x0`: the half circle of radius 1 centred `(x0, 0)` drawn
/// as a quarter arc and, across a declared tangent joint, the tangent
/// arc that continues it (segments 0 and 1, one run), closed by its
/// diameter.
fn d_of_two_arcs(x0: f64) -> Vec<ProgramStep> {
    vec![
        ProgramStep::At(len2([x0, -1.0])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: to(x0 + 1.0, 0.0),
            b: scl((std::f64::consts::PI / 8.0).tan()),
        }),
        ProgramStep::Tangent,
        ProgramStep::TangentArcTo(to(x0, 1.0)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

/// **A run of cocircular arcs is named by its pieces** exactly as a
/// straight run is: an extrude's one cylinder `Lateral` over both
/// quarters, no `LateralEdge` at the station, a rim per piece; a full
/// revolve's one torus `Band` with no `BandRim` at the station. A
/// partial revolve keeps one wall per arc
/// (`work/band/partial-revolve-arc-runs-wait-on-the-meridian-fold.md`),
/// so it names a `Band` per piece and the `BandRim` between them.
#[test]
fn a_run_of_arcs_is_named_by_its_pieces() {
    let (doc, ex) = extruded(d_of_two_arcs(0.0));
    let ev = run(&doc, &Default::default());
    let t = table(&ev, ex);
    let wall = minted(
        EntityKind::Face,
        ex,
        RoleSeg::Lateral(run_of(&doc, ex, &[0, 1])),
    );
    assert!(t.lookup(&wall).is_some(), "the arc run's wall: {wall:?}");
    assert_eq!(
        rows_with(&ev, ex, |s| matches!(s, RoleSeg::Lateral(_))).len(),
        2,
        "the arc run and the diameter"
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
            assert!(t.lookup(&rim).is_some(), "{end:?} rim of arc {k}");
        }
    }
    for angle in [std::f64::consts::FRAC_PI_2, std::f64::consts::TAU] {
        let full = angle == std::f64::consts::TAU;
        let (doc, rev) = revolved(d_of_two_arcs(2.0), angle);
        let ev = run(&doc, &Default::default());
        let t = table(&ev, rev);
        let band = |pieces: &[usize]| {
            minted(
                EntityKind::Face,
                rev,
                RoleSeg::Band(run_of(&doc, rev, pieces)),
            )
        };
        let station = vpiece(&doc, rev, 0, 1);
        let rim = t.lookup(&minted(EntityKind::Edge, rev, RoleSeg::BandRim(station)));
        if full {
            assert!(t.lookup(&band(&[0, 1])).is_some(), "the arc run's band");
            assert!(rim.is_none(), "a station has no rim");
        } else {
            for k in [0, 1] {
                assert!(
                    t.lookup(&band(&[k])).is_some(),
                    "{angle}: the band of arc {k}"
                );
            }
            assert!(rim.is_some(), "{angle}: the rim between the arcs' bands");
        }
    }
}

/// The profile node's step ids, loop 0.
fn step_ids(doc: &ProfileDoc, p: RecipeNodeId) -> Vec<editor_core::StepId> {
    match doc.node(p) {
        Some(Node::Profile(pp)) => pp.ids[0].clone(),
        other => panic!("not a profile: {other:?}"),
    }
}

/// `doc` with profile `p`'s loop 0 re-authored as `steps`, keeping the
/// step ids listed.
fn reprogrammed(
    doc: &ProfileDoc,
    p: RecipeNodeId,
    steps: Vec<ProgramStep>,
    ids: Vec<Option<editor_core::StepId>>,
) -> ProfileDoc {
    editor_core::apply(
        doc,
        &editor_core::DocEdit::SetProgram {
            node: p,
            loops: vec![LoopProgram::Chain(steps)],
            ids: vec![ids],
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the program edit applies")
    .doc
}

/// What a vanished name is offered.
fn offers(doc: &ProfileDoc, name: &StableName) -> Vec<StableName> {
    let ev = run(doc, &Default::default());
    match editor_core::resolve(editor_core::RunCtx { doc, eval: &ev }, name) {
        editor_core::Resolution::Failed(f) => f.offers,
        other => panic!("{name:?} should vanish, resolved {other:?}"),
    }
}

/// **Offers cross an arc station both ways.** A D's half arc split at a
/// declared tangent joint becomes one run wall: the old one-piece
/// wall's name is offered the run wall. Bending the station off the
/// circle breaks the run: the run wall's name is offered each piece's
/// wall. Extruded either way and fully revolved.
#[test]
fn offers_follow_an_arc_station_in_and_out() {
    let wall = |n: RecipeNodeId, revolve: bool, pieces: Vec<editor_core::ProfileEdgeRef>| {
        let r = PieceRun::new(pieces).unwrap();
        minted(
            EntityKind::Face,
            n,
            if revolve {
                RoleSeg::Band(r)
            } else {
                RoleSeg::Lateral(r)
            },
        )
    };
    for (label, angle, d, x0) in [
        ("extrude +", None, 1.0, 0.0),
        ("extrude −", None, -1.0, 0.0),
        ("full revolve", Some(std::f64::consts::TAU), 1.0, 2.0),
    ] {
        let plain = vec![
            ProgramStep::At(len2([x0, -1.0])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: to(x0, 1.0),
                b: scl(1.0),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let (doc, n) = match angle {
            Some(a) => revolved(plain, a),
            None => extruded_by(plain, d),
        };
        let p = profile_of(&doc, n);
        let rev = angle.is_some();
        let held = wall(n, rev, vec![piece(&doc, n, 0, 0)]);
        let i = step_ids(&doc, p);
        let doc2 = reprogrammed(
            &doc,
            p,
            d_of_two_arcs(x0),
            vec![Some(i[0]), Some(i[1]), None, None, Some(i[2])],
        );
        let run_wall = wall(n, rev, vec![piece(&doc2, n, 0, 0), piece(&doc2, n, 0, 1)]);
        assert!(
            table(&run(&doc2, &Default::default()), n)
                .lookup(&run_wall)
                .is_some(),
            "{label}: the run wall is minted"
        );
        assert!(
            offers(&doc2, &held).contains(&run_wall),
            "{label}: the one-piece wall is offered the run wall"
        );
        let quarter = (std::f64::consts::PI / 8.0).tan();
        let bent = vec![
            ProgramStep::At(len2([x0, -1.0])),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: to(x0 + 1.1, 0.0),
                b: scl(quarter),
            }),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: to(x0, 1.0),
                b: scl(quarter),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ];
        let i2 = step_ids(&doc2, p);
        let doc3 = reprogrammed(
            &doc2,
            p,
            bent,
            vec![Some(i2[0]), Some(i2[1]), Some(i2[3]), Some(i2[4])],
        );
        let got = offers(&doc3, &run_wall);
        for k in 0..2 {
            let w = wall(n, rev, vec![piece(&doc3, n, 0, k)]);
            assert!(
                got.contains(&w),
                "{label}: the broken run offers piece {k}'s wall"
            );
        }
    }
}

/// **f64 and `Interval` mint the same arc-run names**: the extruded D,
/// and the D fully revolved on and beside the axis.
#[test]
fn arc_run_names_agree_across_scalars() {
    use geom_core::Interval;
    for (label, x0, angle) in [
        ("extrude", 0.0, None),
        ("full sphere", 0.0, Some(std::f64::consts::TAU)),
        ("full torus", 2.0, Some(std::f64::consts::TAU)),
    ] {
        let (doc, n) = match angle {
            Some(a) => revolved(d_of_two_arcs(x0), a),
            None => extruded_by(d_of_two_arcs(x0), -1.0),
        };
        let ev = run(&doc, &Default::default());
        let evi = editor_core::evaluate::<Interval>(
            &doc,
            None,
            &editor_core::CancelToken::new(),
            &Default::default(),
            Tol::witness(),
        );
        let names = |t: &editor_core::NameTable| {
            let mut v: Vec<String> = t.iter().map(|(k, _)| format!("{k:?}")).collect();
            v.sort();
            v
        };
        let a = names(&ev.value(n).expect("the f64 value").name_table);
        let b = names(
            &evi.value(n)
                .unwrap_or_else(|| panic!("{label}: the Interval value"))
                .name_table,
        );
        let run = run_of(&doc, n, &[0, 1]);
        let seg = if angle.is_some() {
            RoleSeg::Band(run)
        } else {
            RoleSeg::Lateral(run)
        };
        assert!(
            table(&ev, n)
                .lookup(&minted(EntityKind::Face, n, seg))
                .is_some(),
            "{label}: the run wall is minted"
        );
        assert_eq!(a, b, "{label}: f64 and Interval mint the same names");
    }
}

/// The profile a sweep node sweeps.
fn profile_of(doc: &ProfileDoc, sweep: RecipeNodeId) -> RecipeNodeId {
    match doc.node(sweep) {
        Some(Node::Extrude { profile, .. } | Node::Revolve { profile, .. }) => *profile,
        other => panic!("not a sweep: {other:?}"),
    }
}

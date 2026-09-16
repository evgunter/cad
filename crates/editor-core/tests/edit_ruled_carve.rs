//! **A ruled carve's band ends, named at the document layer** — a rod
//! with a flat driven through `Node::Fillet`, and the three roles a
//! transverse cap mints read off the fillet node's name table:
//! [`RoleSeg::EndArc`] (the cut-off arc), [`RoleSeg::FootVertex`] (the
//! two cap feet) and [`RoleSeg::BandCut`] (the surviving rim piece).
//!
//! **What is first here is the RULED band, not the roles.** All three
//! roles already reach `names::emit_fillet` from the registry — the
//! corpus's `die_fillet` mints feet and end arcs at every corner of its
//! cube, and `blend5_rim_support` drives a closed rim. What no document
//! drove is the carve of `sweep::blend::open::ruled`: the band on
//! curved supports cut off at a plane cap, whose ends mint `EndArc` in
//! its TRANSVERSE-CAP configuration and whose `BandCut` carries a cap
//! RIM edge rather than a ladder meridian. Those arms of the emitter
//! had no document-layer consumer, which is what the vocabulary row
//! `cut-off-arc-persists-as-a-corner-arc` rested its argument on.
//!
//! # Why a suite and not a corpus document
//!
//! `tests/corpus` is the Band 4 REGISTRY (`corpus/mod.rs`): a document
//! there buys the whole battery — the interval lane, the persistence
//! round trip, the latency table's baseline row, the name-digest
//! goldens — and pays for it in every one of those baselines. What this
//! row needs is none of that: it needs the carve driven once and its
//! MINTED NAMES read, which is the shape `blend5_rim_support` already
//! set for the annulus. The registry would gain a fixture whose
//! vocabulary tally is already covered (`Fillet` is a covered node
//! kind) and whose mass pin would have to be `None` (a fillet of a
//! cylinder is π-valued), and this suite would gain nothing it asserts.
//!
//! # The walk this suite watches
//!
//! `sweep::blend::open::ruled`'s module header states the carve: per
//! end, split each cap rim edge at its foot, `mef` the cut-off arc
//! across the cap between the two feet, `mef` one trimline per support
//! along the ruling, then `kef`/`kev` away the slivers and the rim
//! remnants. The rows below read that walk's output through the names:
//! the arc's endpoints ARE the two feet, each foot lies in the cap it
//! is named at and on the support it is named on, each trimline runs
//! between its support's two feet, and the surviving rim piece runs
//! between whatever cut it.
//!
//! # Both material sides
//!
//! Two documents, the pair the ruled header pins through the extrude
//! door: the D-profile rod (convex creases, the band REMOVES material)
//! and a rod's section standing on a block's top edge (concave creases,
//! the band ADDS it, and the cap gains the region under the arc). The
//! names are the same set, which is the claim — the emitter reads no
//! convexity. The concave twin also carries a rim that only ONE crease
//! cuts, so one of its `BandCut` pieces runs from a foot to a surviving
//! source vertex rather than from foot to foot.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, LoopProgram,
    NameRef, NameTable, Node, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::{len, scl};
use geom_core::{Point3, Tol};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, ROD_R};
use topo::{Body, EdgeKey, VertexKey};

/// The concave twin's cylinder radius, its axis depth below the block's
/// top plane and the block's length along the ruling — the three
/// numbers `sweep`'s `review_fillet_h7_r1_probes` builds its sunk rod
/// from, which it holds as private constants.
const RC: f64 = 0.5;
const S: f64 = 0.3;
const BLOCK_L: f64 = 1.0;

/// Both fixtures' points are metres of order 1 and every quantity below
/// is a distance, so one absolute window serves them all. It is far
/// above `f64` rounding of these arithmetics and far below any
/// difference the rows are asked to tell apart (the closest pair is a
/// foot and the source vertex it was retracted from, `ROD_FILLET` apart).
const NEAR: f64 = 1e-9;

fn tol() -> Tol {
    Tol::witness()
}

// ---------------------------------------------------------------- //
// The two documents
// ---------------------------------------------------------------- //

/// What the carve left at one end of a surviving cap rim piece.
#[derive(Debug, Clone, Copy)]
enum RimEnd {
    /// The foot the crease at this profile vertex cut the rim at.
    Foot(u32),
    /// The source cap vertex at this profile vertex, untouched.
    Source(u32),
}

/// One ruled-carve fixture: the document, its extrude and fillet nodes,
/// and the structure the rows walk.
struct Ruled {
    what: &'static str,
    doc: ProfileDoc,
    rod: RecipeNodeId,
    fillet: RecipeNodeId,
    /// Per crease: the profile vertex its lateral edge stands at, and
    /// the two profile segments whose walls support it.
    creases: &'static [(u32, [u32; 2])],
    /// Per cap: the rim segments the carve cut, and what each
    /// survivor's two ends are.
    rims: &'static [(u32, [RimEnd; 2])],
    /// How far `p` is from the surface of segment `seg`'s wall — zero
    /// on it. The row that reads a foot's `support` argument checks the
    /// foot against THIS.
    residual: fn(u32, Point3<f64>) -> f64,
}

/// A one-loop extruded document with its ruling creases filleted.
fn carve(
    what: &'static str,
    lp: LoopProgram,
    height: f64,
    creases: &'static [(u32, [u32; 2])],
    rims: &'static [(u32, [RimEnd; 2])],
    residual: fn(u32, Point3<f64>) -> f64,
) -> Ruled {
    let doc = ProfileDoc::empty_derived(what, tol());
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![lp],
        }),
    );
    let (doc, rod) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(height),
        },
    );
    let selection = creases
        .iter()
        .map(|&(v, _)| lateral_edge(rod, v))
        .collect::<Vec<_>>();
    let (doc, fillet) = fixture::insert(
        doc,
        Node::Fillet {
            target: rod,
            radius: len(ROD_FILLET),
            selection,
        },
    );
    Ruled {
        what,
        doc,
        rod,
        fillet,
        creases,
        rims,
        residual,
    }
}

/// **The D-profile rod**: the chord at `x = ROD_FLAT` and the major arc
/// of the `ROD_R` circle, extruded `ROD_L` along `+z`. Its two creases
/// are convex, and the band the carve leaves REMOVES material.
///
/// Segment 0 is the arc (the rod's cylindrical wall), segment 1 the
/// chord (the flat); the lateral edge at profile vertex `j` joins the
/// walls of segments `j − 1` and `j`, so both of them are
/// cylinder-meets-plane creases along the ruling.
fn d_rod() -> Ruled {
    let y = (ROD_R * ROD_R - ROD_FLAT * ROD_FLAT).sqrt();
    let theta = 2.0 * (core::f64::consts::PI - y.atan2(ROD_FLAT));
    let lp = LoopProgram::Chain(vec![
        ProgramStep::At([len(ROD_FLAT), len(y)]),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point([len(ROD_FLAT), len(-y)]),
            b: scl((theta / 4.0).tan()),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    carve(
        "d_rod",
        lp,
        ROD_L,
        &[(0, [1, 0]), (1, [0, 1])],
        &[
            (0, [RimEnd::Foot(0), RimEnd::Foot(1)]),
            (1, [RimEnd::Foot(0), RimEnd::Foot(1)]),
        ],
        |seg, p| match seg {
            0 => p.x.hypot(p.y) - ROD_R,
            1 => p.x - ROD_FLAT,
            other => panic!("the D-rod has no ruled wall {other}"),
        },
    )
}

/// **A rod's section standing on a block's top edge**: the minor arc of
/// the `RC` circle about `(0, −S)` rising above `y = 0`, on a block
/// whose top plane it interrupts. Its two creases are CONCAVE — the
/// band ADDS material and the cap GAINS the region under the arc.
///
/// The arc is segment 3; the top plane is TWO faces (segments 2 and 4),
/// so the two creases share the cylinder support and have different
/// plane supports. Segment 3's cap rim is cut by both creases,
/// segments 2's and 4's by one each.
fn sunk_rod() -> Ruled {
    let xv = (RC * RC - S * S).sqrt();
    let minor = 2.0 * (xv / RC).asin();
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let lp = LoopProgram::Chain(vec![
        ProgramStep::At(pt(-1.0, -1.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(xv, 0.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(pt(-xv, 0.0)),
            b: scl((minor / 4.0).tan()),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(pt(-1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    carve(
        "sunk_rod",
        lp,
        BLOCK_L,
        &[(3, [2, 3]), (4, [3, 4])],
        &[
            (2, [RimEnd::Source(2), RimEnd::Foot(3)]),
            (3, [RimEnd::Foot(3), RimEnd::Foot(4)]),
            (4, [RimEnd::Foot(4), RimEnd::Source(5)]),
        ],
        |seg, p| match seg {
            2 | 4 => p.y,
            3 => (p.x * p.x + (p.y + S) * (p.y + S)).sqrt() - RC,
            other => panic!("the sunk rod has no ruled wall {other}"),
        },
    )
}

fn fixtures() -> [Ruled; 2] {
    [d_rod(), sunk_rod()]
}

// ---------------------------------------------------------------- //
// Reading the document and its names
// ---------------------------------------------------------------- //

/// The extrude emitter's own names, which every band-end role's
/// arguments are drawn from.
fn lateral_edge(rod: RecipeNodeId, vertex: u32) -> StableName {
    fixture::ename(
        rod,
        RoleSeg::LateralEdge(ProfileVertexRef {
            loop_index: 0,
            vertex,
        }),
    )
}

fn wall(rod: RecipeNodeId, segment: u32) -> StableName {
    fixture::fname(
        rod,
        RoleSeg::Lateral(ProfileEdgeRef {
            loop_index: 0,
            segment,
        }),
    )
}

fn rim_edge(rod: RecipeNodeId, end: CapEnd, segment: u32) -> StableName {
    fixture::ename(
        rod,
        RoleSeg::RimEdge(
            end,
            ProfileEdgeRef {
                loop_index: 0,
                segment,
            },
        ),
    )
}

fn cap_vertex(rod: RecipeNodeId, end: CapEnd, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node: rod,
        path: vec![RoleSeg::CapVertex(
            end,
            ProfileVertexRef {
                loop_index: 0,
                vertex,
            },
        )],
    }
}

/// A name minted by `node` in the blend vocabulary.
fn minted(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

/// The one entity a name answers to — the row's loud end when a mint is
/// missing, misspelled or aliased.
fn key_of(t: &NameTable, what: &str, n: &StableName) -> EntityKey {
    match t.lookup(n) {
        Some(Entry::Unique(r)) => r.key,
        other => panic!("{what}: {n:?} is not uniquely named: {other:?}"),
    }
}

fn edge_of(t: &NameTable, what: &str, n: &StableName) -> EdgeKey {
    match key_of(t, what, n) {
        EntityKey::Edge(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not an edge"),
    }
}

fn vertex_of(t: &NameTable, what: &str, n: &StableName) -> VertexKey {
    match key_of(t, what, n) {
        EntityKey::Vertex(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not a vertex"),
    }
}

/// An edge's two end vertices.
fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let edge = body.get_edge(e).expect("a live edge");
    let h = body.get_half_edge(edge.he_plus).expect("a live half-edge");
    let far = body.half_edge_end(edge.he_plus).expect("a forward half");
    [h.start, far]
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    topo::readback::vertex_point(body, v).expect("a live vertex")
}

/// How many names in `t` take `seg`'s role.
fn count(t: &NameTable, seg: fn(&RoleSeg) -> bool) -> usize {
    t.iter().filter(|(n, _)| seg(&n.path[0])).count()
}

fn same_pair(a: [VertexKey; 2], b: [VertexKey; 2]) -> bool {
    (a[0] == b[0] && a[1] == b[1]) || (a[0] == b[1] && a[1] == b[0])
}

const CAPS: [CapEnd; 2] = [CapEnd::Start, CapEnd::End];

/// The foot the carve retracted from cap vertex `(end, crease)` onto
/// the wall of `support`, as the emitter names it.
fn foot_name(f: &Ruled, end: CapEnd, crease: u32, support: u32) -> StableName {
    minted(
        EntityKind::Vertex,
        f.fillet,
        RoleSeg::FootVertex {
            vertex: NameRef::new(cap_vertex(f.rod, end, crease)),
            support: NameRef::new(wall(f.rod, support)),
        },
    )
}

// ---------------------------------------------------------------- //
// The rows
// ---------------------------------------------------------------- //

/// **The cut-off arc spans the cap's two feet.** At each end of each
/// ruled band the emitter mints one [`RoleSeg::EndArc`] keyed by the
/// source vertex the band closes at and the source edge whose blend it
/// bounds; the edge it names runs between exactly the two
/// [`RoleSeg::FootVertex`] vertices of that same cap vertex.
///
/// The runtime values that make it false: the two vertex keys at the
/// ends of the arc edge. An `EndArc` minted for the wrong cap vertex,
/// for the other crease, or across anything but its own two feet fails
/// here, as does an arc the walk did not mint at all.
#[test]
fn a_cut_off_arc_runs_between_the_two_feet_of_the_cap_it_closes_at() {
    for f in fixtures() {
        let ev = run(&f.doc);
        let t = table(&ev, f.fillet);
        let body = corpus::body_of(&ev, f.fillet);
        for end in CAPS {
            for &(crease, supports) in f.creases {
                let what = format!("{}: cap {end:?}, crease {crease}", f.what);
                let arc = minted(
                    EntityKind::Edge,
                    f.fillet,
                    RoleSeg::EndArc {
                        vertex: NameRef::new(cap_vertex(f.rod, end, crease)),
                        edge: NameRef::new(lateral_edge(f.rod, crease)),
                    },
                );
                let feet = supports.map(|s| vertex_of(t, &what, &foot_name(&f, end, crease, s)));
                let got = ends(body, edge_of(t, &what, &arc));
                assert!(
                    same_pair(got, feet),
                    "{what}: the cut-off arc runs {got:?}, not between its feet {feet:?}"
                );
            }
        }
        assert_eq!(
            count(t, |s| matches!(s, RoleSeg::EndArc { .. })),
            2 * f.creases.len(),
            "{}: one cut-off arc per band end, and no other",
            f.what
        );
    }
}

/// **A foot lies in the cap it is named at, on the support it is named
/// on.** [`RoleSeg::FootVertex`] carries two arguments and this row
/// reads both against geometry: the foot's height is the height of the
/// source cap vertex its `vertex` argument names, and its distance from
/// the wall its `support` argument names is zero.
///
/// The runtime values that make it false: the foot's own coordinates. A
/// foot minted against the other cap lands a rod-length away; one minted
/// against the other of its two supports lands off that surface by the
/// band's bite. Both fixtures' walls are told apart by [`Ruled::residual`],
/// so this is the row a swapped `support` argument reds.
#[test]
fn a_cap_foot_lies_in_the_cap_and_on_the_support_its_name_carries() {
    for f in fixtures() {
        let ev = run(&f.doc);
        let (t, source) = (table(&ev, f.fillet), table(&ev, f.rod));
        let (body, rod) = (corpus::body_of(&ev, f.fillet), corpus::body_of(&ev, f.rod));
        for end in CAPS {
            for &(crease, supports) in f.creases {
                // The cap plane, read off the SOURCE vertex the band
                // ends at rather than off the carve's own output.
                let what = format!("{}: cap {end:?}, crease {crease}", f.what);
                let z = point(
                    rod,
                    vertex_of(source, &what, &cap_vertex(f.rod, end, crease)),
                )
                .z;
                for support in supports {
                    let p = point(
                        body,
                        vertex_of(t, &what, &foot_name(&f, end, crease, support)),
                    );
                    assert!(
                        (p.z - z).abs() < NEAR,
                        "{what}: the foot on wall {support} sits at z = {}, not in its cap at {z}",
                        p.z
                    );
                    let d = (f.residual)(support, p);
                    assert!(
                        d.abs() < NEAR,
                        "{what}: the foot named on wall {support} is {d} off it"
                    );
                }
            }
        }
        assert_eq!(
            count(t, |s| matches!(s, RoleSeg::FootVertex { .. })),
            2 * 2 * f.creases.len(),
            "{}: one foot per (cap, crease, support), and no other",
            f.what
        );
    }
}

/// **A trimline runs between its support's two feet.** The carve `mef`s
/// one trimline per support along the ruling, between the feet at the
/// band's two ends; [`RoleSeg::TrimEdge`] carries the crease and that
/// support, and this row reads the edge it names against the two feet
/// carrying the same support.
///
/// The runtime values that make it false: the trimline's two end
/// vertices. This is the row that says a foot is where a trimline
/// MEETS the cap rather than merely somewhere on the support.
#[test]
fn a_trimline_runs_between_the_two_feet_on_its_own_support() {
    for f in fixtures() {
        let ev = run(&f.doc);
        let t = table(&ev, f.fillet);
        let body = corpus::body_of(&ev, f.fillet);
        for &(crease, supports) in f.creases {
            for support in supports {
                let what = format!("{}: crease {crease} on wall {support}", f.what);
                let trim = minted(
                    EntityKind::Edge,
                    f.fillet,
                    RoleSeg::TrimEdge {
                        edge: NameRef::new(lateral_edge(f.rod, crease)),
                        support: NameRef::new(wall(f.rod, support)),
                    },
                );
                let feet =
                    CAPS.map(|end| vertex_of(t, &what, &foot_name(&f, end, crease, support)));
                let got = ends(body, edge_of(t, &what, &trim));
                assert!(
                    same_pair(got, feet),
                    "{what}: the trimline runs {got:?}, not between its feet {feet:?}"
                );
            }
        }
    }
}

/// **The surviving rim piece carries the rim it was cut from, and runs
/// between whatever cut it.** [`RoleSeg::BandCut`] names a cap rim edge
/// of the source; the piece it names runs from each of that rim's own
/// ends to the foot that cut it there, or to the source vertex where no
/// crease reached.
///
/// The runtime values that make it false: the piece's two end vertices.
/// A `BandCut` carrying the neighbouring rim reds here — on the D-rod
/// the two rims' survivors are the arc and the chord of one cap, on the
/// sunk rod segment 3's runs foot-to-foot while 2's and 4's run from a
/// foot to an untouched source vertex.
#[test]
fn a_surviving_rim_piece_carries_the_rim_it_was_cut_from() {
    for f in fixtures() {
        let ev = run(&f.doc);
        let t = table(&ev, f.fillet);
        let body = corpus::body_of(&ev, f.fillet);
        for end in CAPS {
            for &(segment, rim_ends) in f.rims {
                let what = format!("{}: cap {end:?}, rim {segment}", f.what);
                let cut = minted(
                    EntityKind::Edge,
                    f.fillet,
                    RoleSeg::BandCut(NameRef::new(rim_edge(f.rod, end, segment))),
                );
                let want = rim_ends.map(|e| match e {
                    RimEnd::Foot(crease) => {
                        vertex_of(t, &what, &foot_name(&f, end, crease, segment))
                    }
                    RimEnd::Source(v) => vertex_of(
                        t,
                        &what,
                        &minted(
                            EntityKind::Vertex,
                            f.fillet,
                            RoleSeg::FromTarget(NameRef::new(cap_vertex(f.rod, end, v))),
                        ),
                    ),
                });
                let got = ends(body, edge_of(t, &what, &cut));
                assert!(
                    same_pair(got, want),
                    "{what}: the survivor runs {got:?}, not between {want:?}"
                );
            }
        }
        assert_eq!(
            count(t, |s| matches!(s, RoleSeg::BandCut(_))),
            2 * f.rims.len(),
            "{}: one survivor per (cap, cut rim), and no other",
            f.what
        );
    }
}

// ---------------------------------------------------------------- //
// REVIEW PROBES (lane carve-rv, PR #2778). Each records a measured
// blind spot of the rows above; none of them is a row.
// ---------------------------------------------------------------- //

/// **Row 2 cannot tell the sunk rod's two coplanar walls apart.** The
/// doc on `a_cap_foot_lies_in_the_cap_and_on_the_support_its_name_carries`
/// says "both fixtures' walls are told apart by `Ruled::residual`". On
/// `sunk_rod` walls 2 and 4 are two faces of ONE plane (`residual` is
/// `p.y` for both), so a `FootVertex` whose `support` argument named the
/// far wall of the block's top would pass BOTH of that row's checks:
/// the crossed foot is in the same cap plane and has residual zero on
/// the wall it is not on. Rows 3 and 4 are what catch it.
#[test]
fn probe_rv_row2_is_blind_across_the_sunk_rod_s_two_coplanar_walls() {
    let f = sunk_rod();
    let ev = run(&f.doc);
    let (t, source) = (table(&ev, f.fillet), table(&ev, f.rod));
    let (body, rod) = (corpus::body_of(&ev, f.fillet), corpus::body_of(&ev, f.rod));
    for end in CAPS {
        let what = format!("{}: cap {end:?}", f.what);
        // The foot of crease 4 on wall 4, offered where the foot of
        // crease 3 on wall 2 belongs.
        let crossed = point(body, vertex_of(t, &what, &foot_name(&f, end, 4, 4)));
        let z = point(rod, vertex_of(source, &what, &cap_vertex(f.rod, end, 3))).z;
        assert!(
            (crossed.z - z).abs() < NEAR && (f.residual)(2, crossed).abs() < NEAR,
            "{what}: the crossed foot {crossed:?} was expected to satisfy row 2's \
             two checks against (crease 3, wall 2); z want {z}, residual {}",
            (f.residual)(2, crossed)
        );
    }
}

/// **The `NEAR` comment's closest pair is not the one the fixtures
/// produce.** It says the closest pair the rows must tell apart is "a
/// foot and the source vertex it was retracted from, `ROD_FILLET`
/// apart". Measured, the separations run 0.0431 … 0.0599 m, not
/// `ROD_FILLET` = 0.1 m — the window is still four orders clear of the
/// true minimum, but the stated number is not it.
#[test]
fn probe_rv_the_foot_to_source_separation_is_not_rod_fillet() {
    let mut min = f64::INFINITY;
    for f in fixtures() {
        let ev = run(&f.doc);
        let (t, source) = (table(&ev, f.fillet), table(&ev, f.rod));
        let (body, rod) = (corpus::body_of(&ev, f.fillet), corpus::body_of(&ev, f.rod));
        for end in CAPS {
            for &(crease, supports) in f.creases {
                let what = format!("{}: cap {end:?}, crease {crease}", f.what);
                let sv = point(
                    rod,
                    vertex_of(source, &what, &cap_vertex(f.rod, end, crease)),
                );
                for support in supports {
                    let p = point(
                        body,
                        vertex_of(t, &what, &foot_name(&f, end, crease, support)),
                    );
                    let d =
                        ((p.x - sv.x).powi(2) + (p.y - sv.y).powi(2) + (p.z - sv.z).powi(2)).sqrt();
                    min = min.min(d);
                }
            }
        }
    }
    assert!(
        (min - 0.043_099_918_793_752_2).abs() < 1e-12 && min < ROD_FILLET,
        "the closest foot-to-source separation measures {min}, not ROD_FILLET = {ROD_FILLET}"
    );
}

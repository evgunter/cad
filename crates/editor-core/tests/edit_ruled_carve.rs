//! **A ruled carve's band ends, named at the document layer** — a rod
//! with a flat driven through `Node::Fillet`, and the five roles a
//! transverse cap's carve mints, read off the fillet node's name table
//! by six rows: [`RoleSeg::EndArc`] (the cut-off arc),
//! [`RoleSeg::FootVertex`] (the two cap feet), [`RoleSeg::TrimEdge`]
//! (the trimline along each support), [`RoleSeg::BandCut`] (the
//! surviving rim piece) and [`RoleSeg::FromTarget`] (the source cap
//! vertex a survivor still reaches).
//!
//! **What is first here is the RULED band, not the roles.** All five
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
//! the band ADDS it, and the cap gains the region under the arc). Both
//! are the same chord on the same [`ROD_R`] circle at the same
//! [`ROD_FLAT`] standoff (`sweep::test_support::rod_chord_at`): the
//! rod extrudes the arc the flat leaves standing, the sunk rod the arc
//! it cuts away. The sign is not assumed — it is asserted, one
//! comparison each, by
//! [`a_convex_band_removes_material_and_a_concave_one_adds_it`], which
//! is what makes "both material sides" a claim rather than a label.
//!
//! The names are the same set, which is the second claim — the emitter
//! reads no convexity. The concave twin also carries a rim that only
//! ONE crease cuts, so one of its `BandCut` pieces runs from a foot to
//! a surviving source vertex rather than from foot to foot.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    CapEnd, EntityKind, EvalOptions, LoopProgram, NameRef, Node, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg, StableName,
};
// The name-table and body readers, and the name-authoring shorthands,
// live in `fixture` — one home for what this suite and
// `edit_ladder_rim` both read an evaluation with.
use fixture::{
    count, edge_of, ends, face_of, face_vertices, len, minted, point, scl, table, tol, vertex_of,
};
use geom_core::Point3;
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, rod_chord_at};
use topo::{Body, VertexKey};

/// **The closest pair any row here has to tell apart**: a foot and the
/// source cap vertex it was retracted from, measured across both
/// fixtures by
/// [`the_closest_pair_a_row_must_tell_apart_is_a_foot_and_its_source_vertex`],
/// which is also what pins this number.
///
/// It is NOT `ROD_FILLET`: the retraction runs along the support, not
/// along the crease, so it is shorter than the fillet radius — and on
/// the sunk rod's cylinder shorter still, because the concave band's
/// foot sits where the ball rests in the void.
const FOOT_TO_SOURCE: f64 = 0.043_099_918_793_752_2;

/// **The window for the rows that need one**, derived: a seven-decade
/// margin below [`FOOT_TO_SOURCE`]. Both fixtures' points are metres of
/// order 1 and every quantity compared through it is a distance, so one
/// absolute window serves them all, and it sits far above `f64` rounding
/// of these arithmetics (~1e-16) and far below the nearest difference a
/// row is asked to see.
///
/// It is used ONLY where the compared value is computed — a distance
/// from a surface, a separation. Where the comparison is against a
/// coordinate the source body STORES, the rows compare exactly; a
/// window there would report agreement the carve does not actually
/// deliver.
const NEAR: f64 = FOOT_TO_SOURCE * 1e-7;

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

/// Which way a fixture's bands move material — the fixture's material
/// side, asserted by
/// [`a_convex_band_removes_material_and_a_concave_one_adds_it`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    /// Convex creases: the band is carved out of the solid.
    Removes,
    /// Concave creases: the band fills the notch in.
    Adds,
}

/// One ruled-carve fixture: the document, its extrude and fillet nodes,
/// and the structure the rows walk.
struct Ruled {
    what: &'static str,
    doc: ProfileDoc,
    rod: RecipeNodeId,
    fillet: RecipeNodeId,
    /// Which way the carve moves material on this fixture.
    side: Side,
    /// Per crease: the profile vertex its lateral edge stands at, and
    /// the two profile segments whose walls support it.
    creases: &'static [(u32, [u32; 2])],
    /// Per cap: the rim segments the carve cut, and what each
    /// survivor's two ends are.
    rims: &'static [(u32, [RimEnd; 2])],
    /// How far `p` is from the SURFACE segment `seg`'s wall is a region
    /// of — zero on it, and zero on every other face of the same
    /// surface, which is why it is only half of what the foot row
    /// checks (see [`support_face`]).
    residual: fn(u32, Point3<f64>) -> f64,
}

/// A one-loop extruded document with its ruling creases filleted.
fn carve(
    what: &'static str,
    side: Side,
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
            ids: Vec::new(),
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
        .map(|&(v, _)| lateral_edge(&doc, rod, v))
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
        side,
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
    let c = rod_chord_at(ROD_FLAT);
    let lp = LoopProgram::Chain(vec![
        ProgramStep::At([len(ROD_FLAT), len(c.half)]),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point([len(ROD_FLAT), len(-c.half)]),
            b: scl(c.wall_bulge),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    carve(
        "d_rod",
        Side::Removes,
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

/// **A rod's section standing on a block's top edge**: the SECTION arc
/// of the [`ROD_R`] circle about `(0, −ROD_FLAT)` — the piece the rod's
/// own flat cuts away — rising above `y = 0`, on a block whose top
/// plane it interrupts. Same circle, same standoff, same chord as
/// [`d_rod`]; the other arc of it. Its two creases are CONCAVE — the
/// band ADDS material and the cap GAINS the region under the arc.
///
/// The arc is segment 3; the top plane is TWO faces (segments 2 and 4),
/// so the two creases share the cylinder support and have different
/// plane supports. Segment 3's cap rim is cut by both creases,
/// segments 2's and 4's by one each.
fn sunk_rod() -> Ruled {
    let c = rod_chord_at(ROD_FLAT);
    let xv = c.half;
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let lp = LoopProgram::Chain(vec![
        ProgramStep::At(pt(-1.0, -1.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(xv, 0.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(pt(-xv, 0.0)),
            b: scl(c.section_bulge),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(pt(-1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    carve(
        "sunk_rod",
        Side::Adds,
        lp,
        ROD_L,
        &[(3, [2, 3]), (4, [3, 4])],
        &[
            (2, [RimEnd::Source(2), RimEnd::Foot(3)]),
            (3, [RimEnd::Foot(3), RimEnd::Foot(4)]),
            (4, [RimEnd::Foot(4), RimEnd::Source(5)]),
        ],
        |seg, p| match seg {
            2 | 4 => p.y,
            3 => (p.x * p.x + (p.y + ROD_FLAT) * (p.y + ROD_FLAT)).sqrt() - ROD_R,
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
fn lateral_edge(doc: &editor_core::ProfileDoc, rod: RecipeNodeId, vertex: u32) -> StableName {
    fixture::ename(
        rod,
        RoleSeg::LateralEdge(crate::fixture::vpiece(doc, rod, 0, vertex as usize)),
    )
}

fn wall(doc: &editor_core::ProfileDoc, rod: RecipeNodeId, segment: u32) -> StableName {
    fixture::fname(
        rod,
        RoleSeg::Lateral(crate::fixture::piece(doc, rod, 0, segment as usize)),
    )
}

/// Both fixtures' rods extrude ONE profile loop, so a cap entity of
/// theirs is fixed by its cap end and its index in that loop.
fn rim_edge(
    doc: &editor_core::ProfileDoc,
    rod: RecipeNodeId,
    end: CapEnd,
    segment: u32,
) -> StableName {
    fixture::rim_edge(
        rod,
        end,
        crate::fixture::piece(doc, rod, 0, segment as usize),
    )
}

fn cap_vertex(
    doc: &editor_core::ProfileDoc,
    rod: RecipeNodeId,
    end: CapEnd,
    vertex: u32,
) -> StableName {
    fixture::cap_vertex(
        rod,
        end,
        crate::fixture::vpiece(doc, rod, 0, vertex as usize),
    )
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
            vertex: NameRef::new(cap_vertex(&f.doc, f.rod, end, crease)),
            support: NameRef::new(wall(&f.doc, f.rod, support)),
        },
    )
}

/// The carve's own survivor of the wall a `support` argument names —
/// the face the foot has to be a vertex OF, not merely on the surface
/// of. A shrunk support survives as [`RoleSeg::FromTarget`] of its
/// extrude name.
fn support_face(f: &Ruled, support: u32) -> StableName {
    minted(
        EntityKind::Face,
        f.fillet,
        RoleSeg::FromTarget(NameRef::new(wall(&f.doc, f.rod, support))),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol())
        .expect("closed-form mass properties")
        .volume
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
///
/// **What does NOT red it: a swapped `support`.** The two feet of one
/// cap vertex are the same SET whichever wall each of them is named on,
/// and this row reads the set. Permuting the `support` argument across
/// them leaves this row green — measured, not assumed. That is a
/// statement of what this row covers;
/// [`a_cap_foot_lies_in_the_cap_and_on_the_support_its_name_carries`]
/// and [`a_trimline_runs_between_the_two_feet_on_its_own_support`] are
/// where the `support` argument is load-bearing.
#[test]
fn a_cut_off_arc_runs_between_the_two_feet_of_the_cap_it_closes_at() {
    for f in fixtures() {
        let ev = fixture::run(&f.doc, &EvalOptions::default());
        let t = table(&ev, f.fillet);
        let body = corpus::body_of(&ev, f.fillet);
        for end in CAPS {
            for &(crease, supports) in f.creases {
                let what = format!("{}: cap {end:?}, crease {crease}", f.what);
                let arc = minted(
                    EntityKind::Edge,
                    f.fillet,
                    RoleSeg::EndArc {
                        vertex: NameRef::new(cap_vertex(&f.doc, f.rod, end, crease)),
                        edge: NameRef::new(lateral_edge(&f.doc, f.rod, crease)),
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
/// reads both against the body: the foot's height IS the height of the
/// source cap vertex its `vertex` argument names — exactly, the cap
/// plane being stored, not recomputed — and it is a vertex of the
/// carve's own survivor of the wall its `support` argument names, at
/// distance zero from that wall's surface.
///
/// The runtime values that make it false: the foot's own coordinates,
/// and the vertex list of the support face. A foot minted against the
/// other cap lands a rod-length away; one minted against the other of
/// its two supports lands off that surface by the band's bite.
///
/// **The surface alone is not enough, which is why the face is read.**
/// [`Ruled::residual`] measures distance from the SURFACE a wall is a
/// region of, and on `sunk_rod` walls 2 and 4 are two regions of ONE
/// plane (`residual` is `p.y` for both), so a foot naming the far wall
/// of the block's top would sit at residual zero on it. The extent —
/// the face's own vertex list — is what tells them apart, and
/// [`a_coplanar_wall_is_told_from_its_twin_by_the_support_face`]
/// measures exactly that: the crossed foot passing the plane-and-
/// residual pair, and failing the face.
#[test]
fn a_cap_foot_lies_in_the_cap_and_on_the_support_its_name_carries() {
    for f in fixtures() {
        let ev = fixture::run(&f.doc, &EvalOptions::default());
        let (t, source) = (table(&ev, f.fillet), table(&ev, f.rod));
        let (body, rod) = (corpus::body_of(&ev, f.fillet), corpus::body_of(&ev, f.rod));
        for end in CAPS {
            for &(crease, supports) in f.creases {
                // The cap plane, read off the SOURCE vertex the band
                // ends at rather than off the carve's own output.
                let what = format!("{}: cap {end:?}, crease {crease}", f.what);
                let z = point(
                    rod,
                    vertex_of(source, &what, &cap_vertex(&f.doc, f.rod, end, crease)),
                )
                .z;
                for support in supports {
                    let foot = vertex_of(t, &what, &foot_name(&f, end, crease, support));
                    let p = point(body, foot);
                    assert_eq!(
                        p.z, z,
                        "{what}: the foot on wall {support} sits at z = {}, not in its cap at {z}",
                        p.z
                    );
                    let d = (f.residual)(support, p);
                    assert!(
                        d.abs() < NEAR,
                        "{what}: the foot named on wall {support} is {d} off its surface"
                    );
                    let face = face_of(t, &what, &support_face(&f, support));
                    assert!(
                        face_vertices(body, face).contains(&foot),
                        "{what}: the foot named on wall {support} is not a vertex of that \
                         wall's face — it is on the surface but off the face's extent"
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
        let ev = fixture::run(&f.doc, &EvalOptions::default());
        let t = table(&ev, f.fillet);
        let body = corpus::body_of(&ev, f.fillet);
        for &(crease, supports) in f.creases {
            for support in supports {
                let what = format!("{}: crease {crease} on wall {support}", f.what);
                let trim = minted(
                    EntityKind::Edge,
                    f.fillet,
                    RoleSeg::TrimEdge {
                        edge: NameRef::new(lateral_edge(&f.doc, f.rod, crease)),
                        support: NameRef::new(wall(&f.doc, f.rod, support)),
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
        assert_eq!(
            count(t, |s| matches!(s, RoleSeg::TrimEdge { .. })),
            2 * f.creases.len(),
            "{}: one trimline per (crease, support) — both ends of one band share it — \
             and no other",
            f.what
        );
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
        let ev = fixture::run(&f.doc, &EvalOptions::default());
        let t = table(&ev, f.fillet);
        let body = corpus::body_of(&ev, f.fillet);
        for end in CAPS {
            for &(segment, rim_ends) in f.rims {
                let what = format!("{}: cap {end:?}, rim {segment}", f.what);
                let cut = minted(
                    EntityKind::Edge,
                    f.fillet,
                    RoleSeg::BandCut(NameRef::new(rim_edge(&f.doc, f.rod, end, segment))),
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
                            RoleSeg::FromTarget(NameRef::new(cap_vertex(&f.doc, f.rod, end, v))),
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
// The rows the review measured
// ---------------------------------------------------------------- //

/// **A convex band removes material; a concave one adds it.** The two
/// documents are the same chord on the same circle — the D-rod extrudes
/// the arc the flat leaves standing, the sunk rod the arc it cuts away
/// — so nothing but this row says which side of the material each
/// carve is on, and the module header's "both material sides" would
/// hold of two copies of one fixture without it.
///
/// The runtime value that makes it false: the sign of `ΔV`, the
/// filleted body's volume less the extrude's, both read off the
/// evaluated bodies. The magnitude is not pinned here — the closed form
/// is `sweep`'s, and `review_fillet_h7_r1_probes` holds it against the
/// section oracle at both senses. Swapping `sunk_rod`'s bulge for the
/// groove's — the wall arc traversed the other way, so the section
/// DIPS into the block instead of standing on it — reds this row and
/// none of the four name rows; the only other row that sees it is
/// [`the_closest_pair_a_row_must_tell_apart_is_a_foot_and_its_source_vertex`],
/// which pins a measured number of this fixture.
#[test]
fn a_convex_band_removes_material_and_a_concave_one_adds_it() {
    for f in fixtures() {
        let ev = fixture::run(&f.doc, &EvalOptions::default());
        let dv = volume(corpus::body_of(&ev, f.fillet)) - volume(corpus::body_of(&ev, f.rod));
        match f.side {
            Side::Removes => assert!(
                dv < 0.0,
                "{}: a convex band removes material, ΔV = {dv}",
                f.what
            ),
            Side::Adds => assert!(
                dv > 0.0,
                "{}: a concave band adds material, ΔV = {dv}",
                f.what
            ),
        }
    }
}

/// **Two coplanar walls are told apart by the support FACE, not by its
/// plane.** `sunk_rod`'s walls 2 and 4 are two regions of the one plane
/// `y = 0`, so the pair of checks a `FootVertex`'s arguments invite —
/// the cap plane the `vertex` argument fixes, and the distance from the
/// surface the `support` argument names — accepts a foot from the far
/// wall. This row measures both halves of that: the crossed foot
/// SATISFIES the plane-and-residual pair, and is NOT a vertex of the
/// face it would have to be named on.
///
/// The runtime values that make it false: the crossed foot's own
/// coordinates and the vertex list of wall 2's face. It is what makes
/// [`a_cap_foot_lies_in_the_cap_and_on_the_support_its_name_carries`]'s
/// third check load-bearing rather than redundant — drop that check and
/// this fixture's rows go green under a swapped `support`.
#[test]
fn a_coplanar_wall_is_told_from_its_twin_by_the_support_face() {
    let f = sunk_rod();
    let ev = fixture::run(&f.doc, &EvalOptions::default());
    let (t, source) = (table(&ev, f.fillet), table(&ev, f.rod));
    let (body, rod) = (corpus::body_of(&ev, f.fillet), corpus::body_of(&ev, f.rod));
    for end in CAPS {
        let what = format!("{}: cap {end:?}", f.what);
        // The foot of crease 4 on wall 4, offered where the foot of
        // crease 3 on wall 2 belongs.
        let crossed = vertex_of(t, &what, &foot_name(&f, end, 4, 4));
        let p = point(body, crossed);
        let z = point(
            rod,
            vertex_of(source, &what, &cap_vertex(&f.doc, f.rod, end, 3)),
        )
        .z;
        assert!(
            p.z == z && (f.residual)(2, p).abs() < NEAR,
            "{what}: the crossed foot {p:?} was expected to satisfy the plane and residual \
             checks against (crease 3, wall 2); z want {z}, residual {}",
            (f.residual)(2, p)
        );
        let far = face_of(t, &what, &support_face(&f, 2));
        assert!(
            !face_vertices(body, far).contains(&crossed),
            "{what}: the crossed foot IS a vertex of wall 2's face, so the extent check \
             cannot tell the block's top's two faces apart either"
        );
    }
}

/// **The closest pair a row must tell apart is a foot and the source
/// cap vertex it was retracted from**, and it measures
/// [`FOOT_TO_SOURCE`] — not [`ROD_FILLET`], which is the radius the
/// crease is carved at and bounds nothing here: the retraction runs
/// along the support, not along the crease.
///
/// The runtime value that makes it false: the minimum separation over
/// both fixtures, every cap, every crease and both supports. This row
/// is what licenses [`NEAR`]: it is that minimum's 1e-7, so a row
/// comparing through the window cannot be confusing a foot for its
/// source.
#[test]
fn the_closest_pair_a_row_must_tell_apart_is_a_foot_and_its_source_vertex() {
    let mut min = f64::INFINITY;
    for f in fixtures() {
        let ev = fixture::run(&f.doc, &EvalOptions::default());
        let (t, source) = (table(&ev, f.fillet), table(&ev, f.rod));
        let (body, rod) = (corpus::body_of(&ev, f.fillet), corpus::body_of(&ev, f.rod));
        for end in CAPS {
            for &(crease, supports) in f.creases {
                let what = format!("{}: cap {end:?}, crease {crease}", f.what);
                let sv = point(
                    rod,
                    vertex_of(source, &what, &cap_vertex(&f.doc, f.rod, end, crease)),
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
        (min - FOOT_TO_SOURCE).abs() < 1e-12,
        "the closest foot-to-source separation measures {min}, not FOOT_TO_SOURCE = \
         {FOOT_TO_SOURCE}"
    );
    assert!(
        NEAR < min / 1e6,
        "NEAR = {NEAR} is not a decade-clear margin below the separation {min} it is \
         derived from"
    );
}

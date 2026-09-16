//! **A ladder rim's band, named at the document layer** — a plate with
//! two round holes driven through `Node::Fillet`, and the four roles
//! whose ARGUMENTS no document row read before this suite:
//! [`RoleSeg::BandFoot`], [`RoleSeg::BandCross`],
//! [`RoleSeg::BandFace`] and [`RoleSeg::BandSlit`].
//!
//! **What is first here is the ARGUMENT, not the roles.** All four
//! already reach `names::emit_fillet` from the registry — the corpus's
//! `die_composed` and `die_composed_tour` carve LADDER rims at their pip
//! cavities, and `blend5_rim_support` drives the ANNULUS arm — but no
//! row anywhere reads what the four names CARRY. What
//! `emit_blend::name_blend`'s `check_total` buys is that SOME name
//! reaches every output entity; what it cannot see is a name whose
//! argument is wrong. A `BandFoot` carrying the wrong source rim vertex,
//! a `BandCross` carrying the neighbouring meridian, a `BandFace`
//! carrying the other rim's edge set and a `BandSlit` carrying the
//! other meridian all name real entities, pass totality, and pass every
//! count in the tree —
//! [`the_totality_and_the_counts_read_no_argument_at_all`] is that
//! statement, executed.
//!
//! # The walk this suite watches
//!
//! `sweep::blend::surgery`'s ladder arm carves a closed rim that is a
//! RING of its planar support: per rim vertex it splits the one
//! meridian descending from that vertex into the wall (minting the
//! band's MATE-side crossing), struts the HOST side out to the widened
//! trim circle (minting the foot), runs trim arcs around both sides,
//! and closes the ring at one crossing, where the upper meridian
//! remnant fan-merges onto the foot and becomes the band's slit. The
//! rows below read that walk's output through the names: each foot is
//! the radial retraction of the source rim vertex its name carries,
//! each crossing is a split of the meridian its name carries, each
//! band's boundary carries the rim edge set its name carries, and each
//! slit runs from the crossing on its own meridian to the foot at that
//! meridian's rim vertex.
//!
//! # Why a suite and not a corpus document
//!
//! `tests/corpus` is the Band 4 REGISTRY (`corpus/mod.rs`): a document
//! there buys the whole battery — the interval lane, the persistence
//! round trip, the latency table's baseline row, the name-digest
//! goldens — and pays for it in every one of those baselines. What
//! these rows need is none of that: they need a ladder rim carved once
//! and its minted names READ, which is the shape `blend5_rim_support`
//! set for the annulus and `edit_ruled_carve` for the ruled band.
//! Driving `die_composed` from the registry instead would pay that
//! battery a second time for a document already in it, and would read
//! the four arguments through a sphere cavity's two half-caps rather
//! than through the smallest body that mints them.
//!
//! # Two rims, and why one is not enough
//!
//! One hole mints all four roles, and would be the smaller document.
//! It would also make two of the four mutants these rows are written
//! against VACUOUS: `sweep::blend::naming::BlendNaming`'s `bands` and
//! `slits` carry one row per closed rim, so on a single rim permuting
//! either channel's source argument is the identity. Two rims is the
//! smallest document in which every one of the four permutations moves
//! a name.
//!
//! The two holes differ in centre AND radius, so no row can pass by
//! confusing one rim's geometry for the other's, and each hole's two
//! profile vertices sit at opposite azimuths, so no row can pass by
//! confusing a rim's two ends.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeSet, HashSet};

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, LoopProgram,
    NameRef, NameTable, Node, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef,
    ProgramStep, ProgramTarget, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::len;
use geom_core::{Point3, Tol};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, VertexKey};

/// The plate's half-extent in the sketch plane, metres.
const PLATE: f64 = 1.0;
/// The plate's thickness along `+z`, metres.
const THICK: f64 = 1.0;
/// The one blend radius, metres — under both hole radii, and under half
/// the plate's thickness, so each band clears the far cap.
const R: f64 = 0.05;

/// One of the document's two round holes: the profile loop it is, and
/// the circle it is in the sketch plane.
struct Rim {
    loop_index: u32,
    centre: (f64, f64),
    radius: f64,
}

/// The two holes. Different centres and different radii, so a row that
/// confused one rim for the other would have to agree with the wrong
/// circle.
const RIMS: [Rim; 2] = [
    Rim {
        loop_index: 1,
        centre: (-0.45, 0.0),
        radius: 0.25,
    },
    Rim {
        loop_index: 2,
        centre: (0.45, 0.0),
        radius: 0.30,
    },
];

/// **The closest pair any row here has to tell apart**: a band foot and
/// the source rim vertex it was retracted from, and a band crossing and
/// the end of the meridian it splits. Both retractions run along a
/// support meeting the cap plane at a right angle, so both measure the
/// blend radius — measured across the whole document and pinned by
/// [`the_closest_pair_a_row_must_tell_apart_is_a_mint_and_its_source`],
/// which is also what licenses [`NEAR`].
const CLOSEST: f64 = 0.05;

/// **The window for the rows that need one**, derived: a seven-decade
/// margin below [`CLOSEST`]. Every point in this document is metres of
/// order 1 and every quantity compared through it is a distance, so one
/// absolute window serves them all; it sits far above `f64` rounding of
/// these arithmetics (~1e-16) and far below the nearest difference a
/// row is asked to see.
///
/// It is used ONLY where the compared value is computed — a retraction,
/// a separation. Where the comparison is against a coordinate the
/// source body STORES, the rows compare exactly; a window there would
/// report agreement the carve does not actually deliver.
const NEAR: f64 = CLOSEST * 1e-7;

fn tol() -> Tol {
    Tol::witness()
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

// ---------------------------------------------------------------- //
// The document
// ---------------------------------------------------------------- //

/// **The plate**: a square slab of side `2 * PLATE` and thickness
/// [`THICK`], pierced by the two holes of [`RIMS`], with the `End`
/// cap's two hole rims filleted in ONE `Node::Fillet`.
///
/// Each hole rim is a LADDER rim: it is a RING of the cap plane — the
/// planar support, and therefore the host — and each of the two wall
/// faces the extrude mints for a circular loop carries exactly one of
/// its arcs. One node carries both rims, which is what puts two rows in
/// every one of the record channels the mutants permute.
fn plate() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("edit_ladder_rim", tol());
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let outline = LoopProgram::Chain(vec![
        ProgramStep::At([len(-PLATE), len(-PLATE)]),
        ProgramStep::LineTo(ProgramTarget::Point([len(PLATE), len(-PLATE)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(PLATE), len(PLATE)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(-PLATE), len(PLATE)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let mut loops = vec![outline];
    for r in &RIMS {
        loops.push(
            LoopProgram::circle(r.centre.0, r.centre.1, r.radius).expect("a finite hole circle"),
        );
    }
    let (doc, profile) = fixture::insert(doc, Node::Profile(ProfileProgram { plane, loops }));
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(THICK),
        },
    );
    let mut selection: Vec<StableName> = RIMS
        .iter()
        .flat_map(|r| (0..2).map(|s| rim_edge(block, r, s)))
        .collect();
    selection.sort();
    let (doc, fillet) = fixture::insert(
        doc,
        Node::Fillet {
            target: block,
            radius: len(R),
            selection,
        },
    );
    (doc, block, fillet)
}

// ---------------------------------------------------------------- //
// The source vocabulary the arguments are drawn from
// ---------------------------------------------------------------- //

/// One arc of a hole's rim on the filleted cap — a source rim edge, and
/// what a [`RoleSeg::BandFace`] argument is a set of.
fn rim_edge(block: RecipeNodeId, rim: &Rim, segment: u32) -> StableName {
    fixture::ename(
        block,
        RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: rim.loop_index,
                segment,
            },
        ),
    )
}

/// A source rim VERTEX on the filleted cap — what a
/// [`RoleSeg::BandFoot`] argument names.
fn cap_vertex(block: RecipeNodeId, rim: &Rim, vertex: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node: block,
        path: vec![RoleSeg::CapVertex(
            CapEnd::End,
            ProfileVertexRef {
                loop_index: rim.loop_index,
                vertex,
            },
        )],
    }
}

/// The MERIDIAN descending from a rim vertex into the hole's wall — the
/// extrude's lateral edge at the same profile vertex, and what a
/// [`RoleSeg::BandCross`] and a [`RoleSeg::BandSlit`] argument name.
fn meridian(block: RecipeNodeId, rim: &Rim, vertex: u32) -> StableName {
    fixture::ename(
        block,
        RoleSeg::LateralEdge(ProfileVertexRef {
            loop_index: rim.loop_index,
            vertex,
        }),
    )
}

/// The carve's own survivor of the filleted cap — the HOST support face
/// every foot has to be a vertex OF, not merely in the plane of.
fn host_support(block: RecipeNodeId, fillet: RecipeNodeId) -> StableName {
    minted(
        EntityKind::Face,
        fillet,
        RoleSeg::FromTarget(NameRef::new(fixture::fname(
            block,
            RoleSeg::Cap(CapEnd::End),
        ))),
    )
}

/// A name minted by `node` in the blend vocabulary.
fn minted(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// The rim a source name belongs to, decoded from the name's own
/// profile-loop anchoring — which is how a row keyed on an ARGUMENT
/// finds the circle that argument sits on.
fn rim_of(n: &StableName) -> &'static Rim {
    let l = match n.path.first() {
        Some(RoleSeg::LateralEdge(v) | RoleSeg::CapVertex(_, v)) => v.loop_index,
        Some(RoleSeg::RimEdge(_, e)) => e.loop_index,
        other => panic!("{other:?} is not anchored at a profile loop"),
    };
    RIMS.iter()
        .find(|r| r.loop_index == l)
        .unwrap_or_else(|| panic!("{n:?} names profile loop {l}, which no rim of this plate is"))
}

// ---------------------------------------------------------------- //
// Reading the document and its names
// ---------------------------------------------------------------- //

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

fn face_of(t: &NameTable, what: &str, n: &StableName) -> FaceKey {
    match key_of(t, what, n) {
        EntityKey::Face(k) => k,
        other => panic!("{what}: {n:?} names {other:?}, not a face"),
    }
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    topo::readback::vertex_point(body, v).expect("a live vertex")
}

/// An edge's two end vertices.
fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let edge = body.get_edge(e).expect("a live edge");
    let h = body.get_half_edge(edge.he_plus).expect("a live half-edge");
    let far = body.half_edge_end(edge.he_plus).expect("a forward half");
    [h.start, far]
}

/// Every vertex on `f`'s boundary — the face's own EXTENT, read out of
/// the body rather than inferred from the surface it is a region of.
fn face_vertices(body: &Body<f64>, f: FaceKey) -> HashSet<VertexKey> {
    let face = body.get_face(f).expect("a live face");
    let mut out = HashSet::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        match body.get_loop(lk).expect("a live loop").boundary {
            LoopBoundary::Empty { vertex } => {
                out.insert(vertex);
            }
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first).expect("a closed cycle") {
                    out.insert(body.get_half_edge(he).expect("a live half-edge").start);
                }
            }
        }
    }
    out
}

/// Every edge on `f`'s boundary.
fn face_edges(body: &Body<f64>, f: FaceKey) -> HashSet<EdgeKey> {
    let face = body.get_face(f).expect("a live face");
    let mut out = HashSet::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        if let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("a live loop").boundary {
            for he in body.loop_cycle(first).expect("a closed cycle") {
                out.insert(body.get_half_edge(he).expect("a live half-edge").edge);
            }
        }
    }
    out
}

/// How many names in `t` take `seg`'s role.
fn count(t: &NameTable, seg: fn(&RoleSeg) -> bool) -> usize {
    t.iter().filter(|(n, _)| seg(&n.path[0])).count()
}

/// How far `p` stands from the axis of `rim`'s hole. The hole's wall is
/// generated straight along `+z`, so this and the azimuth are the whole
/// of `p`'s position relative to the circle an argument names.
fn axis_distance(rim: &Rim, p: Point3<f64>) -> f64 {
    (p.x - rim.centre.0).hypot(p.y - rim.centre.1)
}

/// The footprint of `p` moved `d` further out along its own ray from
/// `rim`'s axis — where a mint retracted from the source entity at `p`,
/// at that entity's own azimuth, has to stand.
fn retracted(rim: &Rim, p: Point3<f64>, d: f64) -> (f64, f64) {
    let (dx, dy) = (p.x - rim.centre.0, p.y - rim.centre.1);
    let s = (d + dx.hypot(dy)) / dx.hypot(dy);
    (rim.centre.0 + s * dx, rim.centre.1 + s * dy)
}

/// Every source rim vertex of the document, with the hole it belongs to
/// — the population a row saying "the rim its name carries, and not the
/// other" discriminates within.
fn all_rim_vertices(
    src: &NameTable,
    sbody: &Body<f64>,
    block: RecipeNodeId,
) -> Vec<(u32, Point3<f64>)> {
    RIMS.iter()
        .flat_map(|r| (0..2).map(move |j| (r.loop_index, cap_vertex(block, r, j))))
        .map(|(l, n)| (l, point(sbody, vertex_of(src, "a source rim vertex", &n))))
        .collect()
}

fn dist(a: Point3<f64>, b: Point3<f64>) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

// ---------------------------------------------------------------- //
// The rows
// ---------------------------------------------------------------- //

/// **A band foot is the host-support vertex retracted from its source
/// rim vertex.** [`RoleSeg::BandFoot`] carries one argument — a source
/// rim vertex — and this row reads the vertex the name resolves to
/// against it: the foot stands at the SAME height as that vertex,
/// exactly, the cap plane being stored rather than recomputed; it sits
/// on the ray from that hole's axis through that vertex, one blend
/// radius further out, which is where the strut along the host support
/// puts it; and it is a vertex OF the carve's own survivor of the
/// filleted cap, not merely a point in that cap's plane.
///
/// The runtime values that make it false: the foot's own coordinates,
/// and the vertex list of the host support face. A `BandFoot` carrying
/// the other end of its own rim lands at the opposite azimuth, a
/// diameter away; one carrying the other hole's vertex lands on the
/// wrong circle entirely.
///
/// **The extent check is not what tells the rims apart** — all four
/// feet are vertices of the ONE cap face here, and the retraction is
/// what discriminates. What the extent adds is that the foot is a
/// corner of the host support's own boundary, which is the difference
/// between a strut that reached the trimline and a point that merely
/// satisfies the trim circle's equation.
#[test]
fn a_band_foot_is_the_host_support_vertex_retracted_from_its_source_rim_vertex() {
    let (doc, block, fillet) = plate();
    let ev = run(&doc);
    let (t, src) = (table(&ev, fillet), table(&ev, block));
    let (body, sbody) = (corpus::body_of(&ev, fillet), corpus::body_of(&ev, block));
    let cap = face_of(t, "the host support", &host_support(block, fillet));
    let extent = face_vertices(body, cap);
    for rim in &RIMS {
        for j in 0..2 {
            let what = format!("hole {}, profile vertex {j}", rim.loop_index);
            let source = cap_vertex(block, rim, j);
            let s = point(sbody, vertex_of(src, &what, &source));
            let foot = vertex_of(
                t,
                &what,
                &minted(
                    EntityKind::Vertex,
                    fillet,
                    RoleSeg::BandFoot(NameRef::new(source)),
                ),
            );
            let p = point(body, foot);
            assert_eq!(
                p.z, s.z,
                "{what}: the foot stands at z = {}, not in the cap plane of the rim vertex \
                 its name carries, at {}",
                p.z, s.z
            );
            let (wx, wy) = retracted(rim, s, R);
            assert!(
                (p.x - wx).abs() < NEAR && (p.y - wy).abs() < NEAR,
                "{what}: the foot is at ({}, {}); the rim vertex its name carries, retracted \
                 one blend radius along the host support, is at ({wx}, {wy})",
                p.x,
                p.y
            );
            assert!(
                extent.contains(&foot),
                "{what}: the foot is not a vertex of the carve's survivor of the filleted \
                 cap — it is in the host support's plane but off that face's extent"
            );
        }
    }
}

/// **A band crossing lies on the meridian its name carries.**
/// [`RoleSeg::BandCross`] carries the source edge the band's MATE-side
/// trimline crossed — on a ladder rim, the one meridian descending from
/// a rim vertex into the wall. This row reads the vertex the name
/// resolves to against that edge: the crossing stands on the meridian,
/// STRICTLY between its two ends (a split, not an end kept), and off
/// every OTHER meridian of the document.
///
/// The runtime values that make it false: the crossing's own
/// coordinates. A `BandCross` carrying its own hole's other meridian
/// lands a diameter away; one carrying the other hole's lands on the
/// wrong circle.
#[test]
fn a_band_crossing_lies_on_the_meridian_its_name_carries() {
    let (doc, block, fillet) = plate();
    let ev = run(&doc);
    let (t, src) = (table(&ev, fillet), table(&ev, block));
    let (body, sbody) = (corpus::body_of(&ev, fillet), corpus::body_of(&ev, block));
    // Every meridian of the document, as the two ends the source body
    // stores for it — the population the "and not its neighbour" arm
    // rules out.
    let all: Vec<(u32, u32, [Point3<f64>; 2])> = RIMS
        .iter()
        .flat_map(|r| (0..2).map(move |j| (r.loop_index, j, meridian(block, r, j))))
        .map(|(l, j, n)| {
            let [a, b] = ends(sbody, edge_of(src, "a source meridian", &n));
            (l, j, [point(sbody, a), point(sbody, b)])
        })
        .collect();
    for rim in &RIMS {
        for j in 0..2 {
            let what = format!("hole {}, meridian {j}", rim.loop_index);
            let source = meridian(block, rim, j);
            let cross = vertex_of(
                t,
                &what,
                &minted(
                    EntityKind::Vertex,
                    fillet,
                    RoleSeg::BandCross(NameRef::new(source)),
                ),
            );
            let p = point(body, cross);
            for &(l, k, [a, b]) in &all {
                // A lateral edge of an extruded loop runs straight along
                // `+z`, so a point's distance from it is the distance
                // between their footprints; both ends fix that
                // footprint and both are read.
                let d = (p.x - a.x).hypot(p.y - a.y).max((p.x - b.x).hypot(p.y - b.y));
                if (l, k) == (rim.loop_index, j) {
                    assert!(
                        d < NEAR,
                        "{what}: the crossing is {d} off the meridian its name carries"
                    );
                    assert!(
                        dist(p, a) > NEAR && dist(p, b) > NEAR,
                        "{what}: the crossing sits at an END of the meridian its name \
                         carries, so nothing was split there"
                    );
                } else {
                    assert!(
                        d > NEAR,
                        "{what}: the crossing is {d} off hole {l}'s meridian {k} as well, so \
                         lying on a meridian would not say WHICH"
                    );
                }
            }
        }
    }
}

/// **A band face carries the set of rim edges it rounds.**
/// [`RoleSeg::BandFace`]'s argument is the SET of source rim edges the
/// closed chain was — a rim is a cycle with no first edge, so only the
/// set is covariant. This row reads the face the name resolves to
/// against that set twice over: the `BandTrim` names on the face's own
/// BOUNDARY carry exactly those source rim edges, as a set equality and
/// not a count; and every vertex of that boundary is nearer to a source
/// rim vertex of the named rim than to any rim vertex of the other.
///
/// The runtime values that make it false: the face's boundary edge list
/// and its vertices' coordinates. A `BandFace` carrying the other
/// hole's rim edges names a real torus band of a real rim, with the
/// right number of source edges in it, and fails both arms here.
///
/// The two arms are independent witnesses of one claim: `BandTrim`'s
/// `edge` argument travels in a different record channel from the
/// band's, so a permutation of one leaves the other in place, and the
/// second arm reads no name at all.
#[test]
fn a_band_face_carries_the_set_of_rim_edges_it_rounds() {
    let (doc, block, fillet) = plate();
    let ev = run(&doc);
    let (t, src) = (table(&ev, fillet), table(&ev, block));
    let (body, sbody) = (corpus::body_of(&ev, fillet), corpus::body_of(&ev, block));
    let population = all_rim_vertices(src, sbody, block);
    for rim in &RIMS {
        let what = format!("hole {}", rim.loop_index);
        let mut set: Vec<StableName> = (0..2).map(|s| rim_edge(block, rim, s)).collect();
        set.sort();
        let want: BTreeSet<StableName> = set.iter().cloned().collect();
        let band = face_of(
            t,
            &what,
            &minted(EntityKind::Face, fillet, RoleSeg::BandFace(set)),
        );
        let got: BTreeSet<StableName> = face_edges(body, band)
            .into_iter()
            .filter_map(|e| {
                t.iter().find_map(|(n, entry)| match (&n.path[0], entry) {
                    (RoleSeg::BandTrim { edge, .. }, Entry::Unique(r))
                        if r.key == EntityKey::Edge(e) =>
                    {
                        Some((**edge).clone())
                    }
                    _ => None,
                })
            })
            .collect();
        assert_eq!(
            got, want,
            "{what}: the band's boundary trimlines carry {got:?}, not the rim edge set the \
             band's own name carries"
        );
        for v in face_vertices(body, band) {
            let p = point(body, v);
            let nearest = population
                .iter()
                .min_by(|a, b| {
                    dist(p, a.1)
                        .partial_cmp(&dist(p, b.1))
                        .expect("finite separations")
                })
                .expect("the plate has rim vertices");
            assert_eq!(
                nearest.0, rim.loop_index,
                "{what}: a vertex of the band's boundary at {p:?} is nearest hole {}'s rim, \
                 so the band does not round the rim its name carries",
                nearest.0
            );
        }
    }
}

/// **A slit runs along the meridian it was slit along.** The ladder
/// closes its ring at one crossing: the rim vertex there dies, and the
/// upper piece of that vertex's meridian fan-merges onto the foot and
/// becomes the band's slit — the double-traversed torus meridian that
/// keeps the annular band ring-free. [`RoleSeg::BandSlit`] carries the
/// source meridian that piece came from, and this row reads the edge the
/// name resolves to against it: its two ends are exactly the crossing
/// minted on THAT meridian and the foot at THAT meridian's rim vertex,
/// and both stand on the ray from the hole's axis through the meridian
/// — which is what "along the meridian" means for a piece that has left
/// the wall for the band's tube.
///
/// The runtime values that make it false: the slit edge's two end
/// vertices and their coordinates. A `BandSlit` carrying the other
/// meridian of its own hole names a real slit, whose ends are the other
/// crossing and the other foot, a diameter away.
///
/// **Which crossing closes the ring is the walk's own choice** and no
/// claim of this row's: it iterates the slits the table carries and
/// requires one per rim, rather than naming the profile vertex the
/// closure happens to land on.
#[test]
fn a_slit_runs_along_the_meridian_it_was_slit_along() {
    let (doc, block, fillet) = plate();
    let ev = run(&doc);
    let (t, src) = (table(&ev, fillet), table(&ev, block));
    let (body, sbody) = (corpus::body_of(&ev, fillet), corpus::body_of(&ev, block));
    let mut served: Vec<u32> = Vec::new();
    for (n, _) in t.iter() {
        let RoleSeg::BandSlit(source) = &n.path[0] else {
            continue;
        };
        let rim = rim_of(source);
        let what = format!("the slit on hole {}", rim.loop_index);
        served.push(rim.loop_index);
        let j = match source.path.first() {
            Some(RoleSeg::LateralEdge(v)) => v.vertex,
            other => panic!("{what}: {other:?} is not a lateral edge of the source"),
        };
        // The extrude anchors a lateral edge and a cap vertex at the
        // SAME profile vertex, so the meridian the slit names fixes the
        // rim vertex whose foot the closure merged that meridian onto.
        let cross = vertex_of(
            t,
            &what,
            &minted(
                EntityKind::Vertex,
                fillet,
                RoleSeg::BandCross(NameRef::new((**source).clone())),
            ),
        );
        let foot = vertex_of(
            t,
            &what,
            &minted(
                EntityKind::Vertex,
                fillet,
                RoleSeg::BandFoot(NameRef::new(cap_vertex(block, rim, j))),
            ),
        );
        let got = ends(body, edge_of(t, &what, n));
        assert!(
            (got[0] == cross && got[1] == foot) || (got[0] == foot && got[1] == cross),
            "{what}: it runs {got:?}, not from the crossing {cross:?} on the meridian its \
             name carries to that meridian's foot {foot:?}"
        );
        let m = point(sbody, ends(sbody, edge_of(src, &what, source))[0]);
        let axis = axis_distance(rim, m);
        for v in got {
            let p = point(body, v);
            let (wx, wy) = retracted(rim, m, axis_distance(rim, p) - axis);
            assert!(
                (p.x - wx).abs() < NEAR && (p.y - wy).abs() < NEAR,
                "{what}: an end at ({}, {}) is off the azimuth of the meridian its name \
                 carries, which at that distance from the axis runs through ({wx}, {wy})",
                p.x,
                p.y
            );
        }
    }
    served.sort_unstable();
    assert_eq!(
        served,
        RIMS.iter().map(|r| r.loop_index).collect::<Vec<_>>(),
        "one slit keeps each of the two bands ring-free, and no slit names a loop that is \
         not a rim of this plate"
    );
}

// ---------------------------------------------------------------- //
// What the rows above are measured against
// ---------------------------------------------------------------- //

/// **The totality check and the counts read no argument at all.**
/// `emit_blend::name_blend` refuses a table that does not name every
/// output entity, so a green table is already the statement that all
/// four roles emitted; and a count is the right instrument for what the
/// suites beside this one say with one — that a band foot is minted on
/// a rim with no planar support (`blend5_r1_probes`), that one band face
/// rounds a rim and one slit keeps it ring-free
/// (`blend5_rim_support`). Neither reads an ARGUMENT: the role filters
/// discard the `NameRef` with a wildcard.
///
/// So this row is green, and STAYS green, under every permutation the
/// four rows above are written against — permuting the source argument
/// across `BlendNaming`'s `rim_feet`, `meridian_splits`, `bands` or
/// `slits` leaves a total, duplicate-free table with these counts
/// unmoved. That is the whole reason those four rows exist, and this is
/// where it is said with the counts in hand.
///
/// The runtime value that makes it false: the size of each role's slice
/// of the table. A role that stopped emitting, or emitted twice, fails
/// here.
#[test]
fn the_totality_and_the_counts_read_no_argument_at_all() {
    let (doc, _block, fillet) = plate();
    let ev = run(&doc);
    let t = table(&ev, fillet);
    let n = RIMS.len();
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandFace(_))),
        n,
        "one band face rounds each rim"
    );
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandTrim { .. })),
        4 * n,
        "one trimline per (rim arc, support)"
    );
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandFoot(_))),
        2 * n,
        "one host foot per rim vertex"
    );
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandCross(_))),
        2 * n,
        "one mate-side crossing per rim vertex"
    );
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandCut(_))),
        2 * n,
        "one surviving meridian piece per rim vertex"
    );
    assert_eq!(
        count(t, |s| matches!(s, RoleSeg::BandSlit(_))),
        n,
        "one slit keeps each band ring-free"
    );
}

/// **The closest pair a row must tell apart is a mint and the source
/// entity it was made at**, and it measures [`CLOSEST`]: a foot from the
/// rim vertex it was retracted from, and a crossing from the end of the
/// meridian it split. Both supports meet the cap plane at a right angle
/// here, so both separations are the blend radius — which is a
/// statement about this plate's geometry rather than a constant to
/// assume, so the row measures it.
///
/// The runtime value that makes it false: the minimum separation over
/// both rims, both ends of each and both kinds of mint. This row is
/// what licenses [`NEAR`]: it is that minimum's 1e-7, so a row comparing
/// through the window cannot be confusing a mint for its source.
#[test]
fn the_closest_pair_a_row_must_tell_apart_is_a_mint_and_its_source() {
    let (doc, block, fillet) = plate();
    let ev = run(&doc);
    let (t, src) = (table(&ev, fillet), table(&ev, block));
    let (body, sbody) = (corpus::body_of(&ev, fillet), corpus::body_of(&ev, block));
    let mut min = f64::INFINITY;
    for rim in &RIMS {
        for j in 0..2 {
            let what = format!("hole {}, profile vertex {j}", rim.loop_index);
            let v = cap_vertex(block, rim, j);
            let s = point(sbody, vertex_of(src, &what, &v));
            let foot = point(
                body,
                vertex_of(
                    t,
                    &what,
                    &minted(EntityKind::Vertex, fillet, RoleSeg::BandFoot(NameRef::new(v))),
                ),
            );
            min = min.min(dist(foot, s));
            let m = meridian(block, rim, j);
            let cross = point(
                body,
                vertex_of(
                    t,
                    &what,
                    &minted(
                        EntityKind::Vertex,
                        fillet,
                        RoleSeg::BandCross(NameRef::new(m.clone())),
                    ),
                ),
            );
            for e in ends(sbody, edge_of(src, &what, &m)) {
                min = min.min(dist(cross, point(sbody, e)));
            }
        }
    }
    assert!(
        (min - CLOSEST).abs() < 1e-12,
        "the closest mint-to-source separation measures {min}, not CLOSEST = {CLOSEST}"
    );
    assert!(
        NEAR < min / 1e6,
        "NEAR = {NEAR} is not a decade-clear margin below the separation {min} it is \
         derived from"
    );
}

//! Sweep-op name emission (spec D2): a mechanical zip of the sweep
//! emitters' output maps (`Extruded`, `Revolved`, `Lofted` — indexed by
//! the profile's canonical positions) with the role vocabulary, each
//! position spelled by the piece it is (`eval::ProfilePieces`,
//! `names/README.md` "N1, the profile pieces"). Rim edges and cap vertices are derived by
//! COMBINATORIAL adjacency between emitted anchors (unique shared
//! edge; endpoint intersection) — exact wiring facts, never geometric
//! matching.

use std::sync::Arc;

use geom_core::Decide;
use sweep::Extruded;
use topo::{Body, EdgeKey, FaceKey, VertexKey};

use super::emit::{NamingError, Rim, RimShare, edge_ends, ent, name1, rim_between};
use super::role::{
    CapEnd, EntityKind, MeridianEnd, PieceRun, ProfileEdgeRef, ProfileVertexRef, RoleSeg,
};
use super::table::{EntityKey, NameTable};
use crate::eval::ProfilePieces;
use crate::node::RecipeNodeId;

/// **An OFF-AXIS meridian vertex that stays unresolved.** Not a shape
/// limitation: an off-axis vertex always has a rim, and a meridian edge
/// at it (partial chains are total; the full case omits only on-axis
/// segments, whose endpoints are poles), so the rim ∩ meridian anchor
/// always runs — reaching this means the two share both endpoints or
/// none, i.e. the built topology contradicts the key bundle the sweep
/// minted with it. Poles do not come this way at all: they are looked
/// up in the sweep's `poles` export.
#[track_caller]
fn unresolved(rim: EdgeKey, l: usize, v: usize) -> ! {
    unreachable!(
        "the revolve's off-axis meridian vertex at loop {l}, vertex {v} is unresolved: rim \
         {rim:?} shares exactly one endpoint with no meridian beside it, though the sweep \
         mints the rim and the meridians at an off-axis vertex together"
    )
}

/// **A wall and a cap that do not share exactly one edge**, and which
/// way they failed to.
///
/// **The premise, stated exactly**: this emitter builds nothing —
/// `sweep` does — and [`name_loft_topology`] takes the body and the key
/// lists as independent parameters with nothing in the signature tying
/// them. What makes this a kernel bug is that every caller passes a
/// body and a bundle from ONE mint, which is true by call-site
/// inspection and not by any type here. Under that premise a swept wall
/// meets each cap along one rim by construction, so any other
/// cardinality means the body and the bundle disagree.
#[track_caller]
fn cap_rim_contradicted(wall: FaceKey, cap: FaceKey, found: RimShare) -> ! {
    match found {
        RimShare::NotAdjacent => unreachable!(
            "swept wall {wall:?} and cap {cap:?} share no edge, though the sweep mints each \
             wall meeting each cap along one rim, in the body it hands over with them"
        ),
        RimShare::Several => unreachable!(
            "swept wall {wall:?} and cap {cap:?} share more than one edge, though the sweep \
             mints each wall meeting each cap along one rim, in the body it hands over with \
             them"
        ),
    }
}

/// **A canonical position with no piece**: the pieces and the swept
/// maps were built from one profile, so a position the one has and the
/// other lacks is the two disagreeing.
#[track_caller]
fn no_piece(l: usize, at: &[usize]) -> ! {
    unreachable!(
        "the swept canonical position {at:?} of loop {l} has no profile piece, though the \
         pieces and the sweep's maps are built from one validated profile"
    )
}

/// The piece canonical segment `k` of loop `l` is.
#[track_caller]
fn edge_at(pieces: &ProfilePieces, l: usize, k: usize) -> ProfileEdgeRef {
    match pieces.edge(l, k) {
        Some(piece) => piece,
        None => no_piece(l, &[k]),
    }
}

/// The piece canonical vertex `v` of loop `l` is.
#[track_caller]
fn vertex_at(pieces: &ProfilePieces, l: usize, v: usize) -> ProfileVertexRef {
    match pieces.vertex(l, v) {
        Some(piece) => piece,
        None => no_piece(l, &[v]),
    }
}

/// The pieces the run of canonical segments `run` of loop `l` is.
#[track_caller]
fn run_at(pieces: &ProfilePieces, l: usize, run: &[usize]) -> PieceRun {
    match pieces.run(l, run) {
        Some(pieces) => pieces,
        None => no_piece(l, run),
    }
}

/// **How a loft's per-position roles are spelled**: a wall and a seam
/// spanning every section by every section's piece at that position
/// (`RoleSeg::LoftWall`, `RoleSeg::LoftSeam`); one of a loft built one
/// per slab by its slab and the pieces at the slab's two sections
/// (`RoleSeg::LoftSlabWall`, `RoleSeg::LoftSlabSeam`), and an interior
/// section's ring by the section and its own piece; its caps' rims and
/// vertices by the end section's own. One profile per section, first to
/// last.
struct Sections<'a>(&'a [ProfilePieces]);

/// A slab or section index as a role segment holds it.
#[track_caller]
fn index(i: usize) -> u32 {
    u32::try_from(i).unwrap_or_else(|_| {
        unreachable!("a loft's slab or section index {i} fits a u32: a section is a profile")
    })
}

/// **Two adjacent slab walls that do not share exactly one edge**: the
/// loft builds the walls of one segment slab on slab, each meeting the
/// next along one rim of the section between them, so any other
/// cardinality means the body and the bundle disagree (the premise is
/// [`cap_rim_contradicted`]'s).
#[track_caller]
fn section_rim_contradicted(below: FaceKey, above: FaceKey, found: RimShare) -> ! {
    unreachable!(
        "loft slab walls {below:?} and {above:?} share {} edge, though the loft mints each \
         slab wall meeting the next along one rim of the section between them",
        match found {
            RimShare::NotAdjacent => "no",
            RimShare::Several => "more than one",
        }
    )
}

impl Sections<'_> {
    /// The profile a cap end lies on.
    #[track_caller]
    fn end(&self, end: CapEnd) -> &ProfilePieces {
        let section = match end {
            CapEnd::Start => self.0.first(),
            CapEnd::End => self.0.last(),
        };
        let Some(section) = section else {
            unreachable!("a loft skins two sections or more, so it has two ends")
        };
        section
    }

    /// The wall at canonical segment `k` of loop `l` across slab `s`
    /// of `slabs`: across every section where it is the only one.
    #[track_caller]
    fn wall(&self, l: usize, k: usize, s: usize, slabs: usize) -> RoleSeg {
        if slabs == 1 {
            return RoleSeg::LoftWall(self.0.iter().map(|p| edge_at(p, l, k)).collect());
        }
        RoleSeg::LoftSlabWall(
            index(s),
            self.0[s..=s + 1].iter().map(|p| edge_at(p, l, k)).collect(),
        )
    }

    /// The wall–wall edge at canonical vertex `v` of loop `l` across
    /// slab `s` of `slabs`, as [`Self::wall`].
    #[track_caller]
    fn strut(&self, l: usize, v: usize, s: usize, slabs: usize) -> RoleSeg {
        if slabs == 1 {
            return RoleSeg::LoftSeam(self.0.iter().map(|p| vertex_at(p, l, v)).collect());
        }
        RoleSeg::LoftSlabSeam(
            index(s),
            self.0[s..=s + 1]
                .iter()
                .map(|p| vertex_at(p, l, v))
                .collect(),
        )
    }

    /// The edge of interior section `i`'s ring over canonical segment
    /// `k` of loop `l`.
    #[track_caller]
    fn section_edge(&self, i: usize, l: usize, k: usize) -> RoleSeg {
        RoleSeg::LoftSectionEdge(index(i), edge_at(&self.0[i], l, k))
    }

    /// The vertex of interior section `i` over canonical vertex `v` of
    /// loop `l`.
    #[track_caller]
    fn section_vertex(&self, i: usize, l: usize, v: usize) -> RoleSeg {
        RoleSeg::LoftSectionVertex(index(i), vertex_at(&self.0[i], l, v))
    }

    /// The rim where the wall at `(l, k)` meets the cap at `end`.
    #[track_caller]
    fn rim(&self, end: CapEnd, l: usize, k: usize) -> RoleSeg {
        RoleSeg::RimEdge(end, edge_at(self.end(end), l, k).into())
    }

    /// The cap vertex at `end` over canonical vertex `v` of loop `l`.
    #[track_caller]
    fn cap_vertex(&self, end: CapEnd, l: usize, v: usize) -> RoleSeg {
        RoleSeg::CapVertex(end, vertex_at(self.end(end), l, v))
    }
}

/// Names every boundary entity of an extrusion (spec D2's extrude
/// vocabulary): caps, laterals, rims, struts, cap vertices, and the
/// output body. A lateral and its two rims are named by its run of
/// pieces (N1, "Swept walls over a run"); a strut and its two cap
/// vertices exist at a run's leading vertex only.
pub(crate) fn name_extrude<T: Decide>(
    node: RecipeNodeId,
    built: &Extruded<T>,
    pieces: &ProfilePieces,
) -> Result<Arc<NameTable>, NamingError> {
    let body = &built.body;
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (end, face) in [(CapEnd::End, built.top), (CapEnd::Start, built.bottom)] {
        t.insert(
            name1(EntityKind::Face, node, RoleSeg::Cap(end)),
            ent(0, EntityKey::Face(face)),
        )?;
    }
    for (l, walls) in built.walls.iter().enumerate() {
        // A wall's segments are canonical; the vertex the sweep starts
        // each at is `Extruded::start_vertex` (the reversal's one home).
        let n: usize = walls.iter().map(|w| w.segments.len()).sum();
        let vertex = |c: usize| built.start_vertex(n, c);
        for wall in walls {
            let run: &[usize] = &wall.segments;
            let Some(&lead) = run.first() else {
                unreachable!(
                    "extruded wall {:?} of loop {l} sweeps no segment, though the sweep mints \
                     a wall per run of one segment or more",
                    wall.face
                );
            };
            t.insert(
                name1(
                    EntityKind::Face,
                    node,
                    RoleSeg::Lateral(run_at(pieces, l, run)),
                ),
                ent(0, EntityKey::Face(wall.face)),
            )?;
            t.insert(
                name1(
                    EntityKind::Edge,
                    node,
                    RoleSeg::LateralEdge(vertex_at(pieces, l, vertex(lead))),
                ),
                ent(0, EntityKey::Edge(wall.strut)),
            )?;
            let strut = edge_ends(body, wall.strut);
            for (end, rim) in [
                (CapEnd::End, wall.top_rim),
                (CapEnd::Start, wall.bottom_rim),
            ] {
                t.insert(
                    name1(
                        EntityKind::Edge,
                        node,
                        RoleSeg::RimEdge(end, run_at(pieces, l, run)),
                    ),
                    ent(0, EntityKey::Edge(rim)),
                )?;
                // The run's cap vertex is where its strut meets its rim.
                let rim_ends = edge_ends(body, rim);
                let Some(vtx) = common_vertex(strut, rim_ends) else {
                    unreachable!(
                        "extruded wall {:?}'s rim {rim:?} (ends {rim_ends:?}) shares no single \
                         endpoint with its strut {:?} (ends {strut:?}), though the sweep mints \
                         the strut from the rims' common vertex",
                        wall.face, wall.strut
                    );
                };
                t.insert(
                    name1(
                        EntityKind::Vertex,
                        node,
                        RoleSeg::CapVertex(end, vertex_at(pieces, l, vertex(lead))),
                    ),
                    ent(0, EntityKey::Vertex(vtx)),
                )?;
            }
        }
    }
    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// Names every boundary entity of a loft body (M6-3): caps, walls,
/// rims, seams, cap vertices, interior sections' rings and vertices,
/// output body, by the combinatorial zip below — per segment one wall
/// per slab, the first meeting the start cap and the last the end cap
/// along one rim each, and adjacent slabs' walls along one rim of the
/// section between them — with a wall and a seam named by the pieces
/// the skin paired, one per section it spans (`sections`, first to
/// last).
pub(crate) fn name_loft<T: Decide>(
    node: RecipeNodeId,
    built: &sweep::Lofted<T>,
    sections: &[ProfilePieces],
) -> Result<Arc<NameTable>, NamingError> {
    name_loft_topology(
        node,
        &Sections(sections),
        &built.body,
        built.top,
        built.bottom,
        &built.side_faces,
        &built.seam_edges,
    )
}

/// The loft's zip: two caps, per-(loop, segment, slab) walls,
/// per-(loop, vertex, slab) seam edges, each segment's first and last
/// walls meeting the caps along one rim, adjacent slabs' walls along
/// one interior rim, adjacent slabs' seams at one interior vertex.
fn name_loft_topology<T: Decide>(
    node: RecipeNodeId,
    swept: &Sections<'_>,
    body: &Body<T>,
    end_cap: topo::FaceKey,
    start_cap: topo::FaceKey,
    side_faces: &[Vec<Vec<topo::FaceKey>>],
    lateral_edges: &[Vec<Vec<EdgeKey>>],
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (end, face) in [(CapEnd::End, end_cap), (CapEnd::Start, start_cap)] {
        t.insert(
            name1(EntityKind::Face, node, RoleSeg::Cap(end)),
            ent(0, EntityKey::Face(face)),
        )?;
    }
    // The rim where loop `l`, segment `k`'s wall at `end` meets `cap`.
    let cap_rim = |l: usize, k: usize, end: CapEnd, cap: FaceKey| {
        let slabs = &side_faces[l][k];
        let wall = match end {
            CapEnd::Start => slabs.first(),
            CapEnd::End => slabs.last(),
        };
        let Some(&wall) = wall else {
            unreachable!("loft segment ({l}, {k}) has no wall, though the loft mints one a slab")
        };
        match rim_between(body, wall, cap) {
            Rim::One(e) => e,
            Rim::NotOne(found) => cap_rim_contradicted(wall, cap, found),
        }
    };

    // Walls, cap rims and interior rims, indexed by the emitter's
    // canonical (loop, segment, slab) maps and named by the pieces at
    // those positions.
    for (l, segs) in side_faces.iter().enumerate() {
        for (k, walls) in segs.iter().enumerate() {
            for (s, &wall) in walls.iter().enumerate() {
                t.insert(
                    name1(EntityKind::Face, node, swept.wall(l, k, s, walls.len())),
                    ent(0, EntityKey::Face(wall)),
                )?;
            }
            for (end, cap) in [(CapEnd::End, end_cap), (CapEnd::Start, start_cap)] {
                t.insert(
                    name1(EntityKind::Edge, node, swept.rim(end, l, k)),
                    ent(0, EntityKey::Edge(cap_rim(l, k, end, cap))),
                )?;
            }
            // Slab `s + 1` meets slab `s` along section `s + 1`'s rim.
            for (s, pair) in walls.windows(2).enumerate() {
                let rim = match rim_between(body, pair[0], pair[1]) {
                    Rim::One(e) => e,
                    Rim::NotOne(found) => section_rim_contradicted(pair[0], pair[1], found),
                };
                t.insert(
                    name1(EntityKind::Edge, node, swept.section_edge(s + 1, l, k)),
                    ent(0, EntityKey::Edge(rim)),
                )?;
            }
        }
    }

    // Struts, cap vertices and interior vertices. Strut `j` joins the
    // walls of segments `j − 1` and `j`; its cap-`end` endpoint is the
    // unique common vertex with the `end` rim of segment `j`, and slab
    // `s + 1`'s meets slab `s`'s at section `s + 1`.
    for (l, struts) in lateral_edges.iter().enumerate() {
        for (j, slabs) in struts.iter().enumerate() {
            for (s, &strut) in slabs.iter().enumerate() {
                t.insert(
                    name1(EntityKind::Edge, node, swept.strut(l, j, s, slabs.len())),
                    ent(0, EntityKey::Edge(strut)),
                )?;
            }
            for (end, cap) in [(CapEnd::End, end_cap), (CapEnd::Start, start_cap)] {
                let strut = match end {
                    CapEnd::Start => slabs.first(),
                    CapEnd::End => slabs.last(),
                };
                let Some(&strut) = strut else {
                    unreachable!(
                        "loft vertex ({l}, {j}) has no strut, though the loft mints one a slab"
                    )
                };
                let (s0, s1) = edge_ends(body, strut);
                let rim = cap_rim(l, j, end, cap);
                let (r0, r1) = edge_ends(body, rim);
                let Some(vtx) = common_vertex((s0, s1), (r0, r1)) else {
                    unreachable!(
                        "loft strut {strut:?} (ends {:?}) and rim {rim:?} (ends {:?}) share no \
                         single endpoint, though the loft mints the strut between its walls' \
                         rims",
                        (s0, s1),
                        (r0, r1)
                    );
                };
                t.insert(
                    name1(EntityKind::Vertex, node, swept.cap_vertex(end, l, j)),
                    ent(0, EntityKey::Vertex(vtx)),
                )?;
            }
            for (s, pair) in slabs.windows(2).enumerate() {
                let (below, above) = (edge_ends(body, pair[0]), edge_ends(body, pair[1]));
                let Some(vtx) = common_vertex(below, above) else {
                    unreachable!(
                        "loft struts {:?} (ends {below:?}) and {:?} (ends {above:?}) share no \
                         single endpoint, though the loft mints each slab's strut from the \
                         last one's far end",
                        pair[0], pair[1]
                    );
                };
                t.insert(
                    name1(EntityKind::Vertex, node, swept.section_vertex(s + 1, l, j)),
                    ent(0, EntityKey::Vertex(vtx)),
                )?;
            }
        }
    }

    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// Names every boundary entity of a revolution (spec D2: the M2
/// band/pole/seam taxonomy, read off the `Revolved` maps). A wall, its
/// π twin and its meridians are named by the wall's run of pieces (N1,
/// "Swept walls over a run"); an on-axis run's one axis edge likewise.
///
/// Vertex resolution: a partial revolve's off-axis meridian vertices
/// are the two ends of the rim there, each the one it shares with a
/// meridian of that chain (a station has no rim and no entity); a full
/// wire's are
/// the two ends of each half-period rim, the seam one shared with an
/// angle-0 meridian beside it. On-axis (pole) vertices are LOOKED UP in
/// the sweep's `poles` export — a construction record, since the
/// builders know each pole by the operator that minted it. A pole
/// absent from the export is a vertex the sweep deleted (the full
/// case's omitted axis run, a plane disc's centre), so nothing is named
/// for it; had one been wrongly omitted, `check_total` would catch the
/// unnamed body vertex.
pub(crate) fn name_revolve<T: Decide>(
    node: RecipeNodeId,
    built: &sweep::Revolved<T>,
    pieces: &ProfilePieces,
) -> Result<Arc<NameTable>, NamingError> {
    let body = &built.body;
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    let pv = |l: usize, v: usize| vertex_at(pieces, l, v);
    let insert_face = |t: &mut NameTable, seg: RoleSeg, f| {
        t.insert(
            name1(EntityKind::Face, node, seg),
            ent(0, EntityKey::Face(f)),
        )
    };
    let insert_edge = |t: &mut NameTable, seg: RoleSeg, e| {
        t.insert(
            name1(EntityKind::Edge, node, seg),
            ent(0, EntityKey::Edge(e)),
        )
    };
    let insert_vertex = |t: &mut NameTable, seg: RoleSeg, v| {
        t.insert(
            name1(EntityKind::Vertex, node, seg),
            ent(0, EntityKey::Vertex(v)),
        )
    };

    // Each wall's run, as its pieces and its first canonical segment
    // (where the per-segment maps hold the run's one meridian and twin).
    let mut runs: Vec<Vec<(usize, PieceRun)>> = Vec::with_capacity(built.bands.len());
    for (l, bands) in built.bands.iter().enumerate() {
        let mut lr = Vec::with_capacity(bands.len());
        for band in bands {
            let Some(&first) = band.segments.first() else {
                unreachable!(
                    "revolved wall {:?} of loop {l} sweeps no segment, though the sweep mints \
                     a wall per run of one segment or more",
                    band.face
                );
            };
            let run = run_at(pieces, l, &band.segments);
            insert_face(&mut t, RoleSeg::Band(run.clone()), band.face)?;
            lr.push((first, run));
        }
        runs.push(lr);
    }
    for (l, vs) in built.rims.iter().enumerate() {
        for (v, rim) in vs.iter().enumerate() {
            if let Some(e) = rim {
                insert_edge(&mut t, RoleSeg::BandRim(pv(l, v)), *e)?;
            }
        }
    }

    match &built.kind {
        sweep::RevolvedKind::Partial {
            start_cap,
            end_cap,
            start_meridians,
            end_meridians,
        } => {
            insert_face(&mut t, RoleSeg::RevolveCap(MeridianEnd::Start), *start_cap)?;
            insert_face(&mut t, RoleSeg::RevolveCap(MeridianEnd::End), *end_cap)?;
            for (l, (ss, es)) in start_meridians.iter().zip(end_meridians).enumerate() {
                // Each run's one meridian (or axis edge) is held at
                // every segment of the run: name it once, by the run.
                let n = ss.len();
                let mut named = vec![false; n];
                for s in 0..n {
                    if named[s] {
                        continue;
                    }
                    let held: Vec<usize> = (0..n).filter(|&k| ss[k] == ss[s]).collect();
                    let run = run_at(pieces, l, &held);
                    for &k in &held {
                        named[k] = true;
                    }
                    if ss[s] == es[s] {
                        // The shared axis edge of an on-axis run.
                        insert_edge(&mut t, RoleSeg::AxisEdge(run.clone()), ss[s])?;
                    } else {
                        insert_edge(
                            &mut t,
                            RoleSeg::Meridian(MeridianEnd::Start, run.clone()),
                            ss[s],
                        )?;
                        insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::End, run), es[s])?;
                    }
                }
                let rims = &built.rims[l];
                for v in 0..rims.len() {
                    if let Some(p) = built.poles[l][v] {
                        // Pole: the same physical vertex in both chains.
                        insert_vertex(&mut t, RoleSeg::Pole(pv(l, v)), p)?;
                    } else if let Some(rim) = rims[v] {
                        // Off-axis: the rim's two ends, each the one a
                        // meridian of its chain beside it shares.
                        let ends = edge_ends(body, rim);
                        let beside = |chain: &[EdgeKey]| -> VertexKey {
                            let mut shared = [chain[(v + n - 1) % n], chain[v]]
                                .into_iter()
                                .filter_map(|m| common_vertex(ends, edge_ends(body, m)));
                            let Some(vertex) = shared.next() else {
                                unresolved(rim, l, v)
                            };
                            vertex
                        };
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::Start, pv(l, v)),
                            beside(ss),
                        )?;
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::End, pv(l, v)),
                            beside(es),
                        )?;
                    }
                }
            }
        }
        sweep::RevolvedKind::Full {
            wire,
            meridians,
            pi_walls,
            pi_meridians,
            pi_rims,
        } => {
            for (l, lr) in runs.iter().enumerate() {
                for (first, run) in lr {
                    if let Some(e) = meridians[l][*first] {
                        insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::Seam, run.clone()), e)?;
                    }
                    if l > 0 {
                        continue;
                    }
                    if let Some(f) = pi_walls[*first] {
                        insert_face(&mut t, RoleSeg::BandPi(run.clone()), f)?;
                    }
                    if let Some(e) = pi_meridians[*first] {
                        insert_edge(&mut t, RoleSeg::Meridian(MeridianEnd::Pi, run.clone()), e)?;
                    }
                }
            }
            for (v, r) in pi_rims.iter().enumerate() {
                if let Some(e) = r {
                    insert_edge(&mut t, RoleSeg::BandRimPi(pv(0, v)), *e)?;
                }
            }
            let rims = &built.rims[0];
            if *wire {
                // A half-period rim runs from its seam copy (angle 0) to
                // its π copy; the seam end is the one an angle-0
                // meridian beside it shares — a plane wall has none, but
                // a wire vertex is never between two plane walls (they
                // would be one run).
                let n = rims.len();
                let seam_chain = &meridians[0];
                for v in 0..n {
                    if let Some(rim) = rims[v] {
                        let ends = edge_ends(body, rim);
                        let beside = [seam_chain[(v + n - 1) % n], seam_chain[v]];
                        let Some(seam) = beside
                            .iter()
                            .flatten()
                            .find_map(|m| common_vertex(ends, edge_ends(body, *m)))
                        else {
                            unresolved(rim, 0, v)
                        };
                        let pi = if ends.0 == seam { ends.1 } else { ends.0 };
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(0, v)),
                            seam,
                        )?;
                        insert_vertex(
                            &mut t,
                            RoleSeg::MeridianVertex(MeridianEnd::Pi, pv(0, v)),
                            pi,
                        )?;
                    } else if let Some(p) = built.poles[0][v] {
                        insert_vertex(&mut t, RoleSeg::Pole(pv(0, v)), p)?;
                    }
                }
            }
            // Lamina rims (the outer loop off the axis, and every hole
            // loop of a holed full revolve — its cavity shells, off-axis
            // by validated containment): full-period self-loops whose
            // (doubled) endpoint IS the meridian vertex. A station has
            // no rim and no entity.
            let lamina = if *wire { 1 } else { 0 };
            for l in lamina..built.rims.len() {
                for (v, r) in built.rims[l].iter().enumerate() {
                    let Some(e) = r else { continue };
                    let (a, b) = edge_ends(body, *e);
                    if a != b {
                        unreachable!(
                            "lamina full revolve rim {e:?} at loop {l}, vertex {v} runs from \
                             {a:?} to {b:?}, though a full-period rim off the axis is minted as \
                             a self-loop"
                        );
                    }
                    insert_vertex(
                        &mut t,
                        RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(l, v)),
                        a,
                    )?;
                }
            }
        }
    }

    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// The unique vertex two endpoint pairs share (`None` when disjoint
/// or when both endpoints coincide — callers treat as an emission
/// inconsistency).
pub(crate) fn common_vertex(
    a: (VertexKey, VertexKey),
    b: (VertexKey, VertexKey),
) -> Option<VertexKey> {
    let hit0 = a.0 == b.0 || a.0 == b.1;
    let hit1 = a.1 == b.0 || a.1 == b.1;
    match (hit0, hit1) {
        (true, false) => Some(a.0),
        (false, true) => Some(a.1),
        _ => None,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::TAU;

    use geom_core::{Arc2, Point2, Tol};
    use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};

    use super::*;
    use crate::eval::ProfilePieces;

    /// A one-segment circle about `(cx, 0)` of radius `r` (D1's full
    /// turn), its vertex at carrier angle 0.
    fn circle(cx: f64, r: f64) -> ProfileLoop<f64> {
        RawLoop::new([(
            Point2::new(cx + r, 0.0),
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, 0.0),
                radius: r,
                sweep: TAU,
            }),
        )])
    }

    fn node() -> RecipeNodeId {
        RecipeNodeId::new(0, test_utils::refusal::tagged(1))
    }

    /// **A full revolve's axis cavities are named like its outer
    /// shell**: a profile meeting the axis in two or three runs, with
    /// square, round and pointed notches and a hole beside them,
    /// revolves to a body whose every cavity wall, meridian, rim and
    /// pole is named by the profile piece that sweeps it, and the
    /// table is total over the body (`check_total`), poles at both
    /// ends of each gap included.
    #[test]
    fn a_full_revolves_axis_cavities_are_named() {
        let axis = sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        };
        let p = Point2::new;
        let column = [p(0.0, 0.0), p(2.0, 0.0), p(2.0, 5.0), p(0.0, 5.0)];
        let square = [p(0.0, 4.0), p(0.5, 4.0), p(0.5, 3.0), p(0.0, 3.0)];
        let pointed = [p(0.0, 2.0), p(0.5, 2.0), p(0.0, 1.0)];
        let hole = ProfileLoop::polygon([p(1.0, 1.0), p(1.5, 1.0), p(1.5, 2.0), p(1.0, 2.0)]);
        let round = RawLoop::new(
            [p(0.0, 0.0), p(2.0, 0.0), p(2.0, 3.0), p(0.0, 3.0)]
                .map(|q| (q, Segment::Line))
                .into_iter()
                .chain([
                    (
                        p(0.0, 2.0),
                        Segment::Arc(Arc2 {
                            centre: p(0.0, 1.5),
                            radius: 0.5,
                            sweep: -core::f64::consts::PI,
                        }),
                    ),
                    (p(0.0, 1.0), Segment::Line),
                ]),
        );
        let three_runs: Vec<Point2<f64>> = column
            .iter()
            .chain(&square)
            .chain(&pointed)
            .copied()
            .collect();
        // (what, loops, segments per loop, cavities)
        let cases = [
            (
                "three runs",
                vec![ProfileLoop::polygon(three_runs)],
                vec![11],
                2,
            ),
            ("round notch", vec![round], vec![6], 1),
            (
                "a hole beside a gap",
                vec![
                    ProfileLoop::polygon(column.iter().chain(&square).copied()),
                    hole,
                ],
                vec![8, 4],
                2,
            ),
        ];
        for (what, loops, counts, cavities) in cases {
            let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
                .validate(Tol::witness())
                .unwrap_or_else(|e| panic!("{what}: the profile validates: {e:?}"));
            let built = sweep::revolve(&profile, axis, sweep::Revolution::Full, Tol::witness())
                .unwrap_or_else(|e| panic!("{what}: the profile revolves: {e:?}"));
            assert_eq!(built.cavities.len(), cavities, "{what}");
            let pieces = ProfilePieces::numbered(&counts);
            let table = name_revolve(node(), &built, &pieces)
                .unwrap_or_else(|e| panic!("{what}: the revolve is named: {e:?}"));
            if what == "three runs" {
                three_runs_walls_are_named_by_their_pieces(&built, &table, &pieces);
            }
        }
    }

    /// Each wall of the three-run column, cavity walls included, is the
    /// surface its own profile piece sweeps: read through the NAME and
    /// checked against the piece's geometry, so two walls' names
    /// swapped is red.
    fn three_runs_walls_are_named_by_their_pieces(
        built: &sweep::Revolved<f64>,
        table: &NameTable,
        pieces: &ProfilePieces,
    ) {
        use geom::Surface;
        // (piece, the swept surface): a plane at height y, a cylinder
        // of radius r, or a cone.
        enum Wall {
            Plane(f64),
            Cylinder(f64),
            Cone,
        }
        let walls = [
            (0, Wall::Plane(0.0)),
            (1, Wall::Cylinder(2.0)),
            (2, Wall::Plane(5.0)),
            (4, Wall::Plane(4.0)),
            (5, Wall::Cylinder(0.5)),
            (6, Wall::Plane(3.0)),
            (8, Wall::Plane(2.0)),
            (9, Wall::Cone),
        ];
        for (k, want) in walls {
            let name = name1(
                EntityKind::Face,
                node(),
                RoleSeg::Band(run_at(pieces, 0, &[k])),
            );
            let entry = table
                .lookup(&name)
                .unwrap_or_else(|| panic!("piece {k}'s wall is named"));
            let Some(face) = entry.entities()[0].key.face() else {
                panic!("piece {k}'s wall name holds a face")
            };
            let surface = built.body.get_face(face).unwrap().surface;
            let fits = match (want, built.body.get_surface(surface).unwrap()) {
                (Wall::Plane(y), Surface::Plane { origin, .. }) => (origin.y - y).abs() < 1e-12,
                (Wall::Cylinder(r), Surface::Cylinder { radius, .. }) => {
                    (*radius - r).abs() < 1e-12
                }
                (Wall::Cone, Surface::Cone { .. }) => true,
                _ => false,
            };
            assert!(
                fits,
                "piece {k}'s wall name holds the surface piece {k} sweeps"
            );
        }
    }

    /// **A one-segment loop's revolve and loft are named whole**: the
    /// one meridian chain of a part turn is one self-loop at the loop's
    /// one vertex (`name_revolve` reads the rim end that self-loop
    /// shares, through `common_vertex`), so each wedge cap's copy of
    /// that vertex is named; the full turn and the loft
    /// name their one wall, its wrap edge and its vertices. Every table
    /// is total over its body (`check_total`).
    #[test]
    fn a_one_segment_loop_revolves_and_lofts_to_named_bodies() {
        let profile = Profile::new(SketchPlane::<f64>::xy(), vec![circle(3.0, 0.5)])
            .validate(Tol::witness())
            .unwrap();
        let axis = sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        };
        let pieces = ProfilePieces::numbered(&[1]);
        for turn in [
            sweep::Revolution::Partial(1.25),
            sweep::Revolution::Partial(-1.25),
            sweep::Revolution::Full,
        ] {
            let built = sweep::revolve(&profile, axis, turn, Tol::witness()).unwrap();
            name_revolve(node(), &built, &pieces)
                .unwrap_or_else(|e| panic!("{turn:?}: the revolve is named: {e:?}"));
        }
        let section = vec![circle(0.0, 1.0)];
        let lofted = sweep::loft_body::<f64>(
            &[section.clone(), section],
            &sweep::test_support::stacked_at(&[0.0, 2.0]),
            1,
            Tol::witness(),
        )
        .unwrap();
        name_loft(node(), &lofted, &[pieces.clone(), pieces])
            .unwrap_or_else(|e| panic!("the loft is named: {e:?}"));
    }
}

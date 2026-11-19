//! **The snap catalogue** (`[import]`, D7 step 3, Healing): a relation
//! between two surfaces that reads `Zero` at the file's band is made exact
//! by one construction from a closed catalogue, before anything is built.
//!
//! **The catalogue** ([`SnapRelation`], the one place it is stated):
//!
//! - **coaxial**: two round surfaces (cylinder, cone, sphere, torus) whose
//!   axes, or a sphere's centre, read one line. The later record in the
//!   file moves onto the earlier one's axis: its axis takes the earlier
//!   one's direction and its datum point the foot on that line (a sphere's
//!   centre the foot; a sphere kept, the moved axis through its centre;
//!   two spheres, one centre).
//! - **tangent**: a plane and a cylinder the file band reads tangent along
//!   their edge, the axis parallel to the plane. The planes stay and the
//!   cylinder's axis is translated square to itself, its radius kept, to
//!   where it stands its radius from each plane it is tangent to, on the
//!   side it stands: one plane moves it along the plane's normal, two
//!   crossing planes to where their two offsets meet.
//! - **tangent, between two parallel planes**: a cylinder tangent to two
//!   planes the file band reads parallel, its axis between them (a slot's
//!   or a rib's end). The planes stay; the radius becomes half their
//!   spacing and the axis moves along their normal to the mid-plane.
//!
//!   A cylinder tangent to three or more planes, to two parallel planes
//!   its axis does not lie between, or coaxial with another surface while
//!   its tangency moves it, is a class the catalogue does not solve.
//! - **axis-normal**: a plane and a cylinder, cone or torus whose axis
//!   reads along the plane's normal. The later record turns: a plane's
//!   normal takes the axis, a round surface's axis takes the normal, about
//!   the point where the axis meets the plane.
//! - **one locus**: two records of one surface become one
//!   ([`crate::one_record`], run before this, `[topo.one-record]`).
//!
//! **Read at the faces.** A relation is read only between the two faces of
//! an edge, at the extent the two faces' vertices span: a distance in
//! metres, an angle levered at that extent ([`gap`]). Read at the file's
//! band ([`STEP_SNAP_RELATION`]), `Zero` is a relation; at the run's band,
//! `Zero` is a relation already exact. A reading in the file's band is no
//! relation here: the build reads that pair and refuses it as the file's
//! ambiguity.
//!
//! **One construction per class, solved jointly.** The relations form a
//! graph over the surfaces; a connected class all of whose relations are
//! already exact moves nothing. Otherwise each relation not yet exact
//! names its mover (the tangent's cylinder, the coaxial or axis-normal
//! pair's later record), and every move is computed from inputs no
//! construction moves, so no order decides the result: a surface two
//! constructions would move, or one whose construction would read a
//! moved surface, is a class the catalogue does not solve, and refuses.
//! Each move is read at the file's band ([`STEP_SNAP_MOVE`]): `Zero`
//! applies it, in band refuses as the file's ambiguity, apart refuses as
//! unsolved. Then every relation of the class, the exact ones among them,
//! is read again at the run's band, and one that is not `Zero` refuses as
//! unsolved. Each moved surface is reported
//! ([`NormalizationKind::Snapped`]).

use std::collections::BTreeMap;

use geom::Surface;
use geom_core::{Point3, Tol, Vec3};

use crate::entities::SolidSpec;
use crate::error::{BuiltEntity, StepImportError};
use crate::interpret::{FileBand, Read};
use crate::one_record::Record;
use crate::{FaceCensus, NormalizationKind, SnapRelation, StructureNormalization};

/// A catalogue relation's gap between two surfaces, in metres: at the
/// file's band `Zero` is a relation; at the run's, an exact one.
pub(crate) const STEP_SNAP_RELATION: &str = "step_snap_relation";

/// Which side of a plane a cylinder's axis lies, signed along one of two
/// parallel planes' normals, read at the file's band: the axis lies
/// between the two where its sides differ.
pub(crate) const STEP_SNAP_SIDE: &str = "step_snap_side";

/// How far a catalogue construction moves a surface, in metres, read at
/// the file's band: only `Zero` is applied.
pub(crate) const STEP_SNAP_MOVE: &str = "step_snap_move";

/// The lane a relation the file band reads `Zero` and the run reads
/// apart waits on where the catalogue makes it exact by no construction:
/// a relation outside it, or a class its constructions do not solve.
pub(crate) const UNCATALOGUED_LANE: &str = "a snap catalogue construction that makes this \
     relation exact (`work/importgate/a-relation-the-snap-catalogue-does-not-make-exact-refuses.md`)";

/// One surface of the shell: the faces that wear it bit for bit, and the
/// earliest record they state.
struct Node {
    faces: Vec<usize>,
    record: Record,
    surface: Surface<f64>,
}

/// One relation the file band reads `Zero`, along edge `edge`.
struct Relation {
    kind: SnapRelation,
    /// The two surfaces: for a tangent or an axis-normal pair the plane
    /// first.
    nodes: (usize, usize),
    edge: u64,
    /// The extent its two faces span.
    lever: geom_core::Lever<f64>,
    /// Read `Zero` at the run's band.
    exact: bool,
}

/// **Makes each relation the file band reads `Zero` exact** (module docs).
/// `records[i]` is the record `solid.faces[i]` states.
pub(crate) fn snap(
    solid: &mut SolidSpec,
    records: &[Record],
    file: FileBand,
    tol: Tol,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let run = geom_core::Band::linear(tol).map_err(|_| StepImportError::Topology {
        id: solid.id,
        what: "the run's tolerance makes no band to build at",
    })?;
    let mut nodes = nodes(solid, records);
    let relations = relations(solid, &nodes, file, run);
    for class in classes(nodes.len(), &relations) {
        if class.iter().all(|&r| relations[r].exact) {
            continue;
        }
        let moves = solve(&nodes, &relations, &class, file)?;
        for m in &moves {
            nodes[m.node].surface = m.surface.clone();
        }
        for &r in &class {
            let rel = &relations[r];
            let g = gap(
                rel.kind,
                &nodes[rel.nodes.0].surface,
                &nodes[rel.nodes.1].surface,
                rel.lever.length(),
                centre(solid, rel.edge),
            );
            if !run_zero(run, g) {
                return Err(unsolved(
                    file,
                    rel.edge,
                    format!(
                        "a {} relation its class's constructions leave {g:e} m from exact",
                        rel.kind.name()
                    ),
                ));
            }
        }
        for Move {
            node: n,
            surface,
            moved_by,
            relation,
        } in moves
        {
            for &fi in &nodes[n].faces {
                solid.faces[fi].surface = surface.clone();
            }
            sink.push(StructureNormalization {
                face: nodes[n].record.id,
                kind: NormalizationKind::Snapped {
                    surface: nodes[n].record.id,
                    relation,
                    moved_by,
                },
                file_census: FaceCensus::EMPTY,
                kernel_census: FaceCensus::EMPTY,
            });
        }
    }
    Ok(())
}

/// Whether `gap` reads `Zero` at the run's band.
fn run_zero(run: geom_core::Band, gap: f64) -> bool {
    matches!(
        geom_core::k_stats::decide(STEP_SNAP_RELATION, geom_core::Margin::of(gap.abs()), run),
        Ok(sign) if sign.is_zero()
    )
}

/// The shell's surfaces: faces grouped by bit-identical surface, each
/// group under the earliest record its faces state.
fn nodes(solid: &SolidSpec, records: &[Record]) -> Vec<Node> {
    let mut by: BTreeMap<Vec<u64>, usize> = BTreeMap::new();
    let mut nodes: Vec<Node> = Vec::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        let sig = crate::build::surface_sig(&face.surface);
        let n = *by.entry(sig).or_insert_with(|| {
            nodes.push(Node {
                faces: Vec::new(),
                record: records[fi],
                surface: face.surface.clone(),
            });
            nodes.len() - 1
        });
        nodes[n].faces.push(fi);
        nodes[n].record = nodes[n].record.min(records[fi]);
    }
    nodes
}

/// Each face's node.
fn node_of(nodes: &[Node], faces: usize) -> Vec<usize> {
    let mut of = vec![0; faces];
    for (n, node) in nodes.iter().enumerate() {
        for &fi in &node.faces {
            of[fi] = n;
        }
    }
    of
}

/// The relations the file band reads `Zero` between the two faces of each
/// edge.
fn relations(
    solid: &SolidSpec,
    nodes: &[Node],
    file: FileBand,
    run: geom_core::Band,
) -> Vec<Relation> {
    let of = node_of(nodes, solid.faces.len());
    let mut sides: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        for lp in &face.loops {
            for u in &lp.uses {
                sides.entry(u.edge).or_default().push(fi);
            }
        }
    }
    let mut found = Vec::new();
    for (&edge, faces) in &sides {
        let [fa, fb] = faces[..] else { continue };
        let (na, nb) = (of[fa], of[fb]);
        if na == nb {
            continue;
        }
        let lever = reach(solid, [fa, fb]);
        let at = centre(solid, edge);
        let (sa, sb) = (&nodes[na].surface, &nodes[nb].surface);
        for (kind, pair) in candidates(sa, sb, (na, nb)) {
            let (s1, s2) = (&nodes[pair.0].surface, &nodes[pair.1].surface);
            let g = gap(kind, s1, s2, lever.length(), at);
            if file.read(STEP_SNAP_RELATION, g) == Read::Zero {
                let exact = run_zero(run, g);
                // One relation per pair, over every edge it is read along:
                // exact only where it is along each, at the widest lever.
                match found
                    .iter_mut()
                    .find(|r: &&mut Relation| r.kind == kind && r.nodes == pair)
                {
                    Some(same) => {
                        same.exact &= exact;
                        if lever.length() > same.lever.length() {
                            (same.lever, same.edge) = (lever, edge);
                        }
                    }
                    None => found.push(Relation {
                        kind,
                        nodes: pair,
                        edge,
                        lever,
                        exact,
                    }),
                }
                break;
            }
        }
    }
    found
}

/// The catalogue relations a pair of surfaces can stand in, each with the
/// pair in its order (the plane first).
fn candidates(
    a: &Surface<f64>,
    b: &Surface<f64>,
    (na, nb): (usize, usize),
) -> Vec<(SnapRelation, (usize, usize))> {
    use SnapRelation as R;
    let plane = |s: &Surface<f64>| matches!(s, Surface::Plane { .. });
    let cylinder = |s: &Surface<f64>| matches!(s, Surface::Cylinder { .. });
    match (plane(a), plane(b)) {
        (true, true) => vec![],
        (true, false) | (false, true) => {
            let (p, q, other) = if plane(a) { (na, nb, b) } else { (nb, na, a) };
            let mut out = Vec::new();
            if cylinder(other) {
                out.push((R::Tangent, (p, q)));
            }
            if axis(other).is_some() {
                out.push((R::AxisNormal, (p, q)));
            }
            out
        }
        (false, false) if datum(a).is_some() && datum(b).is_some() => {
            vec![(R::Coaxial, (na.min(nb), na.max(nb)))]
        }
        _ => vec![],
    }
}

/// A round surface's axis: its datum point and unit direction (a sphere
/// has none of its own).
fn axis(s: &Surface<f64>) -> Option<(Point3<f64>, Vec3<f64>)> {
    match *s {
        Surface::Cylinder { origin, axis, .. } => Some((origin, axis)),
        Surface::Cone { apex, axis, .. } => Some((apex, axis)),
        Surface::Torus { center, axis, .. } => Some((center, axis)),
        _ => None,
    }
}

/// A round surface's datum point: an axis point, an apex, a centre.
fn datum(s: &Surface<f64>) -> Option<Point3<f64>> {
    match *s {
        Surface::Sphere { center, .. } => Some(center),
        _ => axis(s).map(|(o, _)| o),
    }
}

/// `q`'s distance from the line through `o` along unit `d`.
fn off_line(q: Point3<f64>, (o, d): (Point3<f64>, Vec3<f64>)) -> f64 {
    let v = q - o;
    (v - d * v.dot(d)).norm()
}

/// `q`'s foot on the line through `o` along unit `d`.
fn foot(q: Point3<f64>, (o, d): (Point3<f64>, Vec3<f64>)) -> Point3<f64> {
    o + d * (q - o).dot(d)
}

/// **A relation's gap, in metres**, read at the faces around `at` (the
/// middle of the box their vertices span) over `lever` (its diagonal): a
/// direction's sine levered, and a distance read where the faces are.
/// The pair is in the relation's order.
fn gap(kind: SnapRelation, a: &Surface<f64>, b: &Surface<f64>, lever: f64, at: Point3<f64>) -> f64 {
    match kind {
        SnapRelation::Tangent => {
            let (Surface::Plane { origin, normal, .. }, Surface::Cylinder { radius, .. }) = (a, b)
            else {
                return f64::INFINITY;
            };
            let Some(line) = axis(b) else {
                return f64::INFINITY;
            };
            let on_axis = foot(at, line);
            normal.dot(line.1).abs() * lever
                + ((on_axis - *origin).dot(*normal).abs() - radius).abs()
        }
        SnapRelation::AxisNormal => {
            let (Surface::Plane { normal, .. }, Some((_, d))) = (a, axis(b)) else {
                return f64::INFINITY;
            };
            normal.cross(d).norm() * lever
        }
        SnapRelation::Coaxial => match (axis(a), axis(b)) {
            (Some(la), Some(lb)) => la.1.cross(lb.1).norm() * lever + off_line(foot(at, lb), la),
            (Some(l), None) => datum(b).map_or(f64::INFINITY, |c| off_line(c, l)),
            (None, Some(l)) => datum(a).map_or(f64::INFINITY, |c| off_line(c, l)),
            (None, None) => match (datum(a), datum(b)) {
                (Some(c1), Some(c2)) => (c2 - c1).norm(),
                _ => f64::INFINITY,
            },
        },
    }
}

/// The middle of the box an edge's two faces' vertices span.
fn centre(solid: &SolidSpec, edge: u64) -> Point3<f64> {
    let faces: Vec<usize> = solid
        .faces
        .iter()
        .enumerate()
        .filter(|(_, f)| {
            f.loops
                .iter()
                .any(|l| l.uses.iter().any(|u| u.edge == edge))
        })
        .map(|(fi, _)| fi)
        .collect();
    let (lo, hi) = bounds(solid, &faces);
    Point3::new(
        geom::mid_param(lo[0], hi[0]),
        geom::mid_param(lo[1], hi[1]),
        geom::mid_param(lo[2], hi[2]),
    )
}

/// The diagonal of the box the faces' vertices span, as a lever
/// ([`geom_brep::points_diameter`] of its two far corners).
fn reach(solid: &SolidSpec, faces: [usize; 2]) -> geom_core::Lever<f64> {
    let (lo, hi) = bounds(solid, &faces);
    geom_brep::points_diameter(&[
        Point3::new(lo[0], lo[1], lo[2]),
        Point3::new(hi[0], hi[1], hi[2]),
    ])
}

/// The box the faces' printed vertices span.
fn bounds(solid: &SolidSpec, faces: &[usize]) -> ([f64; 3], [f64; 3]) {
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for &f in faces {
        for l in &solid.faces[f].loops {
            for u in &l.uses {
                let Some(e) = solid.edges.get(&u.edge) else {
                    continue;
                };
                for v in [e.start, e.end] {
                    if let Some(p) = solid.vertices.get(&v) {
                        for (i, c) in [p.x, p.y, p.z].into_iter().enumerate() {
                            lo[i] = lo[i].min(c);
                            hi[i] = hi[i].max(c);
                        }
                    }
                }
            }
        }
    }
    (lo, hi)
}

/// The connected classes of the relation graph, each as its relations'
/// indices, in the order their least relation stands.
fn classes(nodes: usize, relations: &[Relation]) -> Vec<Vec<usize>> {
    let mut parent: Vec<usize> = (0..nodes).collect();
    fn root(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    for r in relations {
        let (a, b) = (root(&mut parent, r.nodes.0), root(&mut parent, r.nodes.1));
        if a != b {
            parent[a.max(b)] = a.min(b);
        }
    }
    let mut by: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i, r) in relations.iter().enumerate() {
        let k = root(&mut parent, r.nodes.0);
        by.entry(k).or_default().push(i);
    }
    let mut out: Vec<Vec<usize>> = by.into_values().collect();
    out.sort();
    out
}

/// One construction's move: the surface it moves, where to, how far, and
/// the relation whose construction it is.
struct Move {
    node: usize,
    surface: Surface<f64>,
    moved_by: f64,
    relation: SnapRelation,
}

/// A symmetric relation's pair as (kept, moved): the earlier record
/// stays.
fn later(nodes: &[Node], r: &Relation) -> (usize, usize) {
    let (a, b) = r.nodes;
    if nodes[a].record <= nodes[b].record {
        (a, b)
    } else {
        (b, a)
    }
}

/// **The class's constructions** (module docs): each mover's new surface
/// and its move, every one computed from surfaces no construction moves.
fn solve(
    nodes: &[Node],
    relations: &[Relation],
    class: &[usize],
    file: FileBand,
) -> Result<Vec<Move>, StepImportError> {
    // Which surface each relation not yet exact moves, and from what.
    let mut movers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut fixed: Vec<(usize, u64)> = Vec::new();
    for &r in class {
        let rel = &relations[r];
        if rel.exact {
            continue;
        }
        let (keep, moved) = match rel.kind {
            SnapRelation::Tangent => (rel.nodes.0, rel.nodes.1),
            SnapRelation::Coaxial | SnapRelation::AxisNormal => later(nodes, rel),
        };
        movers.entry(moved).or_default().push(r);
        fixed.push((keep, rel.edge));
    }
    for &(keep, edge) in &fixed {
        if movers.contains_key(&keep) {
            return Err(unsolved(
                file,
                edge,
                "a relation whose kept surface another of its class's constructions moves"
                    .to_owned(),
            ));
        }
    }
    let mut out = Vec::new();
    for (&n, rs) in &movers {
        let first = &relations[rs[0]];
        let kinds: Vec<SnapRelation> = rs.iter().map(|&r| relations[r].kind).collect();
        if kinds.iter().any(|&k| k != first.kind)
            || (first.kind != SnapRelation::Tangent && rs.len() > 1)
        {
            return Err(unsolved(
                file,
                first.edge,
                format!(
                    "a {} whose surface two of its class's constructions would move",
                    first.kind.name()
                ),
            ));
        }
        let (surface, moved_by) = match first.kind {
            SnapRelation::Tangent => tangent(nodes, relations, class, n, file)?,
            SnapRelation::Coaxial => {
                let (keep, _) = later(nodes, first);
                coaxial(
                    &nodes[keep].surface,
                    &nodes[n].surface,
                    first.lever.length(),
                )
            }
            SnapRelation::AxisNormal => {
                let (keep, _) = later(nodes, first);
                axis_normal(
                    &nodes[keep].surface,
                    &nodes[n].surface,
                    first.lever.length(),
                )
            }
        };
        match file.read(STEP_SNAP_MOVE, moved_by) {
            Read::Zero => {}
            Read::InBand => {
                return Err(
                    file.refused(nodes[n].record.id, file.ambiguous(STEP_SNAP_MOVE, moved_by))
                );
            }
            Read::Apart => {
                return Err(unsolved(
                    file,
                    first.edge,
                    format!(
                        "a {} whose construction moves its surface {moved_by:e} m, past the \
                         file's band",
                        first.kind.name()
                    ),
                ));
            }
        }
        out.push(Move {
            node: n,
            surface,
            moved_by,
            relation: first.kind,
        });
    }
    Ok(out)
}

/// **The tangent construction** for cylinder `n`: its axis translated
/// square to itself to stand its radius from each plane it is tangent to
/// in `class`, on the side it stands.
fn tangent(
    nodes: &[Node],
    relations: &[Relation],
    class: &[usize],
    n: usize,
    file: FileBand,
) -> Result<(Surface<f64>, f64), StepImportError> {
    let mine: Vec<&Relation> = class
        .iter()
        .map(|&r| &relations[r])
        .filter(|r| r.nodes.0 == n || r.nodes.1 == n)
        .collect();
    let edge = mine[0].edge;
    if let Some(other) = mine.iter().find(|r| r.kind == SnapRelation::Coaxial) {
        return Err(unsolved(
            file,
            other.edge,
            "a cylinder tangent to a plane and coaxial with another surface".to_owned(),
        ));
    }
    let planes: Vec<&Relation> = mine
        .iter()
        .copied()
        .filter(|r| r.kind == SnapRelation::Tangent)
        .collect();
    let Surface::Cylinder {
        origin,
        axis: a,
        radius,
        u_ref,
    } = nodes[n].surface
    else {
        return Err(unsolved(file, edge, "a tangency off a cylinder".to_owned()));
    };
    // Each plane's normal and origin, and the axis's signed offset to its
    // tangency.
    let need: Vec<(Vec3<f64>, Point3<f64>, f64)> = planes
        .iter()
        .map(|r| {
            let Surface::Plane {
                origin: p, normal, ..
            } = nodes[r.nodes.0].surface
            else {
                unreachable!("a tangent relation's first surface is its plane")
            };
            let s = normal.dot(origin - p);
            (normal, p, s.signum() * radius - s)
        })
        .collect();
    let (delta, radius_to) = match need[..] {
        [(n1, _, g1)] => (n1 * g1, radius),
        [(n1, p1, g1), (n2, p2, g2)] => {
            let lever = if planes[0].lever.length() >= planes[1].lever.length() {
                planes[0].lever
            } else {
                planes[1].lever
            };
            match geom_brep::directions_parallel(
                geom_brep::Directions::PlaneNormals,
                n1,
                n2,
                lever,
                file.band(),
            ) {
                Ok(false) => {
                    let c = n1.dot(n2);
                    let det = 1.0 - c.powi(2);
                    let x = (g1 - c * g2) / det;
                    let y = (g2 - c * g1) / det;
                    (n1 * x + n2 * y, radius)
                }
                // Two parallel planes: the radius is half their spacing and
                // the axis moves along their normal to the mid-plane, where
                // the axis lies between them.
                Ok(true) => {
                    let (c1, c2, at) = (
                        n1.dot(p1 - Point3::origin()),
                        n1.dot(p2 - Point3::origin()),
                        n1.dot(origin - Point3::origin()),
                    );
                    // Which side of each plane the axis lies, along the
                    // one normal: between them where the two sides differ.
                    let side = |c: f64| {
                        geom_core::k_stats::decide(
                            STEP_SNAP_SIDE,
                            geom_core::Margin::of(at - c),
                            file.band(),
                        )
                        .map_err(|_| {
                            file.refused(nodes[n].record.id, file.ambiguous(STEP_SNAP_SIDE, at - c))
                        })
                    };
                    let (s1, s2) = (side(c1)?, side(c2)?);
                    if s1.is_zero() || s1 == s2 {
                        return Err(unsolved(
                            file,
                            planes[1].edge,
                            "a cylinder tangent to two parallel planes whose axis does not lie \
                             between them"
                                .to_owned(),
                        ));
                    }
                    (n1 * ((c1 + c2) / 2.0 - at), (c2 - c1).abs() / 2.0)
                }
                Err(_) => {
                    return Err(unsolved(
                        file,
                        planes[1].edge,
                        "a cylinder tangent to two planes the file's band does not read \
                         parallel or crossing"
                            .to_owned(),
                    ));
                }
            }
        }
        _ => {
            return Err(unsolved(
                file,
                planes[2].edge,
                "a cylinder tangent to three or more planes".to_owned(),
            ));
        }
    };
    // Square to the axis, so the move is a translation across it.
    let delta = delta - a * delta.dot(a);
    Ok((
        Surface::Cylinder {
            origin: origin + delta,
            axis: a,
            radius: radius_to,
            u_ref,
        },
        delta.norm() + (radius_to - radius).abs(),
    ))
}

/// **The coaxial construction**: `moved` onto `kept`'s axis.
fn coaxial(kept: &Surface<f64>, moved: &Surface<f64>, lever: f64) -> (Surface<f64>, f64) {
    let mut out = moved.clone();
    let mut moved_by = 0.0_f64;
    match (axis(kept), axis(moved)) {
        (Some(line), Some((o, d))) => {
            let dir = if d.dot(line.1) < 0.0 { -line.1 } else { line.1 };
            let at = foot(o, line);
            moved_by = (at - o).norm() + (dir - d).norm() * lever;
            set_axis(&mut out, at, dir);
        }
        (Some(line), None) => {
            if let Some(c) = datum(moved) {
                let at = foot(c, line);
                moved_by = (at - c).norm();
                set_centre(&mut out, at);
            }
        }
        (None, Some((o, d))) => {
            if let Some(c) = datum(kept) {
                let at = o + (c - foot(c, (o, d)));
                moved_by = (at - o).norm();
                set_axis(&mut out, at, d);
            }
        }
        (None, None) => {
            if let (Some(c), Some(m)) = (datum(kept), datum(moved)) {
                moved_by = (c - m).norm();
                set_centre(&mut out, c);
            }
        }
    }
    (out, moved_by)
}

/// **The axis-normal construction**: `moved` turned about where the axis
/// meets the plane, a plane's normal onto the axis or an axis onto the
/// normal.
fn axis_normal(kept: &Surface<f64>, moved: &Surface<f64>, lever: f64) -> (Surface<f64>, f64) {
    let (plane, round) = if matches!(kept, Surface::Plane { .. }) {
        (kept, moved)
    } else {
        (moved, kept)
    };
    let (
        Surface::Plane {
            origin: p,
            normal: n,
            u_ref,
        },
        Some((o, d)),
    ) = (plane, axis(round))
    else {
        return (moved.clone(), f64::INFINITY);
    };
    let (p, n, u_ref) = (*p, *n, *u_ref);
    // Where the axis meets the plane: the two read along one another, so
    // the axis's cosine against the normal is near one.
    let pivot = o + d * (n.dot(p - o) / n.dot(d));
    if matches!(moved, Surface::Plane { .. }) {
        let normal = if n.dot(d) < 0.0 { -d } else { d };
        let u_ref = geom::Frame::new(pivot, normal, u_ref).square().u_ref;
        let moved_by = (normal - n).norm() * lever + n.dot(pivot - p).abs();
        (
            Surface::Plane {
                origin: pivot,
                normal,
                u_ref,
            },
            moved_by,
        )
    } else {
        let dir = if d.dot(n) < 0.0 { -n } else { n };
        let h = (pivot - o).dot(d);
        let at = pivot - dir * h;
        let mut out = moved.clone();
        set_axis(&mut out, at, dir);
        (out, (dir - d).norm() * lever + (at - o).norm())
    }
}

/// `s`'s axis set to run through `at` along unit `dir`, its seam squared
/// to it.
fn set_axis(s: &mut Surface<f64>, at: Point3<f64>, dir: Vec3<f64>) {
    match s {
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        }
        | Surface::Torus {
            center: origin,
            axis,
            u_ref,
            ..
        }
        | Surface::Cone {
            apex: origin,
            axis,
            u_ref,
            ..
        } => {
            *u_ref = geom::Frame::new(at, dir, *u_ref).square().u_ref;
            *origin = at;
            *axis = dir;
        }
        Surface::Sphere { center, .. } => *center = at,
        _ => {}
    }
}

/// `s`'s centre set to `at`.
fn set_centre(s: &mut Surface<f64>, at: Point3<f64>) {
    if let Surface::Sphere { center, .. } = s {
        *center = at;
    }
}

/// The refusal of a class the catalogue does not solve, at edge `edge`.
fn unsolved(file: FileBand, edge: u64, what: String) -> StepImportError {
    StepImportError::NotYetSupported {
        entity: BuiltEntity::Edge,
        id: edge,
        what,
        lane: UNCATALOGUED_LANE,
        file: geom_core::FileCoincidence::new(file.eps_in()),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **`nist_ftc_09`'s five tangent snaps, and the build's reading of
    /// its six tangent edges after them.** The file (ε_in ≈ 33.8 µm)
    /// states five fillet cylinders biting into their planes by 1.5 to
    /// 13 µm: three between two parallel planes (a slot's end, the
    /// radius made half the spacing), two between crossing planes (the
    /// axis translated). Every one snaps. The writer printed each
    /// tangent edge as a crossing line of the pair as it wrote it,
    /// `√(2·r·gap)` from the ruling the snap makes exact, so the build
    /// reads five of the six printed carriers off their description in
    /// or past the file's band and refuses them as the file's; the
    /// sixth, 35 µm off, builds
    /// (`work/importgate/a-tangent-edge-printed-as-a-crossing-line-refuses-at-the-build.md`).
    #[test]
    fn nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them() {
        let tol = geom_core::Tol::witness();
        let text = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp"
        ))
        .unwrap();
        let parsed = crate::parse::parse_file(&text).unwrap();
        let model = crate::entities::resolve(&parsed, None, tol).unwrap();
        let mut snapped: Vec<(u64, SnapRelation)> = model
            .normalizations
            .iter()
            .filter_map(|n| match n.kind {
                NormalizationKind::Snapped {
                    surface,
                    relation,
                    moved_by,
                } => {
                    assert!(
                        moved_by > 0.0 && moved_by < model.uncertainty_m,
                        "#{surface}: a move inside ε_in: {moved_by:e}"
                    );
                    Some((surface, relation))
                }
                _ => None,
            })
            .collect();
        snapped.sort_unstable_by_key(|&(s, _)| s);
        assert_eq!(
            snapped,
            [4962, 4990, 5102, 5231, 5336].map(|s| (s, SnapRelation::Tangent)),
            "the five fillets snap, each by the tangent construction"
        );
        let crate::entities::Shape::Solids(solids) = &model.shape else {
            panic!("a solid file")
        };
        let solid = &solids[0].outer;
        let file = FileBand::of_import(model.uncertainty_m, false, tol).unwrap();
        // Each tangent edge whose pair moved: how far its printed carrier
        // lies from the ruling (`None`: it builds).
        for (edge, off) in [
            (4950, Some(1.802e-4)),
            (4964, Some(1.227e-4)),
            (4978, Some(3.439e-4)),
            (5090, Some(1.150e-4)),
            (5233, Some(2.329e-4)),
            (5338, None),
        ] {
            match (crate::build::choose_edge(solid, edge, tol, file), off) {
                (Ok(()), None) => {}
                (
                    Err(StepImportError::Interpretation {
                        id,
                        refusal:
                            crate::interpret::Refusal::MissInBand {
                                reading, miss: gap, ..
                            }
                            | crate::interpret::Refusal::EvidenceMiss {
                                reading, miss: gap, ..
                            },
                        ..
                    }),
                    Some(off),
                ) => {
                    assert_eq!(id, edge);
                    assert_eq!(reading, crate::build::STEP_EDGE_ON_DESCRIPTION);
                    assert!(
                        (gap - off).abs() < 1e-7,
                        "#{edge}: the printed carrier lies √(2rg) off the ruling: {gap:e}"
                    );
                }
                (other, off) => panic!("#{edge}: expected {off:?}, got {other:?}"),
            }
        }
    }
}

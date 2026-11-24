//! **The snap catalogue** (`[import]`, D7 step 3, Healing): a relation
//! between two surfaces that reads `Zero` at the file's band is made exact
//! by one construction from a closed catalogue, before anything is built.
//!
//! **The catalogue**, stated here and in [`SnapRelation`] alone:
//!
//! - **coaxial**: two round surfaces (cylinder, cone, sphere, torus) whose
//!   axes, or a sphere's centre, read one line. The later record in the
//!   file moves onto the earlier one's axis: turned onto its direction and
//!   moved onto the line about the point of its own axis at its faces (a
//!   moved sphere's centre to its foot on the line; a sphere kept, the
//!   moved axis translated through its centre; two spheres, one centre).
//! - **tangent, one plane or two crossing planes**: a plane and a cylinder
//!   the file band reads tangent along their edge. The planes stay; the
//!   cylinder's axis is translated square to itself, its radius kept, to
//!   stand its radius from each plane it is tangent to, on the side it
//!   stands: along the normal for one plane, to where the two offsets meet
//!   for two crossing planes.
//! - **tangent, between two parallel planes** (a slot's or a rib's end):
//!   the planes stay; the radius becomes half their spacing and the axis
//!   moves along their normal to the mid-plane. The rule is that the axis
//!   lies between the planes ([`STEP_SNAP_SIDE`]); which way the planes
//!   face is not read, since a cylinder tangent to both and between them
//!   leaves no other facing.
//! - **tangent, two planes neither parallel nor crossing**: their
//!   parallelism read in the file's band ([`STEP_SNAP_PARALLEL`]), the
//!   file's ambiguity.
//! - **axis-normal**: a plane and a cylinder, cone or torus whose axis
//!   reads along the plane's normal. The later record turns about the
//!   point where the axis meets the plane: a plane's normal onto the axis,
//!   or a round surface's axis onto the normal.
//! - **one locus**: two records of one surface become one
//!   ([`crate::one_record`], run before this).
//!
//! A cylinder tangent to three or more planes, or to two parallel planes
//! its axis does not lie between, or coaxial with another surface while
//! its tangency moves it, refuses as a class the catalogue does not solve.
//!
//! **Read at the faces.** A relation is read only between the two faces of
//! an edge, over the region they occupy ([`crate::reach::Reach`]): a
//! distance in metres where the faces are, an angle levered at their
//! reach ([`gap`]). Read at the file's band ([`STEP_SNAP_RELATION`]),
//! `Zero` is a relation; at the run's band, `Zero` is a relation already
//! exact. A pair no candidate reads `Zero` and one reads in the file's
//! band refuses here as the file's ambiguity (D7): a snap is a
//! structural act, so in band it refuses on its own reading, whatever
//! the build would read of the pair.
//!
//! **One construction per class.** The relations form a graph over the
//! surfaces; a connected class all of whose relations are already exact
//! moves nothing. Otherwise each relation not yet exact names its mover
//! (the tangent's cylinder, the coaxial or axis-normal pair's later
//! record), and every move is computed from inputs no construction moves,
//! so no order decides the result. Each move is measured over every face
//! its record carries, as the farthest a point of them moves, and read at
//! the file's band ([`STEP_SNAP_MOVE`]): `Zero` applies it, in band
//! refuses as the file's ambiguity, apart refuses as unsolved. Then every
//! relation of the class, the exact ones among them, is read again at the
//! run's band, and one that is not `Zero` refuses as unsolved. Each moved
//! surface is reported ([`NormalizationKind::Snapped`]).
//!
//! **A known limit: the solve is anchored at each relation's earlier
//! record, not at the class's.** A surface two constructions would move,
//! or a construction that would read a surface another one moves (a chain
//! A–B–C of coaxial records, a plate later in the file than two bosses it
//! is normal to), refuses loudly, naming
//! `work/importgate/a-snap-class-whose-solve-moves-a-surface-twice-refuses.md`:
//! whether such a file imports turns on which record it states first.

use std::collections::BTreeMap;

use geom::{Line, Surface};
use geom_core::{Point3, Tol, UnitVec3, Vec3};

use crate::entities::SolidSpec;
use crate::error::{BuiltEntity, StepImportError};
use crate::interpret::{FileBand, Read};
use crate::reach::Reach;
use crate::{FaceCensus, NormalizationKind, SnapRelation, StructureNormalization};

/// A catalogue relation's gap between two surfaces, in metres: at the
/// file's band `Zero` is a relation; at the run's, an exact one.
pub(crate) const STEP_SNAP_RELATION: &str = "step_snap_relation";

/// Which side of a plane a cylinder's axis lies, signed along one of two
/// parallel planes' normals, read at the file's band: the axis lies
/// between the two where its sides differ.
pub(crate) const STEP_SNAP_SIDE: &str = "step_snap_side";

/// Whether the two planes a cylinder is tangent to are parallel: the sine
/// between their normals levered at the cylinder's faces' reach, read at
/// the file's band.
pub(crate) const STEP_SNAP_PARALLEL: &str = "step_snap_parallel";

/// How far a catalogue construction moves a surface, in metres, read at
/// the file's band: only `Zero` is applied.
pub(crate) const STEP_SNAP_MOVE: &str = "step_snap_move";

/// The lane a relation the file band reads `Zero` and the run reads
/// apart waits on where the catalogue makes it exact by no construction:
/// a relation outside it, or a class its constructions do not solve.
pub(crate) const UNCATALOGUED_LANE: &str = "a snap catalogue construction that makes this \
     relation exact (`work/importgate/a-relation-the-snap-catalogue-does-not-make-exact-refuses.md`)";

/// The lane a class whose solve, anchored at each relation's earlier
/// record, would move a surface twice waits on.
pub(crate) const ANCHOR_LANE: &str = "a joint solve anchored at the class's earliest record \
     (`work/importgate/a-snap-class-whose-solve-moves-a-surface-twice-refuses.md`)";

/// One surface record of the shell, and the faces that wear it.
struct Node {
    faces: Vec<usize>,
    record: u64,
    /// Where the record stands in the file ([`SolidSpec::order`]).
    order: (usize, u64),
}

/// One relation the file band reads `Zero`, along edge `edge`.
struct Relation {
    kind: SnapRelation,
    /// The two surfaces: for a tangent or an axis-normal pair the plane
    /// first.
    nodes: (usize, usize),
    edge: u64,
    /// The region the edge's two faces occupy, which the relation is read
    /// over.
    reach: Reach,
    /// Read `Zero` at the run's band.
    exact: bool,
}

/// One construction's move: the surface it moves, where to, how far, and
/// the relation whose construction it is.
struct Move {
    node: usize,
    surface: Surface<f64>,
    moved_by: f64,
    relation: SnapRelation,
}

/// **Makes each relation the file band reads `Zero` exact** (module docs).
pub(crate) fn snap(
    solid: &mut SolidSpec,
    file: FileBand,
    tol: Tol,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let run = geom_core::Band::linear(tol).map_err(|_| StepImportError::Topology {
        id: solid.id,
        what: "the run's tolerance makes no band to build at",
    })?;
    let nodes = nodes(solid);
    let relations = relations(solid, &nodes, file, run)?;
    for class in classes(nodes.len(), &relations) {
        if class.iter().all(|&r| relations[r].exact) {
            continue;
        }
        let moves = solve(solid, &nodes, &relations, &class, file)?;
        for m in &moves {
            let Some(record) = solid.surfaces.get_mut(&nodes[m.node].record) else {
                unreachable!("a node is a record of the shell")
            };
            record.surface = m.surface.clone();
        }
        for &r in &class {
            let rel = &relations[r];
            let g = gap(
                rel.kind,
                &solid.surfaces[&nodes[rel.nodes.0].record].surface,
                &solid.surfaces[&nodes[rel.nodes.1].record].surface,
                &rel.reach,
            );
            if !run_zero(run, g) {
                return Err(unsolved(
                    file,
                    rel.edge,
                    format!(
                        "the {} relation its class's constructions leave {g:e} m from exact",
                        rel.kind.name()
                    ),
                ));
            }
        }
        for Move {
            node: n,
            moved_by,
            relation,
            ..
        } in moves
        {
            sink.push(StructureNormalization {
                face: nodes[n].record,
                kind: NormalizationKind::Snapped {
                    surface: nodes[n].record,
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

/// The shell's surface records, in the order its faces first wear them.
fn nodes(solid: &SolidSpec) -> Vec<Node> {
    let mut by: BTreeMap<u64, usize> = BTreeMap::new();
    let mut nodes: Vec<Node> = Vec::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        let n = *by.entry(face.record).or_insert_with(|| {
            nodes.push(Node {
                faces: Vec::new(),
                record: face.record,
                order: solid.order(face.record),
            });
            nodes.len() - 1
        });
        nodes[n].faces.push(fi);
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
/// edge; a pair one reads in the band, and none `Zero`, refuses (module
/// docs).
fn relations(
    solid: &SolidSpec,
    nodes: &[Node],
    file: FileBand,
    run: geom_core::Band,
) -> Result<Vec<Relation>, StepImportError> {
    let of = node_of(nodes, solid.faces.len());
    let mut sides: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        for lp in &face.loops {
            for u in &lp.uses {
                sides.entry(u.edge).or_default().push(fi);
            }
        }
    }
    let mut found: Vec<Relation> = Vec::new();
    for (&edge, faces) in &sides {
        let [fa, fb] = faces[..] else { continue };
        let (na, nb) = (of[fa], of[fb]);
        if na == nb {
            continue;
        }
        let Some(reach) = Reach::of(solid, [fa, fb]) else {
            continue;
        };
        let (sa, sb) = (
            &solid.surfaces[&nodes[na].record].surface,
            &solid.surfaces[&nodes[nb].record].surface,
        );
        let mut in_band = None;
        let mut read = false;
        for (kind, pair) in candidates(sa, sb, (na, nb)) {
            let (s1, s2) = (
                &solid.surfaces[&nodes[pair.0].record].surface,
                &solid.surfaces[&nodes[pair.1].record].surface,
            );
            let g = gap(kind, s1, s2, &reach);
            let reading = file.read(STEP_SNAP_RELATION, g);
            if reading == Read::InBand && in_band.is_none() {
                let later = nodes[pair.0].order.max(nodes[pair.1].order);
                in_band = Some((later.1, g));
            }
            if reading == Read::Zero {
                read = true;
                let exact = run_zero(run, g);
                // One relation per pair, over every edge it is read along:
                // exact only where it is along each, over the widest reach.
                match found.iter_mut().find(|r| r.kind == kind && r.nodes == pair) {
                    Some(same) => {
                        same.exact &= exact;
                        if reach.lever().length() > same.reach.lever().length() {
                            (same.reach, same.edge) = (reach, edge);
                        }
                    }
                    None => found.push(Relation {
                        kind,
                        nodes: pair,
                        edge,
                        reach,
                        exact,
                    }),
                }
                break;
            }
        }
        if let (false, Some((id, g))) = (read, in_band) {
            return Err(file.refused(id, file.ambiguous(STEP_SNAP_RELATION, g)));
        }
    }
    Ok(found)
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

/// A cylinder's, cone's or torus's axis, its direction unit
/// ([`Surface::axis_line`]); a sphere's centre has no axis of its own.
fn axis(s: &Surface<f64>) -> Option<Line<f64>> {
    match s {
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Torus { .. } => s.axis_line(),
        _ => None,
    }
}

/// A round surface's datum point, as its axis line places it: an axis
/// point, an apex, a centre.
fn datum(s: &Surface<f64>) -> Option<Point3<f64>> {
    match s {
        Surface::Sphere { frame, .. } => Some(frame.origin()),
        _ => axis(s).map(|l| l.origin),
    }
}

/// A cylinder's radius; `None` for any other surface.
fn cylinder_radius(s: &Surface<f64>) -> Option<f64> {
    match *s {
        Surface::Cylinder { radius, .. } => Some(radius),
        _ => None,
    }
}

/// A plane's point and unit normal.
fn plane_of(s: &Surface<f64>) -> Option<(Point3<f64>, Vec3<f64>)> {
    match *s {
        Surface::Plane { frame, .. } => {
            let (origin, normal) = (frame.origin(), frame.axis());
            Some((origin, normal))
        }
        _ => None,
    }
}

/// Where `line` meets the plane through `p` with normal `n`; `None` where
/// the line runs nearer along the plane than across it.
fn meets(line: Line<f64>, (p, n): (Point3<f64>, Vec3<f64>)) -> Option<Point3<f64>> {
    let across = n.dot(line.dir.get());
    (across.abs() > std::f64::consts::FRAC_1_SQRT_2)
        .then(|| line.at(n.dot(p - line.origin) / across))
}

/// **A relation's gap, in metres**, read where the faces are (`reach`):
/// a tangency's offset from the radius at the ends of the faces' extent
/// along the axis; an axis's tilt off a plane's normal levered at the
/// faces' reach from where the axis meets the plane; two axes'
/// separation at the ends of the faces' extent along one of them. The
/// pair is in the relation's order. Infinite for a pair the relation does
/// not hold of.
fn gap(kind: SnapRelation, a: &Surface<f64>, b: &Surface<f64>, reach: &Reach) -> f64 {
    let far = f64::INFINITY;
    match kind {
        SnapRelation::Tangent => {
            let (Some((p, n)), Some(line), Some(radius)) =
                (plane_of(a), axis(b), cylinder_radius(b))
            else {
                return far;
            };
            reach
                .ends_on(line)
                .into_iter()
                .map(|q| (n.dot(q - p).abs() - radius).abs())
                .fold(0.0, f64::max)
        }
        SnapRelation::AxisNormal => {
            let (Some(plane), Some(line)) = (plane_of(a), axis(b)) else {
                return far;
            };
            meets(line, plane).map_or(far, |pivot| {
                plane.1.cross(line.dir.get()).norm() * reach.from(pivot)
            })
        }
        SnapRelation::Coaxial => match (axis(a), axis(b)) {
            (Some(la), Some(lb)) => reach.axes_apart(la, lb),
            (Some(l), None) => datum(b).map_or(far, |c| l.distance(c)),
            (None, Some(l)) => datum(a).map_or(far, |c| l.distance(c)),
            (None, None) => match (datum(a), datum(b)) {
                (Some(c1), Some(c2)) => (c2 - c1).norm(),
                _ => far,
            },
        },
    }
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

/// A symmetric relation's pair as (kept, moved): the earlier record
/// stays.
fn later(nodes: &[Node], r: &Relation) -> (usize, usize) {
    let (a, b) = r.nodes;
    if nodes[a].order <= nodes[b].order {
        (a, b)
    } else {
        (b, a)
    }
}

/// **The class's constructions** (module docs): each mover's new surface
/// and its move, every one computed from surfaces no construction moves.
fn solve(
    solid: &SolidSpec,
    nodes: &[Node],
    relations: &[Relation],
    class: &[usize],
    file: FileBand,
) -> Result<Vec<Move>, StepImportError> {
    // Which surface each relation not yet exact moves, and from what.
    let mut movers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut fixed: Vec<(usize, usize)> = Vec::new();
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
        fixed.push((keep, r));
    }
    for &(keep, r) in &fixed {
        if movers.contains_key(&keep) {
            return Err(anchored(
                file,
                relations[r].edge,
                format!(
                    "the {} relation whose kept surface another of its class's constructions \
                     moves",
                    relations[r].kind.name()
                ),
            ));
        }
    }
    let mut out = Vec::new();
    for (&n, rs) in &movers {
        let first = &relations[rs[0]];
        if rs.iter().any(|&r| relations[r].kind != first.kind)
            || (first.kind != SnapRelation::Tangent && rs.len() > 1)
        {
            return Err(anchored(
                file,
                first.edge,
                "a surface two of its class's constructions would move".to_owned(),
            ));
        }
        // The region every face the moved record carries occupies: the
        // move is measured over it.
        let Some(faces) = Reach::of(solid, nodes[n].faces.iter().copied()) else {
            return Err(unsolved(
                file,
                first.edge,
                "a surface to move whose faces' region does not read".to_owned(),
            ));
        };
        let (surface, moved_by) = match first.kind {
            SnapRelation::Tangent => tangent(solid, nodes, relations, class, n, &faces, file)?,
            SnapRelation::Coaxial => {
                let (keep, _) = later(nodes, first);
                coaxial(
                    &solid.surfaces[&nodes[keep].record].surface,
                    &solid.surfaces[&nodes[n].record].surface,
                    &faces,
                )
            }
            SnapRelation::AxisNormal => {
                let (keep, _) = later(nodes, first);
                axis_normal(
                    &solid.surfaces[&nodes[keep].record].surface,
                    &solid.surfaces[&nodes[n].record].surface,
                    &faces,
                )
            }
        };
        match file.read(STEP_SNAP_MOVE, moved_by) {
            Read::Zero => {}
            Read::InBand => {
                return Err(file.refused(nodes[n].record, file.ambiguous(STEP_SNAP_MOVE, moved_by)));
            }
            Read::Apart => {
                return Err(unsolved(
                    file,
                    first.edge,
                    format!(
                        "the {} relation's construction moving its surface {moved_by:e} m, past \
                         the file's band",
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

/// **The tangent construction** for cylinder `n`, whose faces occupy
/// `faces`: its axis translated square to itself to stand its radius from
/// each plane it is tangent to in `class`, on the side it stands, read at
/// its faces; between two parallel planes, its radius half their spacing
/// and its axis on the mid-plane.
fn tangent(
    solid: &SolidSpec,
    nodes: &[Node],
    relations: &[Relation],
    class: &[usize],
    n: usize,
    faces: &Reach,
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
            "a cylinder whose tangency moves it while it is coaxial with another surface"
                .to_owned(),
        ));
    }
    let planes: Vec<&Relation> = mine
        .iter()
        .copied()
        .filter(|r| r.kind == SnapRelation::Tangent)
        .collect();
    let (
        Surface::Cylinder {
            frame: cylinder_frame,
            radius,
        },
        Some(line),
    ) = (
        &solid.surfaces[&nodes[n].record].surface,
        axis(&solid.surfaces[&nodes[n].record].surface),
    )
    else {
        return Err(unsolved(file, edge, "a tangency off a cylinder".to_owned()));
    };
    let origin = cylinder_frame.origin();
    // The axis's point where its faces are.
    let q = line.foot(faces.centre());
    // Each plane's normal and level along it, and the axis's offset at its
    // faces from where it stands its radius from the plane.
    let need: Vec<Offset> = planes
        .iter()
        .filter_map(|r| plane_of(&solid.surfaces[&nodes[r.nodes.0].record].surface))
        .map(|(p, normal)| {
            let s = normal.dot(q - p);
            (
                normal,
                normal.dot(p - Point3::origin()),
                s.signum() * radius - s,
            )
        })
        .collect();
    let (delta, radius_to) = match need[..] {
        [(n1, _, g1)] => (n1 * g1, *radius),
        [one, two] => two_planes(
            (one, two),
            (q, *radius),
            faces.lever().length(),
            file,
            nodes[n].record,
        )?
        .map_err(|what| unsolved(file, planes[1].edge, what.to_owned()))?,
        _ => {
            return Err(unsolved(
                file,
                planes[2].edge,
                "a cylinder tangent to three or more planes".to_owned(),
            ));
        }
    };
    // Square to the axis, so the move is a translation across it.
    let delta = delta - line.dir.get() * delta.dot(line.dir.get());
    Ok((
        Surface::Cylinder {
            frame: cylinder_frame.with_origin(origin + delta),
            radius: radius_to,
        },
        delta.norm() + (radius_to - radius).abs(),
    ))
}

/// A plane a cylinder is tangent to, as [`two_planes`] reads it: its unit
/// normal, its level along it, and the axis's offset from where it stands
/// its radius from it.
type Offset = (Vec3<f64>, f64, f64);

/// **A cylinder tangent to two planes**, each as its unit normal, its
/// level along it and the axis's offset `g` from where it stands its
/// radius from it: the axis's translation and the radius it takes. The
/// two planes' parallelism is read at the file's band over `lever`, the
/// cylinder's faces' reach ([`STEP_SNAP_PARALLEL`]): crossing, the axis
/// moves to where the two offsets meet, its radius kept; parallel, the
/// parallel arm ([`between_planes`]) about the axis point `q`; in band,
/// the file's ambiguity, named at `record`. The inner error is the class
/// the arm does not solve.
fn two_planes(
    ((n1, c1, g1), (n2, c2, g2)): (Offset, Offset),
    (q, radius): (Point3<f64>, f64),
    lever: f64,
    file: FileBand,
    record: u64,
) -> Result<Result<(Vec3<f64>, f64), &'static str>, StepImportError> {
    let parallel = n1.cross(n2).norm() * lever;
    Ok(match file.read(STEP_SNAP_PARALLEL, parallel) {
        Read::Apart => {
            let c = n1.dot(n2);
            let det = n1.cross(n2).norm().powi(2);
            let x = (g1 - c * g2) / det;
            let y = (g2 - c * g1) / det;
            Ok((n1 * x + n2 * y, radius))
        }
        Read::InBand => {
            return Err(file.refused(record, file.ambiguous(STEP_SNAP_PARALLEL, parallel)));
        }
        // Their levels along the first's normal, the second's read the way
        // its own normal points.
        Read::Zero => {
            let at = n1.dot(q - Point3::origin());
            let c2 = if n1.dot(n2) < 0.0 { -c2 } else { c2 };
            between_planes(at, (c1, c2), file, record)?
                .map(|(shift, radius_to)| (n1 * shift, radius_to))
                .ok_or(
                    "a cylinder tangent to two parallel planes whose axis does not lie between \
                     them",
                )
        }
    })
}

/// **The parallel arm**: for an axis at level `at` between two parallel
/// planes at levels `c1` and `c2` along one normal, the shift that puts it
/// on the mid-plane and the radius that is half their spacing; `None`
/// where the axis does not lie between them. Each side is read at the
/// file's band ([`STEP_SNAP_SIDE`]); a side the band does not read is the
/// file's ambiguity, named at `record`.
fn between_planes(
    at: f64,
    (c1, c2): (f64, f64),
    file: FileBand,
    record: u64,
) -> Result<Option<(f64, f64)>, StepImportError> {
    let side = |c: f64| {
        geom_core::k_stats::decide(STEP_SNAP_SIDE, geom_core::Margin::of(at - c), file.band())
            .map_err(|_| file.refused(record, file.ambiguous(STEP_SNAP_SIDE, at - c)))
    };
    let (s1, s2) = (side(c1)?, side(c2)?);
    Ok((!s1.is_zero() && !s2.is_zero() && s1 != s2)
        .then(|| (geom::mid_param(c1, c2) - at, (c2 - c1).abs() / 2.0)))
}

/// **The coaxial construction**: `moved`, whose faces occupy `faces`,
/// onto `kept`'s axis, and the farthest a point of its faces moves.
fn coaxial(kept: &Surface<f64>, moved: &Surface<f64>, faces: &Reach) -> (Surface<f64>, f64) {
    let mut out = moved.clone();
    match (axis(kept), axis(moved)) {
        (Some(onto), Some(from)) => {
            // Turned onto the kept direction about the moved axis's point
            // at its faces, and carried onto the kept line there.
            let pivot = from.foot(faces.centre());
            let dir = if from.dir.get().dot(onto.dir.get()) < 0.0 {
                -onto.dir
            } else {
                onto.dir
            };
            let to = onto.foot(pivot);
            if carry(&mut out, from, (pivot, to), dir).is_none() {
                return (moved.clone(), f64::INFINITY);
            }
            (
                out,
                (to - pivot).norm() + (dir.get() - from.dir.get()).norm() * faces.from(pivot),
            )
        }
        (Some(onto), None) => {
            let Some(c) = datum(moved) else {
                return (out, f64::INFINITY);
            };
            let to = onto.foot(c);
            set_centre(&mut out, to);
            (out, (to - c).norm())
        }
        (None, Some(from)) => {
            let Some(c) = datum(kept) else {
                return (out, f64::INFINITY);
            };
            let delta = c - from.foot(c);
            if carry(&mut out, from, (from.origin, from.origin + delta), from.dir).is_none() {
                return (moved.clone(), f64::INFINITY);
            }
            (out, delta.norm())
        }
        (None, None) => {
            let (Some(c), Some(m)) = (datum(kept), datum(moved)) else {
                return (out, f64::INFINITY);
            };
            set_centre(&mut out, c);
            (out, (c - m).norm())
        }
    }
}

/// **The axis-normal construction**: `moved`, whose faces occupy `faces`,
/// turned about where the axis meets the plane, a plane's normal onto the
/// axis or an axis onto the normal, and the farthest a point of its faces
/// moves.
fn axis_normal(kept: &Surface<f64>, moved: &Surface<f64>, faces: &Reach) -> (Surface<f64>, f64) {
    let (plane, round) = if matches!(kept, Surface::Plane { .. }) {
        (kept, moved)
    } else {
        (moved, kept)
    };
    let (
        Some((p, n)),
        Some(line),
        Surface::Plane {
            frame: plane_frame, ..
        },
    ) = (plane_of(plane), axis(round), plane)
    else {
        return (moved.clone(), f64::INFINITY);
    };
    let u_ref = plane_frame.u_ref();
    let Some(pivot) = meets(line, (p, n)) else {
        return (moved.clone(), f64::INFINITY);
    };
    if matches!(moved, Surface::Plane { .. }) {
        let normal = if n.dot(line.dir.get()) < 0.0 {
            -line.dir
        } else {
            line.dir
        };
        // The plane keeps its roll, squared to its new normal: the
        // decided mint ([`snapped_frame`]), which refuses a roll along
        // the normal rather than poisoning the frame.
        let Some(frame) = snapped_frame(pivot, normal, u_ref) else {
            return (moved.clone(), f64::INFINITY);
        };
        (
            Surface::Plane { frame },
            (normal.get() - n).norm() * faces.from(pivot),
        )
    } else {
        let dir = if line.dir.get().dot(n) < 0.0 { -n } else { n };
        let mut out = moved.clone();
        // `dir` is the plane's unit normal, either way round: checked,
        // its bits kept, so the carry squares it once, as it did.
        if carry(&mut out, line, (pivot, pivot), UnitVec3::checked(dir)).is_none() {
            return (moved.clone(), f64::INFINITY);
        }
        (out, (dir - line.dir.get()).norm() * faces.from(pivot))
    }
}

/// `s`, whose axis is `from`, moved rigidly: turned to run along unit
/// `dir` about the axis point `pivot`, and carried so `pivot` lands at
/// `to`. Its datum keeps its place along the axis from `pivot`; its seam
/// is squared to the new axis ([`snapped_frame`]). `None` where the seam
/// has no part square to the new axis, a typed refusal of the decided
/// mint, with `s` left as it was.
fn carry(
    s: &mut Surface<f64>,
    from: Line<f64>,
    (pivot, to): (Point3<f64>, Point3<f64>),
    dir: UnitVec3<f64>,
) -> Option<()> {
    match s {
        Surface::Cylinder { frame, .. }
        | Surface::Torus { frame, .. }
        | Surface::Cone { frame, .. } => {
            let along = from.dir.get().dot(frame.origin() - pivot);
            let at = to + dir.get() * along;
            *frame = snapped_frame(at, dir, frame.u_ref())?;
        }
        Surface::Sphere { frame, .. } => {
            *frame = frame.with_origin(frame.origin() + (to - pivot));
        }
        _ => {}
    }
    Some(())
}

/// **A snapped carrier's frame, minted decided**: the new axis `dir`
/// squared as the frame door squares it, and the part of the `seam` it
/// keeps square to it decided at the least band
/// ([`RigidMotion::from_aim_and_reference`](geom_core::RigidMotion::from_aim_and_reference), under
/// [`STEP_SNAP_FRAME`](crate::interpret::STEP_SNAP_FRAME)). It is
/// [`geom::Frame::new`]'s arithmetic step for step, so the bits are the
/// squared frame's; a seam along the axis is `None`, where the undecided
/// square would have poisoned the frame.
fn snapped_frame(
    origin: Point3<f64>,
    dir: UnitVec3<f64>,
    seam: Vec3<f64>,
) -> Option<geom::Frame<f64>> {
    geom_core::RigidMotion::from_aim_and_reference(
        origin,
        UnitVec3::squared(dir.get()),
        seam,
        crate::interpret::STEP_SNAP_FRAME,
        geom_core::Band::least(),
    )
    .ok()
    .map(|m| geom::Frame::of_motion(&m))
}

/// `s`'s centre set to `at`.
fn set_centre(s: &mut Surface<f64>, at: Point3<f64>) {
    if let Surface::Sphere { frame, .. } = s {
        *frame = frame.with_origin(at);
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

/// The refusal of a class whose solve, anchored at each relation's earlier
/// record, would move a surface twice (module docs, the known limit).
fn anchored(file: FileBand, edge: u64, what: String) -> StepImportError {
    StepImportError::NotYetSupported {
        entity: BuiltEntity::Edge,
        id: edge,
        what,
        lane: ANCHOR_LANE,
        file: geom_core::FileCoincidence::new(file.eps_in()),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The parallel arm reads "between" at the file's band**: an axis
    /// between two parallel planes takes the mid-plane and half their
    /// spacing; one beside both is no arm (a cylinder tangent to two
    /// parallel planes with its axis outside them has the two at one
    /// level, which no corpus or native body states, so this row reads the
    /// arm directly); a side inside the file's band is the file's
    /// ambiguity, which the run's finer band would read apart.
    #[test]
    fn the_parallel_arm_reads_between_at_the_files_band() {
        let file = FileBand::test(1e-5);
        assert_eq!(
            between_planes(0.75, (0.0, 2.0), file, 7).unwrap(),
            Some((0.25, 1.0)),
            "between: the mid-plane and half the spacing"
        );
        assert_eq!(
            between_planes(3.0, (0.0, 2.0), file, 7).unwrap(),
            None,
            "beside both: no arm"
        );
        match between_planes(5e-5, (0.0, 1e-4), file, 7) {
            Err(StepImportError::Interpretation {
                id,
                refusal: crate::interpret::Refusal::Ambiguous { reading, .. },
                ..
            }) => assert_eq!((id, reading), (7, STEP_SNAP_SIDE)),
            other => panic!("a side in the file's band is its ambiguity: {other:?}"),
        }
    }

    /// **Two planes a cylinder is tangent to**: crossing, the axis moves to
    /// where the two offsets meet, each plane's offset kept to its own
    /// normal; parallel within the file's band's zero, the parallel arm;
    /// parallel in the band, the file's ambiguity; parallel with the axis
    /// beside both, no arm.
    #[test]
    fn two_planes_are_read_crossing_parallel_or_in_band() {
        let file = FileBand::test(1e-5);
        let (y, x) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let at = |y: f64| Point3::new(0.0, y, 0.0);
        let (delta, radius) = two_planes(
            ((y, 0.0, 3e-6), (x, 0.0, 0.0)),
            (at(1.0), 1.0),
            1.0,
            file,
            7,
        )
        .unwrap()
        .unwrap();
        assert!(
            (delta - y * 3e-6).norm() < 1e-15 && radius == 1.0,
            "crossing: the offset along its own plane's normal: {delta:?}"
        );
        let down = Vec3::new(0.0, -1.0, 0.0);
        assert_eq!(
            two_planes(
                ((y, 0.0, 0.0), (down, -2.0, 0.0)),
                (at(0.75), 1.0),
                1.0,
                file,
                7
            )
            .unwrap()
            .map(|(d, r)| (d.y, r)),
            Ok((0.25, 1.0)),
            "parallel, the axis between: the mid-plane and half the spacing"
        );
        assert!(
            two_planes(
                ((y, 0.0, 0.0), (down, -2.0, 0.0)),
                (at(3.0), 1.0),
                1.0,
                file,
                7
            )
            .unwrap()
            .is_err(),
            "parallel, the axis beside both: no arm"
        );
        let tilt = 5e-5_f64;
        let near = Vec3::new(tilt.sin(), -tilt.cos(), 0.0);
        match two_planes(
            ((y, 0.0, 0.0), (near, -2.0, 0.0)),
            (at(1.0), 1.0),
            1.0,
            file,
            7,
        ) {
            Err(StepImportError::Interpretation {
                id,
                refusal: crate::interpret::Refusal::Ambiguous { reading, .. },
                ..
            }) => assert_eq!((id, reading), (7, STEP_SNAP_PARALLEL)),
            other => panic!("parallel in the band: the file's ambiguity, got {other:?}"),
        }
    }

    /// One cylinder face of radius 1 about the `y` axis, `y` from 0 to 1,
    /// bounded by its two rims.
    fn wall() -> (SolidSpec, Surface<f64>) {
        let y = Vec3::new(0.0, 1.0, 0.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let rim = |at: f64| crate::entities::EdgeSpec {
            start: 1,
            end: 1,
            carrier: geom::Curve3::circle(Point3::new(0.0, at, 0.0), y, 1.0, x),
            t0: 0.0,
            t1: std::f64::consts::TAU,
            reversed: false,
            curve: None,
        };
        let surface = Surface::Cylinder {
            frame: geom::Frame::new(Point3::origin(), y, x),
            radius: 1.0,
        };
        let solid = SolidSpec {
            id: 1,
            faces: vec![crate::entities::FaceSpec {
                id: 2,
                record: 20,
                sense: true,
                loops: [10, 11]
                    .into_iter()
                    .map(|edge| crate::entities::LoopSpec {
                        outer: edge == 10,
                        uses: vec![crate::entities::EdgeUse {
                            edge,
                            forward: true,
                        }],
                    })
                    .collect(),
                band: false,
            }],
            edges: [(10, rim(0.0)), (11, rim(1.0))].into_iter().collect(),
            vertices: [(1, Point3::new(1.0, 0.0, 0.0))].into_iter().collect(),
            surfaces: crate::entities::records_of([(20, surface.clone())]),
            band_seams: std::collections::BTreeSet::new(),
        };
        (solid, surface)
    }

    /// **A move is measured over every face its record carries.** A plane
    /// record worn by two disks, one about the origin and one 10 away,
    /// turned 1e-6 onto the axis of a cylinder about `z` (the earlier
    /// record): the far disk's rim, 11 from the pivot, moves the most.
    #[test]
    fn a_move_is_measured_over_every_face_its_record_carries() {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let circle = |at: Point3<f64>| crate::entities::EdgeSpec {
            start: 1,
            end: 1,
            carrier: geom::Curve3::circle(at, z, 1.0, x),
            t0: 0.0,
            t1: std::f64::consts::TAU,
            reversed: false,
            curve: None,
        };
        let theta = 1e-6_f64;
        let plane = Surface::Plane {
            frame: geom::Frame::new(
                Point3::origin(),
                Vec3::new(theta.sin(), 0.0, theta.cos()),
                Vec3::new(theta.cos(), 0.0, -theta.sin()),
            ),
        };
        let cylinder = Surface::Cylinder {
            frame: geom::Frame::new(Point3::origin(), z, x),
            radius: 1.0,
        };
        let face = |id: u64, record: u64, edges: &[u64]| crate::entities::FaceSpec {
            id,
            record,
            sense: true,
            loops: edges
                .iter()
                .map(|&edge| crate::entities::LoopSpec {
                    outer: true,
                    uses: vec![crate::entities::EdgeUse {
                        edge,
                        forward: true,
                    }],
                })
                .collect(),
            band: false,
        };
        let solid = SolidSpec {
            id: 1,
            faces: vec![
                face(2, 20, &[10]),
                face(3, 20, &[11]),
                face(4, 21, &[10, 12]),
            ],
            edges: [
                (10, circle(Point3::origin())),
                (11, circle(Point3::new(10.0, 0.0, 0.0))),
                (12, circle(Point3::new(0.0, 0.0, 1.0))),
            ]
            .into_iter()
            .collect(),
            vertices: [(1, Point3::new(1.0, 0.0, 0.0))].into_iter().collect(),
            surfaces: crate::entities::records_of([(21, cylinder.clone()), (20, plane.clone())]),
            band_seams: std::collections::BTreeSet::new(),
        };
        let nodes = nodes(&solid);
        assert_eq!(
            nodes
                .iter()
                .map(|n| (n.record, n.faces.clone()))
                .collect::<Vec<_>>(),
            [(20, vec![0, 1]), (21, vec![2])],
            "the plane record and the cylinder's"
        );
        let relations = vec![Relation {
            kind: SnapRelation::AxisNormal,
            nodes: (0, 1),
            edge: 10,
            reach: Reach::of(&solid, [0, 2]).expect("the edge's faces have a region"),
            exact: false,
        }];
        let moves = solve(&solid, &nodes, &relations, &[0], FileBand::test(1e-4)).unwrap();
        let [Move { node, moved_by, .. }] = &moves[..] else {
            panic!("one move")
        };
        assert_eq!(*node, 0, "the plane, the later record, turns");
        assert!(
            *moved_by >= theta * 11.0 * (1.0 - 1e-6) && *moved_by <= theta * 12.0,
            "the far disk's rim, 11 from the pivot: {moved_by:e}"
        );
    }

    /// **A coaxial move reports its turn over the faces' reach.** The
    /// wall turned 1e-5 onto an axis through the origin moves its far rim
    /// by the turn times its reach from the pivot (√1.25 from the faces'
    /// middle on the axis) and its pivot by the turn times 0.5: the move
    /// reported holds both.
    #[test]
    fn a_coaxial_turn_is_reported_over_the_faces_reach() {
        let (solid, moved) = wall();
        let reach = Reach::of(&solid, [0]).expect("the wall has a region");
        let theta = 1e-5_f64;
        let kept = Surface::Cylinder {
            frame: geom::Frame::new(
                Point3::origin(),
                Vec3::new(theta.sin(), theta.cos(), 0.0),
                Vec3::new(theta.cos(), -theta.sin(), 0.0),
            ),
            radius: 1.0,
        };
        let (_, moved_by) = coaxial(&kept, &moved, &reach);
        let least = theta * (0.5 + 1.25_f64.sqrt());
        assert!(
            moved_by >= least * (1.0 - 1e-6) && moved_by <= 1.5 * least,
            "the turn over the reach and the pivot's carry, {least:e}: {moved_by:e}"
        );
    }

    /// **`nist_ftc_09`'s cylinder `#5130` reads tangent to plane `#5116` in
    /// the file's band, and the file refuses at the snap.** The cylinder
    /// (r 0.17985467 in, axis along y at z 4.472383303296 in) lies between
    /// parallel planes `#5116` (z 4.294766606592 in) and `#5144` (z 4.65 in)
    /// and bites each by 56.8 µm, which the file's band (ε_in ≈ 33.8 µm,
    /// k 10) does not decide. The file's ambiguity is refused on the later
    /// record before any class is solved. The pair along `#5132` reads in
    /// band too; `#5118`, along `#5116`, is read first only by edge-id order.
    #[test]
    fn nist09s_cylinder_5130_reads_tangent_in_the_files_band_and_refuses_at_the_snap() {
        let tol = geom_core::Tol::witness();
        let text = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp"
        ))
        .unwrap();
        let parsed = crate::parse::parse_file(&text).unwrap();
        // The tangent candidate's gap from the printed values, in inches:
        // the axis's distance from the plane less the radius.
        let inch = 0.0254;
        let bite = ((4.472_383_303_296 - 4.294_766_606_592) - 0.179_854_67_f64).abs() * inch;
        match crate::entities::resolve(&parsed, None, tol) {
            Err(StepImportError::Interpretation {
                id,
                refusal:
                    crate::interpret::Refusal::Ambiguous {
                        reading, gap, k, ..
                    },
                file,
            }) => {
                assert_eq!(
                    (id, reading),
                    (5130, STEP_SNAP_RELATION),
                    "the later record"
                );
                assert!(
                    (gap - bite).abs() < 1e-15,
                    "the tangent gap |z_axis − z_plane| − r, {bite:e}: {gap:e}"
                );
                assert!(
                    (file.eps_in() - 3.381_637_022_920_199_4e-5).abs() < 1e-18
                        && file.eps_in() < gap
                        && gap < k * file.eps_in(),
                    "the file's declared ε_in {:e}, k {k}: {gap:e} in its band",
                    file.eps_in()
                );
            }
            other => panic!("the in-band pair's ambiguity at #5130, got {other:?}"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod snapped_frame_rows {
    use super::*;

    /// **A carried surface keeps the frame door's bits, and a seam along
    /// the new axis refuses the carry**: a cylinder turned onto a new
    /// axis reads, bit for bit, the frame [`geom::Frame::new`] squares
    /// from the same seam; turned onto its own seam it has no seam left
    /// square to the axis, so the carry is `None` and the surface is left
    /// as it was, where the undecided square would have poisoned its
    /// frame. Red with the undecided square in [`snapped_frame`] (the
    /// degenerate carry then mints a poisoned frame and answers `Some`).
    #[test]
    fn a_carry_keeps_the_doors_bits_and_refuses_a_seam_along_its_new_axis() {
        let cylinder = Surface::Cylinder {
            frame: geom::Frame::new(Point3::new(1.0, 2.0, 3.0), Vec3::unit_z(), Vec3::unit_x()),
            radius: 0.5,
        };
        let from = Line {
            origin: Point3::new(1.0, 2.0, 0.0),
            dir: UnitVec3::exact_z(),
        };
        let pivot = Point3::new(1.0, 2.0, 0.0);
        let dir = Vec3::new(0.1, -0.2, 1.0).normalize();
        let mut carried = cylinder.clone();
        carry(&mut carried, from, (pivot, pivot), UnitVec3::checked(dir)).unwrap();
        let Surface::Cylinder { frame, .. } = carried else {
            panic!("a cylinder stays one")
        };
        let at = pivot + dir * 3.0;
        let squared = geom::Frame::new(at, dir, Vec3::unit_x());
        let bits = |f: &geom::Frame<f64>| {
            [f.origin() - Point3::origin(), f.axis(), f.u_ref()]
                .map(|e| [e.x, e.y, e.z].map(f64::to_bits))
        };
        assert_eq!(
            bits(&frame),
            bits(&squared),
            "the carry is the door's square"
        );
        let mut along = cylinder.clone();
        assert!(
            carry(&mut along, from, (pivot, pivot), UnitVec3::exact_x()).is_none(),
            "a seam along the new axis refuses"
        );
        let Surface::Cylinder { frame: kept, .. } = along else {
            panic!("a cylinder stays one")
        };
        let Surface::Cylinder { frame: was, .. } = cylinder else {
            panic!("a cylinder")
        };
        assert_eq!(
            bits(&kept),
            bits(&was),
            "a refused carry leaves the surface as it was"
        );
        assert!(
            geom::Frame::<f64>::new(Point3::origin(), Vec3::unit_x(), Vec3::unit_x())
                .u_ref()
                .x
                .is_nan(),
            "the undecided square poisons that seam"
        );
    }
}

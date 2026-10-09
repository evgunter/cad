//! **Where a shell stands against another closed surface** — the
//! crate's one witness ladder.
//!
//! Three questions read it: tier 3's check 10 (the winding the other
//! shells of a solid put on each of its shells), the result sort
//! ([`crate::pieces`], which piece's material surrounds a shell), and
//! the boolean's uncut-shell verdict (which side of the other operand an
//! uncut cell complex lies on, `boolean::shell_witness`). A fourth asks
//! it and does not read it yet: the census's cross-solid material probe
//! (`census::sweep_cross_solid_backstop`), which reads vertices only and
//! refuses where all of them touch
//! (`work/inside/the-census-material-probe-reads-only-vertices-so-a-flush-nested-solid-is-undecided.md`).
//! Each asks it
//! under the premise that the complex crosses no surface it is probed
//! against, so it meets one only where it lies ON it, every point of it
//! off those surfaces is on one side of each, and one decisive witness
//! names every side; a second could only agree.
//!
//! # The ladder
//!
//! The witnesses are the complex's own points, one per cell, in
//! increasing dimension:
//!
//! 1. each vertex;
//! 2. each edge's carrier at its parameter midpoint
//!    ([`geom_brep::EdgeCurve::mid_point`]), a point ON the edge
//!    whatever its kind;
//! 3. one point of each planar face's relative interior: the first
//!    candidate — a consecutive vertex triple's centroid, the
//!    midpoint of two of the face's vertices, then the midpoint of a
//!    line run inward from an edge's midpoint to a carrier it meets
//!    ([`across_edges`]) — that [`point_in_face`] certifies strictly
//!    inside the face.
//!
//! The probe is the caller's ([`ladder`]): it reads one witness against
//! whatever the question is about, and answers a side, [`Witness::On`]
//! (the witness lies on a probed surface), or a refusal about that one
//! point ([`PointInSolidError::inconclusive`]): [`Witness::Blocked`] (a
//! limit no tolerance moves — a face the door cannot read near the
//! witness, or a volume that could not side its rays) or
//! [`Witness::InBand`]. Each of the last three is inconclusive and the
//! next witness is read; any other refusal is about the probed surface
//! rather than the point, and propagates. The first decisive witness
//! decides. When none does, the [`Tally`] says how many read on a surface
//! and how many in band, and keeps the refusals ranked as one point's
//! rays are ([`crate::ray_walk::Evidence`]), limit first; the caller
//! decides what that means.
//!
//! A block inside another, flush on four walls, reaches the third rung:
//! its vertices and edges all lie on the other boundary, and the interior
//! of each end face does not. Curved faces offer no interior witness, so
//! a complex every point of which on a planar face, an edge or a vertex
//! lies on another surface reads nothing.

use geom_core::{Band, Decide, Point3, Tol, Vec3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::PointInSolidError;
use crate::boolean::SolidContainment;
use crate::boolean::solid_contain::{SolidFaces, face_plane, point_in_face, point_in_solid_faces};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, ShellKey, VertexKey};
use crate::props::{QuadLane, ShellClassifyError, ShellRole};
use crate::ray_walk::Ranked;

/// What one witness read against the probed surfaces.
#[derive(Debug)]
pub(crate) enum Witness<S> {
    /// A decisive reading.
    Side(S),
    /// The witness lies on a probed surface.
    On,
    /// A limit met near the witness ([`PointInSolidError::confined`],
    /// [`PointInSolidError::sideless`]).
    Blocked(PointInSolidError),
    /// The reading was in band ([`PointInSolidError::in_band`]).
    InBand(PointInSolidError),
}

/// A decisive point-in-solid reading: the point is off the surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Strict {
    /// Inside the material.
    In,
    /// Outside it.
    Out,
}

impl Witness<Strict> {
    /// One point-in-solid reading as a witness: `In` and `Out` decide,
    /// `OnBoundary` and a refusal about the point are inconclusive, and
    /// any other refusal propagates.
    pub(crate) fn of(
        read: Result<SolidContainment, PointInSolidError>,
    ) -> Result<Self, PointInSolidError> {
        match read {
            Ok(SolidContainment::In) => Ok(Self::Side(Strict::In)),
            Ok(SolidContainment::Out) => Ok(Self::Side(Strict::Out)),
            Ok(SolidContainment::OnBoundary) => Ok(Self::On),
            Err(e) if e.confined() || e.sideless() => Ok(Self::Blocked(e)),
            Err(e) if e.inconclusive() => Ok(Self::InBand(e)),
            Err(e) => Err(e),
        }
    }
}

/// What the ladder read off a complex.
#[derive(Debug)]
pub(crate) enum Reading<S> {
    /// The first decisive witness's side.
    Side(S),
    /// No witness decided.
    Undecided(Tally),
}

/// The witnesses of a complex none of which decided.
#[derive(Debug, Default)]
pub(crate) struct Tally {
    /// Witnesses that read [`Witness::On`].
    pub(crate) on_boundary: usize,
    /// Witnesses that read [`Witness::InBand`].
    pub(crate) in_band: usize,
    /// What the refused witnesses kept, ranked as one point's rays are
    /// ([`crate::ray_walk::Evidence`]): a limit before an in-band
    /// reading.
    pub(crate) kept: crate::ray_walk::Evidence<PointInSolidError>,
}

/// The ladder's own refusals, which each caller types as its own
/// (`E: From<LadderRefusal>`).
#[derive(Debug)]
pub(crate) enum LadderRefusal {
    /// The complex does not walk: an entity it names is lost.
    Desync {
        /// The face whose walk failed.
        face: FaceKey,
        /// What did not resolve.
        what: &'static str,
    },
    /// A face's plane or in-face walk refused other than in band.
    Containment(PointInSolidError),
}

impl From<LadderRefusal> for PointInSolidError {
    fn from(refusal: LadderRefusal) -> Self {
        match refusal {
            LadderRefusal::Desync { face, .. } => Self::CorruptFace { face },
            LadderRefusal::Containment(e) => e,
        }
    }
}

/// **The witness ladder** over the cell complex `faces` of `body`
/// (module docs): each witness is handed to `probe`, and the first
/// decisive one is the reading.
///
/// # Errors
///
/// `probe`'s refusal, verbatim; the ladder's own ([`LadderRefusal`])
/// where the complex does not walk or a face's interior candidate
/// refuses other than in band.
pub(crate) fn ladder<T: Decide, S, E: From<LadderRefusal>>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    mut probe: impl FnMut(Point3<T>) -> Result<Witness<S>, E>,
) -> Result<Reading<S>, E> {
    let mut tally = Tally::default();
    let mut side = |q: Point3<T>| -> Result<Option<S>, E> {
        Ok(match probe(q)? {
            Witness::Side(s) => Some(s),
            Witness::On => {
                tally.on_boundary += 1;
                None
            }
            Witness::Blocked(e) => {
                tally.kept.blocked(e);
                None
            }
            Witness::InBand(e) => {
                tally.in_band += 1;
                tally.kept.in_band(e);
                None
            }
        })
    };
    let mut halves: Vec<(FaceKey, HalfEdgeKey)> = Vec::new();
    for &face in faces {
        halves.extend(
            face_loops(body, face)?
                .into_iter()
                .flatten()
                .map(|he| (face, he)),
        );
    }
    let half = |(face, he): (FaceKey, HalfEdgeKey)| {
        body.get_half_edge(he).ok_or(LadderRefusal::Desync {
            face,
            what: "witnessed half-edge no longer resolves",
        })
    };

    let mut seen_vertex: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    for &fh in &halves {
        let v = half(fh)?.start;
        if seen_vertex.insert(v, ()).is_some() {
            continue;
        }
        let p = body
            .get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(LadderRefusal::Desync {
                face: fh.0,
                what: "witnessed vertex has no point",
            })?;
        if let Some(s) = side(p)? {
            return Ok(Reading::Side(s));
        }
    }

    let mut seen_edge: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
    for &fh in &halves {
        let e = half(fh)?.edge;
        if seen_edge.insert(e, ()).is_some() {
            continue;
        }
        let Some(curve) = witnessed_curve(body, fh.0, e)? else {
            continue;
        };
        if let Some(s) = side(curve.mid_point())? {
            return Ok(Reading::Side(s));
        }
    }

    for &face in faces {
        let normal = match face_plane(body, face) {
            Ok((_, normal)) => normal,
            Err(PointInSolidError::KindUnsupported { .. }) => continue,
            Err(e) => return Err(LadderRefusal::Containment(e).into()),
        };
        if let Some(q) = face_interior_point(body, face, normal, band)?
            && let Some(s) = side(q)?
        {
            return Ok(Reading::Side(s));
        }
    }

    Ok(Reading::Undecided(tally))
}

/// The first candidate strictly inside planar `face` (module docs,
/// rung 3): each consecutive vertex triple's centroid, then the
/// midpoint of each pair of the face's vertices, over its outer loop
/// and every ring, then the points across its edges ([`across_edges`]).
/// `None` when no candidate certifies.
fn face_interior_point<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    band: Band,
) -> Result<Option<Point3<T>>, LadderRefusal> {
    let loops = face_loop_points(body, face)?;
    let triples = loops.iter().filter(|p| p.len() >= 3).flat_map(|p| {
        let n = p.len();
        (0..n).map(move |i| triple_centroid(p[i], p[(i + 1) % n], p[(i + 2) % n]))
    });
    let vertices: Vec<Point3<T>> = loops.concat();
    let chords = vertices
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| vertices[i + 1..].iter().map(move |&b| chord_midpoint(a, b)));
    for q in triples.chain(chords) {
        if certified_in_face(body, face, normal, q, band)? {
            return Ok(Some(q));
        }
    }
    for q in across_edges(body, face, normal, band)? {
        if certified_in_face(body, face, normal, q, band)? {
            return Ok(Some(q));
        }
    }
    Ok(None)
}

/// The certified curve of edge `e`, met on `face`'s walk; `None` for
/// null scaffolding, which has no carrier to read.
fn witnessed_curve<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    e: EdgeKey,
) -> Result<Option<&geom_brep::EdgeCurve<T>>, LadderRefusal> {
    Ok(body
        .get_edge(e)
        .and_then(|ed| body.get_curve_geom(ed.curve))
        .ok_or(LadderRefusal::Desync {
            face,
            what: "witnessed edge has no curve",
        })?
        .certified())
}

/// Face-interior candidates that need no vertex (module docs, rung 3).
/// From each edge's midpoint `m` ([`geom_brep::EdgeCurve::mid_point`])
/// runs the line along the inward in-plane normal `w = normal × t`,
/// `t` the edge's direction as its half-edge in `face` walks it: a
/// face's interior lies to the left of each of its half-edges about its
/// outward normal. For each meeting `m + s·w` of that line with a line
/// or conic carrier of the face's loops, `s` decided positive, the
/// candidate is `m + (s/2)·w`. The nearest meeting is no farther than
/// the line's first exit from the face, so its candidate is inside the
/// face however thin the face is; the others are hints the certifier
/// may refuse. Spiric and spline carriers yield no meetings, so a face
/// whose nearest boundary along every such line is one of them offers
/// none; so does an edge with no certified carrier, or with no
/// direction at its midpoint.
fn across_edges<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    band: Band,
) -> Result<Vec<Point3<T>>, LadderRefusal> {
    let desync = |what| LadderRefusal::Desync { face, what };
    let mut carriers: Vec<&geom::Curve3<T>> = Vec::new();
    let mut feet: Vec<(Point3<T>, Vec3<T>)> = Vec::new();
    for he in face_loops(body, face)?.into_iter().flatten() {
        let e = body
            .get_half_edge(he)
            .ok_or(desync("witnessed half-edge no longer resolves"))?
            .edge;
        let Some(curve) = witnessed_curve(body, face, e)? else {
            continue;
        };
        let plus = body
            .get_edge(e)
            .ok_or(desync("witnessed edge no longer resolves"))?
            .he_plus
            == he;
        carriers.push(curve.carrier());
        let (t0, t1) = curve.params();
        let t = curve.carrier().deriv(geom::mid_param(t0, t1));
        let w = normal.cross(if plus { t } else { -t }).normalize();
        if [w.x, w.y, w.z].iter().all(|c| !c.is_poison()) {
            feet.push((curve.mid_point(), w));
        }
    }
    let half = T::from_f64(0.5);
    let mut out = Vec::new();
    for &(m, w) in &feet {
        for c in &carriers {
            for s in line_meetings(c, normal, m, w) {
                if crate::validate::definitely_positive("stands_across_meeting", s, band) {
                    out.push(m + w * (s * half));
                }
            }
        }
    }
    Ok(out)
}

/// The parameters `s` at which the line `m + s·w` (unit `w`, in the
/// plane normal to `normal`) meets carrier `c`, where `c` is a line or
/// a conic in that plane: the line's one meeting, both roots of a
/// conic's quadratic (one double root where the line misses it, read at
/// its nearest approach). Empty for a spiric or a spline.
fn line_meetings<T: Decide>(
    c: &geom::Curve3<T>,
    normal: Vec3<T>,
    m: Point3<T>,
    w: Vec3<T>,
) -> Vec<T> {
    let conic = |center: Point3<T>, axis: Vec3<T>, u: Vec3<T>, a: T, b: T| {
        let v = axis.cross(u);
        let r = m - center;
        let (x0, y0) = (r.dot(u) / a, r.dot(v) / b);
        let (xw, yw) = (w.dot(u) / a, w.dot(v) / b);
        let qa = xw * xw + yw * yw;
        let qb = (x0 * xw + y0 * yw) * T::from_f64(2.0);
        let qc = x0 * x0 + y0 * y0 - T::one();
        let root = (qb * qb - qa * qc * T::from_f64(4.0)).max(T::zero()).sqrt();
        let twice = qa * T::from_f64(2.0);
        vec![(-qb - root) / twice, (-qb + root) / twice]
    };
    match *c {
        geom::Curve3::Line { origin, dir } => {
            vec![(origin - m).cross(dir).dot(normal) / w.cross(dir).dot(normal)]
        }
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => conic(center, axis, u_ref, radius, radius),
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => conic(center, axis, u_ref, major, minor),
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => Vec::new(),
    }
}

/// Each walkable loop of `face` — its outer loop, then every ring — as
/// its half-edge cycle. A loop with no cycle (a lone vertex) has no
/// edge and bounds no area, so it is left out.
fn face_loops<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<HalfEdgeKey>>, LadderRefusal> {
    let desync = |what| LadderRefusal::Desync { face, what };
    let f = body
        .get_face(face)
        .ok_or(desync("face no longer resolves"))?;
    let mut out = Vec::new();
    for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or(desync("face loop no longer resolves"))?
            .boundary
        else {
            continue;
        };
        out.push(
            body.loop_cycle(first)
                .ok_or(desync("face loop not walkable"))?,
        );
    }
    Ok(out)
}

/// [`face_loops`] as each half-edge's start point.
fn face_loop_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<Point3<T>>>, LadderRefusal> {
    face_loops(body, face)?
        .into_iter()
        .map(|cycle| {
            cycle
                .into_iter()
                .map(|he| {
                    body.half_edge_start_point(he).ok_or(LadderRefusal::Desync {
                        face,
                        what: "face vertex has no point",
                    })
                })
                .collect()
        })
        .collect()
}

/// The centroid of three consecutive loop vertices: a face-interior
/// candidate, inside the face only where the corner at `b` is convex.
fn triple_centroid<T: Decide>(a: Point3<T>, b: Point3<T>, c: Point3<T>) -> Point3<T> {
    a + ((b - a) + (c - a)) * T::from_f64(1.0 / 3.0)
}

/// The midpoint of two of a face's vertices: a face-interior candidate
/// wherever the chord between them is a diagonal of the face.
fn chord_midpoint<T: Decide>(a: Point3<T>, b: Point3<T>) -> Point3<T> {
    a.lerp(b, T::from_f64(0.5))
}

/// Does [`point_in_face`] certify `p` strictly inside planar `face`?
/// `false` discards the candidate unprobed: outside, on a loop, or an
/// [`PointInSolidError::inconclusive`] reading — an edge of `face` whose
/// carrier the walk cannot cross among them, and that face then offers
/// no candidate, as a curved face offers none. Any other refusal is an
/// error.
fn certified_in_face<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    p: Point3<T>,
    band: Band,
) -> Result<bool, LadderRefusal> {
    match point_in_face(body, face, normal, p, band) {
        Ok(verdict) => Ok(verdict == Some(true)),
        Err(e) if e.inconclusive() => Ok(false),
        Err(e) => Err(LadderRefusal::Containment(e)),
    }
}

/// One shell of a solid, read for nesting: its key, its role
/// ([`crate::props::shell_role`]) and its faces as a point-in-solid
/// selection, read once and probed many times.
pub(crate) struct ShellRead {
    /// The shell.
    pub(crate) shell: ShellKey,
    /// Its decided role.
    pub(crate) role: ShellRole,
    /// Its faces, as the walk's selection.
    pub(crate) sel: SolidFaces,
}

impl ShellRead {
    /// `shell` read: `None` where its selection cannot be read, and the
    /// shell's typed refusal where its role cannot
    /// ([`crate::props::shell_role`]).
    pub(crate) fn of<T: Decide>(
        body: &Body<T>,
        shell: ShellKey,
        band: Band,
        tol: Tol,
        quad: Option<QuadLane<T>>,
    ) -> Option<Result<Self, ShellClassifyError>> {
        let sel = SolidFaces::of_shell(body, shell).ok()?;
        Some(
            crate::props::shell_role(body, shell, band, tol, quad).map(|(role, _)| Self {
                shell,
                role,
                sel: sel.with_role(role),
            }),
        )
    }
}

/// Where one shell stands among the others of its reading.
pub(crate) enum Insides {
    /// Per shell of the reading, in its order: whether the shell lies
    /// inside that shell's closed surface (`false` for the shell
    /// itself).
    Read(Vec<bool>),
    /// Every witness of the shell lies on another shell, and none was
    /// refused.
    Touching,
    /// A walk refused, or no witness decided and one was refused (the
    /// [`Tally`]'s ranking: the first limit met, else the first in-band
    /// reading).
    Refused(PointInSolidError),
}

/// **Where shell `reads[i]` stands among the others** — the ladder over
/// its faces, each witness probed against every other shell alone with
/// the crate's one point-in-solid walk over that shell's faces. The walk
/// answers whether the point is in the material the selection alone
/// bounds: for an `Outer` shell that is its inside, so `In` is inside;
/// for a `Void` the faces point INTO the cavity, so the material is the
/// cavity's complement and `Out` is inside. A witness is decisive only
/// where it is off every shell probed: on one, it says nothing about
/// nesting, and the next is read.
///
/// `may_enclose` is the caller's screen: a shell it rules out is read
/// as not enclosing this one and is not probed. It must be sound — a
/// shell that could enclose must pass — so only a certificate of
/// disjointness rules one out (the result sort's padded boxes).
pub(crate) fn witness_insides<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    i: usize,
    reads: &[ShellRead],
    may_enclose: &dyn Fn(usize) -> bool,
    band: Band,
    tol: Tol,
) -> Insides {
    let probe = |q: Point3<T>| -> Result<Witness<Vec<bool>>, PointInSolidError> {
        let mut inside = vec![false; reads.len()];
        for (t, other) in reads.iter().enumerate() {
            if t == i || !may_enclose(t) {
                continue;
            }
            inside[t] = match Witness::of(point_in_solid_faces(body, &other.sel, q, band, tol))? {
                Witness::Side(Strict::In) => other.role == ShellRole::Outer,
                Witness::Side(Strict::Out) => other.role == ShellRole::Void,
                Witness::On => return Ok(Witness::On),
                Witness::Blocked(e) => return Ok(Witness::Blocked(e)),
                Witness::InBand(e) => return Ok(Witness::InBand(e)),
            };
        }
        Ok(Witness::Side(inside))
    };
    match ladder(body, reads[i].sel.faces(), band, probe) {
        Ok(Reading::Side(inside)) => Insides::Read(inside),
        Ok(Reading::Undecided(t)) => match t.kept.ranked() {
            Ranked::Blocked(e) | Ranked::InBand(e) => Insides::Refused(e),
            Ranked::Neither => Insides::Touching,
        },
        Err(e) => Insides::Refused(e),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod rung_three_rows {
    use super::*;
    use crate::test_support_fixtures::{plant_disc_face, prism_z};

    /// The box `[0, 2]² × [0, 1]` with a disc of radius 0.2 about
    /// `(1, 1, 1)` planted in its top face: the top face (four corners
    /// and a one-vertex ring), the disc (one vertex, one closed edge),
    /// and the disc's centre.
    fn planted() -> (Body<f64>, FaceKey, FaceKey, Point3<f64>) {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
        let prism = prism_z::<f64>(&square, 0.0, 1.0, tol);
        let mut body = prism.body;
        let outer = body.get_face(prism.top_face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the top face's outer loop is a cycle");
        };
        let center = Point3::new(1.0, 1.0, 1.0);
        let planted = plant_disc_face(&mut body, first, center, 0.2, tol);
        let disc = planted.face;
        // The planted circle is a whole-turn scaffold, which the in-face
        // walk refuses to read (a null self-loop is certified so); at
        // rest it is described in the top face's chart.
        let chart = body.get_face(prism.top_face).unwrap().surface;
        let circle = geom::Curve3::Circle {
            center,
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(circle, 0.0, core::f64::consts::TAU)
            .unwrap()
            .at_rest_in_chart(chart, false);
        body.set_edge_curve(planted.edge, spec, tol).unwrap();
        // The planted disc's loop winds clockwise about the host's
        // normal; its sense turned, it winds counter-clockwise about its
        // own outward normal, as every face's outer loop does at rest.
        let fd = body.get_face(disc).unwrap().clone();
        let surface = body.get_surface(fd.surface).unwrap().clone();
        body.set_face_surface_unvouched_for_tests(
            disc,
            crate::euler::FaceSurface::New {
                surface,
                sense: !fd.sense,
            },
        )
        .unwrap();
        (body, prism.top_face, disc, center)
    }

    fn interior(body: &Body<f64>, face: FaceKey) -> Option<Point3<f64>> {
        let band = Band::linear(Tol::witness()).unwrap();
        let (_, normal) = face_plane(body, face).unwrap();
        face_interior_point(body, face, normal, band).unwrap()
    }

    /// **A disc of one vertex is witnessed at its centre**: no vertex
    /// candidate exists, and the inward line from its edge's midpoint
    /// meets its own circle again across the diameter. Run outward, the
    /// line meets nothing ahead, so a reversed sense offers no witness.
    #[test]
    fn a_one_vertex_disc_is_witnessed_at_its_centre() {
        let (body, _, disc, center) = planted();
        let q = interior(&body, disc).expect("the disc offers a witness");
        assert!(
            (q - center).norm() < 1e-12,
            "the disc's witness {q:?}, not its centre"
        );
    }

    /// **A face whose vertex candidate certifies keeps it**: the top
    /// face's witness is its first corner triple's centroid, ahead of
    /// every candidate across its edges.
    #[test]
    fn a_vertex_candidate_comes_before_the_edges() {
        let (body, top, _, _) = planted();
        let corners = &face_loop_points(&body, top).unwrap()[0];
        let first = triple_centroid(corners[0], corners[1], corners[2]);
        let q = interior(&body, top).expect("the top face offers a witness");
        assert!(
            (q - first).norm() == 0.0,
            "the top face's witness {q:?}, not {first:?}"
        );
    }
}

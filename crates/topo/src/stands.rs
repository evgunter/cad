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
//! The reader of the side at infinity (`solid_contain`
//! `side_at_infinity`) reads its third rung alone, [`planar_interior_point`],
//! as points to aim at rather than witnesses to probe.
//! The result sort, the uncut-shell verdict and the census ask it under
//! the premise that the complex crosses no surface it is probed
//! against, so it meets one only where it lies ON it, every point of it
//! off those surfaces is on one side of each, and one decisive witness
//! names every side; a second could only agree ([`ladder`]). Check 10's
//! premise is the thing it checks, so it reads every witness
//! ([`ladder_agreeing`]), and two that disagree are a crossing.
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
//! rays are ([`geom_core::ladder::Evidence`]), limit first; the caller
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
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, ShellKey, VertexKey};
use crate::face_normal::face_plane;
use crate::live::{Proven, proven};
use crate::props::{QuadLane, ShellRole};
use crate::solid_contain::PointInSolidError;
use crate::solid_contain::SolidContainment;
use crate::solid_contain::{SolidFaces, point_in_face, point_in_solid_faces};
use geom_core::ladder::{Answering, Climb, Disagree, Ranked, Rung, climb_evidence};

/// What one witness read against the probed surfaces, `K` the refusal a
/// probe keeps (the point-in-solid walk's).
#[derive(Debug)]
pub(crate) enum Witness<S, K = PointInSolidError> {
    /// A decisive reading.
    Side(S),
    /// The witness lies on a probed surface.
    On,
    /// A limit met near the witness (the walk's `confined` and
    /// `sideless` refusals).
    Blocked(K),
    /// The reading was in band (the walk's `in_band` refusal).
    InBand(K),
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
    Undecided(Box<Tally>),
}

/// The witnesses of a complex none of which decided.
#[derive(Debug, Default)]
pub(crate) struct Tally {
    /// Witnesses that read [`Witness::On`].
    pub(crate) on_boundary: usize,
    /// Witnesses that read [`Witness::InBand`].
    pub(crate) in_band: usize,
    /// What the refused witnesses kept, ranked as one point's rays are
    /// ([`geom_core::ladder::Evidence`]): a limit before an in-band
    /// reading.
    pub(crate) kept: geom_core::ladder::Evidence<PointInSolidError>,
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

/// One rung of the witness ladder: a cell whose witness is read.
enum Cell {
    Vertex(HalfEdgeKey),
    Edge(HalfEdgeKey),
    Face(FaceKey),
}

impl Cell {
    /// Its rung in the ladder's ascending dimension (module docs): a
    /// vertex before an edge before a face.
    fn rung(&self) -> usize {
        match self {
            Self::Vertex(_) => 0,
            Self::Edge(_) => 1,
            Self::Face(_) => 2,
        }
    }
}

/// The cells of the complex `faces` in ladder order (module docs), and
/// the reader of each cell's witness: `None` for a cell already read or
/// one that offers none.
#[allow(clippy::type_complexity)]
fn cells<'a, T: Decide>(
    body: &'a Body<T>,
    faces: &'a [FaceKey],
    band: Band,
) -> Result<
    (
        Vec<Cell>,
        impl FnMut(Cell) -> Result<Option<Point3<T>>, LadderRefusal> + 'a,
    ),
    LadderRefusal,
> {
    let mut halves: Vec<HalfEdgeKey> = Vec::new();
    for &face in faces {
        halves.extend(face_loops(body, face)?.into_iter().flatten());
    }
    // Every half-edge is one the face walk read, so each hop from it is
    // a link.
    let half = move |he: HalfEdgeKey| proven(&body.half_edges, he, EntityId::HalfEdge);
    let mut seen_vertex: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    let mut seen_edge: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
    let witness = move |cell: Cell| -> Result<Option<Point3<T>>, LadderRefusal> {
        Ok(match cell {
            Cell::Vertex(he) => {
                let v = half(he).start;
                if seen_vertex.insert(v, ()).is_some() {
                    return Ok(None);
                }
                Some(body.resolve_vertex_point(v, Proven))
            }
            Cell::Edge(he) => {
                let e = half(he).edge;
                if seen_edge.insert(e, ()).is_some() {
                    return Ok(None);
                }
                witnessed_curve(body, e).map(|curve| curve.mid_point())
            }
            Cell::Face(face) => planar_interior_point(body, face, band)?,
        })
    };
    let cells = (halves.iter().map(|&he| Cell::Vertex(he)))
        .chain(halves.iter().map(|&he| Cell::Edge(he)))
        .chain(faces.iter().map(|&f| Cell::Face(f)))
        .collect();
    Ok((cells, witness))
}

/// One witness as a rung: a decisive side answers, one on a probed
/// surface passes, an inconclusive reading is kept, and a refusal about
/// the complex or the probed surface aborts. `on_boundary` and
/// `in_band` count the witnesses read on a surface and in band.
fn witness_rung<T: Decide, S, K, E: From<LadderRefusal>>(
    witness: Result<Option<Point3<T>>, LadderRefusal>,
    probe: &mut impl FnMut(Point3<T>) -> Result<Witness<S, K>, E>,
    (on_boundary, in_band): (&mut usize, &mut usize),
) -> Rung<Result<S, E>, K> {
    let q = match witness {
        Ok(Some(q)) => q,
        Ok(None) => return Rung::Passes,
        Err(refusal) => return Rung::Aborts(Err(E::from(refusal))),
    };
    // The witness ladder's table. `insides_at` reads `Witness` the other
    // way up (`On` answers, a side passes and is recorded): one table
    // with the inversion as a parameter would have to carry both the
    // tallies here and the inside recording there.
    match probe(q) {
        Ok(Witness::Side(s)) => Rung::Answers(Ok(s)),
        Ok(Witness::On) => {
            *on_boundary += 1;
            Rung::Passes
        }
        Ok(Witness::Blocked(e)) => Rung::Blocked(e),
        Ok(Witness::InBand(e)) => {
            *in_band += 1;
            Rung::InBand(e)
        }
        Err(e) => Rung::Aborts(Err(e)),
    }
}

/// **The witness ladder** over the cell complex `faces` of `body`
/// (module docs): each witness is handed to `probe`, and the first
/// decisive one is the reading. It is the ladder
/// ([`climb_evidence`]) whose answer is a side, and which a refusal that
/// propagates aborts ([`Rung::Aborts`]). One decisive witness names
/// every side under the premise that the complex crosses no probed
/// surface (module docs), so a witness set aside ahead of it is
/// superseded in the record ([`Answering::Any`]); ahead of an abort it
/// stays live. A witness is found inside its rung, so the readings that
/// find it (a face's interior point) are recorded in rung order with the
/// probe's. A question for which that premise is the thing checked
/// reads [`ladder_agreeing`] instead.
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
    let (cells, mut witness) = cells(body, faces, band).map_err(E::from)?;
    let (mut on_boundary, mut in_band) = (0, 0);
    let climbed = climb_evidence(Answering::Any, cells, |cell| {
        witness_rung(witness(cell), &mut probe, (&mut on_boundary, &mut in_band))
    });
    match climbed {
        Ok((_, answer)) => answer.map(Reading::Side),
        Err(kept) => Ok(Reading::Undecided(Box::new(Tally {
            on_boundary,
            in_band,
            kept,
        }))),
    }
}

/// What every witness of a complex read ([`ladder_agreeing`]).
#[derive(Debug)]
pub(crate) enum Agreed<S> {
    /// Every decisive witness read this side.
    Side(S),
    /// Two decisive witnesses read different sides: the complex crosses
    /// a probed surface. The first side read, and the one that differed.
    Disagree {
        /// The first decisive side.
        first: S,
        /// A later side that differs from it.
        other: S,
    },
    /// No witness decided: each lay on a probed surface or was read in
    /// band or at a limit.
    Undecided,
}

/// **The witness ladder read at every witness** (module docs), for a
/// question whose no-crossing premise is the thing checked: the
/// complex's witnesses are those [`ladder`] reads, and every one is
/// probed ([`geom_core::ladder::climb_agreeing`]). Where every decisive
/// witness reads one side by `agree`, that side; where two differ, the
/// complex crosses a probed surface ([`Agreed::Disagree`]). Its verdict
/// does not depend on the order the witnesses are read in.
///
/// # Errors
///
/// As [`ladder`]: a refusal about the complex or the probed surface
/// stops the reading.
pub(crate) fn ladder_agreeing<T: Decide, S, E: From<LadderRefusal>>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    mut probe: impl FnMut(Point3<T>) -> Result<Witness<S>, E>,
    agree: impl Fn(&S, &S) -> bool,
) -> Result<Agreed<S>, E> {
    let (cells, mut witness) = cells(body, faces, band).map_err(E::from)?;
    // One rung at a time, in ascending dimension (module docs). A rung
    // whose witnesses place decisively answers, and the next is never
    // read: every one of a rung's witnesses is read (`climb_agreeing`
    // does not stop at the first), so a crossing a rung can see is a
    // disagreement within it, and a crossing it cannot — a face poking
    // across between vertices all on one side — is a higher cell's,
    // whose own check reads it (the census's, as a loop's edge crossing
    // between agreeing vertices is in [`crate::one_region`]). Reading the
    // next rung only where this one placed nothing is what keeps a valid
    // body from probing every edge midpoint and face interior once its
    // vertices agree.
    let mut by_rung: [Vec<Cell>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for cell in cells {
        by_rung[cell.rung()].push(cell);
    }
    for rung in by_rung {
        let climbed = geom_core::ladder::climb_agreeing(
            rung,
            |cell| witness_rung(witness(cell), &mut probe, (&mut 0, &mut 0)),
            |a, b| match (a, b) {
                (Ok(a), Ok(b)) => agree(a, b),
                _ => true,
            },
        );
        match climbed {
            Ok(Climb::Answered { answer, .. }) => return answer.map(Agreed::Side),
            Ok(Climb::Passed | Climb::Undecided(_)) => {}
            Err(Disagree { first, other }) => {
                return Ok(Agreed::Disagree {
                    first: first?,
                    other: other?,
                });
            }
        }
    }
    Ok(Agreed::Undecided)
}

/// **A point certified strictly inside `face`, where `face` is planar**
/// (module docs, rung 3): [`face_interior_point`] over the face's
/// outward plane. `None` for a curved face, which offers none, and for
/// a planar face no candidate of which certifies.
///
/// # Errors
///
/// [`LadderRefusal`] where the face does not walk or its in-face walk
/// refuses other than in band.
pub(crate) fn planar_interior_point<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<Point3<T>>, LadderRefusal> {
    let Some((_, normal)) = face_plane(body, face) else {
        return Ok(None);
    };
    face_interior_point(body, face, normal, band)
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
#[track_caller]
fn witnessed_curve<T: Decide>(body: &Body<T>, e: EdgeKey) -> Option<&geom_brep::EdgeCurve<T>> {
    body.edge_curve_linked(e, proven(&body.edges, e, EntityId::Edge))
        .certified()
}

/// The direction into a face from a half-edge of it running along `t`,
/// about the face's outward `normal`: a face's interior lies to the left
/// of each of its half-edges.
pub(crate) fn inward<T: Decide>(normal: Vec3<T>, t: Vec3<T>) -> Vec3<T> {
    normal.cross(t)
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
/// may refuse. A meeting not finite (a parallel line) or not decided
/// beyond the band proposes nothing. Spiric and spline carriers yield
/// no meetings, so a face whose nearest boundary along every such line
/// is one of them offers none; so does an edge with no certified
/// carrier, or with no direction at its midpoint.
fn across_edges<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    normal: Vec3<T>,
    band: Band,
) -> Result<Vec<Point3<T>>, LadderRefusal> {
    let mut carriers: Vec<&geom::Curve3<T>> = Vec::new();
    let mut inward_lines: Vec<geom::Line<T>> = Vec::new();
    for he in face_loops(body, face)?.into_iter().flatten() {
        let e = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
        let Some(curve) = witnessed_curve(body, e) else {
            continue;
        };
        let plus = proven(&body.edges, e, EntityId::Edge).he_plus == he;
        carriers.push(curve.carrier());
        let (t0, t1) = curve.params();
        let t = curve.carrier().deriv(geom::mid_param(t0, t1));
        let w = inward(normal, if plus { t } else { -t }).normalize();
        if [w.x, w.y, w.z].iter().all(|c| !c.is_poison()) {
            inward_lines.push(geom::Line {
                origin: curve.mid_point(),
                dir: w,
            });
        }
    }
    let half = T::from_f64(0.5);
    let mut out = Vec::new();
    for &line @ geom::Line { origin: m, dir: w } in &inward_lines {
        for c in &carriers {
            for s in line_meetings(c, normal, line) {
                if geom_core::is_finite_length(s)
                    && crate::validate::definitely_positive("stands_across_meeting", s, band)
                {
                    out.push(m + w * (s * half));
                }
            }
        }
    }
    Ok(out)
}

/// The parameters `s` at which the line `m + s·w` (unit `w`, in the
/// plane normal to `normal`) meets carrier `c`, where `c` is a line or
/// a conic in that plane: the line's one meeting (not finite where the
/// two lines are parallel), both roots of a conic's quadratic (one
/// double root where the line misses it, read at its nearest
/// approach). Empty for a spiric or a spline.
fn line_meetings<T: Decide>(
    c: &geom::Curve3<T>,
    normal: Vec3<T>,
    geom::Line { origin: m, dir: w }: geom::Line<T>,
) -> Vec<T> {
    let conic = |center: Point3<T>, axis: Vec3<T>, u: Vec3<T>, a: T, b: T| {
        let v = axis.cross(u);
        let r = m - center;
        let (x0, y0) = (r.dot(u) / a, r.dot(v) / b);
        let (xw, yw) = (w.dot(u) / a, w.dot(v) / b);
        let qa = xw.powi(2) + yw.powi(2);
        let qb = (x0 * xw + y0 * yw) * T::from_f64(2.0);
        let qc = x0.powi(2) + y0.powi(2) - T::one();
        let root = (qb.powi(2) - qa * qc * T::from_f64(4.0))
            .max(T::zero())
            .sqrt();
        let twice = qa * T::from_f64(2.0);
        vec![(-qb - root) / twice, (-qb + root) / twice]
    };
    match *c {
        geom::Curve3::Line(geom::Line { origin, dir }) => {
            vec![(origin - m).cross(dir).dot(normal) / w.cross(dir).dot(normal)]
        }
        geom::Curve3::Circle(geom::Circle {
            frame:
                geom::Frame {
                    origin: center,
                    axis,
                    u_ref,
                },
            radius,
        }) => conic(center, axis, u_ref, radius, radius),
        geom::Curve3::Ellipse(geom::Ellipse {
            frame:
                geom::Frame {
                    origin: center,
                    axis,
                    u_ref,
                },
            semi_u,
            semi_v,
        }) => conic(center, axis, u_ref, semi_u, semi_v),
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => Vec::new(),
    }
}

/// Each walkable loop of `face` — its outer loop, then every ring — as
/// its half-edge cycle ([`crate::live::FaceView::cycles`], every hop past
/// the face a link). A loop with no cycle (a lone vertex) has no edge
/// and bounds no area, so it is left out.
fn face_loops<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<HalfEdgeKey>>, LadderRefusal> {
    let f = body.face(face).ok_or(LadderRefusal::Desync {
        face,
        what: "face no longer resolves",
    })?;
    Ok(f.cycles().map(|(_, cycle)| cycle).collect())
}

/// [`face_loops`] as each half-edge's start point.
fn face_loop_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<Vec<Point3<T>>>, LadderRefusal> {
    Ok(face_loops(body, face)?
        .into_iter()
        .map(|cycle| {
            cycle
                .into_iter()
                .map(|he| {
                    let start = proven(&body.half_edges, he, EntityId::HalfEdge).start;
                    body.resolve_vertex_point(start, Proven)
                })
                .collect()
        })
        .collect())
}

/// The centroid of three consecutive loop vertices: a face-interior
/// candidate, inside the face only where the corner at `b` is convex.
fn triple_centroid<T: Decide>(a: Point3<T>, b: Point3<T>, c: Point3<T>) -> Point3<T> {
    a + ((b - a) + (c - a)) * T::from_f64(1.0 / 3.0)
}

/// The midpoint of two of a face's vertices: a face-interior candidate
/// wherever the chord between them is a diagonal of the face.
fn chord_midpoint<T: Decide>(a: Point3<T>, b: Point3<T>) -> Point3<T> {
    a.mid(b)
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
    /// `shell` read, or the shell's typed refusal where its role cannot
    /// be ([`crate::props::shell_role`]).
    pub(crate) fn of<T: Decide>(
        body: &Body<T>,
        shell: ShellKey,
        band: Band,
        tol: Tol,
        quad: Option<QuadLane<T>>,
    ) -> Result<Self, crate::props::ShellClassifyError> {
        crate::props::shell_role(body, shell, band, tol, quad).map(|role| Self {
            shell,
            role,
            sel: SolidFaces::of_shell(body, shell).with_role(role),
        })
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
    let probe = |q| insides_at(body, i, reads, may_enclose, q, band, tol);
    match ladder(body, reads[i].sel.faces(), band, probe) {
        Ok(Reading::Side(inside)) => Insides::Read(inside),
        Ok(Reading::Undecided(t)) => t.kept.refusal().map_or(Insides::Touching, Insides::Refused),
        Err(e) => Insides::Refused(e),
    }
}

/// Where shell `reads[i]` stands among the others, read at every
/// witness ([`ladder_agreeing`]): check 10's reading, whose premise —
/// no shell crosses another — is the thing checked.
pub(crate) enum EveryInside {
    /// Every decisive witness reads these insides ([`Insides::Read`]).
    Read(Vec<bool>),
    /// Two witnesses read different insides: the shell crosses another.
    /// The first read, and the one that differed.
    Crosses(Vec<bool>, Vec<bool>),
    /// No witness decided: every one lies on another shell
    /// ([`Insides::Touching`]), or a walk refused ([`Insides::Refused`]).
    Unread,
}

/// **Where shell `reads[i]` stands among the others, at every witness**:
/// [`witness_insides`] with no screen, every witness probed against
/// every other shell, and two witnesses that disagree read as
/// [`EveryInside::Crosses`].
pub(crate) fn witness_insides_everywhere<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    i: usize,
    reads: &[ShellRead],
    band: Band,
    tol: Tol,
) -> EveryInside {
    let probe = |q| insides_at(body, i, reads, &|_| true, q, band, tol);
    match ladder_agreeing(body, reads[i].sel.faces(), band, probe, |a, b| a == b) {
        Ok(Agreed::Side(inside)) => EveryInside::Read(inside),
        Ok(Agreed::Disagree { first, other }) => EveryInside::Crosses(first, other),
        Ok(Agreed::Undecided) | Err(_) => EveryInside::Unread,
    }
}

/// One witness `q` of shell `reads[i]` probed against every other shell
/// `may_enclose` admits (module docs): per shell, whether `q` lies
/// inside its closed surface.
fn insides_at<T: Decide + crate::props::AtRestPolicy>(
    body: &Body<T>,
    i: usize,
    reads: &[ShellRead],
    may_enclose: &dyn Fn(usize) -> bool,
    q: Point3<T>,
    band: Band,
    tol: Tol,
) -> Result<Witness<Vec<bool>>, PointInSolidError> {
    let mut inside = vec![false; reads.len()];
    // On any shell probed, the witness says nothing about nesting,
    // whatever another shell read ([`climb_evidence`],
    // [`Answering::Any`]: a theorem). A shell read in band or at a limit
    // is kept, and the witness is that reading only where it lies on no
    // shell; a refusal about more than the point aborts.
    let probed = reads
        .iter()
        .enumerate()
        .filter(|&(t, _)| t != i && may_enclose(t));
    // `Witness` read the other way up from the witness ladder's table
    // (`witness_rung`): here `On` answers and a side passes.
    let on = climb_evidence(Answering::Any, probed, |(t, other)| {
        match Witness::of(point_in_solid_faces(body, &other.sel, q, band, tol)) {
            Ok(Witness::Side(side)) => {
                inside[t] = match side {
                    Strict::In => other.role == ShellRole::Outer,
                    Strict::Out => other.role == ShellRole::Void,
                };
                Rung::Passes
            }
            Ok(Witness::On) => Rung::Answers(Ok(())),
            Ok(Witness::Blocked(e)) => Rung::Blocked(e),
            Ok(Witness::InBand(e)) => Rung::InBand(e),
            Err(e) => Rung::Aborts(Err(e)),
        }
    });
    match on {
        Ok((_, on)) => on.map(|()| Witness::On),
        Err(kept) => Ok(match kept.ranked() {
            Ranked::Neither => Witness::Side(inside),
            Ranked::Blocked(e) => Witness::Blocked(e),
            Ranked::InBand(e) => Witness::InBand(e),
            Ranked::Unsettled(_) => unreachable!("no shell's rung is unsettled"),
        }),
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
        let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
        else {
            panic!("the top face's outer loop is a cycle");
        };
        let center = Point3::new(1.0, 1.0, 1.0);
        let planted = plant_disc_face(&mut body, first, center, 0.2, tol);
        let disc = planted.face;
        // The planted circle is a whole-turn scaffold, which the in-face
        // walk refuses to read (a null self-loop is certified so); at
        // rest it is described in the top face's chart.
        let chart = body.get_face(prism.top_face).unwrap().surface;
        let circle = geom::Curve3::circle(
            center,
            Vec3::new(0.0, 0.0, 1.0),
            0.2,
            Vec3::new(1.0, 0.0, 0.0),
        );
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
            q.equals_exactly(first),
            "the top face's witness {q:?}, not {first:?}"
        );
    }
}

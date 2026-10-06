//! **Where a shell stands against another closed surface** — the one
//! witness ladder every such question reads.
//!
//! Three questions ask it: tier 3's check 10 (the winding the other
//! shells of a solid put on each of its shells), the result sort
//! ([`crate::pieces`], which piece's material surrounds a shell), and
//! the boolean's uncut-shell verdict (which side of the other operand an
//! uncut cell complex lies on, `boolean::shell_witness`). Each asks it
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
//!    candidate — a consecutive vertex triple's centroid, then the
//!    midpoint of two of the face's vertices — that
//!    [`point_in_face`] certifies strictly inside the face.
//!
//! The probe is the caller's ([`ladder`]): it reads one witness against
//! whatever the question is about, and answers a side, [`Witness::On`]
//! (the witness lies on a probed surface) or [`Witness::InBand`] (a
//! refusal about that one point, [`PointInSolidError::inconclusive`]).
//! Either of the last two is inconclusive and the next witness is read;
//! any other refusal is about the probed surface rather than the point,
//! and propagates. The first decisive witness decides. When none does,
//! the [`Tally`] says how many read on a surface and how many in band,
//! and the caller decides what that means.
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

/// What one witness read against the probed surfaces.
#[derive(Debug)]
pub(crate) enum Witness<S> {
    /// A decisive reading.
    Side(S),
    /// The witness lies on a probed surface.
    On,
    /// The reading was in band ([`PointInSolidError::inconclusive`]).
    InBand(PointInSolidError),
}

impl Witness<SolidContainment> {
    /// One point-in-solid reading as a witness: `In` and `Out` decide,
    /// `OnBoundary` and an in-band refusal are inconclusive, and any
    /// other refusal propagates.
    pub(crate) fn of(
        read: Result<SolidContainment, PointInSolidError>,
    ) -> Result<Self, PointInSolidError> {
        match read {
            Ok(SolidContainment::OnBoundary) => Ok(Self::On),
            Ok(side) => Ok(Self::Side(side)),
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
    /// The first in-band reading, as evidence: over points, what
    /// [`crate::ray_parity::Abandoned`] keeps over one point's rays.
    pub(crate) first_in_band: Option<PointInSolidError>,
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
            Witness::InBand(e) => {
                tally.in_band += 1;
                tally.first_in_band.get_or_insert(e);
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
        let curve = body
            .get_edge(e)
            .and_then(|ed| body.get_curve_geom(ed.curve))
            .ok_or(LadderRefusal::Desync {
                face: fh.0,
                what: "witnessed edge has no curve",
            })?;
        let Some(curve) = curve.certified() else {
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
/// and every ring. `None` when no candidate certifies.
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
    Ok(None)
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
/// `false` discards the candidate unprobed: outside, on a loop, an
/// [`PointInSolidError::inconclusive`] reading, or an edge of `face`
/// whose carrier the walk cannot cross — that face then offers no
/// candidate, as a curved face offers none. Any other refusal is an
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
        Err(PointInSolidError::EdgeCarrierUnsupported { .. }) => Ok(false),
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
    /// Every witness of the shell lies on another shell, and none read
    /// in band.
    Touching,
    /// A walk refused, or no witness decided and one read in band (the
    /// first in-band reading).
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
                Witness::Side(SolidContainment::In) => other.role == ShellRole::Outer,
                Witness::Side(_) => other.role == ShellRole::Void,
                Witness::On => return Ok(Witness::On),
                Witness::InBand(e) => return Ok(Witness::InBand(e)),
            };
        }
        Ok(Witness::Side(inside))
    };
    match ladder(body, reads[i].sel.faces(), band, probe) {
        Ok(Reading::Side(inside)) => Insides::Read(inside),
        Ok(Reading::Undecided(Tally {
            first_in_band: Some(e),
            ..
        })) => Insides::Refused(e),
        Ok(Reading::Undecided(_)) => Insides::Touching,
        Err(e) => Insides::Refused(e),
    }
}

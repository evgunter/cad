//! The [`tessellate`] entry point: δ validation, deterministic mesh
//! vertex minting, the chord pass, and per-face dispatch.

use std::collections::HashMap;

use geom::Surface;
use geom_core::k_stats::{self, Detached};
use geom_core::{Band, Point3, Tol};
use topo::{Body, FaceKey};

use crate::budget;
use crate::chords::{compute_chords, edge_ends};
use crate::curved::{Shape, tessellate_curved};
use crate::memo::{FaceInputs, FaceMemo, PatchKeys, PatchMemo, Placement};
use crate::nurbs_cert::FaceBounds;
use crate::planar::tessellate_planar;
use crate::sizing::{Eps, SizingTols};
use crate::types::{BoundaryPolyline, FacePatch, Mesh, TessellateError};

/// One corner of a face patch's triangle, named the way the lane that
/// emitted it can name it.
///
/// A lane reads the mesh arena but does not write it, so it cannot know
/// where its own interior points will land in [`Mesh::positions`]: it
/// names a point it shares with another face — a topology vertex or a
/// chord point, both minted before any face runs — by that point's mesh
/// id, and one of its own interior points by that point's index in
/// [`Patch::interior`]. [`Patch::place`] turns the second into the
/// first once [`tessellate`] has assigned the face its base.
///
/// **Two constructors rather than one integer with a threshold.** The
/// two are different id spaces (`DESIGN.md` D9, engineering convention
/// 1: tagged, never in-band), so a local index cannot be read as a mesh
/// id: that half of the class is foreclosed by the type.
/// [`unpaired_chord_segment`]'s class is the OTHER half — two faces
/// emitting one chord point under different `Shared` ids — and nothing
/// here forecloses it, which is why that census stays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PatchVertex {
    /// A mesh id minted before any face ran: a topology vertex, or a
    /// point on some edge's chord polyline.
    Shared(u32),
    /// An index into this patch's own [`Patch::interior`].
    Local(u32),
}

impl PatchVertex {
    /// This corner's position: from the shared prefix, or from the
    /// patch's own interior. The lane certifies against these.
    pub(crate) fn position(self, shared: &[Point3<f64>], interior: &[Point3<f64>]) -> Point3<f64> {
        match self {
            Self::Shared(id) => shared[id as usize],
            Self::Local(i) => interior[i as usize],
        }
    }

    /// This corner's mesh id once the patch's interior begins at
    /// `base`.
    fn rebase(self, base: u32) -> u32 {
        match self {
            Self::Shared(id) => id,
            Self::Local(i) => base + i,
        }
    }
}

/// One face's tessellation AS A VALUE: the interior points the face
/// mints for itself, and its triangles in [`PatchVertex`]s.
///
/// Nothing in this value depends on another face, which is what the
/// per-face memo and the parallel map over faces both need
/// (`work/perf/plan.md` §5). The lanes are pure functions of their
/// face and the shared prefix: the trimmed lane's `bounds:
/// &FaceBounds` is a `FaceKey`-keyed certificate memo the chord pass
/// has finished writing before any lane runs, and each lane reads only
/// its own face's entry (`nurbs_cert::FaceBounds`).
pub(crate) struct Patch {
    /// The face's own grid points, in the order the lane minted them —
    /// which is the order they enter [`Mesh::positions`].
    pub(crate) interior: Vec<Point3<f64>>,
    /// The face's triangles, outward-wound (the [`FacePatch`]
    /// contract), naming shared points by mesh id and interior points
    /// by their index in `interior`.
    pub(crate) triangles: Vec<[PatchVertex; 3]>,
}

impl Patch {
    /// Places the patch in the mesh: its interior appended to
    /// `positions`, its triangles returned as mesh ids.
    ///
    /// Consumes the patch, so the [`PatchVertex`] buffer is released as
    /// the mesh-id one fills rather than living beside it.
    pub(crate) fn place(self, positions: &mut Vec<Point3<f64>>) -> Vec<[u32; 3]> {
        #[allow(clippy::cast_possible_truncation)]
        let base = positions.len() as u32;
        positions.extend(self.interior);
        self.triangles
            .into_iter()
            .map(|t| t.map(|v| v.rebase(base)))
            .collect()
    }

    /// The patch renumbered as if it were the FIRST face placed, for
    /// the per-patch censuses the lanes run on their own emission.
    ///
    /// Those censuses count uses of the edges incident to a named set
    /// of SHARED ids, and both quantities are invariant under any
    /// injective renumbering THAT FIXES THE SHARED IDS — which this is,
    /// for any base at or above `shared.len()`. So the base is
    /// arbitrary within that range and this is the smallest of them.
    ///
    /// It materialises a whole copy of the patch, so a caller that has
    /// nothing to census must not call it (`curved`, `trimmed`).
    #[cfg(debug_assertions)]
    pub(crate) fn census_ids(&self, shared: &[Point3<f64>]) -> Vec<[u32; 3]> {
        #[allow(clippy::cast_possible_truncation)]
        let base = shared.len() as u32;
        self.triangles
            .iter()
            .map(|&t| t.map(|v| v.rebase(base)))
            .collect()
    }
}

/// Tessellates a closed body into a watertight [`Mesh`] within the
/// chordal tolerance `chordal` (δ, meters) of its exact surfaces —
/// watertight as [`crate::validate::check_mesh`] reads it, which a body
/// with two coincident edges between one vertex pair does not yet meet
/// (`work/tesslie/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md`).
///
/// δ is a per-call display/export parameter, deliberately not the
/// kernel ε — see the crate docs for the distinction, the
/// certified-conservative bound, the pure-function invariant, and the
/// determinism contract (byte-identical mesh for identical
/// `(body, chordal)`).
///
/// The input is expected to be a closed solid at rest (tier 2, with
/// tier-3 geometry); tessellation does not re-validate — corrupt input
/// surfaces as typed errors where cheaply detectable (dangling keys,
/// pending faces, certificate failures) and is otherwise
/// garbage-in/garbage-out on the mesh *values*.
/// [`crate::validate::check_mesh`] is the backstop for that, and
/// **this function does not call it**: it is available to a caller,
/// and the acceptance suites run it, but nothing on this path does.
/// What this path does census is the sides two faces' patches can share:
/// one used past two triangles per chord carrying it refuses
/// [`TessellateError::OverusedSide`].
///
/// # Errors
///
/// [`TessellateError`] (closed enum): invalid δ, a pending face, NURBS
/// faces outside the trimmed-NURBS
/// inventory (illegal-rational / C⁰-creased — `nurbs_cert`), unsupported
/// carriers, a curved face whose loops have no reading in one sheet of
/// its chart, a curved face whose iso domain is not its own UV
/// rectangle, empty loops, dangling keys, resolution
/// overflow, certificate failure, CDT insertion failure, a side the
/// joined patches use past two triangles per chord.
pub fn tessellate(body: &Body<f64>, chordal: f64, tol: Tol) -> Result<Mesh, TessellateError> {
    tessellate_impl(body, chordal, tol, None).map(|(mesh, _)| mesh)
}

/// A mesh built through the memo, with each face's key row: the
/// digest of its inputs, so the caller can keep those faces alive
/// across a picture it does not re-tessellate ([`PatchMemo::keep`]),
/// and the memo entry the face was placed from
/// ([`mesh::StoredPatchId`](crate::StoredPatchId)), so a caller
/// caching anything derived from the face's placed corners can key it
/// by that.
#[derive(Clone, Debug)]
pub struct Tessellation {
    /// The mesh — byte-identical to [`tessellate`]'s for the same
    /// `(body, chordal, tol)`.
    pub mesh: Mesh,
    /// One row per face, in face-arena order.
    pub keys: PatchKeys,
}

/// [`tessellate`], answering each face from `memo` where the memo
/// holds a patch under the face's content key and running its lane
/// otherwise — the incremental re-tessellation door the crate's
/// memo-key contract exists for.
///
/// Every face's route and the chord pass run as in [`tessellate`] (the
/// key folds the face's lane, and the chord pass is per edge and cheap);
/// per face the lane is skipped on a memo hit and the stored patch is
/// placed by the same fold. The mesh is byte-identical to
/// [`tessellate`]'s: a hit is by the full key bytes, and the key is
/// every input the lane reads (`crate::memo`'s module docs state it
/// per lane). What a hit does NOT run is the lane itself, its per-patch
/// census and, with the `budget` feature, its recording; the cross-face
/// census at the end runs either way. On a hit the route's doors and its
/// split-side fact record nothing for the face (they are spliced only
/// where the lane runs); a trimmed face's domain read, the route's or
/// the face's own before its key is formed (`Route::ready`), is recorded
/// on a hit as on a miss, since the read keys the patch as the lane's
/// own read did.
/// So a hit's stream is the chord pass's and each trimmed face's domain
/// read.
///
/// The memo's lifetime is the caller's: [`PatchMemo::end_picture`]
/// after every picture is what keeps it one picture's size.
///
/// # Errors
///
/// As [`tessellate`].
pub fn tessellate_with(
    body: &Body<f64>,
    chordal: f64,
    tol: Tol,
    memo: &mut PatchMemo,
) -> Result<Tessellation, TessellateError> {
    tessellate_impl(body, chordal, tol, Some(memo)).map(|(mesh, keys)| Tessellation { mesh, keys })
}

/// Which lane a face takes — the dispatch, named so the memo key can
/// fold the same decision it is made by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Lane {
    /// `planar::tessellate_planar`: CDT of the boundary in a chart
    /// frame derived from the boundary itself.
    Planar,
    /// `curved::tessellate_curved`: the iso-rectangle walk plus UV
    /// grid on a swept analytic chart.
    Curved,
    /// `trimmed::tessellate_trimmed`: the chart lane, on every face
    /// that is no iso-rectangle of its chart, on a ruled rectangle whose
    /// straight side holds a point that the walk does not take, and on any
    /// NURBS chart.
    Trimmed,
    /// The chart lane's walk (`crate::ruled_walk`), on a ruled rectangle
    /// whose straight side holds a point and whose loop is the walk's
    /// (`trimmed::walkable`): no lattice and no CDT, at δ.
    Walked,
}

/// The dispatch.
///
/// Described NURBS faces route through the trimmed lane
/// unconditionally (M7 — the flip of the historical first-arm
/// refusal, whose record is on
/// [`TessellateError::UnsupportedSurface`]): a NURBS face has no
/// swept-rectangle chart, so the pcurve-driven walk is its only lane.
/// Illegal-rational/C⁰ classes refuse
/// [`TessellateError::UnsupportedNurbsFace`] there. An
/// approximating surface meshes through the SAME lane, on its fit: the
/// fit is the geometry, so the triangles it produces are the face's
/// own. The certificate's bound is deliberately NOT folded into the
/// mesh tolerance — widening `tol` by the fit's ε so the mesh
/// certifies against the DESCRIPTION is a separate statement, and this
/// pass makes the plain one.
///
/// **An analytic curved face is routed on its domain**, by the first of
/// these facts that answers, each named by its predicate so a citation
/// survives a fact inserted before it:
///
/// - **A ring** (`face.rings`): a face holding one is no rectangle, and
///   is the trimmed lane's, which reads every loop of its domain.
/// - **A pinch** ([`crate::walk::pinches`]): an outer loop that passes
///   one point record at two distinct vertices — a slit's parted end, or
///   both ends of a slit run the full height of a full turn. The curved
///   lane's walk places each junction by inverting its POSITION through
///   the chart, so two vertices at one position get one chart place, and
///   which side of the slit each copy lies on is a fact of the loop's
///   topology that no position carries. The trimmed lane places each
///   vertex by its own stored chart image, so the face is that lane's,
///   whether or not its domain is an iso rectangle (the full-height slit's
///   is). The body states the two vertices one point by giving them one
///   point record, so the route reads that and no distance.
/// - **A non-iso chart image** ([`geom_brep::Pcurve::is_non_iso_form`]):
///   a face whose outer loop stores a chart image of a form no iso line
///   takes (a projected circle, a focal section, a spiric, a fitted or
///   general spline) is no iso rectangle, and is the trimmed lane's. The
///   form is what the mint chose when its class reading showed none of
///   the chart's iso classes, so the route reads that stored fact and
///   asks no door.
/// - **The doors** ([`crate::curved::iso_rectangle_shape`]) route every
///   other face. A face they certify an iso-parameter rectangle on one
///   chart branch is the curved lane's, and carries their answer as a
///   value ([`crate::curved::Rectangle`]), which is the only way into
///   that lane. A face they decide is none — a notch, an arc through a
///   pole — is the trimmed lane's only where that lane can read its
///   domain (`topo::Body::face_lift`); where it cannot, the doors' own
///   verdict is the refusal, with its recourse. A face the doors cannot
///   decide, or whose stored data contradicts itself, refuses
///   [`TessellateError::UnsupportedCurvedShape`] with their cause: no
///   lane is taken because a door could not answer.
/// - **A split straight side** ([`crate::walk::splits_a_straight_side`]):
///   a rectangle the doors certify on a ruled chart (a cylinder or a
///   cone) whose straight side holds a point between its ends — a ruling
///   split into several edges, or a cone side run through its apex — is
///   the trimmed lane's: the curved lane's theorem for a ruled face holds
///   every triangle between two neighbouring rulings, and that point is a
///   boundary point off its grid. The doors' [`crate::curved::Rectangle`]
///   is dropped here unspent, so the token still means what it says (the
///   doors certified a rectangle) and the curved lane is still reached
///   through it alone; what the route adds is that the lane is handed
///   only a ruled rectangle with bare sides, and the lane's own guard
///   (`curved::SIDE_CHORD_POINT`) refuses one that reaches it otherwise.
///
/// Read once per face per run, by [`Routes::of`], and handed to the
/// chord pass, the sizing and the lanes, so no two of them can answer
/// it differently.
///
/// The route reads a face's domain ([`read_domain`]) where the doors
/// decided it no rectangle, since a domain the chart lane cannot read
/// makes that face no lane's; every other trimmed face's is read at its
/// turn ([`Route::ready`]). Answers the route and, where it read the
/// domain, the K-funnel recording of that read apart from the doors': a
/// memo hit records the domain read, which keys the patch as the lane's
/// own read does, and not the doors', which ran in place of the lane's.
pub(crate) fn route_of(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    band: Band,
) -> Result<(Route, Option<Detached>), TessellateError> {
    Ok(match *surface {
        Surface::Nurbs(_) | Surface::Approx(_) => (Route::Trimmed(Trim::Structural), None),
        Surface::Plane { .. } => (Route::Planar, None),
        _ => {
            let face = body
                .get_face(fk)
                .ok_or(TessellateError::MissingEntity { what: "face" })?;
            if !face.rings.is_empty() || crate::walk::pinches(body, face.outer)? {
                return Ok((Route::Trimmed(Trim::Structural), None));
            }
            let off_iso = crate::walk::loop_half_edges(body, face.outer, fk)?
                .into_iter()
                .any(|(hek, _, _)| {
                    body.pcurve(hek)
                        .is_some_and(|c| c.pcurve().is_non_iso_form())
                });
            if off_iso {
                return Ok((Route::Trimmed(Trim::Structural), None));
            }
            match crate::curved::iso_rectangle_shape(body, fk, face.outer, surface, band)? {
                Shape::Rectangle(rectangle) => {
                    if crate::walk::splits_a_straight_side(body, fk, surface)? {
                        // Fact 4: the token goes unspent; the walk takes the
                        // face where its loop is the walk's.
                        if crate::trimmed::walkable(body, fk, surface, band)? {
                            (Route::Trimmed(Trim::Walked), None)
                        } else {
                            (Route::Trimmed(Trim::Structural), None)
                        }
                    } else {
                        (Route::Curved(rectangle), None)
                    }
                }
                Shape::NoRectangle(verdict) => {
                    let (lift, read) =
                        k_stats::detached(|| read_domain(body, fk, band, Some(verdict)));
                    match lift {
                        Ok(lift) => (Route::Trimmed(Trim::Read(lift)), Some(read)),
                        Err(refusal) => {
                            k_stats::splice(read);
                            return Err(refusal);
                        }
                    }
                }
            }
        }
    })
}

/// **A face's route**: its lane, with what the route knows of it. The
/// curved lane runs on the doors' "rectangle" answer; the trimmed lane on
/// the face's domain, which [`Route::ready`] reads at the face's turn.
pub(crate) enum Route {
    Planar,
    Curved(crate::curved::Rectangle),
    Trimmed(Trim),
}

/// A trimmed face's domain, as far as the route has it.
pub(crate) enum Trim {
    /// A fact of the face sends it to the trimmed lane before or past
    /// the doors (a ring, a stored image of a non-iso form, a ruled
    /// rectangle whose straight side holds a point, a spline chart); its
    /// domain is read at its turn ([`Route::ready`]).
    Structural,
    /// The doors decided the face no rectangle, and the route read its
    /// domain ([`route_of`]).
    Read(topo::pcurves::FaceLift<f64>),
    /// Fact 4 on a face the walk takes ([`Lane::Walked`]); its domain is
    /// read at its turn.
    Walked,
}

/// **The one read of a trimmed face's domain** (`trimmed::face_domain`),
/// which the memo key folds and the lane triangulates, with what its
/// miss answers. Where the doors decided the face no rectangle (their
/// `verdict`), a domain stored with no image is the lane's refusal, and
/// one whose loops have no reading in one sheet of the chart (an arc
/// through a pole, a loop that winds it) is a face no lane takes: the
/// doors' verdict, with its recourse, is the answer. Every other miss is
/// the lane's refusal.
fn read_domain(
    body: &Body<f64>,
    fk: FaceKey,
    band: Band,
    verdict: Option<geom_brep::props::PropsError>,
) -> Result<topo::pcurves::FaceLift<f64>, TessellateError> {
    crate::trimmed::face_domain(body, fk, band).map_err(|unread| match verdict {
        Some(verdict)
            if !matches!(
                &unread,
                TessellateError::DomainUnread { cause, .. }
                    if matches!(**cause, topo::pcurves::RingPlacementError::Gap(_))
            ) =>
        {
            TessellateError::UnsupportedCurvedShape {
                face: fk,
                source: verdict,
            }
        }
        _ => unread,
    })
}

impl Route {
    /// **The route, ready to run**: a trimmed face's domain, read here
    /// where the route did not read it ([`read_domain`]).
    ///
    /// # Errors
    ///
    /// The read's miss.
    pub(crate) fn ready(
        self,
        body: &Body<f64>,
        fk: FaceKey,
        band: Band,
    ) -> Result<Ready, TessellateError> {
        Ok(match self {
            Self::Planar => Ready::Planar,
            Self::Curved(rectangle) => Ready::Curved(rectangle),
            Self::Trimmed(Trim::Read(lift)) => Ready::Trimmed(lift),
            Self::Trimmed(Trim::Structural) => Ready::Trimmed(read_domain(body, fk, band, None)?),
            Self::Trimmed(Trim::Walked) => Ready::Walked(read_domain(body, fk, band, None)?),
        })
    }
}

/// A route with what its lane runs on: [`Route::ready`]'s answer.
pub(crate) enum Ready {
    Planar,
    Curved(crate::curved::Rectangle),
    Trimmed(topo::pcurves::FaceLift<f64>),
    Walked(topo::pcurves::FaceLift<f64>),
}

impl Ready {
    /// The lane this runs.
    fn lane(&self) -> Lane {
        match self {
            Self::Planar => Lane::Planar,
            Self::Curved(_) => Lane::Curved,
            Self::Trimmed(_) => Lane::Trimmed,
            Self::Walked(_) => Lane::Walked,
        }
    }

    /// The trimmed lane's domain; `None` on the other lanes, which read
    /// none.
    fn domain(&self) -> Option<&topo::pcurves::FaceLift<f64>> {
        match self {
            Self::Trimmed(lift) | Self::Walked(lift) => Some(lift),
            Self::Planar | Self::Curved(_) => None,
        }
    }
}

/// One face's entry in [`Routes`]: the lane it was routed to, or the
/// route's refusal, which the run reports at that face's turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Routed {
    To(Lane),
    Refused,
}

/// One face as [`Routes::of`] routed it: the face, its route or the
/// route's refusal, and the K-funnel recordings the route made: the
/// doors' and the split-side fact's (spliced where the lane runs), and
/// the domain read's where the route read the domain (spliced on every
/// run). On a ruled face with a split side the fact also asks whether the
/// walk takes it (`trimmed::walkable`): one `chart_singularity_at_vertex`
/// per singular point of the chart per vertex of the outer loop. The walk
/// itself records nothing of its own: whether a face across can reach a
/// side (`trimmed::may_make`) reads no band. Pinned on a split-side body
/// (`tests/route_rows.rs`, `a_split_side_body_records_the_walks_decisions`).
pub(crate) struct FaceRoute {
    pub(crate) face: FaceKey,
    pub(crate) route: Result<Route, TessellateError>,
    pub(crate) asked: Detached,
    pub(crate) read: Option<Detached>,
}

/// **Every face's lane, read once per run** ([`route_of`]), in face-arena
/// order, with the K-funnel recording each face's read made. The fold
/// splices it in at that face's turn, before the face's own work: the
/// doors' and the split-side fact's where the lane runs, the domain
/// read's on a memo hit as well ([`FaceRoute`]).
pub(crate) struct Routes {
    faces: HashMap<FaceKey, Entry>,
}

/// One face's entry in [`Routes`]: its lane, a curved-lane face's
/// carrying the doors' answer, which the chord rule reads beside the
/// walk ([`Routes::rectangle`]), a trimmed face the walk takes or one it
/// does not, or the route's refusal.
enum Entry {
    Planar,
    Curved(crate::curved::Rectangle),
    Trimmed,
    Walked,
    Refused,
}

impl Routes {
    /// Routes every face of `body` at `band`: the table, and each face's
    /// route with its recording, in arena order.
    pub(crate) fn of(body: &Body<f64>, band: Band) -> (Self, Vec<FaceRoute>) {
        let routed: Vec<_> = body
            .faces()
            .map(|(face, _)| {
                let (route, asked) = k_stats::detached(|| {
                    face_surface(body, face).and_then(|surface| route_of(body, face, surface, band))
                });
                let (route, read) = match route {
                    Ok((route, read)) => (Ok(route), read),
                    Err(error) => (Err(error), None),
                };
                FaceRoute {
                    face,
                    route,
                    asked,
                    read,
                }
            })
            .collect();
        let faces = routed
            .iter()
            .map(|r| {
                let entry = match &r.route {
                    Ok(Route::Planar) => Entry::Planar,
                    Ok(Route::Curved(rectangle)) => Entry::Curved(rectangle.clone()),
                    Ok(Route::Trimmed(Trim::Walked)) => Entry::Walked,
                    Ok(Route::Trimmed(_)) => Entry::Trimmed,
                    Err(_) => Entry::Refused,
                };
                (r.face, entry)
            })
            .collect();
        (Self { faces }, routed)
    }

    /// Face `fk`'s doors' answer, where it routed to the curved lane.
    pub(crate) fn rectangle(&self, fk: FaceKey) -> Option<&crate::curved::Rectangle> {
        match self.faces.get(&fk) {
            Some(Entry::Curved(rectangle)) => Some(rectangle),
            _ => None,
        }
    }

    /// Face `fk`'s entry.
    ///
    /// # Errors
    ///
    /// [`TessellateError::MissingEntity`] for a face the table did not
    /// route: a key no live face of the body holds.
    pub(crate) fn lane(&self, fk: FaceKey) -> Result<Routed, TessellateError> {
        match self.faces.get(&fk) {
            Some(Entry::Planar) => Ok(Routed::To(Lane::Planar)),
            Some(Entry::Curved(_)) => Ok(Routed::To(Lane::Curved)),
            Some(Entry::Trimmed) => Ok(Routed::To(Lane::Trimmed)),
            Some(Entry::Walked) => Ok(Routed::To(Lane::Walked)),
            Some(Entry::Refused) => Ok(Routed::Refused),
            None => Err(TessellateError::MissingEntity { what: "face" }),
        }
    }

    /// The table alone, at the witness tolerance's band: what a unit
    /// test of one pass reads in place of a run's.
    #[cfg(test)]
    #[allow(clippy::expect_used)]
    pub(crate) fn witness(body: &Body<f64>) -> Self {
        Self::of(
            body,
            Band::linear(Tol::witness()).expect("the witness band"),
        )
        .0
    }

    /// This table with face `fk` routed to the curved lane on
    /// `rectangle`, for a row whose subject is what that lane does with a
    /// face the route would not hand it.
    #[cfg(test)]
    pub(crate) fn with_curved(mut self, fk: FaceKey, rectangle: crate::curved::Rectangle) -> Self {
        self.faces.insert(fk, Entry::Curved(rectangle));
        self
    }
}

/// A face's surface ([`Body::face_surface`]), its misses named as the
/// tessellator names them: the one door the tessellator reads a face's
/// surface through, so a pending face is refused here and nowhere
/// past it.
pub(crate) fn face_surface(
    body: &Body<f64>,
    fk: FaceKey,
) -> Result<&Surface<f64>, TessellateError> {
    body.face_surface(fk).map_err(|miss| match miss {
        topo::SurfaceMiss::Stale => TessellateError::MissingEntity { what: "face" },
        topo::SurfaceMiss::Pending => TessellateError::UnsupportedSurface { face: fk },
    })
}

/// One face's slot in [`tessellate_impl`]'s indexed parallel map: the
/// lane's answer, and everything the arena-order fold still owes for
/// that face.
///
/// Every field is a VALUE the map computed and the fold applies. That
/// is what makes the fold the only order-dependent thing in the pass:
/// the patch enters the mesh arena at this face's turn, the memo work
/// is counted and inserted at this face's turn, and the meter's rows
/// are handed over at this face's turn — however the map scheduled the
/// faces.
struct FaceWork<'m> {
    face: FaceKey,
    /// What the budget meter recorded while this face's lane ran, on
    /// whatever thread that was (`budget::record`).
    meter: budget::FaceRecording,
    /// What the K-funnel recorded while it ran — the verdicts, the
    /// escalations and (under `probe`) the margin samples, taken under
    /// a frame of the lane's own (`geom_core::k_stats::detached`) and
    /// spliced into the caller's in the fold.
    decided: Detached,
    /// The memo's work for this face, absent on the memo-free path.
    memo: Option<FaceMemo>,
    /// What the fold puts in the mesh arena for this face — the lane's
    /// own patch, or the memo entry it borrows — or the error its lane
    /// refused with, carried rather than propagated so the fold reports
    /// the first refusal in ARENA order rather than the map's first.
    place: Result<Placement<'m>, TessellateError>,
}

/// The one implementation behind both doors: `memo` is `None` for
/// [`tessellate`] and the path is then exactly the memo-free one.
fn tessellate_impl(
    body: &Body<f64>,
    chordal: f64,
    tol: Tol,
    memo: Option<&mut PatchMemo>,
) -> Result<(Mesh, PatchKeys), TessellateError> {
    if !(chordal.is_finite() && chordal > 0.0) {
        return Err(TessellateError::InvalidChordalTolerance { value: chordal });
    }
    let ambient = tol;
    let eps = Eps::at(tol);
    // Props' decision band, built once at operation entry as
    // `Band::linear` prescribes, and read by the route's doors and domain
    // read, the walk's wrap strip and the trimmed lane's singular
    // vertices (`SizingTols::band`). Eager on purpose: a run whose
    // ε cannot form a band refuses every body alike, the all-planar
    // ones included, rather than meshing until the first curved face
    // (`TessellateError::Band` says why).
    let band = Band::linear(tol).map_err(|error| TessellateError::Band { error })?;

    // Mesh vertex ids: topology vertices first, arena order (D9).
    let mut positions = Vec::new();
    let mut vids = HashMap::new();
    for (vk, p) in body.vertex_points() {
        #[allow(clippy::cast_possible_truncation)]
        vids.insert(vk, positions.len() as u32);
        positions.push(p);
    }

    // Certified whole-patch NURBS bounds, assembled once per face and
    // shared by both passes that need them (`chords::FaceBounds`).
    let mut bounds = FaceBounds::new();

    // Every face's route, read once, before anything sizes a face.
    let (routes, routed) = Routes::of(body, band);
    // Chord pass: per-edge polylines, computed once (crate docs);
    // `chord_ts` is the matching parameter schedule (the trimmed lane
    // evaluates pcurves on it — one derivation, both consumers).
    let chords = compute_chords(body, chordal, &routes, &vids, &mut positions, &mut bounds)?;
    // Every id a face can SHARE with another face is now minted:
    // topology vertices, then chord points, then (per face, below) that
    // face's own interior grid. So a shared id is exactly an id below
    // this mark — which is what the census at the end of this function
    // tests with an integer compare rather than a lookup, and what
    // makes `positions[..shared_below]` the whole of the mesh positions
    // a face reads from outside itself.
    let shared_below = positions.len();
    let mut boundaries = Vec::new();
    for (ek, _) in body.edges() {
        let (start_vertex, end_vertex) = edge_ends(body, ek)?;
        let points = chords
            .ids
            .get(&ek)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chords",
            })?
            .clone();
        boundaries.push(BoundaryPolyline {
            edge: ek,
            points,
            start_vertex,
            end_vertex,
        });
    }

    // Per-face dispatch: **D9 idiom 1**, an indexed parallel map over
    // the face arena into a pre-sized buffer, combined by the
    // sequential arena-order fold below, which is **idiom 2**
    // (`DESIGN.md`'s D9 addendum; `work/perf/plan.md` §2.2). Nothing
    // here re-derives determinism: the map's combination is positional,
    // so the mesh is the same bytes at any thread count, and the fold
    // is where every order-dependent thing happens — the mesh arena,
    // the memo's counters and entries, and the budget meter's rows.
    //
    // Of what other faces' work writes, a face reads only finished
    // values: the mesh positions `positions[..shared_below]` (above),
    // the chord pass's ids, parameters and holders, and `bounds`
    // (`nurbs_cert::FaceBounds`), which the chord pass has finished
    // writing; the memo is read through its pure half
    // (`PatchMemo::lookup`). What each lane reads is `memo`'s list.
    //
    // ERRORS. The refusal a caller sees is the first in ARENA order,
    // never the first the map finished. The price is that every face is
    // computed before the fold picks it — work a refusing body does and
    // throws away, at every width, on a path that never produces an
    // answer.
    //
    // PEAK MEMORY. Every face's patch is live at once here, where the
    // serial loop held one at a time: the bound is the whole mesh's
    // interior points and triangles, plus (with a memo) one
    // `StoredPatch` per missed face — about one extra copy of the mesh
    // rather than one copy of its largest face. Measured, on the tour's
    // hollow ring (four faces, the thing that makes a patch big here):
    // at 164 940 triangles peak RSS is 18.6 MB serial against 19.8 MB
    // at four threads, and at 546 864 triangles 46.0 MB against
    // 54.1 MB — +18 % where it shows, on a figure the mesh value itself
    // dominates. A body with MANY faces pays less, not more: the
    // patches are smaller.
    let tol = SizingTols {
        delta: chordal,
        eps,
        band,
    };
    let (faces, recorded): (Vec<_>, Vec<_>) = routed
        .into_iter()
        .map(|r| ((r.face, r.route), (r.asked, r.read)))
        .unzip();
    // The meter's arming as a value: a lane runs on a worker thread,
    // which nobody armed (`budget`'s module docs).
    let arming = budget::arming();
    let reader = memo.as_deref();
    let work: Vec<FaceWork<'_>> = {
        let shared = &positions[..shared_below];
        k_stats::par_map(faces, |(fk, route)| {
            // ONE wrapper for both of the tessellation's
            // thread-local channels, because they are one problem:
            // a channel the CALLER owns and the worker does not.
            // The budget meter's per-face rows
            // (`budget::record`) and the K-funnel's verdicts,
            // escalations and `probe` samples
            // (`k_stats::detached`) are both recorded into
            // thread-locals by the lane, both travel back in this
            // face's slot, and the fold hands both on in arena
            // order — so an armed meter and a `Bracket` around
            // `tessellate` see what a serial walk would have
            // written, element for element, at any thread count.
            //
            // NO PORTABILITY GATE (`geom_core::sym::
            // decisions_are_thread_portable`, which the evaluator's
            // node map and `topo::props`' face walks read), and the
            // reason is this crate's scalar policy: `mesh` takes
            // `&Body<f64>` and instantiates nothing else, so a
            // lane's decisions never reach `Sym::sign_within`. Both
            // halves of that test are writes from there — the
            // session (the decision itself and its receipt) and the
            // shape report (a row per decision, session or not) —
            // so neither can arise here.
            let (((memo_work, place), meter), decided) = k_stats::detached(|| {
                budget::record(arming, || {
                    let surface = match face_surface(body, fk) {
                        Ok(surface) => surface,
                        Err(error) => return (None, Err(error)),
                    };
                    // The trimmed lane's domain, read once and handed to
                    // the memo key and the lane alike (`Route::ready`).
                    let route = match route.and_then(|route| route.ready(body, fk, tol.band)) {
                        Ok(route) => route,
                        Err(error) => return (None, Err(error)),
                    };
                    let lane = route.lane();
                    // The lanes, each over `shared`, the mesh positions
                    // it shares (what else each reads is `memo`'s list).
                    //
                    // The planar lane derives its chart frame from the
                    // face's own boundary (planar.rs module docs, #284)
                    // — the stored plane axes are deliberately not
                    // passed: imported axes carry translator noise that
                    // projects valid boundaries below spade's
                    // coordinate domain.
                    let run = || match &route {
                        Ready::Trimmed(lift) | Ready::Walked(lift) => {
                            crate::trimmed::tessellate_trimmed(
                                body,
                                fk,
                                surface,
                                &chords,
                                shared,
                                &tol,
                                &bounds,
                                lift,
                                crate::trimmed::BOUNDARY_CLEARANCE,
                                lane == Lane::Walked,
                            )
                        }
                        Ready::Planar => tessellate_planar(body, fk, &chords.ids, shared),
                        Ready::Curved(rectangle) => tessellate_curved(
                            body,
                            fk,
                            surface,
                            rectangle,
                            &chords.ids,
                            shared,
                            &tol,
                        ),
                    };
                    let Some(memo) = reader else {
                        return (None, run().map(Placement::Lane));
                    };
                    let inputs = match FaceInputs::gather(
                        body,
                        fk,
                        lane,
                        surface,
                        &chords,
                        shared,
                        chordal,
                        ambient,
                        route.domain(),
                    ) {
                        Ok(inputs) => inputs,
                        Err(error) => return (None, Err(error)),
                    };
                    let lookup = match memo.lookup(&inputs).answered() {
                        Ok((placement, work)) => return (Some(work), Ok(placement)),
                        Err(lookup) => lookup,
                    };
                    match run() {
                        Ok(patch) => {
                            let (placement, work) = lookup.ran(patch);
                            (Some(work), Ok(placement))
                        }
                        Err(error) => (Some(lookup.refused()), Err(error)),
                    }
                })
            });
            FaceWork {
                face: fk,
                meter,
                decided,
                memo: memo_work,
                place,
            }
        })
    };

    // The fold (idiom 2), in face-arena order, and in TWO passes over
    // the same slots.
    //
    // Placement first, for every face, while the memo is only read:
    // a hit is renamed straight out of the entry the map found
    // (`memo::Placement::Memo`), and an insert made during the fold
    // could otherwise replace an entry a later hit still points at.
    // Recording second, once no placement borrows the memo. The two
    // passes are independent — one writes the mesh arena, the other the
    // memo's counters and entries — so splitting them changes nothing
    // either produces, and both walk the faces in arena order.
    let mut patches = Vec::with_capacity(work.len());
    let mut pending: Vec<Option<FaceMemo>> = Vec::with_capacity(work.len());
    let mut refusal = None;
    for (face, (asked, read)) in work.into_iter().zip(recorded) {
        // Both channels, in arena order, up to AND INCLUDING the face
        // that refuses: the serial walk recorded whatever the refusing
        // face decided before it refused, and nothing after it. A face's
        // route is read before its lane runs, so it is spliced first:
        // its doors where the lane runs (a memo hit runs no lane), its
        // domain read on every run (`tessellate_with`).
        if !matches!(face.place, Ok(Placement::Memo { .. })) {
            k_stats::splice(asked);
        }
        if let Some(read) = read {
            k_stats::splice(read);
        }
        k_stats::splice(face.decided);
        budget::absorb(face.meter);
        pending.push(face.memo);
        match face.place {
            // Each face's interior takes the arena as it stands at that
            // face's turn, in face-arena order (D9).
            //
            // THE JOIN, where `[mesh.winding]` is read: every triangle of
            // every face, whichever lane made it and whether the patch is
            // the lane's or the memo's, against the positions it has in
            // the mesh. A face with a triangle f64 cannot wind refuses.
            Ok(placement) => {
                let triangles = placement.place(&mut positions);
                if let Some((_, unresolved)) =
                    crate::validate::first_unresolved(&positions, &triangles)
                {
                    refusal = Some(TessellateError::UnresolvedWinding {
                        face: face.face,
                        unresolved: Box::new(unresolved),
                    });
                    break;
                }
                patches.push(FacePatch {
                    face: face.face,
                    triangles,
                });
            }
            // The first refusal in arena order. Its own memo work and
            // meter rows are already in hand — the serial loop counted
            // and recorded them before it ran the lane — and no later
            // face's are.
            Err(error) => {
                refusal = Some(error);
                break;
            }
        }
    }

    let mut keys = PatchKeys::default();
    if let Some(memo) = memo {
        for work in pending.into_iter().flatten() {
            memo.record(work, &mut keys);
        }
    }
    if let Some(error) = refusal {
        return Err(error);
    }

    // The one place patches become the mesh. No lane-produced body is
    // known to reach the census inside it (the trimmed lane's shared-side
    // cure clears every one measured), so its rows drive `joined` itself
    // on a real mesh (`side_census`), and this call is held by reading.
    #[allow(clippy::cast_possible_truncation)]
    let mesh = joined(
        Mesh {
            positions,
            patches,
            boundaries,
        },
        shared_below as u32,
    )?;
    Ok((mesh, keys))
}

/// **The faces' patches as the body's mesh**, once the census of the
/// sides where they meet has read them. Two patches can share a triangle
/// side only between two shared ids (topology vertices and chord points,
/// below `shared_below`), so the census reads those sides and nothing
/// else: its footprint is the cross-face question, where `check_mesh`
/// reads every edge of every patch, overwhelmingly patch-interior ones,
/// and re-checks winding and degeneracy, which are other rows' classes.
/// It reads ids and counts, no tolerance.
///
/// A side of more than two triangles per chord carrying it is an edge
/// that is not manifold, whichever lane made it: two faces across one
/// chord chain that both use a chord between two of its points as an
/// interior edge use it twice each, and no patch sees that alone. It
/// refuses [`TessellateError::OverusedSide`].
fn joined(mesh: Mesh, shared_below: u32) -> Result<Mesh, TessellateError> {
    let polylines: Vec<&[u32]> = mesh
        .boundaries
        .iter()
        .map(|b| b.points.as_slice())
        .collect();
    let uses = side_uses(
        &polylines,
        mesh.patches.iter().map(|p| p.triangles.as_slice()),
        shared_below,
    );
    if let Some((side, uses)) = overused_side(&uses) {
        return Err(TessellateError::OverusedSide {
            faces: faces_using(&mesh, side),
            uses,
        });
    }
    // D2 addendum row 5: a shared boundary the two adjacent faces failed
    // to identify with each other leaves each side's copy short of two
    // uses per chord, which `curved`'s per-patch re-derivation cannot see
    // (issue 897). Debug builds re-derive it here, over the same counts.
    #[cfg(debug_assertions)]
    {
        let bad = unpaired_chord_segment(&uses);
        debug_assert!(
            bad.is_none(),
            "chord segment {:?} is an edge of {} face triangles, not two per chord \
             carrying it. The census counts and cannot say why. Known causes: the faces meeting on that edge \
             emitted the segment under different ids, or a face there emitted no \
             triangle along it (issue 897)",
            bad.map(|(e, _)| e),
            bad.map_or(0, |(_, n)| n)
        );
    }
    Ok(mesh)
}

/// **The triangle uses of every side between two shared ids**, across
/// the patches, with the chords carrying each side as a segment: per
/// side, `(chords, uses)`.
///
/// `shared_below` is the first id minted after the chord pass. Ids are
/// minted topology-vertices-then-chords-then-per-face-grid (D9's
/// determinism order, at the top of [`tessellate`]), so an id at or
/// above the mark is one face's private grid point: a side with such an
/// end is one patch's alone and is never counted. Testing that first is
/// what keeps this scan an integer compare on the overwhelming majority
/// of triangle sides rather than a map probe.
fn side_uses<'a>(
    polylines: &[&[u32]],
    patches: impl IntoIterator<Item = &'a [[u32; 3]]>,
    shared_below: u32,
) -> HashMap<(u32, u32), (usize, usize)> {
    let mut uses: HashMap<(u32, u32), (usize, usize)> = HashMap::new();
    for ids in polylines {
        for w in ids.windows(2) {
            uses.entry(crate::walk::edge_key(w[0], w[1])).or_default().0 += 1;
        }
    }
    for &t in patches.into_iter().flatten() {
        for (a, b) in crate::walk::sides(t) {
            // `a < b` (`edge_key`): both ends are shared when `b` is.
            if b < shared_below {
                uses.entry((a, b)).or_default().1 += 1;
            }
        }
    }
    uses
}

/// **The least side used by more than two triangles per chord carrying
/// it** (two where no chord carries it), with its use count.
///
/// A chord segment is a side of each face along its edge once, so two
/// uses per chord; two coincident edges between one vertex pair (two
/// solids touching along a line) are one id pair carried twice, which
/// their four faces use four times. That body's mesh is the open
/// question of
/// `work/tesslie/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md`,
/// and this census leaves it as it was. Any other side is a triangle side
/// inside a patch, two uses in all: past that, two patches both use it
/// as an interior edge, or one patch uses it past its own two.
fn overused_side(uses: &HashMap<(u32, u32), (usize, usize)>) -> Option<((u32, u32), usize)> {
    uses.iter()
        .filter(|&(_, &(chords, n))| n > 2 * chords.max(1))
        .map(|(&side, &(_, n))| (side, n))
        .min()
}

/// The faces whose patches hold a triangle with side `side`, in arena
/// order (the patches').
fn faces_using(mesh: &Mesh, (a, b): (u32, u32)) -> Vec<FaceKey> {
    mesh.patches
        .iter()
        .filter(|p| {
            p.triangles
                .iter()
                .any(|&t| crate::walk::sides(t).contains(&(a, b)))
        })
        .map(|p| p.face)
        .collect()
}

/// The least chord segment used by fewer than two face triangles per
/// chord carrying it, if any: the cross-face identification
/// re-derivation (issue 897), on [`side_uses`]' counts.
///
/// Every edge of the body carries a chord polyline whose segments the
/// two faces meeting on that edge both insert as CDT constraints, so
/// each segment is a triangle edge twice per chord carrying it: once
/// per side, or twice within one patch where a `Seam` edge is traversed
/// both ways by the same face. A use past that is [`overused_side`]'s,
/// refused typed before this runs.
///
/// A count of 1 on a single chord is the class this guard exists for —
/// the two sides emitted the segment under DIFFERENT ids, so neither
/// copy pairs up. A face that emits no triangle along the segment leaves
/// 1 (its neighbour's) or 0 too. The count cannot tell causes apart, so
/// the report states the count, the expected two per chord, and the
/// causes known — without claiming the list is complete.
///
/// **PRECONDITION: the body is CLOSED, and that is the caller's, not
/// this census's.** [`tessellate`]'s contract says the input is a
/// closed solid at rest and that it does not re-validate; it never
/// calls `topo::validate_closed`. On an OPEN body — a tier-1-legal
/// scaffolding strut, say, which `topo::validate` accepts and
/// `validate_closed` rejects — a chord polyline exists that no face
/// triangle can use twice, and this census reports it. That firing is
/// a broken PRECONDITION, not the D2-row-5 kernel bug the assert is
/// worded for. It stays a `debug_assert` on that basis: the
/// precondition is documented at the door, an open body is already
/// outside what `tessellate` promises anything about, and no shipped
/// build is made to panic by it that was not already garbage-in.
///
/// **No closed body is known to reach it.** A closed body one of whose
/// curved faces walks to a zero-width domain would emit nothing for
/// that face and fire this census, and every such body measured is
/// answered before it (`mesh/tests/loops_with_no_rim.rs`,
/// `mesh/tests/the_one_face_ball.rs`): a rimless loop on one column
/// refuses `SingleColumnCurvedFace` (the torus face bounded by one
/// meridian circle,
/// `a_rim_free_loop_on_a_chart_with_no_pole_refuses_single_column`), a
/// slit on two coincident edges has no lift for the band's columns and
/// refuses `DomainUnread`
/// (`two_coincident_edges_have_no_lift_for_the_walk_to_read`), a slit face
/// between the one-face ball's tips refuses `SingleColumnCurvedFace`
/// (`a_slit_between_the_tips_is_not_the_strip`), a planar lamina bounded
/// by two coincident edges refuses `BoundaryBoundsNoRegion`
/// (`mesh/tests/faces_that_bound_no_region.rs`), and the one-face ball,
/// its wrap edge whole or split, is placed as the wrap strip
/// (`a_split_wrap_edge_is_one_strip_whichever_half_the_walk_opens_on`). A walk that put the strip's two
/// sides on one column again is what this census would catch.
///
/// **The route is documented, not demonstrated, and the difference is
/// recorded rather than glossed.** A reviewer's probe
/// (`r2_mesh6_probes::r2_scaffold_strut_body_through_tessellate`)
/// mints the strut body through the Euler doors and calls
/// [`tessellate`] on it; the call refuses EARLIER and typed —
/// `UnsupportedSurface`, because faces assembled that way carry no
/// surface description — so the census is never reached and no open
/// body is yet known to reach it. The probe is kept as the record of
/// that attempt: it pins where the door actually stops, which is the
/// honest state of the precondition claim.
#[cfg(debug_assertions)]
fn unpaired_chord_segment(
    uses: &HashMap<(u32, u32), (usize, usize)>,
) -> Option<((u32, u32), usize)> {
    uses.iter()
        .filter(|&(_, &(chords, n))| chords > 0 && n < 2 * chords)
        .map(|(&side, &(_, n))| (side, n))
        .min()
}

// GATED ON THE GUARD IT TESTS: every row here calls
// `unpaired_chord_segment`, which
// is `#[cfg(debug_assertions)]`, so with debug-assertions OFF the
// subject does not exist and neither should the rows. Without this the
// lib test target fails to COMPILE in that configuration.
#[cfg(all(test, debug_assertions))]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The cross-face identification census (issue 897), red first.
    //!
    //! The class is the one the per-patch re-derivation in `curved`
    //! cannot see by construction: each patch is internally consistent
    //! — every one of its own edges used at most twice — and the
    //! failure is only visible when the two patches are read together.
    //! The rows below build exactly that mesh.

    use super::*;

    fn unpaired(
        poly: &[&[u32]],
        patches: &[&[[u32; 3]]],
        mark: u32,
    ) -> Option<((u32, u32), usize)> {
        unpaired_chord_segment(&side_uses(poly, patches.iter().copied(), mark))
    }

    /// Every edge use within one patch, so a row can show the patch is
    /// internally clean while the pair of patches is not.
    fn per_patch_max_use(tris: &[[u32; 3]]) -> usize {
        let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
        for &t in tris {
            for side in crate::walk::sides(t) {
                *uses.entry(side).or_insert(0) += 1;
            }
        }
        uses.values().copied().max().unwrap_or(0)
    }

    #[test]
    fn a_shared_chord_segment_used_once_per_side_pairs_up() {
        // Chord polyline 0-1-2 on the edge between two faces; each
        // face emits both segments once. Ids 9+ are grid points.
        let poly: [&[u32]; 1] = [&[0, 1, 2]];
        let tris = [[0, 1, 9], [1, 2, 9], [1, 0, 10], [2, 1, 10]];
        assert_eq!(
            unpaired(&poly, &[&tris], 3),
            None,
            "two faces that identified the boundary leave every segment at two uses"
        );
    }

    #[test]
    fn two_coincident_chords_on_one_id_pair_pair_up_at_four_uses() {
        // Two straight edges between vertices 0 and 1 (solids touching
        // along a line): four faces, one use each. Three is a face that
        // emitted nothing along it.
        let poly: [&[u32]; 2] = [&[0, 1], &[0, 1]];
        let tris = [[0, 1, 9], [1, 0, 10], [0, 1, 11], [1, 0, 12]];
        assert_eq!(unpaired(&poly, &[&tris], 2), None);
        assert_eq!(
            unpaired(&poly, &[&tris[..3]], 2),
            Some(((0, 1), 3)),
            "a face missing along a doubled chord is caught"
        );
    }

    #[test]
    fn a_seam_edge_traversed_twice_by_one_face_pairs_up() {
        // The full-2π case: ONE patch supplies both uses. The census
        // counts uses, not sides, which is what makes this legal.
        let poly: [&[u32]; 1] = [&[0, 1]];
        let tris = [[0, 1, 9], [1, 0, 10]];
        assert_eq!(unpaired(&poly, &[&tris], 2), None);
    }

    #[test]
    fn a_boundary_the_second_face_renumbered_is_caught() {
        // RED FIRST. The second face emits the same chord points under
        // its own ids (3, 4, 5) instead of the shared 0, 1, 2 — the
        // cross-face identification failure. Both patches stay
        // internally consistent, so nothing per-patch can see it.
        let a = [[0, 1, 9], [1, 2, 9]];
        let b = [[4, 3, 10], [5, 4, 10]];
        assert!(per_patch_max_use(&a) <= 2, "patch A is internally clean");
        assert!(per_patch_max_use(&b) <= 2, "patch B is internally clean");
        let poly: [&[u32]; 1] = [&[0, 1, 2]];
        let bad = unpaired(&poly, &[&a, &b], 9);
        assert!(
            matches!(bad, Some((_, 1))),
            "an unidentified shared boundary leaves each side's copy at ONE use, got {bad:?}"
        );
    }

    #[test]
    fn a_segment_no_face_emitted_at_all_is_caught() {
        // The other side of `n != 2`: a hole rather than a mismatch.
        let poly: [&[u32]; 1] = [&[0, 1]];
        assert_eq!(unpaired(&poly, &[&[]], 2), Some(((0, 1), 0)));
    }

    #[test]
    fn ids_at_or_above_the_mark_are_never_probed() {
        // The mark is what keeps the scan an integer compare on grid
        // edges. Below it the same pair IS probed and counted, so the
        // two halves of the row differ only in the mark.
        let poly: [&[u32]; 1] = [&[0, 1]];
        let tris = [[0, 1, 9]];
        assert_eq!(unpaired(&poly, &[&tris], 1), Some(((0, 1), 0)));
        assert_eq!(unpaired(&poly, &[&tris], 2), Some(((0, 1), 1)));
    }
}

// NOT GATED ON `debug_assertions`: the subject is the fold itself,
// which runs in every profile.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod patch_tests {
    //! Placing a patch: shared corners keep their mesh ids, local ones
    //! take the base, and the interior lands at the base.
    //!
    //! The 20-body digests in `tests/d9_mesh_goldens.rs` are the
    //! contract; this row is the arithmetic under them, so a wrong base
    //! names itself here instead of moving 14 digest slots.

    use super::{Patch, PatchVertex};
    use geom_core::Point3;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    #[test]
    fn a_placed_patch_keeps_shared_ids_and_offsets_local_ones() {
        // Two faces into one arena that already holds 5 shared points.
        let mut positions: Vec<Point3<f64>> = (0..5).map(|i| p(f64::from(i))).collect();

        let first = Patch {
            interior: vec![p(100.0), p(101.0)],
            triangles: vec![
                [
                    PatchVertex::Shared(3),
                    PatchVertex::Local(0),
                    PatchVertex::Local(1),
                ],
                [
                    PatchVertex::Shared(0),
                    PatchVertex::Shared(4),
                    PatchVertex::Local(1),
                ],
            ],
        };
        assert_eq!(
            first.place(&mut positions),
            vec![[3, 5, 6], [0, 4, 6]],
            "the first face's interior starts at the arena's length, 5"
        );
        assert_eq!(positions.len(), 7, "its two interior points were appended");
        assert_eq!(positions[5].x, 100.0, "local 0 is the point at base + 0");

        let second = Patch {
            interior: vec![p(200.0)],
            triangles: vec![[
                PatchVertex::Local(0),
                PatchVertex::Shared(1),
                PatchVertex::Shared(2),
            ]],
        };
        assert_eq!(
            second.place(&mut positions),
            vec![[7, 1, 2]],
            "the second face's base is the arena AFTER the first face's interior"
        );
        assert_eq!(positions[7].x, 200.0);
    }

    #[test]
    fn a_patch_with_no_interior_is_placed_unchanged() {
        // The planar lane's shape: every corner shared, nothing
        // appended, so the base is unobservable.
        let mut positions: Vec<Point3<f64>> = (0..4).map(|i| p(f64::from(i))).collect();
        let patch = Patch {
            interior: Vec::new(),
            triangles: vec![[
                PatchVertex::Shared(0),
                PatchVertex::Shared(2),
                PatchVertex::Shared(3),
            ]],
        };
        assert_eq!(patch.place(&mut positions), vec![[0, 2, 3]]);
        assert_eq!(positions.len(), 4);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn census_ids_fixes_the_shared_ids_and_separates_the_local_ones() {
        // The census's premise: the renumbering is injective AND
        // leaves every shared id where it was, so a set of shared ids
        // still names the same corners in the renumbered patch.
        let shared: Vec<Point3<f64>> = (0..3).map(|i| p(f64::from(i))).collect();
        let patch = Patch {
            interior: vec![p(9.0), p(10.0)],
            triangles: vec![[
                PatchVertex::Shared(2),
                PatchVertex::Local(0),
                PatchVertex::Local(1),
            ]],
        };
        assert_eq!(patch.census_ids(&shared), vec![[2, 3, 4]]);
    }

    mod side_census {
        //! The side census where the patches join: a side of more than two
        //! triangles per chord carrying it refuses, on every profile.

        use super::super::*;

        fn overused(
            poly: &[&[u32]],
            patches: &[&[[u32; 3]]],
            mark: u32,
        ) -> Option<((u32, u32), usize)> {
            overused_side(&side_uses(poly, patches.iter().copied(), mark))
        }

        #[test]
        fn two_faces_using_one_chord_of_a_chain_as_an_interior_edge_overuse_it() {
            // Chain 0-1-2-3 between two faces, each with no interior point:
            // both triangulate the four points with the diagonal 0-2, which
            // each patch uses twice, as any interior edge. Every chord
            // segment is used once a side.
            let a = [[0, 1, 2], [0, 2, 3]];
            let b = [[2, 1, 0], [3, 2, 0]];
            let poly: [&[u32]; 1] = [&[0, 1, 2, 3]];
            assert_eq!(overused(&poly, &[&a, &b], 4), Some(((0, 2), 4)));
            // With the second face on the other diagonal, every side is two
            // triangles'.
            let c = [[3, 1, 0], [3, 2, 1]];
            assert_eq!(overused(&poly, &[&a, &c], 4), None);
        }

        #[test]
        fn two_coincident_chords_on_one_id_pair_are_not_overused_at_four_uses() {
            let poly: [&[u32]; 2] = [&[0, 1], &[0, 1]];
            let tris = [[0, 1, 9], [1, 0, 10], [0, 1, 11], [1, 0, 12]];
            assert_eq!(overused(&poly, &[&tris], 2), None);
            let five = [[0, 1, 9], [1, 0, 10], [0, 1, 11], [1, 0, 12], [0, 1, 13]];
            assert_eq!(overused(&poly, &[&five], 2), Some(((0, 1), 5)));
        }

        /// **A body's own patches join; with one face's triangles copied
        /// into another patch they do not.** The cube's mesh as `tessellate`
        /// makes it, its patches handed to the join with a second copy of the
        /// first face's triangles in the second face's patch: each side of
        /// those triangles is then a side of more triangles than its chords
        /// allow.
        #[test]
        fn a_mesh_whose_patches_overuse_a_side_does_not_join() {
            let tol = geom_core::Tol::witness();
            let body = sweep::test_support::cube::<f64>(1.0, tol);
            let mesh = crate::tessellate(&body, 0.1, tol).expect("the cube meshes");
            let shared_below = 1 + mesh
                .boundaries
                .iter()
                .flat_map(|b| b.points.iter().copied())
                .max()
                .expect("the cube has edges");
            let faces = [mesh.patches[0].face, mesh.patches[1].face];
            let mut overused = mesh.clone();
            let copy = overused.patches[0].triangles.clone();
            overused.patches[1].triangles.extend(copy);
            assert!(
                joined(mesh, shared_below).is_ok(),
                "the cube's own patches join"
            );
            match joined(overused, shared_below) {
                Err(TessellateError::OverusedSide { faces: got, uses }) => {
                    assert!(
                        faces.iter().all(|f| got.contains(f)),
                        "the patch copied from and the one copied into: {got:?}"
                    );
                    assert!(uses > 2, "{uses} uses");
                }
                other => panic!("a copied face's sides refuse OverusedSide, got {other:?}"),
            }
        }

        #[test]
        fn a_side_with_a_private_end_is_never_counted() {
            // Three triangles on 0-9 are one patch's own business at mark 9,
            // and an overuse at mark 10.
            let tris = [[0, 9, 1], [9, 0, 2], [0, 9, 3]];
            assert_eq!(overused(&[], &[&tris], 9), None);
            assert_eq!(overused(&[], &[&tris], 10), Some(((0, 9), 3)));
        }
    }
}

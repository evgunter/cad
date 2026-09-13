//! The [`tessellate`] entry point: δ validation, deterministic mesh
//! vertex minting, the chord pass, and per-face dispatch.

use std::collections::HashMap;

use geom::Surface;
use geom_core::k_stats::{self, Detached};
use geom_core::{Band, Point3, Tol};
use topo::{Body, FaceKey};

use rayon::prelude::*;

use crate::budget;
use crate::chords::{compute_chords, edge_vertices};
use crate::curved::tessellate_curved;
use crate::memo::{FaceInputs, FaceMemo, PatchKeys, PatchMemo, Placement};
use crate::nurbs_cert::FaceBounds;
use crate::planar::tessellate_planar;
use crate::sizing::{Eps, SizingTols, sizing_target};
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
/// chordal tolerance `chordal` (δ, meters) of its exact surfaces.
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
/// `Nurbs` placeholders, certificate failures) and is otherwise
/// garbage-in/garbage-out on the mesh *values*.
/// [`crate::validate::check_mesh`] is the backstop for that, and
/// **this function does not call it**: it is available to a caller,
/// and the acceptance suites run it, but nothing on this path does.
///
/// # Errors
///
/// [`TessellateError`] (closed enum): invalid δ, the `Nurbs`
/// placeholder, described NURBS faces outside the trimmed-NURBS
/// inventory (illegal-rational / C⁰-creased — `nurbs_cert`), unsupported
/// carriers, rings on curved faces, a curved face whose iso domain is
/// not its own UV rectangle, empty loops, dangling keys, resolution
/// overflow, certificate failure, CDT insertion failure.
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
/// The chord pass runs as in [`tessellate`] (it is per edge and
/// cheap); per face the lane is skipped on a memo hit and the stored
/// patch is placed by the same fold. The mesh is byte-identical to
/// [`tessellate`]'s: a hit is by the full key bytes, and the key is
/// every input the lane reads (`crate::memo`'s module docs state it
/// per lane). What a hit does NOT run is the lane's own per-patch
/// census and, with the `budget` feature, its recording; the
/// cross-face census at the end runs either way.
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
    /// `trimmed::tessellate_trimmed`: the pcurve-driven lane, on a
    /// cylinder chart with a trim carrier or on any NURBS chart.
    Trimmed,
}

/// The dispatch.
///
/// Described NURBS faces route through the trimmed lane
/// unconditionally (M7 — the flip of the historical first-arm
/// refusal, whose record is on
/// [`TessellateError::UnsupportedSurface`]): a NURBS face has no
/// swept-rectangle chart, so the pcurve-driven walk is its only lane.
/// The placeholder still refuses typed inside the lane;
/// illegal-rational/C⁰ classes refuse
/// [`TessellateError::UnsupportedNurbsFace`] there too. An
/// approximating surface meshes through the SAME lane, on its fit: the
/// fit is the geometry, so the triangles it produces are the face's
/// own. The certificate's bound is deliberately NOT folded into the
/// mesh tolerance — widening `tol` by the fit's ε so the mesh
/// certifies against the DESCRIPTION is a separate statement, and this
/// pass makes the plain one.
///
/// Structural routing (M5 PR 11): a conic/B-spline trim carrier means
/// the face is not an iso-rectangle — the pcurve-driven trimmed lane
/// takes it. The converse does NOT follow: this is a test on carrier
/// KINDS, and iso carriers (`Line`, `Circle`) can bound a
/// NON-rectangular domain — a keyway or milled flat on a cylinder is
/// exactly that shape, and nothing on this path screens loop SHAPE. So
/// an iso boundary reaching `tessellate_curved` is a routing decision,
/// not a guarantee about the domain; the domain itself is checked
/// there, twice over — its SHAPE through props' iso-rectangle door
/// before the walk (`curved::require_iso_rectangle_face`, refusing
/// [`TessellateError::UnsupportedCurvedShape`]) and the walk's
/// consistency after it (`curved::require_swept_rectangle`, refusing
/// [`TessellateError::UnsupportedCurvedDomain`]).
fn lane_of(body: &Body<f64>, fk: FaceKey, surface: &Surface<f64>) -> Result<Lane, TessellateError> {
    Ok(match *surface {
        Surface::Nurbs(_) | Surface::Approx(_) => Lane::Trimmed,
        Surface::Plane { .. } => Lane::Planar,
        _ if crate::trimmed::has_trim_carrier(body, fk)? => Lane::Trimmed,
        _ => Lane::Curved,
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
    // `Band::linear` prescribes; the curved lane's shape door is its
    // only consumer (`SizingTols::band`). Eager on purpose: a run whose
    // ε cannot form a band refuses every body alike, the all-planar
    // ones included, rather than meshing until the first curved face
    // (`TessellateError::Band` says why).
    let band = Band::linear(tol).map_err(|error| TessellateError::Band { error })?;
    let delta_s = sizing_target(chordal);

    // Mesh vertex ids: topology vertices first, arena order (D9).
    let mut positions = Vec::new();
    let mut vids = HashMap::new();
    for (vk, v) in body.vertices() {
        let p = body
            .get_point(v.point)
            .ok_or(TessellateError::MissingEntity {
                what: "vertex point",
            })?;
        #[allow(clippy::cast_possible_truncation)]
        vids.insert(vk, positions.len() as u32);
        positions.push(*p);
    }

    // Certified whole-patch NURBS bounds, assembled once per face and
    // shared by both passes that need them (`chords::FaceBounds`).
    let mut bounds = FaceBounds::new();

    // Chord pass: per-edge polylines, computed once (crate docs);
    // `chord_ts` is the matching parameter schedule (the trimmed lane
    // evaluates pcurves on it — one derivation, both consumers).
    let chords = compute_chords(body, delta_s, &vids, &mut positions, &mut bounds)?;
    // Every id a face can SHARE with another face is now minted:
    // topology vertices, then chord points, then (per face, below) that
    // face's own interior grid. So a shared id is exactly an id below
    // this mark — which is what the census at the end of this function
    // tests with an integer compare rather than a lookup, and what
    // makes `positions[..shared_below]` the whole of what a face reads
    // from outside itself.
    let shared_below = positions.len();
    let mut boundaries = Vec::new();
    for (ek, _) in body.edges() {
        let (start_vertex, end_vertex) = edge_vertices(body, ek)?;
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
    // What a face reads is `positions[..shared_below]` and nothing else
    // it does not own (above), the chord pass has finished writing
    // `bounds` (`nurbs_cert::FaceBounds`), and the memo is read through
    // its pure half (`PatchMemo::lookup`).
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
        delta_s,
        eps,
        band,
    };
    let faces: Vec<_> = body.faces().collect();
    // The meter's arming as a value: a lane runs on a worker thread,
    // which nobody armed (`budget`'s module docs).
    let arming = budget::arming();
    let reader = memo.as_deref();
    let work: Vec<FaceWork<'_>> = {
        let shared = &positions[..shared_below];
        faces
            .par_iter()
            .map(|&(fk, face)| {
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
                // NO PORTABILITY GATE, unlike `topo::props`' face walk,
                // and the reason is this crate's scalar policy: `mesh`
                // takes `&Body<f64>` and instantiates nothing else, so
                // a lane's decisions never reach `Sym::sign_within` and
                // never consult a symbolic session. What a session
                // makes non-portable there — the decision itself, its
                // receipt, the shape report — cannot arise here.
                let (((memo_work, place), meter), decided) = k_stats::detached(|| {
                    budget::record(arming, || {
                        let Some(surface) = body.get_surface(face.surface) else {
                            return (
                                None,
                                Err(TessellateError::MissingEntity {
                                    what: "face surface",
                                }),
                            );
                        };
                        let lane = match lane_of(body, fk, surface) {
                            Ok(lane) => lane,
                            Err(error) => return (None, Err(error)),
                        };
                        // The lanes, each over `shared` — the whole of what
                        // a face reads from outside itself.
                        //
                        // The planar lane derives its chart frame from the
                        // face's own boundary (planar.rs module docs, #284)
                        // — the stored plane axes are deliberately not
                        // passed: imported axes carry translator noise that
                        // projects valid boundaries below spade's
                        // coordinate domain.
                        let run = || match lane {
                            Lane::Trimmed => crate::trimmed::tessellate_trimmed(
                                body, fk, surface, &chords, shared, &tol, &bounds,
                            ),
                            Lane::Planar => tessellate_planar(body, fk, &chords.ids, shared),
                            Lane::Curved => {
                                tessellate_curved(body, fk, surface, &chords.ids, shared, &tol)
                            }
                        };
                        let Some(memo) = reader else {
                            return (None, run().map(Placement::Lane));
                        };
                        let inputs = match FaceInputs::gather(
                            body, fk, lane, surface, &chords, shared, chordal, ambient,
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
            .collect()
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
    for face in work {
        // Both channels, in arena order, up to AND INCLUDING the face
        // that refuses: the serial walk recorded whatever the refusing
        // face decided before it refused, and nothing after it.
        k_stats::splice(face.decided);
        budget::absorb(face.meter);
        pending.push(face.memo);
        match face.place {
            // Each face's interior takes the arena as it stands at that
            // face's turn, in face-arena order (D9).
            Ok(placement) => patches.push(FacePatch {
                face: face.face,
                triangles: placement.place(&mut positions),
            }),
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

    let mesh = Mesh {
        positions,
        patches,
        boundaries,
    };

    // D2 addendum row 5, and the CROSS-FACE half of the class
    // `curved`'s per-patch re-derivation cannot see: that census reads
    // ONE patch's identified edges, so a boundary the two adjacent
    // faces failed to identify with each other is outside its
    // footprint by construction (issue 897 says so, and it is right).
    // Re-derive it here, over the only ids two faces can share — the
    // chord segments of the body's own edges — and over nothing else.
    //
    // WHY NOT `check_mesh`, which is the oracle for the non-manifold
    // shape of this class (though not for every shape a collapsed walk
    // produces: a face whose polygon collapses onto one rim level
    // emits NO triangles, its chord segments are used by no face, and
    // `check_mesh` passes the empty patch — the oblique lens with
    // debug assertions off; this census is what sees it): it was the
    // first candidate and it was MEASURED against this one.
    //
    // THE PRICE ARGUMENT IS NARROWER THAN IT LOOKS, and is stated at
    // its real width. On sub-millisecond bodies the round-to-round
    // spread swamps both columns, and a reading there is not evidence
    // either way. The rows that decide are the donut's — 648 to
    // 16 080 triangles over δ = 0.1 to 0.004, dev profile with this
    // crate at opt-level 2, median of four warm rounds whose spread
    // is under 2 %: `check_mesh` costs 7 % to 8 % of `tessellate`,
    // this census 0.1 % to 0.4 %. That gap is the price argument, and
    // it is the whole of it.
    //
    // The rest is FOOTPRINT, which does not depend on the clock:
    // `check_mesh` censuses every edge of every patch — overwhelmingly
    // patch-interior grid edges that no cross-face question is about —
    // and re-checks winding and degeneracy, which are other rows'
    // classes. This census reads the chord segments and nothing else,
    // so its footprint IS the class:
    // an unidentified shared boundary makes each side's copy a
    // one-use edge, which is what `n != 2` catches. The narrower guard
    // is not a second copy of the oracle; it is the class's own
    // question, and `check_mesh` remains available to a caller.
    #[cfg(debug_assertions)]
    {
        let polylines: Vec<&[u32]> = chords.ids.values().map(Vec::as_slice).collect();
        let patch_triangles: Vec<&[[u32; 3]]> = mesh
            .patches
            .iter()
            .map(|p| p.triangles.as_slice())
            .collect();
        #[allow(clippy::cast_possible_truncation)]
        let bad = unpaired_chord_segment(&polylines, &patch_triangles, shared_below as u32);
        debug_assert!(
            bad.is_none(),
            "chord segment {:?} is used by {} face triangles rather than 2: the \
             faces meeting on that edge did not identify it (issue 897)",
            bad.map(|(e, _)| e),
            bad.map_or(0, |(_, n)| n)
        );
    }

    Ok((mesh, keys))
}

/// The chord segment that is NOT used by exactly two face triangles,
/// if any — the cross-face identification re-derivation (issue 897).
///
/// Every edge of the body carries a chord polyline whose segments the
/// two faces meeting on that edge both insert as CDT constraints, so
/// in a watertight emission each segment is a triangle edge exactly
/// twice: once per side, or twice within one patch where a `Seam` edge
/// is traversed both ways by the same face. A count of 1 is the class
/// this guard exists for — the two sides emitted the segment under
/// DIFFERENT ids, so neither copy pairs up.
///
/// `shared_below` is the first id minted after the chord pass. Ids are
/// minted topology-vertices-then-chords-then-per-face-grid (D9's
/// determinism order, at the top of [`tessellate`]), so an id at or
/// above the mark is one face's private grid point and can never be a
/// chord segment endpoint. Testing that first is what keeps this scan
/// an integer compare on the overwhelming majority of triangle edges
/// rather than a map probe.
///
/// **PRECONDITION: the body is CLOSED, and that is the caller's, not
/// this census's.** [`tessellate`]'s contract says the input is a
/// closed solid at rest and that it does not re-validate; it never
/// calls `topo::validate_closed`. On an OPEN body — a tier-1-legal
/// scaffolding strut, say, which `topo::validate` accepts and
/// `validate_closed` rejects — a chord polyline exists that no face
/// triangle can use twice, and this census reports it. That firing is
/// a broken PRECONDITION, not the D2-row-5 kernel bug the assert is
/// worded for, and it is the one way the guard can be reached by input
/// rather than by defect. It stays a `debug_assert` on that basis: the
/// precondition is documented at the door, an open body is already
/// outside what `tessellate` promises anything about, and no shipped
/// build is made to panic by it that was not already garbage-in.
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
///
/// **This reads no tolerance.** It is a census of ids and counts;
/// `Eps` has no role in it, and a band would be the wrong instrument
/// for a question whose answer is an integer.
#[cfg(debug_assertions)]
fn unpaired_chord_segment(
    polylines: &[&[u32]],
    patch_triangles: &[&[[u32; 3]]],
    shared_below: u32,
) -> Option<((u32, u32), usize)> {
    let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
    for ids in polylines {
        for w in ids.windows(2) {
            uses.insert(crate::walk::edge_key(w[0], w[1]), 0);
        }
    }
    for t in patch_triangles.iter().copied().flatten() {
        for k in 0..3 {
            let (a, b) = (t[k], t[(k + 1) % 3]);
            if a < shared_below
                && b < shared_below
                && let Some(n) = uses.get_mut(&crate::walk::edge_key(a, b))
            {
                *n += 1;
            }
        }
    }
    uses.iter().find(|&(_, &n)| n != 2).map(|(&e, &n)| (e, n))
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

    /// Every edge use within one patch, so a row can show the patch is
    /// internally clean while the pair of patches is not.
    fn per_patch_max_use(tris: &[[u32; 3]]) -> usize {
        let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
        for t in tris {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
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
            unpaired_chord_segment(&poly, &[&tris], 3),
            None,
            "two faces that identified the boundary leave every segment at two uses"
        );
    }

    #[test]
    fn a_seam_edge_traversed_twice_by_one_face_pairs_up() {
        // The full-2π case: ONE patch supplies both uses. The census
        // counts uses, not sides, which is what makes this legal.
        let poly: [&[u32]; 1] = [&[0, 1]];
        let tris = [[0, 1, 9], [1, 0, 10]];
        assert_eq!(unpaired_chord_segment(&poly, &[&tris], 2), None);
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
        let bad = unpaired_chord_segment(&poly, &[&a, &b], 9);
        assert!(
            matches!(bad, Some((_, 1))),
            "an unidentified shared boundary leaves each side's copy at ONE use, got {bad:?}"
        );
    }

    #[test]
    fn a_segment_no_face_emitted_at_all_is_caught() {
        // The other side of `n != 2`: a hole rather than a mismatch.
        let poly: [&[u32]; 1] = [&[0, 1]];
        assert_eq!(unpaired_chord_segment(&poly, &[&[]], 2), Some(((0, 1), 0)));
    }

    #[test]
    fn ids_at_or_above_the_mark_are_never_probed() {
        // The mark is what keeps the scan an integer compare on grid
        // edges. Below it the same pair IS probed and counted, so the
        // two halves of the row differ only in the mark.
        let poly: [&[u32]; 1] = [&[0, 1]];
        let tris = [[0, 1, 9]];
        assert_eq!(
            unpaired_chord_segment(&poly, &[&tris], 1),
            Some(((0, 1), 0))
        );
        assert_eq!(
            unpaired_chord_segment(&poly, &[&tris], 2),
            Some(((0, 1), 1))
        );
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
}

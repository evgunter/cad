//! `shell` — hollowing, sealed and opened.
//!
//! **Shelling** turns a solid into a thin-walled one: the boundary is
//! offset inward by the wall thickness, and the hollow is either kept
//! closed (a cavity) or opened by designating faces whose material is
//! removed, leaving annular rims where the wall's thickness shows
//! (`docs/OFFSET-DESIGN.md`'s vocabulary, unchanged).
//!
//! # The sealed arm, and what it deliberately does not run
//!
//! `shell(body, t)` is `body − offset_inward(body, t)` BY DEFINITION,
//! and that definition is boolean-family. Its EXECUTION is not: when
//! every face's offset certifies, the two boundaries provably do not
//! cross, so SSI, the crossing census and the classification walk have
//! nothing to do — and worse, the general path's containment
//! examination is extent-box coarse, so routing through it would refuse
//! bodies the construction has already proven nested. The sealed shell
//! therefore executes as the boolean's DEGENERATE NO-CROSSING ARM:
//!
//! 1. one clone, every boundary face replaced by its inward offset
//!    ([`crate::replace_face_offset`] per face) — the result is the
//!    material to remove, a positively oriented closed body;
//! 2. that body inserted through the shared void-insertion door
//!    ([`crate::boolean::voids::insert_void`]) with carried evidence;
//! 3. one validation.
//!
//! Cost is offset mint + certification + one structural insertion. No
//! SSI runs, and that is pinned structurally rather than asserted in
//! prose (`shell_runs_no_intersection_machinery`).
//!
//! # The evidence, and where it comes from
//!
//! The void door never derives containment. What this verb carries to
//! it is the offset construction's OWN decides: a face's inward offset
//! mints only when its d-vs-reach margin is certifiably positive (the
//! realized radius for a cylinder, sphere or torus tube; the
//! apex-window margin for a cone; unbounded for a plane), and those are
//! exactly the margins that say the offset surface has not reached the
//! spine it would fold on. Every face minting is therefore the
//! construction's own strict-inside claim, and it is passed verbatim as
//! [`VoidContainment::Carried`] with [`Sign::Positive`] — the RING
//! pattern, one dimension up from a profile's hole-inside-outer margin.
//!
//! **What that carries, stated exactly.** It is a per-face (LOCAL)
//! reach claim. It does not certify that two non-adjacent walls stay
//! clear of each other — a slab shelled by more than half its own
//! thickness has every per-face margin positive and no room. The
//! cavity's own tier-3 validation is what catches that class here, and
//! the general clearance margin over a parameter box is M10's
//! machinery, not this verb's; the gap is named rather than papered.
//!
//! # The opened arm
//!
//! `shell_open(body, t, open_faces)` is the sealed construction plus
//! rim surgery, and it composes rather than inventing:
//!
//! 1. the sealed shell, exactly as above — so the evidence handed to
//!    the void door is the strict one, before anything is opened;
//! 2. per designated face, its CAVITY counterpart offset back OUTWARD
//!    by `t` ([`crate::replace_face_offset`] again), which lands it on
//!    the designated face's own surface and — because the door
//!    re-describes a moved face's boundary against its untouched
//!    neighbours — extends the cavity's side walls up to meet it;
//! 3. `kfmrh` on the pair: the cavity counterpart dies, its outer loop
//!    becomes a RING of the designated face, and the cavity shell fuses
//!    into the outer one.
//!
//! The result is a CLOSED thin solid with one shell: the designated
//! face is now annular — the rim, where the wall thickness shows. Genus
//! rises and nothing opens, so D1's manifold-first stance is untouched
//! and every tier-1/2 invariant holds. The surgery is the ring-topology
//! band precedent verbatim; no new machinery, and in particular no
//! ladder of quads (which would chamfer the opening rather than rim
//! it — the geometry that made step 2 necessary).

use geom_core::{Band, Decide, Real, Sign, Tol};

use crate::body::Body;
use crate::boolean::voids::{VoidContainment, VoidEvidence, VoidInsertError, insert_void};
use crate::entity::{EntityId, FaceKey, LoopBoundary, ShellKey, SolidKey};
use crate::euler::EulerOpError;
use crate::props::PropsQuadLane;
use crate::replace_face::ReplaceFaceError;
use crate::validate::{ValidationError, validate_geometric};

/// Typed refusal of the shell verb (closed enum, D4 ¶3).
#[derive(Clone, Debug)]
pub enum ShellError<T: Real> {
    /// The wall thickness is not certifiably positive: a zero or
    /// negative wall is not a thin solid, and the ambiguity band
    /// escalates rather than guessing.
    Thickness {
        /// The thickness as given, echoed as data.
        thickness: T,
    },
    /// The body is not a single solid. Shelling is a per-solid verb and
    /// the void door inserts into ONE solid; a multi-solid operand
    /// would need a designation this verb does not take.
    NotOneSolid {
        /// How many solids the body carries.
        solids: usize,
    },
    /// A face's inward offset refused. This is the validity gate AND
    /// the evidence in one: the margin that says the offset has not
    /// reached its own reach is the margin that says the cavity is
    /// strictly inside.
    Face {
        /// The face whose offset refused.
        face: FaceKey,
        /// The face-replacement door's typed refusal, verbatim.
        error: Box<ReplaceFaceError<T>>,
    },
    /// A designated open face does not resolve in the operand.
    OpenFaceStale {
        /// The unresolvable designation.
        face: FaceKey,
    },
    /// A face was designated open twice.
    OpenFaceRepeated {
        /// The repeated designation.
        face: FaceKey,
    },
    /// Every face of a shell was designated open: there would be no
    /// wall left to show a thickness, and no boundary to rim.
    OpenFacesExhaustShell {
        /// The shell whose faces were all designated.
        shell: ShellKey,
    },
    /// Removing the designated faces disconnects a shell's boundary:
    /// the remaining faces fall into more than one component, so the
    /// rims would bound separate pieces rather than one thin solid.
    OpenFacesDisconnect {
        /// The shell the designation cut in two.
        shell: ShellKey,
        /// How many components the remainder falls into.
        components: usize,
    },
    /// A designated face is not planar. Its rim would be a CURVED face
    /// carrying a ring loop, which the closed-form property inventory
    /// has no reading for (the same kernel-wide limitation the fillet
    /// band's ring-free annulus works around). Refused rather than
    /// built into a body whose volume cannot be computed.
    OpenFaceRingUnsupported {
        /// The designated face.
        face: FaceKey,
        /// Its surface kind.
        kind: geom_brep::SurfaceKind,
    },
    /// The void-insertion door refused.
    Insert {
        /// The door's typed refusal, verbatim.
        error: VoidInsertError,
    },
    /// The rim surgery's Euler step refused.
    Rim {
        /// The designated face whose rim could not be minted.
        face: FaceKey,
        /// The operator's typed refusal.
        error: EulerOpError,
    },
    /// The body's referential coherence broke mid-construction — a
    /// kernel bug, surfaced rather than swallowed.
    Corrupt {
        /// The key that stopped resolving.
        key: EntityId,
    },
    /// The assembled result does not validate, so it is discarded.
    NotValid {
        /// The validator's report.
        errors: Vec<ValidationError>,
    },
}

impl<T: Real> core::fmt::Display for ShellError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Thickness { thickness } => write!(
                f,
                "shell: the wall thickness ({thickness:?} m) is not certifiably positive, so \
                 there is no thin solid to build"
            ),
            Self::NotOneSolid { solids } => write!(
                f,
                "shell: the operand carries {solids} solids — shelling is a per-solid verb and \
                 this door takes no designation to pick one"
            ),
            Self::Face { face, error } => write!(
                f,
                "shell: {face:?}'s inward offset refused, which is both the validity gate and \
                 the containment evidence: {error}"
            ),
            Self::OpenFaceStale { face } => {
                write!(
                    f,
                    "shell: the designated open face {face:?} does not resolve"
                )
            }
            Self::OpenFaceRepeated { face } => {
                write!(f, "shell: {face:?} was designated open twice")
            }
            Self::OpenFacesExhaustShell { shell } => write!(
                f,
                "shell: every face of {shell:?} was designated open — nothing would be left to \
                 carry a wall thickness, so there is no rim to mint"
            ),
            Self::OpenFacesDisconnect { shell, components } => write!(
                f,
                "shell: removing the designated faces leaves {shell:?}'s boundary in \
                 {components} components — the rims would bound separate pieces rather than one \
                 thin solid"
            ),
            Self::OpenFaceRingUnsupported { face, kind } => write!(
                f,
                "shell: the designated face {face:?} carries a {kind:?}, and its rim would be a \
                 curved face with a ring loop — a shape the closed-form property inventory has \
                 no reading for, so nothing is built"
            ),
            Self::Insert { error } => {
                write!(f, "shell: the void-insertion door refused: {error}")
            }
            Self::Rim { face, error } => {
                write!(f, "shell: the rim surgery on {face:?} refused: {error}")
            }
            Self::Corrupt { key } => write!(
                f,
                "shell: {key:?} stopped resolving mid-construction (kernel bug)"
            ),
            Self::NotValid { errors } => write!(
                f,
                "shell: the assembled thin solid is not valid ({} errors); it is discarded",
                errors.len()
            ),
        }
    }
}

impl<T: Real> std::error::Error for ShellError<T> {}

// ---------------------------------------------------------------------
// The verb
// ---------------------------------------------------------------------

/// The sealed hollow: every boundary face replaced by its inward
/// offset, the offset boundary inserted as a cavity (module docs).
///
/// `thickness` is the wall thickness in meters and is a MAGNITUDE —
/// each face's own offset direction comes from its orientation, so a
/// reversed face offsets against its chart normal without the caller
/// knowing which faces those are.
///
/// # Errors
///
/// [`ShellError`] — the thickness gate, the per-face offset refusals
/// (which are the containment evidence's own decides), the void door's
/// refusals, and a result that does not validate.
pub fn shell<T: Decide + PropsQuadLane>(
    body: &Body<T>,
    thickness: T,
    tolerance: f64,
    band: Band,
    tol: Tol,
) -> Result<Body<T>, ShellError<T>> {
    shell_open(body, thickness, &[], tolerance, band, tol)
}

/// The opened hollow: [`shell`], then the designated faces re-authored
/// as annular rims (module docs). An empty `open_faces` is exactly
/// [`shell`].
///
/// The result is a CLOSED thin solid in every case — the designated
/// faces do not become holes, they become rims.
///
/// # Errors
///
/// [`ShellError`] — [`shell`]'s, plus the designation gates (a face
/// must resolve, be named once, leave a nonempty and connected
/// remainder) and the rim surgery's own refusal.
pub fn shell_open<T: Decide + PropsQuadLane>(
    body: &Body<T>,
    thickness: T,
    open_faces: &[FaceKey],
    tolerance: f64,
    band: Band,
    tol: Tol,
) -> Result<Body<T>, ShellError<T>> {
    // ---- Decide: the thickness. ----
    match crate::validate::decide("shell_thickness", geom_core::Margin::of(thickness), band) {
        Ok(Sign::Positive) => {}
        _ => return Err(ShellError::Thickness { thickness }),
    }

    // ---- Decide: one solid. ----
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let [solid] = solids[..] else {
        return Err(ShellError::NotOneSolid {
            solids: solids.len(),
        });
    };

    // ---- Decide: the designation. ----
    check_designation(body, open_faces)?;

    // ---- The cavity: one clone, every CHART inward. ----
    //
    // By chart, not by face: a full revolve splits its wall into two
    // bands over one cylinder, and such a surface has to move as one
    // (the face-replacement door's own group form says why). Grouping
    // is by surface key, in face-arena order, so the walk is
    // deterministic.
    let charts = chart_groups(body);
    let mut cavity = body.clone();
    for group in &charts {
        let face = group[0];
        let d = inward(&cavity, face, thickness)?;
        crate::replace_faces_offset(&mut cavity, group, d, tolerance, band, tol).map_err(
            |error| ShellError::Face {
                face,
                error: Box::new(error),
            },
        )?;
    }

    // ---- The evidence: the construction's own decides, carried. ----
    //
    // Every face above minted, which is exactly to say every d-vs-reach
    // margin decided `Positive`. That IS the strict-inside claim (module
    // docs on what it carries and what it does not), so it is stated
    // once, per cavity shell, rather than re-derived by the door.
    let evidence = VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: Sign::Positive,
                    },
                )
            })
            .collect(),
    };

    // ---- The insertion (the degenerate no-crossing arm). ----
    let mut out = body.clone();
    // The cavity is a CLONE of the operand, so a designated face's
    // counterpart carries the same key in the cavity's key space —
    // which is the space `VoidInserted` maps from.
    let inserted = insert_void(&mut out, solid, cavity, &evidence, tol)
        .map_err(|error| ShellError::Insert { error })?;

    // ---- The rim surgery, per designated face. ----
    for &outer_face in open_faces {
        let cavity_face = outer_face;
        let rim_source = inserted.face(cavity_face).ok_or(ShellError::Corrupt {
            key: EntityId::Face(cavity_face),
        })?;
        // Lift the cavity's counterpart back onto the designated face's
        // own surface. The distance is read from the two PLANES rather
        // than negated from the way in: the graft's reversal negates a
        // stored plane normal (`revert`'s own contract), so "the way
        // back" is not the arithmetic negation of "the way in", and
        // deriving it from geometry is both shorter and sign-safe.
        let back = lift_to(&out, rim_source, outer_face)?;
        crate::replace_face_offset(&mut out, rim_source, back, tolerance, band, tol).map_err(
            |error| ShellError::Face {
                face: rim_source,
                error: Box::new(error),
            },
        )?;
        // Read AFTER the lift: `FaceSurface::New` minted a fresh key
        // for the moved chart, and that key — not the one the graft
        // brought in — is what the ring's descriptions now name.
        let dead_surface = out
            .get_face(rim_source)
            .ok_or(ShellError::Corrupt {
                key: EntityId::Face(rim_source),
            })?
            .surface;
        // The connected sum: the counterpart dies, its outer loop
        // becomes the designated face's RING, and the cavity shell
        // fuses into the outer one.
        let fused = out
            .kfmrh(outer_face, rim_source)
            .map_err(|error| ShellError::Rim {
                face: outer_face,
                error,
            })?;
        // The ring's edges still NAME the surface that just died. They
        // lie on the rim's surface now — the lift put the two planes on
        // top of each other — so the re-description is a key swap with
        // the carrier untouched, and the attach layer certifies it
        // against the geometry rather than taking the swap on trust.
        rename_ring_surface(&mut out, fused.ring, dead_surface, outer_face, tol)?;
    }

    // ---- One validation. ----
    validate_geometric(&out, tol).map_err(|errors| ShellError::NotValid { errors })?;
    Ok(out)
}

/// The signed distance along `from`'s chart normal that lands it on
/// `onto`'s plane. Both are planar — a curved designation is refused
/// upstream — so this is one dot product and no solve.
fn lift_to<T: Real>(body: &Body<T>, from: FaceKey, onto: FaceKey) -> Result<T, ShellError<T>> {
    let plane =
        |face: FaceKey| -> Result<(geom_core::Point3<T>, geom_core::Vec3<T>), ShellError<T>> {
            let data = body.get_face(face).ok_or(ShellError::Corrupt {
                key: EntityId::Face(face),
            })?;
            match body.get_surface(data.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => Ok((*origin, *normal)),
                _ => Err(ShellError::Corrupt {
                    key: EntityId::Face(face),
                }),
            }
        };
    let (o_from, n_from) = plane(from)?;
    let (o_onto, _) = plane(onto)?;
    Ok((o_onto - o_from).dot(n_from))
}

/// Re-points every description on `ring` that names `dead` at `rim`'s
/// surface, re-certifying each through the attach layer.
fn rename_ring_surface<T: Decide>(
    body: &mut Body<T>,
    ring: crate::entity::LoopKey,
    dead: crate::geometry::SurfaceKey,
    rim: FaceKey,
    tol: Tol,
) -> Result<(), ShellError<T>> {
    let corrupt = |key| ShellError::Corrupt { key };
    let live = body
        .get_face(rim)
        .ok_or_else(|| corrupt(EntityId::Face(rim)))?
        .surface;
    let LoopBoundary::Cycle { first } = body
        .get_loop(ring)
        .ok_or_else(|| corrupt(EntityId::Loop(ring)))?
        .boundary
    else {
        return Ok(());
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or_else(|| corrupt(EntityId::HalfEdge(first)))?;
    let mut specs = Vec::new();
    for he in cycle {
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| corrupt(EntityId::HalfEdge(he)))?
            .edge;
        let curve = body
            .get_curve_geom(
                body.get_edge(edge)
                    .ok_or_else(|| corrupt(EntityId::Edge(edge)))?
                    .curve,
            )
            .and_then(crate::null::CurveGeom::certified)
            .ok_or_else(|| corrupt(EntityId::Edge(edge)))?;
        let (param_start, param_end) = curve.params();
        specs.push((
            edge,
            geom_brep::EdgeCurveSpec {
                description: crate::replace_face::remap_description(
                    *curve.description(),
                    dead,
                    live,
                ),
                carrier: curve.carrier().clone(),
                param_start,
                param_end,
            },
        ));
    }
    for (edge, spec) in specs {
        body.set_edge_curve(edge, spec, tol)
            .map_err(|error| ShellError::Rim { face: rim, error })?;
    }
    Ok(())
}

/// The body's faces grouped by the surface they wear, in face-arena
/// order — the unit a chart moves in.
fn chart_groups<T: Real>(body: &Body<T>) -> Vec<Vec<FaceKey>> {
    let mut keys: Vec<crate::geometry::SurfaceKey> = Vec::new();
    let mut groups: Vec<Vec<FaceKey>> = Vec::new();
    for (key, face) in body.faces() {
        match keys.iter().position(|k| *k == face.surface) {
            Some(i) => groups[i].push(key),
            None => {
                keys.push(face.surface);
                groups.push(vec![key]);
            }
        }
    }
    groups
}

/// The signed offset distance that moves `face` INTO the material: the
/// chart normal points out of the solid on a positively-sensed face and
/// into it on a reversed one, so the caller's thickness magnitude never
/// has to know which is which.
fn inward<T: Real>(body: &Body<T>, face: FaceKey, thickness: T) -> Result<T, ShellError<T>> {
    let sense = body
        .get_face(face)
        .ok_or(ShellError::Corrupt {
            key: EntityId::Face(face),
        })?
        .sense;
    Ok(if sense { -thickness } else { thickness })
}

/// The designation gates: every named face resolves, is named once, and
/// leaves its shell with a nonempty, connected remainder.
fn check_designation<T: Real>(body: &Body<T>, open_faces: &[FaceKey]) -> Result<(), ShellError<T>> {
    for (i, face) in open_faces.iter().enumerate() {
        let Some(data) = body.get_face(*face) else {
            return Err(ShellError::OpenFaceStale { face: *face });
        };
        if open_faces[..i].contains(face) {
            return Err(ShellError::OpenFaceRepeated { face: *face });
        }
        let surface = body.get_surface(data.surface).ok_or(ShellError::Corrupt {
            key: EntityId::Face(*face),
        })?;
        if !matches!(surface, geom::Surface::Plane { .. }) {
            return Err(ShellError::OpenFaceRingUnsupported {
                face: *face,
                kind: geom_brep::SurfaceKind::of(surface),
            });
        }
    }
    if open_faces.is_empty() {
        return Ok(());
    }
    for (shell, data) in body.shells() {
        let remaining: Vec<FaceKey> = data
            .faces
            .iter()
            .copied()
            .filter(|f| !open_faces.contains(f))
            .collect();
        if remaining.is_empty() {
            return Err(ShellError::OpenFacesExhaustShell { shell });
        }
        let components = count_components(body, &remaining)?;
        if components != 1 {
            return Err(ShellError::OpenFacesDisconnect { shell, components });
        }
    }
    Ok(())
}

/// How many edge-adjacency components `faces` falls into — the
/// validator's own pass-11 relation, restricted to a subset.
fn count_components<T: Real>(body: &Body<T>, faces: &[FaceKey]) -> Result<usize, ShellError<T>> {
    let mut seen: Vec<FaceKey> = Vec::new();
    let mut components = 0usize;
    for seed in faces {
        if seen.contains(seed) {
            continue;
        }
        components += 1;
        let mut work = vec![*seed];
        seen.push(*seed);
        while let Some(face) = work.pop() {
            for neighbour in face_neighbours(body, face)? {
                if faces.contains(&neighbour) && !seen.contains(&neighbour) {
                    seen.push(neighbour);
                    work.push(neighbour);
                }
            }
        }
    }
    Ok(components)
}

/// The faces `face` shares an edge with.
fn face_neighbours<T: Real>(body: &Body<T>, face: FaceKey) -> Result<Vec<FaceKey>, ShellError<T>> {
    let corrupt = |key| ShellError::Corrupt { key };
    let data = body
        .get_face(face)
        .ok_or_else(|| corrupt(EntityId::Face(face)))?;
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let loop_data = body
            .get_loop(lk)
            .ok_or_else(|| corrupt(EntityId::Loop(lk)))?;
        let LoopBoundary::Cycle { first } = loop_data.boundary else {
            continue;
        };
        let cycle = body
            .loop_cycle(first)
            .ok_or_else(|| corrupt(EntityId::HalfEdge(first)))?;
        for he in cycle {
            let mate = body
                .mate(he)
                .ok_or_else(|| corrupt(EntityId::HalfEdge(he)))?;
            let mate_data = body
                .get_half_edge(mate)
                .ok_or_else(|| corrupt(EntityId::HalfEdge(mate)))?;
            let parent = body
                .get_loop(mate_data.parent_loop)
                .ok_or_else(|| corrupt(EntityId::Loop(mate_data.parent_loop)))?
                .face;
            if parent != face && !out.contains(&parent) {
                out.push(parent);
            }
        }
    }
    Ok(out)
}

//! Per-shell signed volume over the planar/line subset — the
//! outward-vs-void classifier for multi-shell solids (crate docs,
//! "Solids, shells, and voids").
//!
//! # Formula (divergence theorem, closed forms on the PLANAR subset)
//!
//! `V = (1/3) ∮ p·n dA`. On a planar face every point satisfies
//! `p·n̂ = o·n̂` with `o` the stored plane origin, so the face
//! contributes `(o·A⃗_f)/3` where `A⃗_f = (1/2) Σ (a−o)×(b−o)` over the
//! directed boundary segments `a→b` of all its loops (outer CCW +
//! rings CW as stored — rings subtract automatically). The per-segment
//! closed form `(a−o)×(b−o)/2` is mathematically exact for line
//! carriers, but the accumulation is **rounded f64 arithmetic** (bit
//! exact only when the inputs are dyadic, as in the test corpus); the
//! walk **verifies** every carrier is a line and every surface a
//! plane, refusing anything else as
//! [`StepExportError::CurvedShellClassification`] (never a
//! silently-approximated classification).
//!
//! Since M5 PR 13 the emitter prints the whole elementary-surface
//! subset, so this walk is now the NARROWER of the two: a curved face
//! reaching here is a *classifier* limit, not an export limit. The
//! reduction it performs — `p·n̂` constant over a face, so the surface
//! integral collapses to a boundary sum — is a planarity identity with
//! no curved-face counterpart in closed form, and the sign it produces
//! decides material vs void, so a numerically approximated stand-in
//! would be exactly the silent lie the refusal exists to prevent.
//! Only MULTI-shell solids reach this code (`writer.rs`), and the one
//! curved multi-shell body constructible at rest today is S12's
//! two-stub `boss ∖ plate` complement; every other curved body at rest
//! is single-shell and exports untouched by this module.
//!
//! # Orientation comes from the winding, not from the normal (S10)
//!
//! Note what the formula above does **not** read: the face's stored
//! surface normal. `A⃗_f` is accumulated from the loops' stored
//! traversal, and interior-left ties that traversal to the face's
//! OUTWARD normal — so `A⃗_f` is the outward-oriented area vector by
//! derivation. This matters since M5 S10: the stored chart normal is
//! NOT the outward normal in general (the outward normal is
//! `topo::Face::sense_sign() · chart_normal`), but the walk never
//! depended on it and needs no repair. `sense_sign` must NOT be
//! applied here: `revert`
//! reverses the loops and flips `sense` together, so multiplying would
//! negate the volume twice and misclassify exactly the reversed shells
//! it would have been added for (S10 category B — the same
//! disposition as `topo::props` and the tessellator's winding sites).
//!
//! # Why the sign read is safe (headroom, not exactness)
//!
//! With the loops obeying interior-left (M1 ratification), a shell
//! bounding material from outside integrates to `+enclosed volume`
//! and a void cavity wall (whose loops wind about normals pointing
//! into the cavity, away from material) integrates
//! to `−cavity volume`. The classification read is a plain f64 sign
//! comparison, **not a Q1 trilean** — an export-layer decision, made
//! safe by headroom rather than certification: f64 accumulation error
//! here is a few ulps per term (relative ~1e-16 of the coordinate
//! scale cubed), so flipping the sign of a genuinely enclosed volume
//! would need the shell's feature scale to be ≲ 1.7e-17 of its
//! coordinate scale — many orders of magnitude below the K·ε band any
//! Q1-certified construction enforces (at ε = 1e-6 the kernel refuses
//! to *build* features near 1e-5, long before this sum could dither).
//! The genuinely unclassifiable outcomes — an exactly-0.0 or
//! non-finite sum — refuse as
//! [`StepExportError::ShellVolumeIndeterminate`], and note that
//! single-shell solids never reach this classifier at all (lib docs:
//! tier-2 validity + the +V invariant own lone-shell orientation).

use geom::Curve3;
use geom::Surface;
use geom_core::{Point3, Vec3};
use topo::{Body, LoopBoundary, Shell, ShellKey};

use crate::StepExportError;
use crate::writer::{carrier_kind, certified_carrier, surface_kind};

/// The signed volume enclosed by `shell` (module docs).
///
/// # Errors
///
/// [`StepExportError`] — out-of-subset geometry
/// ([`StepExportError::CurvedShellClassification`]: since M5 PR 13 the
/// EMITTER prints the curved subset, so a curved face reaching this
/// walk is a classifier limit, not an export limit, and the message
/// says so), empty loops, null scaffolding, or unresolvable keys.
pub(crate) fn shell_signed_volume(
    body: &Body<f64>,
    shell_key: ShellKey,
    shell: &Shell,
) -> Result<f64, StepExportError> {
    let mut six_v = 0.0_f64;
    for &face_key in &shell.faces {
        let face = body.get_face(face_key).ok_or(StepExportError::Corrupt {
            what: "shell face key does not resolve",
        })?;
        let surface = body
            .get_surface(face.surface)
            .ok_or(StepExportError::Corrupt {
                what: "face surface key does not resolve",
            })?;
        let Surface::Plane { origin, .. } = *surface else {
            return Err(StepExportError::CurvedShellClassification {
                shell: shell_key,
                face: face_key,
                kind: surface_kind(surface),
            });
        };
        // 2·A⃗_f, accumulated in loop-storage order (D9: fixed order).
        // The face's S10 `sense` is deliberately absent: this vector is
        // winding-derived and therefore already outward-oriented for
        // either sense (module docs, the double-count hazard).
        let mut area2 = Vec3::zero();
        for &loop_key in std::iter::once(&face.outer).chain(face.rings.iter()) {
            let loop_ = body.get_loop(loop_key).ok_or(StepExportError::Corrupt {
                what: "face loop key does not resolve",
            })?;
            let first = match loop_.boundary {
                LoopBoundary::Empty { .. } => {
                    return Err(StepExportError::EmptyLoop { loop_: loop_key });
                }
                LoopBoundary::Cycle { first } => first,
            };
            let cycle = body.loop_cycle(first).ok_or(StepExportError::Corrupt {
                what: "loop cycle does not close",
            })?;
            for he_key in cycle {
                let he = body.get_half_edge(he_key).ok_or(StepExportError::Corrupt {
                    what: "half-edge key does not resolve",
                })?;
                // Verify the carrier is in the line subset — the
                // segment closed form below is exact only for lines.
                let edge = body.get_edge(he.edge).ok_or(StepExportError::Corrupt {
                    what: "half-edge edge key does not resolve",
                })?;
                let carrier = certified_carrier(body, he.edge, edge)?;
                if !matches!(carrier, Curve3::Line { .. }) {
                    return Err(StepExportError::CurvedShellClassification {
                        shell: shell_key,
                        face: face_key,
                        kind: carrier_kind(carrier),
                    });
                }
                let a = vertex_position(body, he.start)?;
                let end = body.half_edge_end(he_key).ok_or(StepExportError::Corrupt {
                    what: "half-edge end does not resolve",
                })?;
                let b = vertex_position(body, end)?;
                area2 = area2 + (a - origin).cross(b - origin);
            }
        }
        six_v += (origin - Point3::origin()).dot(area2);
    }
    Ok(six_v / 6.0)
}

/// The stored position of a vertex.
fn vertex_position(
    body: &Body<f64>,
    vertex: topo::VertexKey,
) -> Result<Point3<f64>, StepExportError> {
    let v = body.get_vertex(vertex).ok_or(StepExportError::Corrupt {
        what: "vertex key does not resolve",
    })?;
    body.get_point(v.point)
        .copied()
        .ok_or(StepExportError::Corrupt {
            what: "vertex point key does not resolve",
        })
}

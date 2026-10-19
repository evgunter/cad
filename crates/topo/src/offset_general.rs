//! `offset_surfaces_together` — the GENERAL simultaneous offset door:
//! every chart of a solid moved at once, whatever its surfaces.
//!
//! # Why one door for the whole solid
//!
//! A chart moved alone against held neighbours derives each of its
//! edges as the section of one moved surface with one HELD surface,
//! and composing that over every chart re-derives each edge again when
//! its neighbour moves. The intermediate section is not on the way to
//! the final one: beside a lofted wall it lies `d·cot φ` from the moved
//! fit's row (φ the interior dihedral), which changes sign at 90°, so
//! on a twisted wall it falls outside the moved fit's window along part
//! of a seam, and whether a body offsets at all depends on the order
//! its charts are taken and on intermediate bodies nobody asked for. At
//! an oblique corner the composition transports the corner once per
//! moving chart, which is no corner of the offset body at all
//! ([`crate::offset_planes_together`]'s arithmetic).
//!
//! This door mints every new surface first, and then:
//!
//! - **each edge is the section of its two MOVED surfaces** through the
//!   C5 table, the old carrier seeding the branch and its sense. It is
//!   transported where the pair's relative motion carries it rigidly:
//!   where both sides' offset actions put every point of it in the same
//!   place (a G1 pair whose normals and distances agree along the edge,
//!   two coplanar charts moved alike), and a seam a chart shares with
//!   itself moves with that chart;
//! - **each corner is a root of the moved surfaces meeting it**, under
//!   the pairwise-agreement rule, every surface standing as moved; a
//!   moved surface no candidate lies on refuses the corner, naming its
//!   vertex and an edge meeting it on that surface.
//!
//! The edges and corners are the ones [`crate::replace_faces_offset`]
//! derives for a single chart, read with every surface moved: one move
//! IS that door, and its body is this door's.
//!
//! # Its closed forms
//!
//! [`crate::offset_planes_together`] (every face a plane: Cramer at each
//! corner, each edge its line translated) and
//! [`crate::offset_charts_together`] (an axial solid: each corner in the
//! meridian half-plane) are this door's closed-form arms, and they stay
//! separate doors. Each is exact arithmetic on its own family, carries
//! an edge's conventional data through the move, and is the door its
//! solids have always gone through, so their outcomes and stored bits
//! are what `shell` has built on; the general door's sections would
//! re-mint every line's anchor and root every corner. `shell`'s ladder
//! sends a solid to them first and here only when it is neither.
//!
//! # Scope
//!
//! Every face of every SOLID the moves touch must be in the moving set,
//! the siblings' gate ([`ReplaceFaceError::TogetherPartialSet`]); a
//! chart named at a decided-zero distance holds and keeps its key. A
//! wall–wall seam between two moved fits is their NURBS × NURBS section
//! through the old seam's middle (`geom_brep::Section::branch_at`).
//!
//! # Charts, not material
//!
//! Each move's distance is along its chart's stored normal, as at the
//! siblings: a door stated against charts takes construction state, a
//! [`Body`] tier 2 in and tier 2 out (`crates/topo/README.md`, "Shell
//! and offset surgery").

use geom_core::{Band, Decide, Tol};

use crate::body::Body;
use crate::entity::FaceKey;
use crate::offset_together::ChartMove;
use crate::replace_face::{OffsetOutcome, ReplaceFaceError};

/// **Offset every chart of the solids `moves` touch, at once** (module
/// docs).
///
/// `band` is the run's band at `tol`, as the siblings take it; every
/// decision of the call classifies at it. The body is **untouched on every `Err`**: every surface, edge and
/// corner is decided read-only, the writes go to a clone, and the clone
/// is adopted only once it validates at tier 2. The door **ends with
/// the join** (`docs/DESIGN.md`, maximal edges).
///
/// # Errors
///
/// [`ReplaceFaceError`]: the move set's own gates (`EmptyGroup`,
/// `StaleFace`, [`ReplaceFaceError::TogetherChartMixed`],
/// [`ReplaceFaceError::TogetherFaceRepeated`],
/// [`ReplaceFaceError::TogetherPartialSet`]), then every refusal of
/// [`crate::replace_faces_offset`]'s past its group gates, raised for
/// whichever chart, edge or corner it is about.
pub fn offset_surfaces_together<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
) -> Result<OffsetOutcome, ReplaceFaceError<T>> {
    let source = crate::SourceGeometry::of(body);
    let joins = offset_surfaces_together_staged(body, moves, band, tol, true)?;
    Ok(OffsetOutcome::of(source, joins))
}

/// [`offset_surfaces_together`], ending with the join where `join` is
/// set. Unset, the result is construction state a later step must join:
/// the shell's cavity and lift offsets, which key their naming rows by
/// the moved body's cells.
pub(crate) fn offset_surfaces_together_staged<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    moves: &[ChartMove<T>],
    band: Band,
    tol: Tol,
    join: bool,
) -> Result<Vec<crate::boolean::EdgeJoin>, ReplaceFaceError<T>> {
    let seen = crate::offset_together::well_formed(body, moves)?;
    let scope = crate::offset_together::scope_of_moves(body, moves)?;
    crate::offset_together::covers_its_solids(body, &scope, &seen)?;
    let mut moving: Vec<(&[FaceKey], T)> = Vec::with_capacity(moves.len());
    for m in moves {
        if crate::offset_restate::chart_moves(m.distance, band)? {
            moving.push((&m.faces, m.distance));
        }
    }
    if moving.is_empty() {
        // Nothing moves: the scope is as it was found, joined where asked.
        let mut staged = body.clone();
        let joins =
            crate::replace_face::staged_join(&mut staged, join, tol, &|v| scope.holds_vertex(v))?;
        body.adopt(staged);
        return Ok(joins);
    }
    // The core's own group gates pass on a well-formed move set and are
    // the single-face verb's; they run again here, on charts known to
    // be whole and distinct, at the cost of one face lookup each.
    crate::replace_face::offset_charts_staged(body, &moving, band, tol, join)
}

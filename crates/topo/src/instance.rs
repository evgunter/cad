//! **The disjoint-graft door** — bringing another body's single solid
//! into this body as a SEPARATE solid.
//!
//! The boolean pipeline's [`combine`](crate::boolean) door transplants
//! a source body's solid into an EXISTING destination solid, so the two
//! operands' shells end up bounding one volume: that is fusion, and it
//! is the only cross-body operation the Euler layer's `CrossSolid`
//! refusal sanctions because a fused result needs the seam zip
//! afterwards to be a body at all.
//!
//! This door is the other half of the same primitive and asserts
//! strictly less: the source's shells arrive under a **fresh solid of
//! their own**, so nothing is fused, no seam is implied, and the result
//! is the disjoint union of two bodies' contents in one arena. The
//! transplant itself is `combine`'s, called verbatim — same fresh keys
//! in deterministic slot order (D9), same verbatim provenance, same
//! `GeomSource` and pcurve-cache carry, same description surface-key
//! remap with its re-certification. The ONLY difference is the
//! destination: an empty solid minted here instead of one already
//! holding shells.
//!
//! **Who needs it.** A STEP assembly states N placed INSTANCES of M
//! component representations. Each instance's frame is its own rigid
//! map, and [`transform_rigid`](crate::transform_rigid) — the kernel's
//! placement door, which re-checks rigidity with decided predicates and
//! re-certifies every carrier — maps a WHOLE body. So an instance is
//! materialized by building its component alone, mapping that lone body
//! through the kernel door, and grafting the placed result in. The body
//! that ships is then, entity for entity, the union of the bodies that
//! were individually certified and gated — not a re-derivation of them.
//!
//! Validity is the caller's to establish, exactly as with `combine`:
//! this is a raw transplant, and the at-rest validator is what says
//! whether the result is a body. Two solids that OVERLAP are a false
//! body and tier 3 says so; two that merely touch land in the
//! tier-3-not-3′ gap (declared contact is the boolean pipeline's
//! currency, and a disjoint graft declares none).

use crate::body::Body;
use crate::boolean::BooleanError;
use crate::entity::{Solid, SolidKey};

/// Grafts `src`'s single solid into `dst` as a NEW solid, returning its
/// key (module docs).
///
/// `src` must be a single-solid body; its shells arrive whole, in
/// source order, under the minted solid. The minted solid's provenance
/// is `src`'s own solid provenance, transplanted verbatim like every
/// other record the graft carries (a graft is not a re-birth).
///
/// # Errors
///
/// [`BooleanError`] — the transplant's own refusals: `JoinDesync` when
/// `src` is not a well-formed single-solid body, `GraftRecertify` when
/// a transplanted edge description does not re-certify against the
/// destination's surfaces.
pub fn graft_disjoint<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
) -> Result<SolidKey, BooleanError> {
    let Some(provenance) = src
        .solids()
        .next()
        .and_then(|(k, _)| src.solid_provenance.get(k).cloned())
    else {
        return Err(BooleanError::JoinDesync {
            what: "graft source is not a well-formed single-solid body",
        });
    };
    let target = dst.add_solid(Solid { shells: Vec::new() }, provenance);
    crate::boolean::combine::graft_solid(dst, target, src)?;
    Ok(target)
}

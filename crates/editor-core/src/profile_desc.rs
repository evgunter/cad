//! The canonical profile payload (M4 PR 2, spec D1; PR 1's banked
//! `Doc<P>` obligation): `Doc` instantiates with the profile crate's
//! PUBLIC description type — [`profile::Profile`] at `f64`, the
//! document's exact stored-scalar representation — wrapped only to
//! carry the document-value semantics the profile crate deliberately
//! does not define (equality, and bit-equality for spec D7's replay
//! comparators). PR 1's genericity stays: `Doc<P>` remains generic,
//! tests keep their fake payloads; [`ProfileDoc`] is the canonical
//! instantiation.

use geom_core::Real;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};

use crate::doc::Doc;

/// The canonical `P`: the profile crate's description type at the
/// document's stored scalar (`f64`, exact bits — spec D2's replay
/// substrate), plus document-value equality.
///
/// # Equality is BIT equality
///
/// `PartialEq` here compares every float BY BITS (`0.0 ≠ -0.0`,
/// bit-distinct NaNs would differ — though doc floats are never NaN by
/// the PR 1 refusal doors). Rationale: [`crate::node::Node::bit_eq`]
/// compares the opaque payload with the payload's own `PartialEq` and
/// documents that its float semantics are THIS crate's contract when
/// `P` is instantiated — the replay-identity comparator (spec D7)
/// needs bits, so the canonical payload's equality IS bit equality.
#[derive(Debug, Clone)]
pub struct ProfileDesc(pub Profile<f64>);

/// The canonical document type (spec D1's `Doc = DocOf<ProfileDesc>`
/// shape, without renaming PR 1's generic).
pub type ProfileDoc = Doc<ProfileDesc>;

impl ProfileDesc {
    /// Embeds the stored exact-`f64` description into any [`Real`]
    /// scalar for evaluation (the same `from_f64` embedding the
    /// parameter environment uses — spec D4 of PR 1).
    pub fn embed<T: Real>(&self) -> Profile<T> {
        let plane = SketchPlane::new(embed_affine(&self.0.plane.placement));
        let loops = self
            .0
            .loops
            .iter()
            .map(|lp| {
                ProfileLoop::new(
                    lp.vertices
                        .iter()
                        .map(|v| ProfileVertex {
                            pos: geom_core::Point2::new(T::from_f64(v.pos.x), T::from_f64(v.pos.y)),
                            bulge: T::from_f64(v.bulge),
                        })
                        .collect(),
                )
            })
            .collect();
        Profile::new(plane, loops)
    }

    /// Every float of the description, in deterministic traversal
    /// order, as bits — the content-key feed (spec D4) and the
    /// equality substrate.
    pub fn float_bits(&self) -> Vec<u64> {
        let mut out = Vec::new();
        let a = &self.0.plane.placement;
        for v in [a.linear.c0, a.linear.c1, a.linear.c2, a.translation] {
            out.extend([v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]);
        }
        for lp in &self.0.loops {
            // Loop boundary marker: keeps (2 loops of 1 vertex) and
            // (1 loop of 2 vertices) key-distinct.
            out.push(u64::MAX);
            for v in &lp.vertices {
                out.extend([v.pos.x.to_bits(), v.pos.y.to_bits(), v.bulge.to_bits()]);
            }
        }
        out
    }
}

fn embed_affine<T: Real>(a: &geom_core::Affine3<f64>) -> geom_core::Affine3<T> {
    let v = |w: geom_core::Vec3<f64>| {
        geom_core::Vec3::new(T::from_f64(w.x), T::from_f64(w.y), T::from_f64(w.z))
    };
    geom_core::Affine3::from_parts(
        geom_core::Mat3::from_cols(v(a.linear.c0), v(a.linear.c1), v(a.linear.c2)),
        v(a.translation),
    )
}

impl PartialEq for ProfileDesc {
    fn eq(&self, other: &Self) -> bool {
        self.float_bits() == other.float_bits()
    }
}

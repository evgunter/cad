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
            .map(|lp| ProfileLoop {
                vertices: lp
                    .vertices
                    .iter()
                    .map(|v| ProfileVertex {
                        pos: geom_core::Point2::new(T::from_f64(v.pos.x), T::from_f64(v.pos.y)),
                        bulge: T::from_f64(v.bulge),
                    })
                    .collect(),
                // Declared-tangent joints are indices (scalar-free):
                // carried verbatim — the declaration is part of the
                // description and is re-verified at validation in the
                // evaluation scalar.
                tangent_joints: lp.tangent_joints.clone(),
            })
            .collect();
        Profile::new(plane, loops)
    }

    /// The description's canonical TOKEN stream, deterministic
    /// traversal order — the content-key feed (spec D4), the equality
    /// substrate, and the persist door's float walk.
    ///
    /// Tagged by construction (M4 PR 6 fix pass, ruled with Evan):
    /// structure and data travel under DISTINCT tags, so no float
    /// value can alias a boundary. This replaces the former
    /// `float_bits` stream whose in-band `u64::MAX` loop marker was
    /// itself a valid NaN bit pattern — the sentinel-collision class
    /// that bit twice (#101 MINOR-1, PR 6 review MAJOR-1). A NaN here
    /// encodes as `Float(0xFFFF_FFFF_FFFF_FFFF)` and can never be
    /// mistaken for structure; consumers key off the TAG.
    pub fn tokens(&self) -> Vec<DescToken> {
        let mut out = Vec::new();
        let a = &self.0.plane.placement;
        for v in [a.linear.c0, a.linear.c1, a.linear.c2, a.translation] {
            out.extend([v.x, v.y, v.z].map(|f| DescToken::Float(f.to_bits())));
        }
        for lp in &self.0.loops {
            // Loop boundary token: keeps (2 loops of 1 vertex) and
            // (1 loop of 2 vertices) key-distinct — by TAG, never by
            // a magic payload value.
            out.push(DescToken::LoopStart);
            for v in &lp.vertices {
                out.extend([v.pos.x, v.pos.y, v.bulge].map(|f| DescToken::Float(f.to_bits())));
            }
            // Declared-tangent joints (#101): semantic description
            // data, keyed as their canonical SET (sorted,
            // deduplicated — the field is set-semantic: `[1, 1]` and
            // `[1]` describe the same loop and must key identically,
            // #101 review NOTE-1). `Index` tokens are tag-distinct
            // from float data and from boundaries, so no count prefix
            // is needed and no payload (e.g. a garbage joint index of
            // u64::MAX) can forge structure — the tagged retype
            // subsumes #101's length-prefix fix (MINOR-1) and this
            // PR's door blind spot (MAJOR-1) in one shape.
            let mut joints: Vec<u64> = lp.tangent_joints.iter().map(|&j| j as u64).collect();
            joints.sort_unstable();
            joints.dedup();
            out.extend(joints.into_iter().map(DescToken::Index));
        }
        out
    }
}

/// One token of [`ProfileDesc::tokens`]'s canonical traversal. Every
/// token is (tag, payload) — no in-band magic values anywhere in the
/// stream (module docs on the retired `u64::MAX` marker). `Index` is
/// the structural-integer tag (first consumer: profile tangency
/// joints, M4 #101's vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescToken {
    /// A loop boundary.
    LoopStart,
    /// One float's exact bits (data — any bit pattern, NaNs included;
    /// the persist door refuses non-finite PAYLOADS by walking these
    /// tags, skipping nothing).
    Float(u64),
    /// A structural integer (an index, never float data).
    Index(u64),
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
        self.tokens() == other.tokens()
    }
}

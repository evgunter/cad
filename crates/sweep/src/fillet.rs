//! **Constant-radius rolling-ball fillets** — the verb's one public
//! path, mirroring [`crate::chamfer`]'s: the door is named by its
//! module, not re-exported at the crate root as well.
//!
//! The machinery lives in [`crate::blend`], the shared home of both
//! edge blends: the fillet's validity battery (all six of C8's
//! predicates are the fillet's questions), the analytic arm table
//! (plane–plane cylinders, plane–sphere tori, the coaxial curved
//! arms), the in-place composition surgery, and the shared
//! verb-neutral refusal vocabulary. What is fillet-alone is marked
//! there on the arms that take it: the two rolling-ball predicates
//! (radius-vs-curvature headroom, spine regularity), the sphere-octant
//! corner patch, and the closed-rim torus bands — a chamfer meters
//! neither ball fact and has no closed-chain band.
//!
//! The door refuses [`crate::blend::BlendRefusal`] carrying
//! [`crate::blend::BlendKind::Fillet`]: the verb is attached here,
//! once, and nowhere below.

pub use crate::blend::build::{Filleted, fillet_edges};

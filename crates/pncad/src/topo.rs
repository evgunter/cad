//! Topology: bodies, Booleans, splitting, transforms, the validation
//! ladder, mass properties and readback — **curated**.
//!
//! # The boundary this module draws
//!
//! **Topo's construction tier is the kernel's internal API** (DESIGN D4
//! ¶1 (iv)): the Euler operators, the edge and face description setters,
//! and the row and joint doors, gathered as `topo::construction::Construct`
//! with the sites, surfaces and results they take and return. Every
//! curve-and-face pair those doors read is one a kernel crate composed,
//! so a miss there is the kernel's defect; a verb that takes a user's
//! curve refuses it at its own door, with its own lever. The tier is not
//! re-exported here.
//!
//! The doors are trait methods, not inherent ones, which is what makes
//! leaving them out total: `Body` must stay nameable (every consumer
//! holds one), and an inherent method travels with its type through any
//! re-export. With the trait unnamed, `body.mev(..)` does not resolve.
//!
//! Everything else in `topo` comes through whole, so the façade stays
//! closed over error payloads (`EulerOpError` and its `BadArgument` and
//! `RechartDoor` included). The tier's modules are shadowed below by
//! private empty modules: `construction`, every module it re-exports the
//! tier's vocabulary from, and `movefac`, which holds tier doors and
//! no public item. An item declared in a module takes precedence over a
//! glob import of the same name, and a private one does not resolve
//! from outside. Those modules also hold names the façade does carry:
//! topo's root re-exports them, so they arrive through the glob all the
//! same (`EulerOpError`, `BadArgument` and `RechartDoor` from `euler`,
//! `CurveGeom` from `null`).
//!
//! ```compile_fail,E0603
//! // The tier's trait is not reachable through the façade.
//! use pncad::topo::construction::Construct;
//! ```
//!
//! ```compile_fail,E0624
//! // Nor is an Euler operator on a body the façade hands out.
//! fn grow(body: &mut pncad::topo::Body<f64>) {
//!     let _ = body.mvfs(pncad::geom_core::Point3::new(0.0, 0.0, 0.0), true);
//! }
//! ```
//!
//! ```compile_fail,E0603
//! // Nor the tier's own vocabulary, by its module path.
//! fn site(_: pncad::topo::euler::MevSite) {}
//! ```
//!
//! What a consumer willing to depend on `topo` directly reaches is the
//! honest limit of this module's claim, as for `profile`: the tier is
//! off the presented surface; it is not out of existence.

// Shadowing the glob is this module's whole job (module docs), so the
// lint that flags a private item hiding a glob re-export is the intent.
#![allow(hidden_glob_reexports)]

pub use ::topo::*;

// The construction tier's modules (module docs).
mod attach {}
mod construction {}
mod euler {}
mod euler_kill {}
mod euler_ring {}
mod movefac {}
mod null {}
mod split {}
mod surgery {}

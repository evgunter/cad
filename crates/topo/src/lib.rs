//! Topology arenas, [`Body<T>`], and the structural validation harness —
//! the `topo` layer with M1's half-edge representation (D1 in
//! `docs/DESIGN.md`).
//!
//! A B-rep body is a plain value: typed generational arenas
//! (slotmap-style, D1) for the seven manifold topology kinds — solid,
//! shell, face, loop, half-edge, edge, vertex — plus geometry arenas,
//! plus a D5 provenance record per entity from birth. M1 PR 1 landed the
//! half-edge structure itself (adjacency, orientation conventions, the
//! tier-1 structural validator); PR 2 added the first Euler operators —
//! [`Body::mvfs`](crate::construction::Construct::mvfs), [`Body::mev`](crate::construction::Construct::mev), [`Body::mef`](crate::construction::Construct::mef) (see [`euler`]) — the
//! sanctioned construction path; PR 3 added the ring/genus operators —
//! [`Body::kemr`](crate::construction::Construct::kemr), [`Body::mekr`](crate::construction::Construct::mekr), [`Body::kfmrh`](crate::construction::Construct::kfmrh) — and the non-Euler
//! [`Body::ring_move`](crate::construction::Construct::ring_move) helper (see [`euler_ring`]); PR 4 completed the
//! ten-operator catalog with the kill-direction duals — [`Body::kvfs`](crate::construction::Construct::kvfs),
//! [`Body::kev`](crate::construction::Construct::kev), [`Body::kef`](crate::construction::Construct::kef), [`Body::mfkrh`](crate::construction::Construct::mfkrh) (see [`euler_kill`]) —
//! plus the make/kill roundtrip property-test machinery; PR 5 completed
//! the validator (validity tiers — [`fn@validate`] accepts every
//! Euler-reachable state, [`validate_closed`] the finished closed
//! solids) and retired the raw-insertion builder to `pub(crate)`: **the
//! Euler operators are the only public construction path** (D1).
//!
//! **M2 PR 3 — real geometry.** The M0 placeholder geometry retired:
//! the curve arena holds certified `geom_brep::EdgeCurve`s (D2
//! intensional description + carrier cache + certification record) and
//! the surface arena holds `geom::Surface`s. Edge-minting
//! operators take an uncertified `EdgeCurveSpec` and run the D4 ¶2
//! certification gate before mutating (chord-line sugar:
//! [`Body::mev_line`](crate::construction::Construct::mev_line) / [`Body::mef_chord`](crate::construction::Construct::mef_chord) / [`Body::mekr_chord`](crate::construction::Construct::mekr_chord) /
//! [`Body::mfkrh_plug`](crate::construction::Construct::mfkrh_plug)); face-minting operators take a
//! [`FaceSurface`](crate::construction::FaceSurface) spec; the two attachment setters
//! ([`Body::set_face_surface`](crate::construction::Construct::set_face_surface), [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve) — see
//! [`attach`]) cover the construction-order cases mint-time attachment
//! cannot reach. Tier 3 ([`validate_geometric`]) re-runs every
//! certification at rest and adds the dihedral classification pass.
//!
//! # Orientation conventions
//!
//! Documented **once**, in the [`entity`] module docs: the interior-left
//! rule (counterclockwise outer loops viewed from outside), antiparallel
//! half-edge pairs, derived end vertices, the vertex-orbit step, and the
//! GWB-is-mirrored transcription warning. Start there before touching
//! topology code.
//!
//! # The genericity boundary (Q1, settled by this crate's shape)
//!
//! `Body<T>` is **scalar-free topology plus geometry arenas over `T`**.
//! Topology entities contain no scalar values and no comparisons — only
//! typed keys — while the geometry arenas (points, curves, surfaces) are
//! the only `T`-carrying storage. Topology never branches on `T`: every
//! topology-determining decision was made at construction time through
//! named predicates (Q1 in `docs/DESIGN.md`), and a body is the *result*
//! of those decisions. An interval replay therefore materializes a
//! `Body<Interval>` with identical topology keys and interval-valued
//! geometry.
//!
//! # Determinism (D9)
//!
//! Arena iteration is slot-index order — deterministic given identical
//! construction history, which the pure-replay model guarantees. The
//! standing invariant (documented at [`body`]): no arena iteration may
//! feed a geometry decision unless the construction sequence is itself
//! deterministic; accordingly this crate contains no `HashMap`/`HashSet`.
//! Half-edge traversal is **bounded** — the walks cap at the arena
//! length and fail loud instead of spinning on corrupt input (the kernel
//! never hangs, D9).
//!
//! # Example
//!
//! Build the unit cube through the Euler operators — the §9.4.2-minimal
//! sequence, 1 `mvfs` + 7 `mev` + 5 `mef` — and validate it at both
//! tiers. Every intermediate state is tier-1 valid ([`fn@validate`]); the
//! finished cube is a tier-2 closed solid ([`validate_closed`]).
//!
//! ```
//! use geom_core::Point3;
//! use topo::Body;
//! use topo::construction::{Construct, MefSite, MevSite};
//! use geom_core::Tol;
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let tol = Tol::witness();
//! let pt = Point3::new;
//! let mut body = Body::<f64>::new();
//!
//! // The seed: solid + shell + one face holding lone vertex A.
//! let seed = body.mvfs(pt(0.0, 0.0, 0.0), true)?;
//! // The bottom rim A → B → C → D, grown by three mev …
//! let e_ab = body.mev_line(MevSite::Lone { r#loop: seed.r#loop }, pt(1.0, 0.0, 0.0), tol)?;
//! let strut = |he| MevSite::Fan { he1: he, he2: he };
//! let e_bc = body.mev_line(strut(e_ab.he_minus), pt(1.0, 1.0, 0.0), tol)?;
//! let e_cd = body.mev_line(strut(e_bc.he_minus), pt(0.0, 1.0, 0.0), tol)?;
//! // … and closed by a mef splitting off the bottom face.
//! let he_dc = body.find_half_edge(seed.face, e_cd.vertex, e_bc.vertex).unwrap();
//! let f_bot = body.mef_chord(MefSite::Chords { he1: he_dc, he2: e_ab.he_plus }, tol)?;
//! // Four vertical struts up from the rim …
//! let e_aa = body.mev_line(strut(e_ab.he_plus), pt(0.0, 0.0, 1.0), tol)?;
//! let e_bb = body.mev_line(strut(e_bc.he_plus), pt(1.0, 0.0, 1.0), tol)?;
//! let e_cc = body.mev_line(strut(e_cd.he_plus), pt(1.0, 1.0, 1.0), tol)?;
//! let e_dd = body.mev_line(strut(f_bot.he_plus), pt(0.0, 1.0, 1.0), tol)?;
//! // … and four mef close the side faces (the seed face becomes the top).
//! let chord = |he1, he2| MefSite::Chords { he1, he2 };
//! let f_front = body.mef_chord(chord(e_aa.he_minus, e_bb.he_minus), tol)?;
//! body.mef_chord(chord(e_bb.he_minus, e_cc.he_minus), tol)?;
//! body.mef_chord(chord(e_cc.he_minus, e_dd.he_minus), tol)?;
//! body.mef_chord(chord(e_dd.he_minus, f_front.he_plus), tol)?;
//!
//! assert_eq!(body.vertices().count(), 8);
//! assert_eq!(body.edges().count(), 12);
//! assert_eq!(body.faces().count(), 6);
//! assert_eq!(topo::validate(&body), Ok(()));        // tier 1: euler-valid
//! assert_eq!(topo::validate_closed(&body), Ok(())); // tier 2: closed solid
//!
//! // Mid-construction scaffolding is tier-1 legal but not tier-2: a
//! // strut fails `validate_closed` with a typed, entity-named error.
//! let scaffold = body.mev_line(strut(e_ab.he_plus), pt(2.0, 0.0, 0.0), tol)?;
//! assert_eq!(topo::validate(&body), Ok(()));
//! assert_eq!(
//!     topo::validate_closed(&body),
//!     Err(vec![topo::ValidationError::ScaffoldingStrutVertex {
//!         vertex: scaffold.vertex,
//!     }]),
//! );
//! # Ok(()) }
//! # run().unwrap();
//! ```

pub mod attach;
pub mod body;
pub mod boolean;
pub(crate) mod bound_side;
pub mod cell_region;
pub(crate) mod census;
pub mod chart;
pub(crate) mod chart_groups;
pub mod chart_iso;
pub mod chart_region;
pub mod clearance;
pub mod construction;
pub(crate) mod injective;
// The shared chord-join core — ch. 14's `join`/`cut` mechanics and the
// section-chord geometry, a top-level sibling of `boolean/` and
// `splitting/` for the reason its own docs give. Non-doc comment for
// the same rustdoc reason as the sector modules below.
pub(crate) mod chord_join;
pub mod coherence;
pub mod coincidence;
pub mod contact;
pub mod entity;
pub mod euler;
pub mod euler_kill;
pub mod euler_ring;
pub(crate) mod fuse;
// The one door for a planar face's outward normal — at the crate root
// because its consumers now span both halves and the shared sector
// walk; its own docs carry the argument. Non-doc comment for the same
// rustdoc reason as the sector modules below.
pub mod face_boxes;
pub mod face_normal;
#[cfg(test)]
pub(crate) mod fixtures;
// This crate's own sources, read as source. A sibling of `fixtures`
// rather than a section of it: that module's subject is canonical
// bodies, this one's is a Rust reader. Non-doc comment for the
// same rustdoc reason as the sector modules below.
pub mod geometry;
#[cfg(test)]
pub(crate) mod source_walk;

#[cfg(test)]
pub(crate) mod cert_m3r1_probes;
pub mod instance;
pub(crate) mod invalid_margin;
#[cfg(test)]
pub(crate) mod iso;
pub mod joint;
pub(crate) mod lineage;
pub mod link;
pub(crate) mod live;
// The one statement of a stored planar loop's signed winding, shared by
// the merge's role assigner and tier 3's check 6. Non-doc comment for
// the same rustdoc reason as the sector modules below.
pub(crate) mod loop_winding;
pub mod merge_faces;
pub mod mirror;
pub mod movefac;
#[cfg(test)]
mod n2r1_probes;
pub mod null;
pub mod offset_axial;
pub mod offset_derive;
pub mod offset_general;
pub mod offset_nappe;
pub(crate) mod offset_restate;
pub mod offset_together;
pub mod pcurves;
pub mod pieces;
pub(crate) mod policy_lane;
pub mod props;
pub mod provenance;
pub mod query;
pub(crate) mod ray_walk;
pub mod readback;
pub mod replace_face;
#[cfg(test)]
pub(crate) mod revalue_pin;
pub mod revert;
#[cfg(test)]
mod review_d18;
#[cfg(test)]
mod review_d18_probes;
#[cfg(test)]
mod review_d21_probes;
#[cfg(test)]
mod review_m0_pr7;
#[cfg(test)]
mod review_m1_pr1;
#[cfg(test)]
mod review_m1_pr2;
#[cfg(test)]
mod review_m1_pr3;
#[cfg(test)]
mod review_m1_pr4;
#[cfg(test)]
pub(crate) mod review_m1_pr5_internal;
pub(crate) mod ring_path;
#[cfg(test)]
mod row_walk_proofs;
// The shared vertex-neighborhood sector modules — top-level siblings
// of `boolean/` and `splitting/` on purpose: both lanes ask these
// questions, so neither hosts them. Each module's own docs carry
// the placement argument. Non-doc comments deliberately: an outer
// `///` here would merge into the module's own `//!` docs and make
// rustdoc resolve their intra-doc links in THIS module's scope instead
// of that one's.
pub(crate) mod one_region;
pub(crate) mod sector_face;
pub(crate) mod sector_shape;
pub mod separation;
#[cfg(test)]
pub(crate) mod seqgen;
pub mod shell;
pub(crate) mod side_reader;
pub mod split;
pub mod splitting;
pub(crate) mod stands;
pub(crate) mod strut;
pub mod surface_boundary;
pub mod surgery;
// Existence and visibility are two questions, gated separately; the
// module's own docs are the statement of both. EXISTENCE: the items
// must be compiled wherever any of their three consumers is — the
// debug postcondition, the in-crate oracles, and this crate's `tests/`
// binaries through the feature.
#[cfg(any(debug_assertions, test, feature = "test-support"))]
// `doc(hidden)` for the same reason as the door below. This module is
// the ONLY private module of the crate that a doc build renders — not
// because `--document-private-items` shows everything private, but
// because the others (`fixtures`, `seqgen`, `tier3_tests`) are
// `#[cfg(test)]` and do not exist in a doc build at all, while this one
// exists whenever `debug_assertions` does.
#[doc(hidden)]
mod test_support_impl;
// The Euler-op fixture family, a sibling of the module above rather
// than a section of it: nothing here has a non-test consumer, so it is
// gated on the test arms alone and its file-level `#![allow]` stays
// exactly as wide as the code that earns it. Its own docs state what
// separates it from `fixtures`.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
mod test_support_fixtures;
// The holes-meeting-at-a-vertex fixture geometry and its corner check,
// shared with editor-core's rows over the same bodies.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
mod test_support_meeting;
// One `ValidationError` of every arm, for the rows that render them —
// this crate's Display-coverage row and a downstream refusal-budget
// row — so it sits behind the same door, on the same gate.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
mod test_support_samples;
// The boolean's volume relations, a test oracle the suites run on their
// own results.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
mod test_support_volume;
// VISIBILITY: the only reason to export them is a test naming them from
// another crate, so the public door opens on the test arms alone —
// `topo::test_support` does not resolve in a plain build of any profile.
#[cfg(any(test, feature = "test-support"))]
// `doc(hidden)` because this repo's own rustdoc gate (scripts/doc-gate.sh)
// runs `--all-features`, which turns `test-support` ON: without this the
// gate would publish, as public API, a module whose first line says it is
// not one. Hiding it keeps the rendered docs and the door's own claim
// saying the same thing. It hides nothing from a test — `doc(hidden)` is
// a rustdoc directive, not a visibility one.
#[doc(hidden)]
pub mod test_support {
    //! The public door onto this crate's test vocabulary, open exactly
    //! when a test needs to name it from another crate. Two modules
    //! come through it: [`crate::test_support_impl`], whose docs state
    //! both gates, why they differ and which home a new item belongs
    //! in, and [`crate::test_support_fixtures`], the Euler-op fixture
    //! family, which carries this module's own gate exactly.
    use geom_core::Real;

    use crate::body::Body;

    /// The at-rest window's reasons for an incidence refusal, one per
    /// read (`validate::incidence_reasons`): a test outside this crate
    /// reads them rather than copying their words.
    pub mod incidence_reasons {
        /// Decided off its face.
        pub const OFF: &str = crate::validate::incidence_reasons::OFF;
        /// Read within the band at a pose.
        pub const AT_POSE: &str = crate::validate::incidence_reasons::AT_POSE;
        /// The incidence reading landed in the band.
        pub const UNSHOWN: &str = crate::validate::incidence_reasons::UNSHOWN;
    }

    /// **The resolution the suites' pinned numbers are stated at**:
    /// [`crate::Settle::Resolution`] of 1024·ε of mean boundary
    /// displacement. A test that reads a volume or an area as a number,
    /// against a closed form or a pin, measures through this: its
    /// tolerances and pins were set where every face met 1024·ε, and a
    /// face that converges within the work wall keeps its bits under it.
    /// A test whose assertion needs another width states that one.
    #[must_use]
    pub fn pinned_resolution<T: Real>(tol: geom_core::Tol) -> crate::Settle<T> {
        crate::Settle::Resolution(quantity::Length::from_meters(pinned_len(tol)))
    }

    /// **`body`'s volume and area as numbers met at
    /// [`pinned_resolution`]**: [`crate::mass_properties`] under it, read
    /// through [`crate::MassMeasurement::met`], so a row that compares a
    /// number with a closed form or a pin reads it only where every face
    /// met the resolution. `what` names the body in the panic.
    ///
    /// # Panics
    ///
    /// Where the door refuses, or the resolution went unmet.
    #[must_use]
    #[allow(clippy::panic)]
    pub fn pinned_volume(
        what: &str,
        body: &crate::AtRestBody<f64>,
        tol: geom_core::Tol,
    ) -> crate::MassProperties<f64> {
        match crate::mass_properties(body, tol, pinned_resolution(tol)) {
            Ok(measured) => match measured.met() {
                Ok(enclosure) => enclosure,
                Err(unmet) => panic!("{what}: {unmet}"),
            },
            Err(refusal) => panic!("{what}: mass properties refused: {refusal}"),
        }
    }

    /// [`pinned_resolution`]'s length (m), for a row that hands a face's
    /// lane the resolution itself.
    #[must_use]
    pub fn pinned_len(tol: geom_core::Tol) -> f64 {
        1024.0 * tol.eps()
    }

    // `UNIT_SQUARE` is deliberately NOT here: `tests/cube_doors_agree.rs`
    // is the only suite that wants the literal, and it restates it on
    // purpose — a guard that reached for the constant the builder uses
    // would be comparing that constant against itself.
    pub use crate::null::with_ring_root;
    pub use crate::props::{
        DoorOutcome, DoorRead, FaceReading, FaceTerm, every_door, face_reading,
        face_reading_structural, fold_order_bound, mass_properties_one_point,
        mass_properties_with_face_terms, measure_with_unmet_width, quadrature_rounds, read_points,
        region_carries_of, shell_references,
    };
    pub use crate::test_support_fixtures::{
        CubeOps, CylFrame, CylKey, FaceGeometry, NullStrutListing, Prism, PrismOps, RingFaceOps,
        StraddleSeat, arc_chain_over_the_jump, assert_every_chord_named_by_the_fence, brick,
        brick_with_fresh_keys, computed_edge_channels, cube_into, cyl_arc_at, cyl_wall_sheet,
        cyl_wall_sheet_keyed, declined_cube, describe_as_intersections, drill_hole,
        every_chord_stated_as_its_pair, face_facing, fresh_keyed_body, geometric_cube, holed_block,
        identity_map, kill_under_a_null_strut, line, mapped_cube, plane, plane_every_face,
        plant_disc_face, plant_ring_face, prism, prism_ops, prism_z, set_sheet_sense, split_plane,
        straddle_seat, wedge_on_a_block_top,
    };
    pub use crate::test_support_impl::ArenaCounts;

    /// **A placement the census decided overlaps**: a finding
    /// [`crate::census_interference_solids`] reads as two solids'
    /// interference. The one home of the test that a row asserting a
    /// placement clears, or refuses undecided, reads, so a new decided
    /// verdict reaches all of them.
    #[must_use]
    pub fn is_decided_overlap<T: geom_core::Real>(
        body: &crate::Body<T>,
        e: &crate::ValidationError,
    ) -> bool {
        crate::census_interference_solids(body, e).is_some()
    }

    /// **The census's verdicts on where two solids sit**: each decided
    /// overlap ([`is_decided_overlap`]) and each
    /// [`crate::ValidationError::CensusUndecidable`] naming two solids.
    #[must_use]
    pub fn placement_findings<'e, T: geom_core::Real>(
        body: &crate::Body<T>,
        errors: &'e [crate::ValidationError],
    ) -> Vec<&'e crate::ValidationError> {
        errors
            .iter()
            .filter(|e| {
                is_decided_overlap(body, e)
                    || matches!(
                        e,
                        crate::ValidationError::CensusUndecidable {
                            a: crate::EntityId::Solid(_),
                            b: crate::EntityId::Solid(_),
                            ..
                        }
                    )
            })
            .collect()
    }

    /// **The raw measurement door**: [`crate::mass_properties`]' walk over
    /// a body that has not passed tier 3. The public door takes a
    /// finished body (`&AtRestBody`, Ev's decision on PR 4822); a test
    /// whose subject is the walk on a body tier 3 refuses (a flipped
    /// seam region, a corrupt arena, a shell wound against its role)
    /// measures it here, and says so in one line at the call. No
    /// production caller has a reason to: a body it built is finished
    /// through `AtRestBody::validate`.
    ///
    /// # Errors
    ///
    /// As [`crate::mass_properties`].
    pub use crate::props::raw_door::measure_body as measure_unvalidated;

    /// **The raw classification door**: [`crate::classify_shells`] over a
    /// body that has not passed tier 3, as [`measure_unvalidated`] is to
    /// the measurement.
    ///
    /// # Errors
    ///
    /// As [`crate::classify_shells`].
    pub use crate::props::raw_door::classify_body as classify_unvalidated;

    /// **The raw closed-form measurement door**:
    /// [`crate::mass_properties_structural`] over a body that has not
    /// passed tier 3, as [`measure_unvalidated`] is to
    /// [`crate::mass_properties`].
    ///
    /// # Errors
    ///
    /// As [`crate::mass_properties_structural`].
    pub use crate::props::raw_door::measure_body_structural as measure_unvalidated_structural;

    /// **The raw closed-form classification door**:
    /// [`crate::classify_shells_structural`] over a body that has not
    /// passed tier 3.
    ///
    /// # Errors
    ///
    /// As [`crate::classify_shells_structural`].
    pub use crate::props::raw_door::classify_body_structural as classify_unvalidated_structural;

    /// **The raw restricted classification door**:
    /// [`crate::classify_shells_of`] over a body that has not passed
    /// tier 3.
    ///
    /// # Errors
    ///
    /// As [`crate::classify_shells_of`].
    pub use crate::props::raw_door::classify_body_of as classify_unvalidated_of;

    /// Every shell's stored face list rotated left by `k` (modulo its
    /// length): the same solid stored in another face order, which the
    /// tiers do not read. Each shell's face count, in arena order.
    pub fn rotate_shell_faces<T: Real>(body: &mut Body<T>, k: usize) -> Vec<usize> {
        body.shells
            .iter_mut()
            .map(|(_, shell)| {
                let n = shell.faces.len();
                if n > 0 {
                    shell.faces.rotate_left(k % n);
                }
                n
            })
            .collect()
    }

    /// How far each solid's aimed rays start from `aim`, over two
    /// (`side_reader`'s `beyond`): the reach's pins read its scale.
    pub fn aim_reach(body: &Body<f64>, aim: geom_core::Point3<f64>) -> Vec<Result<f64, String>> {
        crate::side_reader::beyond_report(body, aim)
    }

    /// What each solid's aimed rays read before the volume, as text
    /// (`side_reader`): the edge-meeting ray's probes read it.
    pub fn aimed_rays(body: &Body<f64>, band: geom_core::Band) -> Vec<String> {
        crate::side_reader::aimed_rays_report(body, band)
    }

    /// What each shell's aimed rays read before the volume, as text
    /// (`side_reader`): the trimmed-sphere aims' pin reads it.
    pub fn shell_aims(body: &Body<f64>, band: geom_core::Band) -> Vec<String> {
        crate::side_reader::shell_aims_report(body, band)
    }

    /// Each face's aimed ray's own reading, every face aimed at
    /// (`side_reader`): the curved edge-meeting ray's probes read it.
    pub fn each_aim(body: &Body<f64>, band: geom_core::Band) -> Vec<String> {
        crate::side_reader::each_aim_report(body, band)
    }

    /// One aimed ray at `aim` on `face`, at the aimed incidence to
    /// `normal` (`side_reader`): the probes aim it at and about an edge.
    pub fn ray_at(
        body: &Body<f64>,
        face: crate::FaceKey,
        aim: geom_core::Point3<f64>,
        normal: geom_core::Vec3<f64>,
        band: geom_core::Band,
    ) -> String {
        crate::side_reader::ray_at_report(body, face, aim, normal, band)
    }

    pub use crate::side_reader::EdgeFrame;

    /// Every crossing of the ray `q + s·d` with the solid holding `face`,
    /// its faces in an order turned left by `turn` and reversed where
    /// asked, as `(from, to, count, sideless)` (`side_reader`): the probes
    /// read a whole ray's count off it.
    pub fn ray_counts(
        body: &Body<f64>,
        face: crate::FaceKey,
        ray: (geom_core::Point3<f64>, geom_core::Vec3<f64>),
        band: geom_core::Band,
        order: (usize, bool),
    ) -> Result<Vec<(f64, f64, i32, bool)>, String> {
        crate::side_reader::ray_counts(body, face, ray, band, order)
    }

    /// Each edge's frames on each of its faces, at each fraction of
    /// `fractions` along it (`side_reader`): the probes aim at and about
    /// an edge from them.
    pub fn edge_frames(
        body: &Body<f64>,
        fractions: &[f64],
        band: geom_core::Band,
    ) -> Vec<EdgeFrame> {
        crate::side_reader::edge_frames(body, fractions, band)
    }

    /// Rays aimed at and beside every edge, `offsets` into each face
    /// (`side_reader`): the curved edge-meeting ray's probes read it.
    pub fn edge_rays(body: &Body<f64>, offsets: &[f64], band: geom_core::Band) -> Vec<String> {
        crate::side_reader::edge_rays_report(body, offsets, band)
    }

    /// The pad a face's reach box takes past the band, and each face's
    /// share of the census's gaps (`boolean::boxes::sweep_pad`): the
    /// spelling a test outside this crate reads it by.
    pub fn sweep_pad(band: geom_core::Band) -> f64 {
        crate::boolean::boxes::sweep_pad(band)
    }

    /// **Restates the surface at `key`** as `surface`: a row that needs
    /// a datum no construction mints (a cone's seam off square to its
    /// axis) builds the body and then restates its carrier. Returns
    /// whether `key` was live.
    pub fn restate_surface<T: Real>(
        body: &mut Body<T>,
        key: crate::SurfaceKey,
        surface: geom::Surface<T>,
    ) -> bool {
        body.surfaces.get_mut(key).map(|s| *s = surface).is_some()
    }

    /// `host` holding `cavity` as [`crate::insert_void`] would graft it,
    /// without the revert's refusal: `cavity`'s every face side reversed
    /// and no sheet re-paired, so coincident edges on one line land each
    /// bounding the complement of its own wedge.
    ///
    /// # Panics
    ///
    /// Where `solid` does not resolve in `host`, or `cavity` holds other
    /// than one solid.
    #[must_use]
    #[allow(clippy::panic)]
    pub fn grafted_reverted_unrefused<T: geom_core::Decide>(
        host: &Body<T>,
        solid: crate::SolidKey,
        cavity: &Body<T>,
    ) -> Body<T> {
        let mut out = host.clone();
        if let Err(e) =
            crate::boolean::combine::graft_solids_disjoint(&mut out, &[solid], &cavity.reverted())
        {
            panic!("the graft refused: {e:?}");
        }
        out
    }

    /// A test's hand-built contact records: each pair of `body`'s cells
    /// as a record citing row 0 of the producing op's coincidences.
    ///
    /// # Panics
    ///
    /// Where a pair is not a record ([`crate::Contact::new`]).
    #[allow(clippy::panic)]
    pub fn records<T: Real>(
        body: &Body<T>,
        pairs: impl IntoIterator<Item = (crate::Cell, crate::Cell)>,
    ) -> crate::ContactRecords {
        pairs
            .into_iter()
            .map(|(x, y)| match crate::Contact::new(body, x, y) {
                Ok(c) => crate::Cited::new(c, crate::Cites::decided(0)),
                Err(e) => panic!("{x:?}, {y:?} is not a contact record: {e:?}"),
            })
            .collect()
    }
    /// Holes meeting at one vertex of a plate's top
    /// ([`crate::test_support_meeting`]).
    pub mod meeting {
        pub use crate::test_support_meeting::{
            ESCALATING_DIPS, FIN_BOTTOM, Hole, MEET, PLATE, Point, Pose, along_lying_ray,
            apex_pyramid, apex_pyramid_kept, arch, arch_cone, at, bearing, branching_cone, comb,
            corners, corners_disjoint, crown, crown_body, cycles_of, ell, ell_and_wedges, fin,
            fin_crown_body, four_wedges, inner_rows, leaned, lying, mix, near_flat, nest,
            nest_polygon, notch, notch_rows, on_the_lying_faces, orders, posed_box, posed_boxes,
            posed_crown, posed_crown_kept, posed_fin_crown, posed_prism, posed_pyramid,
            posed_pyramid_at, poses, pyramid_volumes, shape, single_arch, sunk_on_an_edge,
            three_wedges, two_wedges, wedge, wedges_on_one_side, zigzag,
        };
    }

    /// **The merge without the join**, for a fixture that is
    /// construction state on purpose: a body whose edges carry
    /// station vertices a row exercises (a split rim, a stationed
    /// wall), which the public door's join
    /// ([`Body::merge_coplanar_faces`]) would take away.
    ///
    /// # Errors
    ///
    /// As [`Body::merge_coplanar_faces`].
    pub fn merge_unjoined<T: crate::AtRestPolicy>(
        body: &mut Body<T>,
        tol: geom_core::Tol,
    ) -> Result<crate::MergeCoplanarOutcome, crate::MergeCoplanarError> {
        body.merge_coplanar_faces_unjoined(&crate::boolean::GluedSurfaces::none(), tol)
    }

    /// `body` finished for a door that takes finished bodies (the
    /// boolean's): through the scalar's at-rest gate
    /// ([`crate::AtRestPolicy::gate_at_rest_kept`]).
    ///
    /// # Panics
    ///
    /// Naming `what` and the validator's findings, where the gate
    /// refuses it — a fixture that is not a finished body.
    #[must_use]
    #[allow(clippy::panic)]
    #[track_caller]
    pub fn finished<T: crate::AtRestPolicy>(
        what: &str,
        body: Body<T>,
        tol: geom_core::Tol,
    ) -> crate::AtRestBody<T> {
        match T::gate_at_rest_kept(body, tol) {
            Ok(body) => body,
            Err(e) => panic!("{what} is not a finished body: {e:?}"),
        }
    }

    /// **What of `body` lies along a line**: the line through `origin`
    /// along the unit `dir`, and each edge both of whose ends lie on it
    /// (within 1e-9) as their parameters along it, the lesser first, and
    /// each vertex on it as its parameter, each list sorted and rounded
    /// to 1e-6. Two coincident edges read as one span twice.
    ///
    /// # Panics
    ///
    /// Where an edge's half or end, or a vertex's point, does not
    /// resolve: a torn body.
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn along_line(
        body: &Body<f64>,
        origin: geom_core::Point3<f64>,
        dir: geom_core::Vec3<f64>,
    ) -> (Vec<(f64, f64)>, Vec<f64>) {
        let at = |v: crate::entity::VertexKey| {
            let p = crate::readback::vertex_point(body, v).expect("a vertex's point");
            let t = (p - origin).dot(dir);
            let off = (p - origin - dir * t).norm();
            (off < 1e-9).then_some((t * 1e6).round() / 1e6)
        };
        let mut spans: Vec<(f64, f64)> = body
            .edges()
            .filter_map(|(_, edge)| {
                let start = body.get_half_edge(edge.he_plus).expect("a half").start;
                let end = body.half_edge_end(edge.he_plus).expect("an end");
                let (a, b) = (at(start)?, at(end)?);
                Some((a.min(b), a.max(b)))
            })
            .collect();
        spans.sort_by(f64_pair_order);
        let mut vertices: Vec<f64> = body.vertices().filter_map(|(v, _)| at(v)).collect();
        vertices.sort_by(f64::total_cmp);
        (spans, vertices)
    }

    /// Two pairs of floats in order, first by first.
    fn f64_pair_order(x: &(f64, f64), y: &(f64, f64)) -> core::cmp::Ordering {
        x.0.total_cmp(&y.0).then(x.1.total_cmp(&y.1))
    }

    /// Which bridge a graft ran ([`take_graft_bridges`]).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GraftBridge {
        /// Re-certified every carrier against the destination.
        Recertify,
        /// Carried the source's certificates, handles rewritten.
        RemapKeys,
    }

    /// The bridge of every graft this thread ran since the last call,
    /// in call order, draining the record.
    pub fn take_graft_bridges() -> Vec<GraftBridge> {
        crate::boolean::combine::take_bridges()
            .into_iter()
            .map(|b| match b {
                crate::boolean::combine::BridgeKind::Recertify => GraftBridge::Recertify,
                crate::boolean::combine::BridgeKind::RemapKeys => GraftBridge::RemapKeys,
            })
            .collect()
    }
    pub use crate::test_support_samples::{arc_end_clear_of_corners, validation_error_samples};

    /// Runs `f` with every boolean on this thread taking its vertex
    /// pairs, and each pair's crossing records, in reverse order, and
    /// returns how many reductions inside [`ops_under_test`] held two
    /// crossing pairs at one vertex: a row that asserts the same results
    /// both ways round reads that count to know it was not vacuous.
    pub fn with_vertex_pairs_reversed<R>(f: impl FnOnce() -> R) -> (R, usize) {
        crate::boolean::insert::with_vertex_pairs_reversed(f)
    }

    /// Runs `f` as the ops a row tests, apart from the booleans that
    /// built their operands ([`with_vertex_pairs_reversed`]).
    pub fn ops_under_test<R>(f: impl FnOnce() -> R) -> R {
        crate::boolean::insert::ops_under_test(f)
    }

    /// **The measurement door** ([`crate::props::measure_to`]) over every
    /// face of `body`, measured and certified through the certified lane,
    /// under `settle`: why it stopped and the enclosure it reached, or the
    /// refusal's text: [`crate::mass_properties`] over a body tier 3 has
    /// not admitted, read as its stop and volume enclosure.
    ///
    /// # Errors
    ///
    /// The door's refusal, as text, or a tolerance that forms no band.
    pub fn measure_to(
        body: &Body<f64>,
        settle: crate::Settle<f64>,
        tol: geom_core::Tol,
    ) -> Result<(crate::Stop, crate::VolumeEnclosure<f64>), String> {
        let band = geom_core::Band::linear(tol).map_err(|e| format!("{e:?}"))?;
        let lane = Some(crate::props::QuadLane::certified());
        let faces = crate::query::all_faces(body);
        crate::props::measure_to(body, &faces, band, tol, (lane, lane), settle)
            .map(|(stop, certificate)| (stop, certificate.enclosure()))
            .map_err(|refusal| format!("{refusal:?}"))
    }

    pub use crate::test_support_volume::{VolumeOracle, assert_volume_relations, volume_relations};

    /// The two operand clones as the boolean's join leaves them, and
    /// the interior-loop guard's verdict on them.
    #[derive(Debug)]
    pub struct JoinedOperands {
        /// The A operand as the join leaves it.
        pub a: Body<f64>,
        /// The B operand as the join leaves it.
        pub b: Body<f64>,
        /// The interior-loop guard's verdict, which the pipeline raises
        /// on the built body after tier validation, and this door does
        /// not raise.
        pub interior_loops: Result<(), crate::BooleanError>,
    }

    /// The boolean pipeline through its join: both operand clones with
    /// every null edge killed, before the finish and the closing mint
    /// (`boolean::through_the_join`). `None` where the pipeline answers
    /// without a join to stop at.
    ///
    /// # Errors
    ///
    /// The pipeline's refusal on the way to its join.
    pub fn boolean_through_the_join(
        op: crate::BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        tol: geom_core::Tol,
    ) -> Result<Option<JoinedOperands>, crate::BooleanError> {
        crate::boolean::through_the_join(op, a, b, tol)
    }

    /// The join's own refusal of `op` under `decls`
    /// (`boolean::join_refusal`): `None` where the join connects.
    ///
    /// # Errors
    ///
    /// The reduction's refusal.
    pub fn boolean_join_refusal(
        op: crate::BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        tol: geom_core::Tol,
    ) -> Result<Option<crate::BooleanError>, crate::BooleanError> {
        crate::boolean::join_refusal(op, a, b, tol)
    }

    /// The operand's maximal-faces gate (F7) alone, at `tol`'s band —
    /// for a body below tier 3, which no boolean door takes, so the
    /// gate's own reading of it stays measurable
    /// (`boolean::maximal_faces_gate`).
    ///
    /// # Errors
    ///
    /// The gate's refusal.
    pub fn maximal_faces_gate(
        body: &Body<f64>,
        operand: crate::Operand,
        tol: geom_core::Tol,
    ) -> Result<(), crate::BooleanError> {
        crate::boolean::maximal_faces_gate(body, operand, tol)
    }

    /// Every segment the join of `op` builds, one-site loops included:
    /// the pair-record count and each segment's two germ sites
    /// (`boolean::section_segment_sites`).
    /// `None` where the reduction registers no pair.
    ///
    /// # Errors
    ///
    /// The reduction's refusal, or the matcher's.
    pub fn boolean_segment_sites(
        op: crate::BooleanOp,
        a: &crate::AtRestBody<f64>,
        b: &crate::AtRestBody<f64>,
        tol: geom_core::Tol,
    ) -> Result<Option<crate::boolean::SegmentSites>, crate::BooleanError> {
        crate::boolean::section_segment_sites(op, a, b, tol)
    }

    /// The direct split run through its join: the scratch body with
    /// every null edge killed, before the finish and the closing mint
    /// (`splitting::through_the_join`).
    ///
    /// # Errors
    ///
    /// The reduction's or the join's refusal.
    pub fn split_through_the_join<T: geom_core::Decide + crate::props::AtRestPolicy>(
        operand: &crate::AtRestBody<T>,
        plane: &crate::SplitPlane<T>,
        tol: geom_core::Tol,
    ) -> Result<Body<T>, crate::SplitError> {
        crate::splitting::through_the_join(operand, plane, tol)
    }

    /// The split's reduction over a closed body tier 3 does not finish
    /// (`splitting::reduce`, [`crate::split_reduce`] past its
    /// finished-body gate). For the rows whose pose no finished body
    /// realizes — a prism with a straight profile corner keeps the edge
    /// between its coplanar walls a scaffold — and which pin that
    /// refusal themselves. It reads tier 2 itself: the pipeline past the
    /// gate holds a null edge unreachable, so a body tier 2 refuses
    /// panics here, naming its findings.
    ///
    /// # Errors
    ///
    /// The reduction's refusal.
    pub fn split_reduce_unfinished<T: geom_core::Decide + crate::props::AtRestPolicy>(
        body: &Body<T>,
        plane: &crate::SplitPlane<T>,
        tol: geom_core::Tol,
    ) -> Result<crate::SplitReduction<T>, crate::SplitReduceError> {
        tier_two_clean("split_reduce_unfinished", body);
        crate::splitting::reduce(body, plane, tol)
    }

    /// The premise of the doors past the split's finished-body gate:
    /// `body` passes tier 2.
    #[allow(clippy::panic)]
    fn tier_two_clean<T: geom_core::Real>(door: &str, body: &Body<T>) {
        if let Err(errors) = crate::validate_closed(body) {
            panic!("{door}: tier 2 refuses the body, outside the door's premise: {errors:?}");
        }
    }

    /// The split's carrier gate and vertex sweep over a closed body tier
    /// 3 does not finish ([`crate::vertex_sides`] past its finished-body
    /// gate), for the rows whose fixture relabels a face to a kind the
    /// carrier gate reads, which strands the face's edges below tier 3.
    /// It reads tier 2 itself, as [`split_reduce_unfinished`] does: a
    /// body tier 2 refuses panics here, naming its findings.
    ///
    /// # Errors
    ///
    /// The carrier gate's or the sweep's refusal.
    #[allow(clippy::type_complexity)]
    pub fn split_carrier_gate<T: geom_core::Decide>(
        body: &Body<T>,
        plane: &crate::SplitPlane<T>,
        tol: geom_core::Tol,
    ) -> Result<
        (
            slotmap::SecondaryMap<crate::VertexKey, crate::PlaneSide>,
            Vec<crate::VertexKey>,
        ),
        crate::SplitReduceError,
    > {
        tier_two_clean("split_carrier_gate", body);
        crate::splitting::carrier_gate_and_sides(body, plane, tol)
    }

    /// **Which decision a Boolean refusal came from**, as the executed-
    /// offer harness keys it (`test_utils::offer`), and whether it is the
    /// kernel's own defect. Two refusals of one decision carry one key: a
    /// coincidence's key drops what its door read of the declaration, and
    /// a definite arm carries its decision's (D4 ¶1 (iv): one story).
    pub fn offer_key(err: &crate::BooleanError) -> (String, bool) {
        use crate::{BooleanDecision, BooleanError, BooleanErrorKind, SphereQuestion};
        let decision = decision_key;
        let key = match err {
            BooleanError::Escalated { decision: d, .. }
            | BooleanError::CurvedPierceEscalated { decision: d, .. } => decision(*d),
            BooleanError::SpheresMeet { .. } => {
                decision(BooleanDecision::Sphere(SphereQuestion::Nested))
            }
            BooleanError::CurvedSectorSideUnsupported { .. } => {
                decision(BooleanDecision::PierceCurvature)
            }
            BooleanError::DegenerateTorus { convention, .. } => {
                decision(BooleanDecision::Torus(*convention))
            }
            other => format!("{:?}", other.kind()),
        };
        let defect = err.to_string().contains(geom_core::KERNEL_DEFECT_ENDING)
            || matches!(
                err.kind(),
                BooleanErrorKind::ClassificationInvariant | BooleanErrorKind::JoinDesync
            );
        (key, defect)
    }

    /// A decision's [`offer_key`]: a coincidence's drops what its door
    /// read of the declaration.
    fn decision_key(d: crate::BooleanDecision) -> String {
        match d {
            crate::BooleanDecision::Coincidence(which, _) => format!("Coincidence({which:?})"),
            crate::BooleanDecision::ShellRole { .. } => "ShellRole".to_owned(),
            other => format!("{other:?}"),
        }
    }

    /// Whether two [`offer_key`]s name one decision: equal, or a lever
    /// gate and the reading it meters (the gate passing leaves the same
    /// question to that reading), each pair derived from the map the
    /// raisers route by (`LeverArm::reading`).
    pub fn offer_same_decision(a: &str, b: &str) -> bool {
        use crate::{BooleanDecision, DeclarationRead, LeverArm};
        let key = decision_key;
        a == b
            || LeverArm::ALL.into_iter().any(|gate| {
                let (g, r) = (
                    key(BooleanDecision::LeverArm(gate)),
                    key(gate.reading(DeclarationRead::Moot)),
                );
                (a, b) == (g.as_str(), r.as_str()) || (a, b) == (r.as_str(), g.as_str())
            })
    }

    /// **The refusals an executed offer meets further along whose own
    /// story is not yet true**, each with the row that owns it: the
    /// earlier offer is true of its decision (its decision no longer
    /// refuses at 0.9 × its value), and the later refusal's story is that
    /// refusal's obligation (`test_utils::offer::judge_laters`).
    pub const LATER_STORIES_OWNED: &[(&str, &str)] = &[(
        "Containment",
        "work/inside/point-in-solid-escalation-carries-no-decision.md",
    )];

    /// The offers the executed-offer census counts as run in `sweep`,
    /// whose doors build the solids they need (a ball): each decision's
    /// key, the side of zero its margin lies on, and the child row of
    /// `sweep`'s `offer_rows` that executes it. That suite checks it runs
    /// each one.
    pub const OFFERS_EXECUTED_IN_SWEEP: &[(&str, bool, &str)] = &[
        ("Sphere(Apart)", true, "apart_in_band"),
        ("Sphere(Apart)", true, "apart_in_the_zero_band"),
        ("Sphere(Nested)", true, "nested_in_band"),
        ("Sphere(Nested)", true, "nested_in_the_zero_band"),
        ("Sphere(AgainstPlane)", false, "clear_of_a_slab_in_band"),
        ("Sphere(AgainstPlane)", true, "into_a_slab_in_band"),
        (
            "Sphere(Apart)",
            true,
            "apart_off_axis_in_the_zero_band_union",
        ),
        (
            "Sphere(Apart)",
            true,
            "apart_off_axis_in_the_zero_band_subtract",
        ),
        (
            "Sphere(Apart)",
            true,
            "apart_off_axis_in_the_zero_band_intersect",
        ),
        ("Sphere(Apart)", true, "apart_off_axis_in_band_union"),
        ("Sphere(Apart)", true, "apart_off_axis_in_band_subtract"),
        (
            "Sphere(Nested)",
            true,
            "nested_off_axis_in_the_zero_band_union",
        ),
        (
            "Sphere(Nested)",
            true,
            "nested_off_axis_in_the_zero_band_subtract",
        ),
        ("Sphere(Nested)", true, "nested_off_axis_in_band_subtract"),
        (
            "Coincidence(EdgeOnPlane)",
            false,
            "ball_under_a_slab_in_band_union",
        ),
        (
            "Coincidence(EdgeOnPlane)",
            true,
            "ball_under_a_slab_in_band_subtract",
        ),
        (
            "Coincidence(EdgeOnPlane)",
            false,
            "ball_beside_a_slab_in_band_union",
        ),
        (
            "Coincidence(EdgeOnPlane)",
            true,
            "ball_beside_a_slab_in_band_subtract",
        ),
    ];

    /// The topology-arena lengths of `body`. A free function because
    /// `Body::arena_counts` is `pub(crate)` — an inherent method's
    /// reach follows its own visibility, not its module's, so making
    /// that one `pub` would be public API on [`Body`] in every profile.
    /// This lives behind the door instead.
    pub fn arena_counts<T: Real>(body: &Body<T>) -> ArenaCounts {
        body.arena_counts()
    }

    /// **The section certificate's per-pair report** on the crossings
    /// path: every in-scope pair of `a` × `b` whose boxes overlap, read
    /// as the interior-loop guard reads it over the glue door and the
    /// reduction `op` would run (the glued pairs exempt), as
    /// `(A face, B face, outcome)`. The outcome is `Ok` with each
    /// component's witness (`Unbounded`, `Essential(F|G)`, `Out(F|G)`,
    /// `LoneEvented`) or `Err` with the refusal (`Reach`, `Tangent(..)`,
    /// `Loop`, `Undecided`, `LoneVertex`, `Unread`), spelled by `Debug`.
    ///
    /// # Errors
    ///
    /// The reduction's own refusals.
    pub fn section_report(
        op: crate::BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        tol: geom_core::Tol,
    ) -> Result<Vec<(crate::FaceKey, crate::FaceKey, String)>, crate::BooleanError> {
        Ok(crate::boolean::section_report(op, a, b, tol)?
            .into_iter()
            .map(|p| (p.a_face, p.b_face, format!("{:?}", p.verdict)))
            .collect())
    }

    /// The no-crossings path's certificates on `a` × `b` — the sphere
    /// extent scan, then the section pass — as that path runs them over
    /// `op`'s glue door and reduction, whatever the crossing layer
    /// found. `Ok` with the number of sphere re-cuts the scan asked for
    /// and the number of refusals held for the `On` verdict (pairs the
    /// reduction settled one carrier), which this door does not run.
    ///
    /// # Errors
    ///
    /// The glue door's and the reduction's refusals, and either
    /// certificate's.
    pub fn no_crossings_certificates(
        op: crate::BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        tol: geom_core::Tol,
    ) -> Result<(usize, usize), crate::BooleanError> {
        crate::boolean::no_crossings_certificates(op, a, b, tol)
    }

    /// **The section pass's per-pair report on the no-crossings path**,
    /// spelled as [`section_report`]'s: every pair the pass examines,
    /// read as the pass reads it over `op`'s glue door and reduction,
    /// as `(A face, B face, outcome)`.
    ///
    /// # Errors
    ///
    /// The glue door's and the reduction's refusals.
    pub fn no_crossings_section_report(
        op: crate::BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        tol: geom_core::Tol,
    ) -> Result<Vec<(crate::FaceKey, crate::FaceKey, String)>, crate::BooleanError> {
        Ok(crate::boolean::no_crossings_section_report(op, a, b, tol)?
            .into_iter()
            .map(|p| (p.a_face, p.b_face, format!("{:?}", p.verdict)))
            .collect())
    }

    /// **The section certificate as if the sweep had found nothing**,
    /// spelled as [`section_report`]'s: every pair the no-crossings pass
    /// would examine, no pair exempt and no event. No production path
    /// reads it; a certificate arm's rows take it to reach the no-event
    /// decision on a pose the crossing layer would see.
    ///
    /// # Errors
    ///
    /// The box builder's own errors.
    pub fn no_event_section_report(
        a: &Body<f64>,
        b: &Body<f64>,
        tol: geom_core::Tol,
    ) -> Result<Vec<(crate::FaceKey, crate::FaceKey, String)>, crate::BooleanError> {
        Ok(crate::boolean::no_event_section_report(a, b, tol)?
            .into_iter()
            .map(|p| (p.a_face, p.b_face, format!("{:?}", p.verdict)))
            .collect())
    }

    /// **The interior-loop guard's reading of two carriers, with no
    /// faces**: the section certificate's classification through the
    /// march door, over the reach `(centre, radius)`, then its per-pair
    /// rule with no event, no face describing one sheet and no witness
    /// placed, spelled by `Debug` as [`section_report`]'s outcomes are.
    /// A component the rule clears without a face clears here; any other
    /// refuses undecided.
    #[must_use]
    pub fn section_verdict(
        f: &geom::Surface<f64>,
        g: &geom::Surface<f64>,
        (centre, radius): (geom_core::Point3<f64>, f64),
        band: geom_core::Band,
    ) -> String {
        use crate::boolean::section_cert::{Reach, Sheet, certify, classify};
        let section = classify(
            f,
            g,
            Reach { centre, radius },
            Some(geom_brep::SsiLane::marched()),
            band,
        );
        format!(
            "{:?}",
            certify(&section, false, band, |_| Sheet::Not, |_, _| [None, None])
        )
    }

    /// Does `face` lift to one sheet for the section certificate's W2
    /// (`boolean::ops::ChartCache::describes` answers `Sheet::One`)?
    /// The verdict the certificate reads per face; a torn face, or one
    /// whose walk is unread, answers `false`.
    pub fn face_describes<T: crate::props::AtRestPolicy>(
        body: &Body<T>,
        face: crate::FaceKey,
        band: geom_core::Band,
    ) -> bool {
        let Some(surface) = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
        else {
            return false;
        };
        matches!(
            crate::boolean::ChartCache::default().describes(
                crate::Operand::A,
                body,
                face,
                &surface.clone(),
                band,
            ),
            Ok(crate::boolean::section_cert::Sheet::One)
        )
    }

    /// **Where `face` holds `p`, as the census confirms it**: the read
    /// that places a `(vertex, face)` record at rest, and that the
    /// boolean's carry asks of a divided face's pieces
    /// ([`crate::boolean::face_placement`]). `Ok(Some(_))` placed (a
    /// point decided off a planar face's plane is `Out`), `Ok(None)` a
    /// curved trim's unread remainder, `Err` the escalations or refusal
    /// the census would push.
    ///
    /// # Errors
    ///
    /// The census's own findings for the read, as above.
    pub fn face_holds<T: geom_core::Decide>(
        body: &Body<T>,
        face: crate::FaceKey,
        p: geom_core::Point3<T>,
        band: geom_core::Band,
    ) -> Result<Option<crate::FaceContainment>, Vec<crate::ValidationError>> {
        let mut errors = Vec::new();
        crate::census::face_holds(body, face, p, band, "pm_census_confirm_vf", &mut errors)
            .ok_or(errors)
    }

    /// Is `p`, on `face`'s plane, inside the face? `point_in_solid`'s
    /// planar in-face test, which the ray sweep reaches only through a
    /// hit it decides to take — named here so a row can ask it about a
    /// point directly: `Some(true)` inside, `Some(false)` outside,
    /// `None` on the boundary.
    ///
    /// # Errors
    ///
    /// The walk's own [`crate::PointInSolidError`]; a face that is not
    /// planar is `KindUnsupported`.
    pub fn point_in_face<T: geom_core::Decide>(
        body: &Body<T>,
        face: crate::FaceKey,
        p: geom_core::Point3<T>,
        band: geom_core::Band,
    ) -> Result<Option<bool>, crate::PointInSolidError> {
        let (_, normal) = crate::boolean::solid_contain::face_plane(body, face)?;
        crate::boolean::solid_contain::point_in_face(body, face, normal, p, band)
    }
}
#[cfg(test)]
mod r2_probes;
#[cfg(test)]
mod tier3_tests;
pub mod transform;
#[cfg(test)]
mod turn_rows;
pub mod validate;
mod wedge_reach;
pub use wedge_reach::{WedgeReach, edge_faces, edge_wedge_reach, wedge_reach};
pub(crate) mod vertex_section;

pub use body::Body;
pub use boolean::{
    AimedPair, AlongFace, Apart, ArcEndGaps, BoolNullEdgeRecord, BooleanBody, BooleanDecision,
    BooleanError, BooleanErrorKind, BooleanInputs, BooleanJoin, BooleanMerge, BooleanNaming,
    BooleanOp, BooleanReduction, BooleanResult, BooleanResultKind, CarriedRecord, CarrierDesc,
    CarrierEqError, CarrierRelation, CarryRefusal, Cell, Coincide, CoincidenceMeasure,
    CompletedPolygonPair, ConsumedExtent, Contact, ContactKind, ContactPairError, ContactRecords,
    ContainDecision, ContainError, Contradiction, DeclarationRead, DiscardRow, EdgeCarrierSite,
    EdgeInFaceGap, EdgeJoin, EdgePieceClass, EeContact, EfContact, FaceContainment, FaceDescent,
    FfContact, Fusions, GlueVerdict, JOIN_LEVER, JOIN_SUBJECT, JoinReading, JoinRefusal,
    JoinUndecided, JudgedRow, LeverArm, LineWedgeFault, MemberFace, MemberInputs, MemberLineage,
    NaryEnd, NaryJudgement, NaryMember, NaryOp, NaryRefusal, NaryRun, NaryStep, NodeWhy,
    NullEdgePairRecord, Operand, OperandKeys, OperandRecords, PairFace, PairRefusalSite, PairSite,
    PairUnread, PierceRingRecord, PlaneDesc, PlaneEqError, PlaneRelation, PlaneRung,
    PointInSolidError, ReductionContacts, RefusedRow, SectionUnbuilt, SectorRead, SectorRung,
    SelfCheck, SettleError, Settler, Settling, ShellOrientation, SideCode, SlitFaceGap,
    SlitLineGap, SolidContainment, SolidFaces, SphereQuestion, SpiricLoops, SubstitutedFace,
    SweepStrategy, SweepTrace, TorusConvention, UncutFace, UnreadPartner, VeContact, VfContact,
    VoidContainment, VoidEvidence, VoidInsertError, VoidInserted, VvContact, WallRung,
    boolean_op_with, boolean_reduce, carrier_eq, contfp, curved_face_containment, decision_words,
    face_carrier, flush_pair_relation, insert_void, insert_voids, intersect,
    is_conventional_vertex, join_covers, joinable_vertices, joined_cell, joined_edge, nary_boolean,
    nary_boolean_apart, oriented_plane_eq, point_in_solid, point_in_solid_faces, point_in_solid_of,
    rest_locus, subtract, tangent_pair_relation, union,
};
pub use joint::{Deck, JointElement};
pub use lineage::{
    ConeCopies, Division, FaceDivisions, Fate, Lineage, LineageError, LineageKey, LineageRows,
    Minted, Remainder, RowDefect, RowRefusal, SourceGeometry, SplitEdgeDivisions, Substitutions,
};
pub use side_reader::{FAMILY as SIDE_READER_FAMILY, SetAside, SetAsideWhy};
// The contact vocabulary (C3/C4), defined once at the lowest crate
// that can hold it: upward layers RE-EXPORT these, never redefine.
#[cfg(feature = "sweep-testing")]
pub use boolean::{
    PlantedDegradation, SharedClass, StagePoints, arm_output_stage_points, forget_branches,
    swap_split_roles, sweep_records, sweep_split, sweep_traces, sweep_traces_with_pad,
    take_output_stage_points, take_shared_points,
};
#[cfg(feature = "sweep-testing")]
pub use chord_join::face_azimuth_window_traces;
// The census's idealized/realized pair (its `Candidates`): the
// vocabulary always, the door on the boolean sweep's terms.
pub use census::{
    CensusStrategy, CensusTrace, FlushAtRest, RecordedAtRest, SweepPairs, census_cell_solid,
    census_contact_row, census_entity_solid, census_interference_solids, census_rest_decision,
    census_uncleared_solids, line_segments_meet, record_at_rest, segments_parallel,
};
#[cfg(feature = "sweep-testing")]
pub use census::{census_traces, census_traces_planted};
pub use contact::{
    CONTACT_RECOURSE, CONTRADICTION_REASON, CONTRADICTION_RECOURSE, ContactFinding, ContactPair,
    ContactRefusal, GlueClass,
};
pub use entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
pub use euler::{BadArgument, EulerOpError, RechartDoor};
pub use link::{Link, LinkRefusal, Material, Round, TouchUnread};
// The types that appear in this crate's own operator signatures, so a
// consumer of the ops needs no direct geom-* imports for the common
// path (the full geometry vocabulary still lives in those crates).
pub use chart::{Chart, ChartKind};
pub use chart_iso::{TravKind, classify_kind, iso_side_starts, mid_azimuth, unwrap_near};
pub use chart_region::{
    CarrierPremise, ChartOverlap, ChartRegionError, RegionLane, WITNESS_CELL_CAP,
    WITNESS_SEGMENT_CAP, chart_region_overlap, glued_pair_overlap,
};
pub use clearance::ClearanceLane;
pub use coherence::{
    CoherenceCondition, CoherenceFinding, CoherenceReport, StructureRead, Unexaminable, Unexamined,
    examine_chart_coherence, gap_is_noise,
};
pub use coincidence::{
    Backing, Cited, Cites, Coincidence, DecisionSite, Discharge, Relation, RowCell,
};
pub use geom::Curve3;
pub use geom::Surface;
pub use geom_brep::{
    CertifyError, ChartCurve, ChartWindow, CurveSelector, EdgeAuthority, EdgeCurve, EdgeCurveSpec,
    EdgeDescription, EdgeDescriptionSpec, Pcurve, PcurveCache, PcurveCertifyError, SharedRow,
    Structural, SurfacePair,
};
pub use geometry::{CurveKey, PointKey, SurfaceKey};
pub use instance::{
    GraftKeys, graft_disjoint, graft_disjoint_all, graft_disjoint_all_keyed, per_part_gate_owed,
};
pub use merge_faces::{
    DihedralReading, EdgeDescribeFailure, MergeCoplanarError, MergeCoplanarOutcome, MergeDecision,
    MergeKind, MergedGroup, OutlineVerdict, SkippedMerge,
};
pub use null::CurveGeom;
pub use offset_axial::{is_axial, offset_charts_together};
pub use offset_derive::{CornerVerdict, SectionLane, SectionVerdict};
pub use offset_general::offset_surfaces_together;
pub use offset_nappe::{Nappe, face_nappe, group_nappe};
pub use offset_together::{ChartMove, offset_planes_together};
pub use pcurves::{
    LoopWinding, PcurveMintError, SiteRowRefusal, loop_windings, mint_pcurves, mint_pcurves_of,
    pcurve_of,
};
pub use props::{
    AtRestOutcome, AtRestPolicy, CertifiedBody, Certifier, MassMeasurement, MassProperties,
    MassPropsError, QuadLane, RoleNames, Settle, SettleVerdict, ShellClassification,
    ShellClassifyError, ShellClassifyPayload, ShellDoor, ShellRole, SignCertificate, SignLimit,
    SignNames, Stop, Unmet, VolumeEnclosure, classify_shells, classify_shells_of,
    classify_shells_structural, mass_properties, mass_properties_structural,
};
pub use provenance::{Provenance, SplitLineageCycle};
// The query VOCABULARY rides at the root like every other type;
// the query DOORS (materializers, predicates) keep their module
// identity, like `readback`'s.
pub use face_boxes::{FaceBox, FaceBoxes};
pub use mirror::{Reflection, mirror_body};
pub use pieces::PieceSortError;
pub use query::{
    CurveKind, CurveKindSet, POSE_UNIT_NORM, RimBreak, RimError, SurfaceKind, SurfaceKindSet,
};
pub use readback::{EdgeSide, EdgeSides, EulerCounts, EulerParityError, Pose, ReadbackError};
pub use replace_face::{
    OffsetOutcome, ReplaceFaceError, replace_face_offset, replace_faces_offset,
};
#[cfg(any(feature = "test-support", feature = "sweep-testing"))]
#[doc(hidden)]
pub use replace_face::{
    offset_charts_edge_plans_for_tests, offset_corner_arms_for_tests, offset_edge_plans_for_tests,
    offset_edge_specs_for_tests,
};
pub use revert::RevertError;
pub use separation::{Separation, SolidOwners, SolidSeparation, SolidsMeet};
pub use shell::{
    HoleRim, RimNaming, RimShell, ShellError, ShellNaming, ShellRetired, Shelled, shell, shell_open,
};
pub use splitting::{
    ApexSectionReading, ConicCrossingsCase, ConicRootFault, CrossingDecision, Escalation,
    KnifeEdge, KnifeEdgeSite, LoopContainment, LoopDecision, NullEdgeRecord, OffPlane,
    OffPlaneCause, PlaneSide, PointInLoopError, Section, SectionEdge, SectionEndReading,
    SectionError, SectionGermFault, SectionGermReading, SectionPinMiss, SectionPolygon,
    SectionRegion, SectorEntry, SectorEntryKind, SectorReading, SplitContacts, SplitError,
    SplitFinishError, SplitJoinError, SplitPart, SplitPlane, SplitReduceError, SplitReduction,
    SplitResult, TangencyReadings, Uncrossable, UncrossableCarrier, classify_neighborhood,
    plane_section, point_in_loop, split, split_reduce, vertex_sides,
};
pub use transform::{TransformError, transform_rigid};
pub use validate::{
    AtRestBody, CensusContact, CensusSubject, CensusUnsupportedCause, ContactMark,
    CrossingSideVerdict, OrientationUnread, RingContact, RingPairContact, SingularityLies,
    StaleDeclaration, Undecidable, Unfinished, ValidationError, WedgeCheck, contact_marks,
    contact_marks_structural, validate, validate_closed, validate_geometric,
    validate_geometric_certificate, validate_geometric_certificate_structural,
    validate_geometric_structural, validate_pseudomanifold, validate_pseudomanifold_certificate,
    validate_pseudomanifold_certificate_structural, validate_pseudomanifold_structural,
};

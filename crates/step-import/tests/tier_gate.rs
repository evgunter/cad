//! **M7-7 acceptance: the shared at-rest gate over the whole corpus**
//! (issue #260, ruling (a)).
//!
//! Since M7-7, `import_step` hands each `MANIFOLD_SOLID_BREP` on its
//! own body, and then the assembled body, to
//! `topo::validate_geometric` — the kernel's own at-rest validator,
//! the same function a native body's caller runs — and ships only
//! bodies it passes. This suite is the per-file record of what that
//! means, for EVERY committed STEP file the workspace holds: the two
//! fixture roots are walked, so a fixture added without a row here
//! turns this suite red rather than quietly escaping the gate.
//!
//! Three things are asserted per file at the file's own declared ε_in,
//! and additionally at the overrides 1e-6 / 1e-12 for the files
//! [`EPS_IN_SWEPT`] names — measured 2026-08-22 to be the only ones
//! whose outcome moves with ε_in at all. See that constant for the
//! measurement and for what the reduction gives up:
//!
//! * **the disposition** — solid, wireframe, or a typed refusal with
//!   its reason. Every file in [`CORPUS`] not marked `EpsSensitive`
//!   holds ONE disposition across the whole matrix, and that
//!   constancy is the claim; the counts are [`CORPUS`]'s own length
//!   and the `EpsSensitive` markers in it, so they are not
//!   transcribed here. The `EpsSensitive` files are pinned cell by
//!   cell in [`EPS_ROWS`], each cell carrying the live signature of the
//!   sub-reason that actually fires there — because a row that may
//!   Pass at one ambient ε and refuse typed at another can otherwise
//!   be green for the wrong reason. No ε is ever special-cased into
//!   silence: a refusal that fires at a tighter tolerance is pinned AS
//!   the honest posture, and the reason it moved is written down;
//! * **tier-validity of every shipped body, positively** — the gate is
//!   re-run on the body `import_step` handed out. That is redundant
//!   only while the gate is wired: delete or narrow the call and these
//!   rows are what catches the invalid body going out the door; and
//! * **the census of every shipped body**, which is what makes the
//!   rows able to see entity loss at all (R1 MINOR-2: a `take(1)` in
//!   `build_body` used to leave this whole suite green). Four corpus
//!   files carry two solids each — `compound_two`,
//!   `twobody_importexport`, `cq_red_cube_blue_cylinder`,
//!   `kiss_assembly` — so the solid counts are also the standing
//!   evidence that the per-solid pass runs on real corpus geometry.
//!
//! **Scope** (R1 MINOR-1, RESOLVED in M9-2): the aggregate gate is
//! now the tier-3′ form, recording what it finds between solids
//! (D7 step 4, `topo::record_at_rest`) — an imported assembly whose
//! parts touch with vertex/line/planar boundary evidence certifies
//! with its touches recorded (`kiss_assembly`'s row below and
//! `review_r1_tier_gate_probes.rs`), and one whose parts overlap
//! refuses; cross-solid
//! curved proximity and nested instance extents refuse UNDECIDABLE
//! (the census's conservative backstop); the full class-by-class
//! reach is the census module docs' envelope statement.
//!
//! **Measured (M7-7), at the ambient default:** no committed corpus
//! file fails the gate. 44 solids pass, 8 files refuse for reasons that predate this
//! unit (one of them, `band_c180`, at the gate itself — the inside-out
//! torus band, refusing now through the general mechanism that
//! replaced its band-only backstop), and one file is a wireframe.
//! The one body class the gate newly refuses is the
//! rational-walled loft, which has no committed fixture and whose row
//! lives in `nurbs_import.rs`.
//!
//! **S58 / #649 (2026-08-19) added four rows**, #649's own fixtures:
//! `iso-rect/cross.step` and `iso-rect/tee.step`, valid solids whose
//! cylinder walls have a plus- and a T-shaped domain, and
//! `iso-rect/rect.step` / `iso-rect/xsplit.step`, the controls. All
//! four pass: a cylinder wall's flux is its chart Green form, which
//! measures any rim-and-ruling domain exactly (the cross once measured
//! 19% low under the span-sum rule).
//!
//! **Issue 723 (2026-08-29) added the two `halfcap/` rows; one of
//! them is a newly-passing body class.** Both twins are the same
//! half-of-a-spherical-cap solid whose sphere face's meridian side is
//! a pole-crossing great-circle arc. `halfcap.step` (the arc split by
//! one ordinary vertex) USED to pass this gate and then measure 47%
//! low with `pad = 0.0`; `halfcap_nosplit.step` USED to refuse
//! `DegenerateFace` on the endpoint fold's `lo == hi`. With the
//! sphere's `v`-extent derived from each arc's stored span, both pass
//! and `halfcap_pole.rs` holds both to the exact closed-form volume.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use Disposition::{EpsSensitive, Pass, Refused, Wireframe};
use geom_core::{Band, Tol};
use step_import::{ImportOptions, StepImport, StepImportError, import_step};
use test_utils::vacuity::Exposure;

/// What a corpus file does at import, at every tolerance in the sweep.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Disposition {
    /// Imports as a solid, and the shipped body passes the gate, with
    /// this census: (solids, shells, faces, edges, vertices).
    ///
    /// The census is not decoration. A disposition alone cannot see a
    /// reader that silently drops solids — the R1 review's `take(1)`
    /// mutation of `build_body` left this suite entirely green — and
    /// the solid count is exactly the number the per-solid gate now
    /// iterates, so a body arriving with fewer solids than the file
    /// states means fewer solids were gated.
    Pass(usize, usize, usize, usize, usize),
    /// Imports as a curve-set wireframe (no body, nothing to gate).
    Wireframe,
    /// Refuses typed; the string is a distinctive fragment of the
    /// refusal's own message, so the ROW says why, not just that.
    Refused(&'static str),
    /// This file's disposition genuinely MOVES with the ambient ε, and
    /// its cells are pinned one by one in [`EPS_ROWS`] — a marker, not
    /// an outcome, so no cell of it can be satisfied by accident.
    EpsSensitive,
}

/// The ambient tolerances this suite pins cell by cell — the hosted
/// matrix's three rows (`CAD_TOLERANCE_EPS` unset, `1e-6`, `1e-12`).
const PINNED_AMBIENT: [f64; 3] = [geom_core::tolerance::DEFAULT_EPS, 1e-6, 1e-12];

/// The `eps_in` overrides swept per ambient row, tagged as the failure
/// messages name them. **Index 0 is the default row** — the file's own
/// declared ε_in, the one every corpus file is swept at
/// unconditionally; [`eps_in_rows_for`] slices from here.
const EPS_IN_ROWS: [(&str, Option<f64>); 3] =
    [("file", None), ("1e-6", Some(1e-6)), ("1e-12", Some(1e-12))];

/// **The files swept at all three ε_in tags.** Every other corpus file
/// is imported once, at its own declared ε_in.
///
/// This is the 2026-08-13 audit's dm1 exemption, generalised on a
/// measurement rather than on cost alone.
///
/// # What the ε_in sweep is FOR, and why one file carries it
///
/// ε_in is the INTERPRETATION tolerance — the budget the adoption
/// ladder spends deciding what a file's carriers certify AS. It can
/// only change a disposition for a file with something sitting near
/// that budget. Every committed file but one states itself to full
/// double precision and certifies with enormous margin, so its
/// disposition is a constant function of ε_in — which the sweep was
/// re-measuring 61 times a run, at every ambient band. `ftc11_uref_off`
/// is the exception BY CONSTRUCTION: it is the deliberately degenerate
/// band fixture, whose ~1.6e-6 m seam residual is engineered to sit
/// where a tolerance decides it.
///
/// # The measurement, taken rather than assumed (2026-08-22)
///
/// Every file in [`CORPUS`] was imported at all three ε_in tags at all
/// three [`PINNED_AMBIENT`] bands — 558 imports — and the outcomes
/// compared in FULL: the whole census for a solid, the whole refusal
/// message for a refusal, not the coarse [`Disposition`] class. Across
/// the entire corpus **exactly one file's outcome moves with ε_in**,
/// and it is `ftc11_uref_off` (`file` imports, its generator a wrap
/// edge at the rim vertices' azimuth; `1e-6` and `1e-12` reached the
/// intersection arm, the param span, or passed — see [`EPS_ROWS`],
/// where since U-lever they pass at every band but 1e-6). The other 61
/// are invariant to the byte.
///
/// Comparing full messages rather than classes is what makes that
/// negative result worth acting on: a file that changed WHY it refused
/// while staying `Refused` would have shown up here.
///
/// # What it costs, said plainly, and what is NOT lost
///
/// Given up: the EXECUTED per-run re-measurement of ε_in-invariance for
/// 61 files. It is a recorded finding now, not a live one, exactly as
/// dm1's has been since 2026-08-13. A file that BECOMES ε_in-sensitive
/// would go unnoticed until someone re-takes the measurement above.
///
/// Not given up: the AMBIENT sweep, which is the axis these files
/// actually move on, and which still runs on every file at every band;
/// every file's disposition, census and refusal reason at its own
/// declared ε_in, every run; and the full ε_in sweep on the one file
/// that is ε_in-sensitive, still pinned cell by cell in [`EPS_ROWS`].
///
/// Measured saving: ~2.8 s of this row's ~8.1 s at CI's opt-2 settings
/// (4-vCPU box, 2026-08-22; CI's 2-vCPU runner differs). `dm1` was
/// already exempt and is not in the figure.
const EPS_IN_SWEPT: [&str; 1] = [FTC11];

/// The `eps_in` rows THIS file is swept at — see [`EPS_IN_SWEPT`] for
/// the measurement that decides which files get all three.
fn eps_in_rows_for(rel: &str) -> &'static [(&'static str, Option<f64>)] {
    if EPS_IN_SWEPT.contains(&rel) {
        &EPS_IN_ROWS
    } else {
        &EPS_IN_ROWS[..1]
    }
}

/// **The ε-row pins.** The `EpsSensitive` corpus files' dispositions are a function
/// of the ambient ε, and hiding that behind one row per file would make
/// the suite either red for an honest reason or green for a wrong one.
/// Each cell is `(file, ambient ε, eps_in row, disposition)`; the
/// coverage test below holds this table to exactly one cell per
/// (`EpsSensitive` file × [`PINNED_AMBIENT`] × the ε_in rows that file
/// is actually swept at, per [`eps_in_rows_for`]), and to nothing else,
/// so a pin can neither go missing nor rot unread — and a cell for a
/// combination that no longer runs is red, not silently unread.
///
/// Both movements are the kernel deciding HONESTLY at the tolerance it
/// was given, and the fragments below are each sub-reason's own live
/// signature, never the shared preamble:
///
/// * **`ftc11_uref_off`** is the deliberately-degenerate band fixture:
///   its rim vertices sit ~1.6e-6 m off the surface's `u_ref` azimuth.
///   At its own ε_in the band's minted generator runs through them and
///   is the wall's wrap edge there (D1: a wrap edge sits where the
///   construction cut), so it imports at every ambient band. At the
///   finer ε_in overrides the offset is past the mint's vertex budget,
///   so the mint splits the rims at the `u_ref` azimuth and leaves a
///   1.6e-6 m edge beside each rim vertex: at ambient 1e-6 that span
///   lands INSIDE the ambiguity band and the refusal is an ESCALATION;
///   at ambient 1e-9 and 1e-12 it is outside every band, and the file
///   imports. Until the wedge between an edge's faces was levered at
///   the faces' extent (`topo::wedge_reach`, TANG's U-lever) the
///   1e-9 cells refused there too: the Intersection transversality
///   precondition read the two walls' angle at the 1.6e-6 m edge's own
///   length, where they read as one ("tangent planes coincide"); at the
///   walls' extent they cross clearly.
const EPS_ROWS: [(&str, f64, &str, Disposition); 39] = [
    // -- tests/fixtures/wild/stepcode/TAIL_TURBINE.stp -----------------
    // Its cylinder × sphere section, at its own ε_in: the edge across the
    // marched loop's start at the default and fine bands, its sibling's
    // certificate at 1e-6.
    (
        TAIL_TURBINE,
        1e-9,
        "file",
        Refused(TAIL_TURBINE_SECTION_ENDS),
    ),
    (
        TAIL_TURBINE,
        1e-6,
        "file",
        Refused(TAIL_TURBINE_SECTION_UNCERTIFIED),
    ),
    (
        TAIL_TURBINE,
        1e-12,
        "file",
        Refused(TAIL_TURBINE_SECTION_ENDS),
    ),
    // -- tests/fixtures/cert1-r1/nearpolar_*.step ---------------------
    // The AMBIENT sweep only, at the files' own ε_in (they state
    // themselves to full double precision). At ambient 1e-6 both
    // twins refuse at interpretation — the file is read no finer than
    // the run, and the rim the build makes lies in that band of the
    // printed one (~2.2e-6 m) without reading one with it; at the default
    // and fine bands both certify with the exact closed-form volume
    // (`cert1_r1_import_probes.rs` holds the value), the split twin
    // with its ordinary vertex joined, so with the no-split census.
    // Each meridian arc crosses the pole, which import makes a vertex
    // (`chart_singularity_vertex.rs`), so the census is that of the arc
    // split there.
    (NEARPOLAR_SPLIT, 1e-9, "file", Pass(1, 1, 3, 4, 3)),
    (
        NEARPOLAR_SPLIT,
        1e-6,
        "file",
        Refused(NEARPOLAR_RIM_IN_BAND),
    ),
    (NEARPOLAR_SPLIT, 1e-12, "file", Pass(1, 1, 3, 4, 3)),
    (NEARPOLAR_NOSPLIT, 1e-9, "file", Pass(1, 1, 3, 4, 3)),
    (
        NEARPOLAR_NOSPLIT,
        1e-6,
        "file",
        Refused(NEARPOLAR_RIM_IN_BAND),
    ),
    (NEARPOLAR_NOSPLIT, 1e-12, "file", Pass(1, 1, 3, 4, 3)),
    // -- tests/fixtures/declined/cyl_declined_loose.step --------------
    // The declined rim nudged 8e-11 m: at the fine band the circle
    // composite reads its stand-off about 2.7 times over and refuses
    // in band, where three samples read it Zero
    // (`work/exch/the-printed-on-built-certificate-reads-curved-pairs-loosely.md`).
    (CYL_DECLINED_LOOSE, 1e-9, "file", Pass(1, 1, 3, 3, 2)),
    (CYL_DECLINED_LOOSE, 1e-6, "file", Pass(1, 1, 3, 3, 2)),
    (
        CYL_DECLINED_LOOSE,
        1e-12,
        "file",
        Refused(CYL_DECLINED_LOOSE_IN_BAND),
    ),
    // -- tests/fixtures/band/ftc11_uref_off.stp -----------------------
    (FTC11, 1e-9, "file", Pass(1, 1, 6, 12, 8)),
    // Read at an ε_in of 1e-6, a rim vertex 1.6e-6 m from the seam
    // azimuth lies in the file's band: whether it is the seam's vertex
    // is undecided, and the import refuses that ambiguity.
    (FTC11, 1e-9, "1e-6", Refused(FTC11_SEAM_VERTEX_IN_BAND)),
    (FTC11, 1e-9, "1e-12", Pass(1, 1, 6, 12, 8)),
    (FTC11, 1e-6, "file", Pass(1, 1, 6, 12, 8)),
    // At ambient 1e-6 the file is read no finer than the run, so the
    // same vertex lies in band whatever ε_in it declares or is given.
    (FTC11, 1e-6, "1e-6", Refused(FTC11_SEAM_VERTEX_IN_BAND)),
    (FTC11, 1e-6, "1e-12", Refused(FTC11_SEAM_VERTEX_IN_BAND)),
    (FTC11, 1e-12, "file", Pass(1, 1, 6, 12, 8)),
    (FTC11, 1e-12, "1e-6", Refused(FTC11_SEAM_VERTEX_IN_BAND)),
    (FTC11, 1e-12, "1e-12", Pass(1, 1, 6, 12, 8)),
    // -- tests/fixtures/halfcap/halfcap_eps{6,7}.step (Ev, PR 4251) ---
    // The AMBIENT sweep only, at the files' own ε_in. The split vertex
    // sits 1e-8 m (eps6) and 1e-9 m (eps7) off the pole, and whether
    // it IS the pole is import's first reading, at the coarser of the
    // file's band and the run's: at 1e-6 it reads AT the pole, so the
    // pole is already a vertex and the file ships as stated; at the
    // default band it lies in the band of the pole and the import
    // refuses rather than guess; at 1e-12 it is off the pole, so the
    // arc is split there and the vertex, a regular point, joined.
    (HALFCAP_EPS6, 1e-9, "file", Refused(HALFCAP_EPS6_POLE)),
    (HALFCAP_EPS6, 1e-6, "file", Pass(1, 1, 3, 4, 3)),
    (HALFCAP_EPS6, 1e-12, "file", Pass(1, 1, 3, 4, 3)),
    (HALFCAP_EPS7, 1e-9, "file", Refused(HALFCAP_EPS7_POLE)),
    (HALFCAP_EPS7, 1e-6, "file", Pass(1, 1, 3, 4, 3)),
    (HALFCAP_EPS7, 1e-12, "file", Pass(1, 1, 3, 4, 3)),
    // -- tests/fixtures/wild/stepcode/dm1-id-214.stp (#327) -----------
    // The AMBIENT sweep only, at this file's own ε_in: three cells,
    // all at the assembled body's at-rest gate. Every component adopts
    // and mints its pcurves (wall `#382`'s sub-patch rows hold their
    // arc rims at end weights 1 and 2, and the l-bracket imports alone,
    // `r1_dm1_probe`), and the bolts and nuts sit on the bracket, so
    // two instances share vertex positions the gate finds unrecorded.
    //
    // It was nine cells until the 2026-08-13 test-time audit — the six
    // dropped ones were the `1e-6` and `1e-12` ε_in tags, and they all
    // held the SAME disposition as the `file` tag beside them, at every
    // band: ε_in moved nothing, which was itself the measurement
    // (recognition certifies this file's carriers with ~14 decades of
    // margin, so no interpretation budget in this sweep changes what is
    // promoted). That measurement is now RECORDED here rather than
    // re-executed every run; see [`eps_in_rows_for`] for what the
    // three imports it cost were buying and what was given up.
    (DM1, 1e-9, "file", Refused(DM1_CONTACT_UNRECORDED)),
    (DM1, 1e-6, "file", Refused(DM1_CONTACT_UNRECORDED)),
    (DM1, 1e-12, "file", Refused(DM1_CONTACT_UNRECORDED)),
    // -- tests/fixtures/poleguard/*.step (issue 896) ------------------
    // The AMBIENT sweep only, at the files' own ε_in (they state
    // themselves to full double precision). The twins' near-pole feature
    // is a circle of radius 0.9e-9 m by construction, which the band the
    // file is read at, `(e, K·e)` for `e` the larger of its ε_in
    // (1e-10 m) and the run's ε, cannot tell from zero at any of the
    // three bands: it reads zero at 1e-9 and 1e-6 and in band at 1e-12.
    // So each twin refuses at interpretation, where the circle is read,
    // before any span is certified or the pole beside it placed —
    // `poleguard.rs` holds the route argument.
    (POLEBAND, 1e-9, "file", Refused(POLE_TWIN_CIRCLE_UNREAD)),
    (POLEBAND, 1e-6, "file", Refused(POLE_TWIN_CIRCLE_UNREAD)),
    (POLEBAND, 1e-12, "file", Refused(POLE_TWIN_CIRCLE_UNREAD)),
    // The ε-relative sibling's circle is 0.9e-12 m, inside its file's
    // ε_in (1e-10 m) at every band, so it refuses the same way.
    (
        POLEBAND12,
        1e-9,
        "file",
        Refused(POLE_SIBLING_CIRCLE_UNREAD),
    ),
    (
        POLEBAND12,
        1e-6,
        "file",
        Refused(POLE_SIBLING_CIRCLE_UNREAD),
    ),
    (
        POLEBAND12,
        1e-12,
        "file",
        Refused(POLE_SIBLING_CIRCLE_UNREAD),
    ),
    (POLEFRUSTUM, 1e-9, "file", Refused(POLE_TWIN_CIRCLE_UNREAD)),
    (POLEFRUSTUM, 1e-6, "file", Refused(POLE_TWIN_CIRCLE_UNREAD)),
    (POLEFRUSTUM, 1e-12, "file", Refused(POLE_TWIN_CIRCLE_UNREAD)),
];

const FTC11: &str = "tests/fixtures/band/ftc11_uref_off.stp";
const CYL_DECLINED_LOOSE: &str = "tests/fixtures/declined/cyl_declined_loose.step";
const CYL_DECLINED_LOOSE_IN_BAND: &str = "(step_edge_on_description), in the band it is read at";
/// FTC11's rim vertex beside the seam azimuth, read in the file's band.
const FTC11_SEAM_VERTEX_IN_BAND: &str = "(step_band_seam_vertex) lies in the";
pub(crate) const TAIL_TURBINE: &str = "tests/fixtures/wild/stepcode/TAIL_TURBINE.stp";
/// TAIL_TURBINE's refusal at the default and fine bands: its two records
/// of one cylinder are one, and the cylinder × sphere section edge `#217`
/// reads its built end before its start
/// (`work/importgate/a-marched-section-edge-across-its-loops-start-refuses-at-import.md`).
const TAIL_TURBINE_SECTION_ENDS: &str = "edge #217: a built spline carrier whose end reads at or \
     before its start";
/// TAIL_TURBINE's refusal at ambient 1e-6: the other edge of that
/// section, `#170`, does not certify.
const TAIL_TURBINE_SECTION_UNCERTIFIED: &str = "edge #170: no intensional description certifies";
pub(crate) const NEARPOLAR_SPLIT: &str = "tests/fixtures/cert1-r1/nearpolar_split.step";
const NEARPOLAR_NOSPLIT: &str = "tests/fixtures/cert1-r1/nearpolar_nosplit.step";
/// The nearpolar twins' coarse-band reason: at ambient 1e-6 the file is
/// read no finer than the run, and the rim the build makes from its plane
/// and its sphere lies within that band of the printed rim without
/// reading one with it.
const NEARPOLAR_RIM_IN_BAND: &str =
    "(step_edge_on_description) lies in the band the file is read at";
const DM1: &str = "tests/fixtures/wild/stepcode/dm1-id-214.stp";
const POLEBAND: &str = "tests/fixtures/poleguard/poleband.step";
const POLEBAND12: &str = "tests/fixtures/poleguard/poleband_eps12.step";
const POLEFRUSTUM: &str = "tests/fixtures/poleguard/polefrustum.step";
/// The poleguard twins' near-pole circle, whose 0.9e-9 m radius the band
/// the file is read at cannot tell from zero.
const POLE_TWIN_CIRCLE_UNREAD: &str = "a circle's radius, which must be positive: the file states \
     8.999999995834905e-10 m, which the band it is read at";
/// The ε-relative sibling's near-pole circle, 0.9e-12 m in radius.
const POLE_SIBLING_CIRCLE_UNREAD: &str = "a circle's radius, which must be positive: the file \
     states 9.000006867897335e-13 m, which the band it is read at";
// dm1's `#389` polyline gap ("edge #389: no intensional description
// certifies", with an empty attempt list) is RETIRED, by execution and
// not by unreachability: degree-1 carriers promote to `Curve3::Line`
// (#388) and a promoted slit holds its wall's boundary-column
// candidate in either traversal order —
// `r1_dm1_probe::the_l_bracket_alone_adopts_its_reversed_slit` imports
// the component that carries `#389` and gets past every polyline edge.
/// dm1's frontier at every band: the assembled body's at-rest gate,
/// on a vertex two instances share. Named specifically so a refusal
/// that drifted to a different door cannot stand in.
const DM1_CONTACT_UNRECORDED: &str = "is an unrecorded contact";
const HALFCAP_EPS6: &str = "tests/fixtures/halfcap/halfcap_eps6.step";
const HALFCAP_EPS7: &str = "tests/fixtures/halfcap/halfcap_eps7.step";
/// The halfcap near-pole twins' default-band refusal: the split vertex
/// lies in the RUN's band of the pole, so whether the pole is a vertex
/// of the face is undecided, and the import door ends the sized
/// decision with its margin and the tolerance that decides it. Pinned
/// as the whole message, face, margin, band and number included.
const HALFCAP_EPS6_POLE: &str = "step import: face #56: whether a sphere pole or cone apex, \
     or where a meridian minted from one meets a face's boundary, lies on that boundary or at \
     one of its vertices is undecided: margin 9.999999999789241e-9 \
     lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the face's boundary, or its \
     vertex, onto that point or clear of it, or, if this distance is intended, tighten \
     the tolerance below 9.999e-10 m";
/// As [`HALFCAP_EPS6_POLE`], for the twin a decade nearer the pole.
const HALFCAP_EPS7_POLE: &str = "step import: face #56: whether a sphere pole or cone apex, \
     or where a meridian minted from one meets a face's boundary, lies on that boundary or at \
     one of its vertices is undecided: margin 1.0000000011961913e-9 \
     lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the face's boundary, or its \
     vertex, onto that point or clear of it, or, if this distance is intended, tighten \
     the tolerance below 1e-10 m";

/// `nist_ftc_09`'s refusal at every band. Its five fillet cylinders bite
/// into their planes by 1.5 to 13 µm, tangent at the file's band
/// (ε_in ≈ 34 µm) and crossing at the run's, and the snap catalogue makes
/// each tangency exact (`snap.rs`'s
/// `nist09s_five_cylinders_snap_and_its_printed_tangent_edges_are_read_off_them`).
/// The writer printed each of those edges as a crossing line of the pair
/// as it wrote it, `√(2·r·gap)` from the ruling, so the first, `#4950`,
/// lies 180 µm off its description: in the file's band, the file's
/// ambiguity.
const NIST09_PRINTED_OFF_ITS_TANGENCY: &str = "entity #4950: the file's statements miss each \
     other by 0.0001802051876711938 m (step_edge_on_description), in the band it is read at";

/// Every committed STEP file, with the disposition measured at M7-7.
/// Paths are relative to this crate's manifest directory (the `../`
/// rows are `step-export`'s corpus, which this crate imports from).
const CORPUS: &[(&str, Disposition)] = &[
    ("tests/fixtures/band/band_a.stp", Pass(1, 1, 2, 4, 2)),
    ("tests/fixtures/band/band_a180.stp", Pass(1, 1, 2, 4, 2)),
    ("tests/fixtures/band/band_b180.stp", Pass(1, 1, 2, 4, 2)),
    (
        "tests/fixtures/band/band_c180.stp",
        Refused("shared at-rest validation gate"),
    ),
    (
        "tests/fixtures/band/band_d180.stp",
        Refused("ORIENTATION-INVERTED cylinder band"),
    ),
    (
        "tests/fixtures/band/band_d_invcyl.stp",
        Refused("ORIENTATION-INVERTED cylinder band"),
    ),
    ("tests/fixtures/band/ftc11_uref_off.stp", EpsSensitive),
    ("tests/fixtures/band/washer180.stp", Pass(1, 1, 3, 3, 2)),
    ("tests/fixtures/band/washer90.stp", Pass(1, 1, 3, 3, 2)),
    // A quarter-cylinder wedge whose bottom rim's spline promotes to its
    // circle (`an_open_arc_trims_inside_what_its_spline_covers`): on the
    // half circle the spline covers, and refused where the edge names
    // the arc it does not.
    ("tests/fixtures/arc-trim/w_semi.step", Pass(1, 1, 5, 9, 6)),
    (
        "tests/fixtures/arc-trim/w_comp.step",
        Refused("outside the piece the file's spline covers"),
    ),
    // The kernel's cylinder with its rims restated as splines
    // (`a_closed_rim_promotes_whatever_its_spline_form`): each imports
    // as the cylinder it is.
    (
        "tests/fixtures/closed-rim/cyl_cubic.step",
        Pass(1, 1, 3, 3, 2),
    ),
    (
        "tests/fixtures/closed-rim/cyl_quad.step",
        Pass(1, 1, 3, 3, 2),
    ),
    (
        "tests/fixtures/closed-rim/cyl_single.step",
        Pass(1, 1, 3, 3, 2),
    ),
    // -- tests/fixtures/declined/ -------------------------------------
    // Edges curve recognition declines, read on their descriptions
    // (`an_edge_is_read_on_its_description_whatever_recognition_says`).
    (
        "tests/fixtures/declined/cyl_declined.step",
        Pass(1, 1, 3, 3, 2),
    ),
    (
        "tests/fixtures/declined/cyl_declined_loose.step",
        EpsSensitive,
    ),
    (
        "tests/fixtures/declined/side_column_declined.step",
        Pass(1, 1, 6, 12, 8),
    ),
    // The C⁰-knot split: each wall is two sub-patches, each ridge edge
    // two sub-arcs and each elliptic cone rim two arcs, and a knot on an
    // edge alone is a vertex the closing join keeps; the C prism's top is
    // three pieces, and the half ridge's top stays whole along its knot
    // line; the circular half cone is promoted to a cone first, so it
    // keeps the file's census (`a_foreign_carrier_splits_at_each_c0_knot`).
    (
        "tests/fixtures/c0-knot/c_prism.step",
        Pass(1, 1, 12, 30, 20),
    ),
    ("tests/fixtures/c0-knot/half_cone.step", Pass(1, 1, 4, 6, 4)),
    (
        "tests/fixtures/c0-knot/half_elliptic_cone.step",
        Pass(1, 1, 5, 9, 6),
    ),
    (
        "tests/fixtures/c0-knot/half_ridge.step",
        Pass(1, 1, 6, 12, 8),
    ),
    ("tests/fixtures/c0-knot/ridge_u.step", Pass(1, 1, 7, 15, 10)),
    ("tests/fixtures/c0-knot/ridge_v.step", Pass(1, 1, 7, 15, 10)),
    (
        "tests/fixtures/c0-knot/seam_edge.step",
        Pass(1, 1, 6, 13, 9),
    ),
    // A half cylinder and a half cone frustum on rational walls, each
    // also stated with its parameters swapped: each wall promotes whole to
    // its own kind either way round, so each keeps the file's census
    // (`a_promoted_kind_is_the_same_whichever_way_the_net_runs`).
    (
        "tests/fixtures/promotion/a_half_cone.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/promotion/a_half_cone_T.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/promotion/a_half_cyl.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/promotion/a_half_cyl_T.step",
        Pass(1, 1, 4, 6, 4),
    ),
    ("tests/fixtures/freecad/box.step", Pass(1, 1, 6, 12, 8)),
    (
        "tests/fixtures/freecad/box_fillet_corner.step",
        Pass(1, 1, 10, 21, 13),
    ),
    (
        "tests/fixtures/freecad/box_fillet_edge.step",
        Pass(1, 1, 7, 15, 10),
    ),
    (
        "tests/fixtures/freecad/box_hole.step",
        Pass(1, 1, 7, 15, 10),
    ),
    (
        "tests/fixtures/freecad/box_importexport.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "tests/fixtures/freecad/compound_two.step",
        Pass(2, 2, 7, 13, 10),
    ),
    ("tests/fixtures/freecad/cone_apex.step", Pass(1, 1, 2, 2, 2)),
    (
        "tests/fixtures/freecad/cone_trunc.step",
        Pass(1, 1, 3, 3, 2),
    ),
    ("tests/fixtures/freecad/cylinder.step", Pass(1, 1, 3, 3, 2)),
    (
        "tests/fixtures/freecad/fuse_boxes.step",
        Pass(1, 1, 14, 32, 20),
    ),
    ("tests/fixtures/freecad/sphere.step", Pass(1, 1, 1, 1, 2)),
    ("tests/fixtures/freecad/torus.step", Pass(1, 1, 1, 2, 1)),
    (
        "tests/fixtures/freecad/twobody_importexport.step",
        Pass(2, 2, 7, 13, 10),
    ),
    // -- tests/fixtures/cert1-r1/ (reviewer probes, adopted) ----------
    // R1's adversarial near-polar variants of the halfcap generator:
    // `nearpolar_*` puts the rim 0.0208 rad off the pole; `polesplit_*`
    // is issue 723's body with the split vertex EXACTLY on the pole
    // (the pole-membership decide sits on its Zero through this door).
    // `cert1_r1_import_probes.rs` holds all four to the closed form.
    // Every one states a meridian arc across the pole or ending at it;
    // where the pole lies inside the arc, import splits it there
    // (`chart_singularity_vertex.rs`), so all four ship one census.
    // The nearpolar twins are AMBIENT-sensitive: their rim sits
    // 0.0208 rad off the pole on a 0.208 mm circle, and at ambient
    // 1e-6 the rim/plane wedge angle's adoption margin (~8.6e-6)
    // lands in the escalation band — the coarse band honestly cannot
    // tell this near-tangency from a tangency. Pinned cell by cell
    // in `EPS_ROWS`.
    (
        "tests/fixtures/cert1-r1/nearpolar_nosplit.step",
        EpsSensitive,
    ),
    ("tests/fixtures/cert1-r1/nearpolar_split.step", EpsSensitive),
    (
        "tests/fixtures/cert1-r1/polesplit_nosplit.step",
        Pass(1, 1, 3, 4, 3),
    ),
    (
        "tests/fixtures/cert1-r1/polesplit_split.step",
        Pass(1, 1, 3, 4, 3),
    ),
    // -- tests/fixtures/halfcap/ (issue 723) --------------------------
    // Half of a spherical cap, whose sphere face's meridian side is one
    // POLE-CROSSING great-circle arc — the sphere's v-extent must come
    // from the arc's stored span, not its endpoint latitudes. The two
    // twins are the same solid; the split one carries one ordinary
    // vertex on the arc. That vertex once flipped the disposition —
    // no-split refused degenerate (endpoint fold saw lo == hi) while
    // split MEASURED, tier 3 green, 47% low at pad = 0.0. Both now
    // pass, and `halfcap_pole.rs` holds both to the exact closed-form
    // volume. The arc holds the pole inside it, which import makes a
    // vertex (`chart_singularity_vertex.rs`); the split twin's ordinary
    // vertex is joined at import (`docs/DESIGN.md`, maximal edges), so
    // it ships the no-split twin's census.
    ("tests/fixtures/halfcap/halfcap.step", Pass(1, 1, 3, 4, 3)),
    // The near-pole split twins: the same solid with the ordinary
    // vertex 1e-6 / 1e-7 rad off the pole, ambient-sensitive since the
    // import joins (`EPS_ROWS`).
    ("tests/fixtures/halfcap/halfcap_eps6.step", EpsSensitive),
    ("tests/fixtures/halfcap/halfcap_eps7.step", EpsSensitive),
    (
        "tests/fixtures/halfcap/halfcap_nosplit.step",
        Pass(1, 1, 3, 4, 3),
    ),
    // -- tests/fixtures/iso-rect/ (S58 / #649) ------------------------
    // #649's own fixtures. Both plus-domain solids are geometrically
    // VALID — manifold, closed, χ = 2 — and both pass: a cylinder wall's
    // flux is its chart Green form, which measures the plus and the tee
    // exactly. `cross` once imported and measured 19% low with
    // `pad = 0.0`, and was then refused on the iso-rectangle predicate;
    // `s58_iso_rectangle.rs` holds all four to their EXACT volumes and
    // runs `merge_coplanar_faces` on `xsplit` — #649's second door.
    // `rect` is the control (a genuine iso-rectangle of the same Δu and
    // v extent) and `xsplit` is the same solid as `cross` authored with
    // rectangular sub-faces.
    ("tests/fixtures/iso-rect/cross.step", Pass(1, 1, 14, 36, 24)),
    ("tests/fixtures/iso-rect/rect.step", Pass(1, 1, 6, 12, 8)),
    ("tests/fixtures/iso-rect/tee.step", Pass(1, 1, 10, 24, 16)),
    (
        "tests/fixtures/iso-rect/xsplit.step",
        Pass(1, 1, 18, 40, 24),
    ),
    // The C⁰-knot half cone with its bottom rim restated as the wall's
    // own row (`a_promotion_no_edge_carries_is_not_reported`).
    (
        "tests/fixtures/promoted-rim/half_cone_spline_rim.step",
        Pass(1, 1, 5, 9, 6),
    ),
    // The M7-8 cube with its straight columns re-timed
    // (`a_retimed_straight_rim_imports_on_its_wall`).
    (
        "tests/fixtures/retimed-rim/side_column.step",
        Pass(1, 1, 6, 12, 8),
    ),
    // -- tests/fixtures/retimed-seam/ --------------------------------
    // The nonuniform loft with one seam straight on a re-timed column
    // (`a_straight_seam_on_a_retimed_column_imports`).
    (
        "tests/fixtures/retimed-seam/seam_line.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "tests/fixtures/retimed-seam/seam_spline.step",
        Pass(1, 1, 6, 12, 8),
    ),
    // -- tests/fixtures/m7-8-cube/ -----------------------------------
    // The unit cube with one bowed NURBS wall whose rims are M7-8
    // intersections, the input of `placed_m7_8_instance.rs`. Its source
    // body is not at rest (scaffold chords), and the reader adopts the
    // file as a finished cube.
    (
        "tests/fixtures/m7-8-cube/m7_8_cube.step",
        Pass(1, 1, 6, 12, 8),
    ),
    // -- tests/fixtures/poleguard/ (issue 896) ------------------------
    // The mesh walk's undeclared-pole guard, import-door half: a
    // sphere truncated 9e-8 rad below its pole, so its top rim's
    // vertices sit 0.9e-9 m from a chart pole no vertex declares.
    // `poleguard.rs` states the route argument (any authoring of the
    // state carries a boundary feature of at most 2π·ε, which the
    // K = 10 band cannot certify clear); here the twins are corpus
    // like any other file, AMBIENT-sensitive by construction, pinned
    // cell by cell in `EPS_ROWS`: the sub-band span escalates at the
    // default band, certifies ZERO at 1e-6, and at 1e-12 — where the
    // spans certify — the near-tangent rim/sphere contact refuses one
    // level up, at adoption.
    ("tests/fixtures/poleguard/poleband.step", EpsSensitive),
    // The ε-relative sibling: the same band form with the vertex
    // 0.9e-12 m from the pole, so the 1e-12 band also pins a fixture
    // whose near-pole feature is INSIDE it (the two twins above sit
    // 900× outside that band and pin the adoption bar there instead).
    ("tests/fixtures/poleguard/poleband_eps12.step", EpsSensitive),
    ("tests/fixtures/poleguard/polefrustum.step", EpsSensitive),
    // -- tests/fixtures/occ/ ------------------------------------------
    // Open CASCADE's own statements of a sphere face reaching a pole
    // (`gen_occ.py`): a cap and a side-cut ball, each a seam ending at a
    // pole vertex of valence 1, the wrap tip, which import adopts as
    // stated (`chart_singularity_vertex.rs`); and the side-cut ball whose
    // cut misses OCC's seam, stated as a vertex loop beside the cut
    // circle, which no re-mint covers.
    ("tests/fixtures/occ/cap.step", Pass(1, 1, 2, 2, 2)),
    ("tests/fixtures/occ/side_cut.step", Pass(1, 1, 2, 4, 4)),
    (
        "tests/fixtures/occ/side_cut_ring.step",
        Refused("mixing a VERTEX_LOOP with other bounds"),
    ),
    // -- tests/fixtures/occ_voids/: Open CASCADE's BREP_WITH_VOIDS ---
    (
        "tests/fixtures/occ_voids/box_cavity.step",
        Pass(1, 2, 12, 24, 16),
    ),
    (
        "tests/fixtures/occ_voids/cylinder_sphere_cavity.step",
        Pass(1, 2, 4, 4, 4),
    ),
    (
        "tests/fixtures/occ_voids/two_cavities.step",
        Pass(1, 3, 18, 36, 24),
    ),
    // -- tests/fixtures/rim-only-cap/ ---------------------------------
    // A sphere pole or cone apex in a face's closure: inside the face
    // (one rim, a tilted rim, a cone's rim, a rim wearing a meridian spur
    // that stops short of the pole), where import mints one wrap edge to
    // it, or at the tip of a seam that is its only edge, the wrap tip as
    // stated (`chart_singularity_vertex.rs`). Each ships a revolved cap's
    // census: the face and its disc, the rim and the wrap edge.
    (
        "tests/fixtures/rim-only-cap/conecap.step",
        Pass(1, 1, 2, 2, 2),
    ),
    (
        "tests/fixtures/rim-only-cap/rimonly1.step",
        Pass(1, 1, 2, 2, 2),
    ),
    (
        "tests/fixtures/rim-only-cap/rimonly2.step",
        Pass(1, 1, 2, 2, 2),
    ),
    (
        "tests/fixtures/rim-only-cap/seamed.step",
        Pass(1, 1, 2, 2, 2),
    ),
    ("tests/fixtures/rim-only-cap/spur.step", Pass(1, 1, 2, 2, 2)),
    (
        "tests/fixtures/rim-only-cap/tilted.step",
        Pass(1, 1, 2, 2, 2),
    ),
    // -- tests/fixtures/seam-region/ ---------------------------------
    // A ball with the cap x > 8 cut away and its sphere split at the
    // equator into two faces of one surface, each holding a chart pole
    // inside it, which import makes the tip of one wrap edge
    // (`poles_inside_a_seam_region.rs`). Its twin splits the lower face
    // along the meridian through the south pole, which import splits
    // there.
    (
        "tests/fixtures/seam-region/side_cut_equator_split.step",
        Pass(1, 1, 3, 7, 6),
    ),
    (
        "tests/fixtures/seam-region/side_cut_meridian_split.step",
        Pass(1, 1, 4, 9, 7),
    ),
    // A ball stated as two lunes of one sphere, its poles each where
    // the two meridians meet (`records_follow_the_file.rs`'s pinch).
    (
        "tests/fixtures/seam-region/two_lunes.step",
        Pass(1, 1, 2, 2, 2),
    ),
    // #653's import route: one D-prism, stated four ways. The two
    // `split_*` files state the cylindrical face's vertical boundary as
    // two collinear `EDGE_CURVE`s, which is what every exporter emits
    // when a vertex lands mid-side; the two `*_oblique` files place the
    // part by a general rotation. Tessellating them is
    // `split_iso_side.rs`'s row — here they are corpus like any other
    // file, and the census below is the pin that says the difference is
    // exactly the split and nothing else.
    //
    // The `split_*` censuses equal the `plain_*` ones: the two collinear
    // sub-edges meet at a vertex nothing else does, so the import ends
    // by joining them (`docs/DESIGN.md`, maximal edges) and reports the
    // join as a `JoinedEdges` normalization. The `*_oblique` rows equal
    // their `*_axis` counterparts exactly, so the placement changes no
    // count at all.
    (
        "tests/fixtures/split-iso/plain_axis.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/split-iso/plain_oblique.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/split-iso/split_axis.step",
        Pass(1, 1, 4, 6, 4),
    ),
    // `split_axis` with one collinear half of its cylinder-side edge a
    // straight degree-1 spline: the line is kept and the two join.
    (
        "tests/fixtures/line-beside-spline/mixed_lower.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/line-beside-spline/mixed_upper.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/split-iso/split_oblique.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "tests/fixtures/wild/adafruit/1982_MPR121.step",
        Pass(1, 1, 10, 24, 16),
    ),
    (
        "tests/fixtures/wild/adafruit/328_2500mAh_battery.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "tests/fixtures/wild/adafruit/64_Halfsize_Breadboard.step",
        Pass(1, 1, 18, 48, 32),
    ),
    (
        "tests/fixtures/wild/adafruit/805_slide_switch.step",
        Pass(1, 1, 19, 46, 30),
    ),
    (
        "tests/fixtures/wild/adafruit/931_OLED_128x32_I2C.step",
        Pass(1, 1, 24, 60, 40),
    ),
    (
        "tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp",
        Refused(NIST09_PRINTED_OFF_ITS_TANGENCY),
    ),
    (
        "tests/fixtures/wild/nist/nist_ftc_11_asme1_rb.stp",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "tests/fixtures/wild/occ-oss/b123d_nema17_bracket.step",
        Refused("(SURFACE_CURVE) is outside the imported subset"),
    ),
    (
        "tests/fixtures/wild/occ-oss/cq_red_cube_blue_cylinder.step",
        Pass(2, 2, 9, 15, 10),
    ),
    (TAIL_TURBINE, EpsSensitive),
    (
        // **Two halves of this refusal are now retired.** M8
        // instancing took the placement half (dm1's seven occurrences
        // of three component representations materialize as seven
        // placed instances); #327 took the D7 half — the rims of the
        // file's seven RATIONAL cylinders arrive as rational-quadratic
        // NURBS carriers, stage-1 CURVE recognition certifies them as
        // circles against an exact ring-composite bound and promotes
        // them, and every edge of every instance adopts with its
        // pcurve minted and certified.
        //
        // What is left is the SHARED AT-REST GATE on those same
        // rational walls: the exact-B-rep volume's quadrature
        // enclosure stalls short of its target within the fixed round
        // budget — the same lane, and the same budget, a NATIVELY
        // built rational-walled loft refuses on too. The fragment names that
        // stall specifically rather than the gate's preamble, because
        // the preamble would also match a tier-1/2 verdict, which
        // would be a regression and not this lane.
        //
        // **ε-SENSITIVE since #327**, and the sweep is the reason to
        // know it: at the two FINE ambient bands the frontier is the
        // rational-flux stall above, but at ambient 1e-6 the frontier
        // is the aggregate gate's convergence-ambiguity escalation
        // (the `#389` ladder gap this cell once exposed is retired —
        // #388's reversed wall-column candidate). Three cells
        // in `EPS_ROWS`, one per ambient band. This was the FIRST file
        // the ε_in sweep stopped running (the 2026-08-13 audit); since
        // 2026-08-22 that is the corpus-wide default and the exemption
        // is the other way round — see `EPS_IN_SWEPT`. Either way its
        // ε_in-invariance is recorded rather than re-measured, and the
        // ambient sweep — the axis it MOVES on — is untouched.
        DM1,
        EpsSensitive,
    ),
    (
        "tests/fixtures/wild/stepcode/io1-cm-214.stp",
        Refused("expected a doubled backslash"),
    ),
    (
        "tests/fixtures/wild/stepcode/sg1-c5-214.stp",
        Pass(1, 1, 16, 32, 20),
    ),
    (
        "../step-export/tests/fixtures/ball.step",
        Pass(1, 1, 1, 1, 2),
    ),
    (
        "../step-export/tests/fixtures/boss_union.step",
        Pass(1, 1, 10, 21, 14),
    ),
    (
        "../step-export/tests/fixtures/composed_die.step",
        Pass(1, 1, 68, 132, 87),
    ),
    (
        "../step-export/tests/fixtures/cone.step",
        Pass(1, 1, 2, 2, 2),
    ),
    (
        "../step-export/tests/fixtures/cube.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/cut_cylinder.step",
        Pass(1, 1, 4, 6, 4),
    ),
    (
        "../step-export/tests/fixtures/die.step",
        Pass(1, 1, 11, 24, 16),
    ),
    (
        "../step-export/tests/fixtures/die_pips.step",
        Pass(1, 1, 27, 54, 50),
    ),
    (
        "../step-export/tests/fixtures/donut.step",
        Pass(1, 1, 2, 4, 2),
    ),
    (
        "../step-export/tests/fixtures/filleted_die.step",
        Pass(1, 1, 26, 48, 24),
    ),
    (
        // The touching two-solid assembly: the shared gate is the
        // tier-3′ form, recording the corner kiss between its two
        // solids as a contact (`topo::record_at_rest`; the record is
        // pinned in review_r1_tier_gate_probes.rs).
        "../step-export/tests/fixtures/kiss_assembly.step",
        Pass(2, 2, 22, 48, 32),
    ),
    (
        "../step-export/tests/fixtures/lily_lantern.step",
        Pass(1, 1, 4, 5, 3),
    ),
    (
        "../step-export/tests/fixtures/loft_prism.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/nonuniform_loft.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/notched.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        "../step-export/tests/fixtures/nurbs_wireframe.step",
        Wireframe,
    ),
    (
        // The #327 reporting witness: a `GEOMETRIC_CURVE_SET` stating
        // dm1's carrier form verbatim (degree 2, 7 points, weights
        // `1, ½, …`, knots at multiples of √3, the 3×120°
        // construction, r = 5 mm). It is here because the corpus table
        // is the WHOLE corpus by construction — and its disposition is
        // the same `Wireframe` every curve-set file gets: promotion
        // changes the carrier's DESCRIPTION, never the file's
        // disposition. `curve_promotion_report.rs` is what asserts the
        // promotion itself.
        "tests/fixtures/curveset_rational_circle.step",
        Wireframe,
    ),
    (
        "../step-export/tests/fixtures/swept_elbow.step",
        Pass(1, 1, 6, 12, 8),
    ),
    (
        // The hollow box: one BREP_WITH_VOIDS, its cavity a second
        // shell of the one solid.
        "../step-export/tests/fixtures/voided.step",
        Pass(1, 2, 12, 24, 16),
    ),
    (
        "../step-export/tests/fixtures/washer.step",
        Pass(1, 1, 4, 6, 4),
    ),
];

/// Every `.step` / `.stp` file under `dir`, recursively, sorted.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("reading {dir:?}: {e}"))
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, out);
        } else {
            let s = p.to_string_lossy().to_lowercase();
            if s.ends_with(".step") || s.ends_with(".stp") {
                out.push(p);
            }
        }
    }
}

/// The corpus as the filesystem holds it, keyed the way the table is.
fn discovered() -> Vec<String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&root.join("tests/fixtures"), &mut files);
    walk(&root.join("../step-export/tests/fixtures"), &mut files);
    files
        .iter()
        .map(|p| {
            p.strip_prefix(&root)
                .unwrap_or(p)
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

/// The table covers the corpus exactly — no file escapes the sweep by
/// being new, and no row survives its fixture's deletion.
#[test]
fn the_table_is_the_whole_corpus() {
    let mut found = discovered();
    found.sort();
    let mut tabled: Vec<String> = CORPUS.iter().map(|(p, _)| (*p).to_owned()).collect();
    tabled.sort();
    assert_eq!(
        found, tabled,
        "the committed STEP corpus and this suite's table have diverged"
    );
}

/// The claim that holds at ANY ambient ε, pinned matrix or not: the
/// outcome is TYPED — a solid whose SHIPPED body the gate itself
/// passes, a wireframe, or a typed refusal — and never a body the
/// kernel would call geometrically false. A row that asserted nothing
/// off the matrix would be a green tick for work not done.
fn assert_typed_outcome(who: &str, got: Result<StepImport, StepImportError>) {
    match got {
        Ok(StepImport::Solid { body, .. }) => assert_eq!(
            topo::validate_geometric(&*body, Tol::witness()),
            Ok(()),
            "{who}: import shipped a body its own gate refuses"
        ),
        Ok(StepImport::Wireframe { .. }) | Err(_) => {}
    }
}

/// The [`EPS_ROWS`] pins cover exactly the `EpsSensitive` files, at
/// exactly the pinned ambient tolerances, with exactly one cell per
/// `eps_in` row THAT FILE IS SWEPT AT — so a marker cannot go unpinned
/// and a pin cannot outlive the marker that reads it.
///
/// The table is held EXHAUSTIVE in both directions, which is what keeps
/// [`eps_in_rows_for`]'s exemption honest rather than a hole: every
/// (file, ambient, tag) the sweep executes must have exactly one cell,
/// AND the table's total size must be exactly the number of executed
/// combinations — so a cell that goes missing is red, and so is a cell
/// left behind for a tag that is no longer swept (which would otherwise
/// sit there unread, claiming a measurement nothing performs).
/// [`EPS_IN_SWEPT`] must name files the corpus actually holds.
///
/// The list is the only thing standing between a file and a silently
/// single-row sweep, so a typo in it — or a fixture renamed out from
/// under it — must be loud rather than a quietly narrower gate.
#[test]
fn the_eps_in_swept_files_are_corpus_files() {
    for swept in EPS_IN_SWEPT {
        assert!(
            CORPUS.iter().any(|(p, _)| *p == swept),
            "EPS_IN_SWEPT names {swept:?}, which is not in CORPUS — the ε_in sweep it \
             asks for runs on nothing"
        );
    }
}

#[test]
fn every_eps_sensitive_row_is_pinned_cell_by_cell() {
    let mut markers: Vec<&str> = CORPUS
        .iter()
        .filter(|(_, d)| *d == EpsSensitive)
        .map(|(p, _)| *p)
        .collect();
    markers.sort_unstable();
    let mut pinned: Vec<&str> = EPS_ROWS.iter().map(|(p, ..)| *p).collect();
    pinned.sort_unstable();
    pinned.dedup();
    assert_eq!(
        markers, pinned,
        "an EpsSensitive row with no cells, or cells for a file that is not EpsSensitive"
    );
    let mut want_cells = 0;
    for file in &markers {
        for ambient in PINNED_AMBIENT {
            for (tag, _) in eps_in_rows_for(file) {
                want_cells += 1;
                let hits = EPS_ROWS
                    .iter()
                    .filter(|(p, a, t, _)| p == file && *a == ambient && t == tag)
                    .count();
                assert_eq!(
                    hits, 1,
                    "{file} @ ambient {ambient:e} / eps {tag}: {hits} pins"
                );
            }
        }
    }
    assert_eq!(
        EPS_ROWS.len(),
        want_cells,
        "EPS_ROWS holds cells for (file, ambient, eps_in) combinations the sweep does \
         not execute — a pin nothing reads is a measurement nobody makes"
    );
}

/// What this row must do, or `None` when the ambient ε is off the
/// pinned matrix and only the every-ε obligation is claimed.
fn expected(rel: &str, row: Disposition, eps_tag: &str) -> Option<Disposition> {
    if row != EpsSensitive {
        return Some(row);
    }
    let ambient = geom_core::Tol::witness().get().eps;
    EPS_ROWS
        .iter()
        .find(|(p, a, t, _)| *p == rel && *a == ambient && *t == eps_tag)
        .map(|(.., d)| *d)
}

/// **Whether the corpus file `rel` refuses at the run's ε**, read off its
/// pin at its own ε_in: its [`CORPUS`] row, or for an `EpsSensitive` file
/// its [`EPS_ROWS`] cell. `None` for a file not in the corpus, or an
/// `EpsSensitive` one off the pinned ambient matrix, where its disposition
/// is not known. With [`refusal_at_run_eps`], the one place other suites
/// read a corpus file's disposition from.
pub(crate) fn refuses_at_run_eps(rel: &str) -> Option<bool> {
    let &(_, row) = CORPUS.iter().find(|(p, _)| *p == rel)?;
    expected(rel, row, "file").map(|d| matches!(d, Refused(_)))
}

/// **The refusal fragment the corpus file `rel` is pinned to at the run's
/// ε**, read off the same pin as [`refuses_at_run_eps`]. `None` where that
/// pin is no refusal, or not known.
pub(crate) fn refusal_at_run_eps(rel: &str) -> Option<&'static str> {
    let &(_, row) = CORPUS.iter().find(|(p, _)| *p == rel)?;
    match expected(rel, row, "file")? {
        Refused(fragment) => Some(fragment),
        _ => None,
    }
}

/// Every corpus file's disposition, and the positive tier-validity of
/// every body that ships — at each tolerance in the sweep.
#[test]
fn every_corpus_import_passes_the_shared_gate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut exposure = Exposure::new("tier_gate's import-door reading");
    // File-major, so each file is read once and each file's own ε_in
    // sweep (see `eps_in_rows_for`) is visible at the loop head.
    for &(rel, row) in CORPUS {
        let text = std::fs::read_to_string(root.join(rel))
            .unwrap_or_else(|e| panic!("reading {rel}: {e}"));
        for &(eps_tag, eps_in) in eps_in_rows_for(rel) {
            let options = ImportOptions { eps_in };
            let who = format!("{rel} @ eps {eps_tag}");
            let Some(want) = expected(rel, row, eps_tag) else {
                // Off the pinned ambient matrix. The disposition of an
                // ε-sensitive file is not knowable here, but the gate's
                // claim still is, and asserting it is not nothing.
                assert_typed_outcome(&who, import_step(&text, &options, Tol::witness()));
                continue;
            };
            match (import_step(&text, &options, Tol::witness()), want) {
                (Ok(StepImport::Solid { body, .. }), Pass(s, sh, f, e, v)) => {
                    assert_eq!(
                        topo::validate_geometric(&*body, Tol::witness()),
                        Ok(()),
                        "{who}: the SHIPPED body must be gate-clean — import handed out a \
                         body its own gate refuses, which can only mean the gate is no \
                         longer wired"
                    );
                    assert_eq!(
                        (
                            body.solids().count(),
                            body.shells().count(),
                            body.faces().count(),
                            body.edges().count(),
                            body.vertices().count()
                        ),
                        (s, sh, f, e, v),
                        "{who}: census (solids, shells, faces, edges, vertices) — a \
                         shipped body missing entities the file states is a silent \
                         loss, and a missing SOLID is also a solid the per-solid gate \
                         never saw"
                    );
                }
                (Ok(StepImport::Wireframe { .. }), Wireframe) => {}
                (Err(e), Refused(fragment)) => {
                    let msg = e.render();
                    assert_adoption_reading(&who, &e, &mut exposure);
                    assert!(
                        msg.contains(fragment),
                        "{who}: refused for a DIFFERENT reason than the table records \
                         (want a message containing {fragment:?}): {msg}"
                    );
                }
                (got, want) => panic!("{who}: disposition changed — want {want:?}, got {got:?}"),
            }
        }
    }
    // The guard's floor: on the pinned matrix the corpus reaches both an
    // offer it checks and a reading that withholds one, so a guard that
    // parsed nothing cannot pass. Off it, no refusal cell is read.
    exposure.report();
    if PINNED_AMBIENT.contains(&Tol::witness().get().eps) {
        exposure.require_each(
            &[OFFERS, WITHHELD],
            1,
            "the import door's guard read no offer, or no withheld one, across the corpus",
        );
    } else {
        test_utils::vacuity::stood_down(
            "ambient ε off the pinned matrix",
            "the import door's guard floor: no refusal cell is read here",
        );
    }
}

/// A certification refusal at adoption reads as at rest (D4 ¶1), and an
/// interpretation refusal reads the file at its band: no ending blames
/// the kernel alone, and no offer to tighten keeps a size at or below
/// the file's declared coincidence distance ε_in by tightening alone. Each offer's value is `|m|/K` at the margin's
/// nearer end, so the size it keeps, that value times K, lies past
/// ε_in — or the offer is the second of two steps, after declaring the
/// file's uncertainty below that size. Counts each offer it parsed, and
/// each reading that withheld one, into `exposure`.
fn assert_adoption_reading(who: &str, e: &StepImportError, exposure: &mut Exposure) {
    let (StepImportError::Adoption { file, .. }
    | StepImportError::Assembly { file, .. }
    | StepImportError::Interpretation { file, .. }) = e
    else {
        return;
    };
    let msg = e.render();
    assert!(
        !msg.contains(geom_core::KERNEL_DEFECT_ENDING),
        "{who}: an adoption refusal ends as if read at a build: {msg}"
    );
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let k = band.escalate() / band.zero();
    let pieces: Vec<&str> = msg.split("tighten the tolerance below ").collect();
    for (before, offer) in pieces.iter().zip(&pieces[1..]) {
        exposure.note(OFFERS);
        let below: f64 = offer
            .split_once(" m")
            .and_then(|(v, _)| v.parse().ok())
            .unwrap_or_else(|| panic!("{who}: an offer to tighten names no value: {msg}"));
        if below * k > file.eps_in() {
            continue;
        }
        // The first step, in either door's words: the adoption's
        // "uncertainty declared below X m and" (X the size, the offer at
        // X/K), or the interpretation's "ε_in below X m (…) and" (the
        // band's zero below the size over K, so both at one value).
        let adoption: Option<f64> = before
            .strip_suffix(" m and ")
            .and_then(|b| b.rsplit_once("uncertainty declared below "))
            .and_then(|(_, v)| v.parse().ok());
        let interpretation: Option<f64> = before
            .strip_suffix(" and ")
            .and_then(|b| b.rsplit_once("ε_in below "))
            .and_then(|(_, rest)| rest.split_once(" m"))
            .and_then(|(v, _)| v.parse().ok());
        let paired = adoption
            .is_some_and(|d| d <= file.eps_in() && (d / k - below).abs() <= below * 1e-9)
            || interpretation
                .is_some_and(|d| d <= file.eps_in() && (d - below).abs() <= below * 1e-9);
        assert!(
            paired,
            "{who}: a refusal offers to keep a size of {:e} m, within the file's \
             ε_in = {:e} m, without declaring the file's uncertainty below it: {msg}",
            below * k,
            file.eps_in()
        );
    }
    exposure.add(
        WITHHELD,
        msg.matches("so the file cannot tell it from zero").count()
            + msg
                .matches("so the file might not tell it from zero")
                .count()
            + msg.matches("does not tell from zero").count(),
    );
}

/// [`assert_adoption_reading`]'s tally: offers to tighten it parsed.
const OFFERS: &str = "offers to tighten parsed";
/// [`assert_adoption_reading`]'s tally: readings at the import door that
/// withheld the offer to tighten alone, for a size the file cannot tell
/// from zero.
const WITHHELD: &str = "readings that withheld the offer";

/// The reader touches the kernel's at-rest validators at exactly TWO
/// places (the #260 ask: make skipping them structurally hard). The
/// gate is asked about several subjects — each solid alone, then the
/// assembled body — but always through `lib.rs`'s `gate` (the scalar's
/// finish, tier 3, for each solid and for the assembled body) and
/// `gate3`'s census over the finished assembled body (the rest of tier
/// 3′), each of which maps the verdicts to a typed refusal and does
/// nothing else. Import owns no
/// validation logic of its own: no third entry, no kind predicate
/// deciding who is asked, no verdict filter deciding which failures
/// count. This counts the validator call sites in the crate's sources
/// and pins the count at two; a third is not automatically wrong, but
/// it is exactly the shape the old band-only backstop had, so it must
/// be argued for here rather than appear.
///
/// # What the needle covers, and what it does not
///
/// COVERS the tier-3 and tier-3′ door FAMILIES by name — every
/// spelling of `validate_geometric` and `validate_pseudomanifold`
/// (structural, `_declared`, `_certificate`), the scalar's finish and
/// its census over a finished body — and `contact_marks`,
/// which is not one of those doors but runs the SAME tier-3 battery
/// over the same body, so a reader that grew a gate there would be
/// growing exactly the second entry this row exists to refuse.
///
/// DOES NOT COVER a battery composed by hand out of parts that are not
/// gates: `validate_closed` (tier 2) followed by `mass_properties` is
/// two public doors, neither of them an at-rest validator, and a reader
/// that spelled its gate that way would read as zero sites here. That
/// is a real hole and it is named rather than papered over — the needle
/// is a name census, so it can only see doors that exist and are
/// called by name. What closes it is not a wider pattern but the
/// refusal rows below, which assert the reader's typed verdicts ARE the
/// kernel's: a hand-rolled battery does not produce
/// `StepImportError::TierInvalid` carrying `topo::ValidationError`s.
#[test]
fn exactly_one_validation_call_site_in_the_reader() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sites = Vec::new();
    // The needle is a CALL, so comments and literal bodies are both
    // blanked: a commented-out door must not answer for a live one,
    // and prose naming the validator must not manufacture a site. The
    // blanked view keeps line structure, so the line number is real.
    //
    // It matches the door FAMILY, not one spelling. Each tier has more
    // than one entry — `validate_geometric` and
    // `validate_geometric_certificate`, `validate_pseudomanifold` and
    // `validate_pseudomanifold_certificate`, plus the structural and
    // `_declared` forms — and they are the same door with the same
    // verdicts. A needle pinned to one spelling would let the reader
    // grow a second entry by taking a sibling, which is exactly the
    // shape this row exists to catch.
    //
    // `contact_marks` is in the set for the same reason and it is NOT a
    // sibling spelling: it is a different door that runs the same
    // tier-3 battery over the same body (its own product is the marks
    // channel). A gate grown there would be a third entry wearing a
    // fourth name.
    // `topo::finish` is the scalar's finish, the tier-3 door `gate`
    // goes through: the same battery under the one constructor of a
    // finished body. `census_at_rest` is tier 3′'s census over a
    // finished body, the rest of `validate_pseudomanifold`.
    const DOORS: [&str; 5] = [
        "validate_geometric",
        "validate_pseudomanifold",
        "contact_marks",
        "topo::finish(",
        "census_at_rest",
    ];
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).unwrap();
        for (i, line) in test_utils::source::code_only(&text).lines().enumerate() {
            if DOORS.iter().any(|door| line.contains(door)) {
                sites.push(format!("{}:{}", path.display(), i + 1));
            }
        }
    }
    // Two doors, still zero opinions: the finish (`gate`) and the
    // aggregate's census over the finished body (`gate3`). Anything
    // beyond these two is validation logic growing in the reader.
    assert_eq!(
        sites.len(),
        2,
        "the reader must call the kernel's at-rest validators at exactly the two named \
         doors — the finish (`gate`) and the aggregate's census (`gate3`). Every \
         spelling of both door families is counted, and so is `contact_marks`, which \
         runs the same battery; what is NOT counted is a battery hand-composed out of \
         non-gate parts (see this row's docs): {sites:?}"
    );
}

/// The typed refusal is a VALIDITY refusal about the file's geometry,
/// carrying the kernel's verdicts verbatim — not prose, and not the
/// Corrupt-class kernel-bug voice. The inside-out torus band is the
/// standing fixture for it.
#[test]
fn the_refusal_carries_the_kernels_verdicts() {
    let text = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/band/band_c180.stp"),
    )
    .unwrap();
    let e = import_step(&text, &ImportOptions::default(), Tol::witness()).unwrap_err();
    let StepImportError::TierInvalid { solid, errors } = &e else {
        panic!("expected the gate's typed refusal, got: {e:?}");
    };
    assert_eq!(
        *solid, None,
        "a one-solid file's subject is the assembled body itself"
    );
    assert!(
        matches!(
            errors.as_slice(),
            [topo::ValidationError::NegativeVolume { .. }]
        ),
        "the verdicts are the kernel's own, unfiltered and unrephrased: {errors:?}"
    );
    let msg = e.render();
    for want in [
        "shared at-rest validation gate",
        "NegativeVolume",
        "a solid encloses negative volume, so it is inside-out",
    ] {
        assert!(msg.contains(want), "the message must name {want:?}: {msg}");
    }
}

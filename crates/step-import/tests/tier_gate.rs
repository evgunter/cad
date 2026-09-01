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
//! now the tier-3′ form over the import-side declaration channel
//! (D7 step 4 executed) — an imported assembly whose parts touch
//! with vertex/line/planar boundary evidence refuses UNDECLARED and
//! certifies WITH the declaration (`kiss_assembly`'s row below and
//! `review_r1_tier_gate_probes.rs` pin both directions); cross-solid
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
//! **S58 / #649 (2026-08-19) added four rows; one of them is a
//! newly-refused body class.**
//! `iso-rect/cross.step` is the one that moved: a valid, manifold,
//! closed solid that USED to pass this gate and then measure 19% low
//! with `pad = 0.0`, and that the one iso-rectangle predicate now
//! refuses here. `iso-rect/tee.step` never passed this gate — #649
//! records import already refusing it, on `props_du_consistent`,
//! because its one-sided arm makes the rim-group span sums disagree.
//! What S58 moved for the tee is the **reason** in the refusal string,
//! not its disposition, and the row is pinned on the new reason.
//! `iso-rect/rect.step` / `iso-rect/xsplit.step` beside them are the
//! controls that keep the tightening from being a blanket refusal.
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
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

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
/// and it is `ftc11_uref_off` at all three bands (`file` refuses on the
/// seam halfplane; `1e-6` and `1e-12` reach the intersection arm, the
/// param span, or pass — see [`EPS_ROWS`]). The other 61 are invariant
/// to the byte.
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

/// **The ε-row pins.** Six corpus files' dispositions are a function
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
/// * **`ftc11_uref_off`** is the deliberately-degenerate band fixture.
///   Its seam residual is ~1.6e-6 m, so at ambient 1e-6 that margin
///   lands INSIDE the ambiguity band (zero = ε, escalate = Kε) and the
///   refusal is an ESCALATION rather than a definite verdict; at
///   ambient 1e-12 the same margin is decisively outside every band, so
///   the coincidence predicates that refused it at coarser ε ("tangent
///   planes coincide", the Intersection transversality precondition)
///   no longer fire, and the file imports. A coincidence test refusing
///   what is TOO CLOSE must stop refusing as ε shrinks; that direction
///   is the predicate being right, not the gate being loosened.
/// * **`nist_ftc_09_asme1_rd`** refuses at ambient 1e-12 with definite
///   `EndpointStart` residuals on both the seam and the mapped-curve
///   arm. This is the floor `wild.rs` already measures and documents:
///   the NIST inch translator prints ~12 significant digits, so the
///   file does not state itself to 1e-12 m, and the adoption ladder
///   says so by name instead of certifying a carrier it cannot.
const EPS_ROWS: [(&str, f64, &str, Disposition); 30] = [
    // -- tests/fixtures/cert1-r1/nearpolar_*.step ---------------------
    // The AMBIENT sweep only, at the files' own ε_in (they state
    // themselves to full double precision). At ambient 1e-6 both
    // twins refuse at EDGE ADOPTION — the rim/plane wedge angle's
    // certification margin (~8.6e-6 rad) is inside the ambiguity
    // band, so no props arithmetic is even reached; at the default
    // and fine bands both certify with the exact closed-form volume
    // (`cert1_r1_import_probes.rs` holds the value).
    (NEARPOLAR_SPLIT, 1e-9, "file", Pass(1, 1, 3, 4, 3)),
    (
        NEARPOLAR_SPLIT,
        1e-6,
        "file",
        Refused(NEARPOLAR_WEDGE_ESCALATED),
    ),
    (NEARPOLAR_SPLIT, 1e-12, "file", Pass(1, 1, 3, 4, 3)),
    (NEARPOLAR_NOSPLIT, 1e-9, "file", Pass(1, 1, 3, 3, 2)),
    (
        NEARPOLAR_NOSPLIT,
        1e-6,
        "file",
        Refused(NEARPOLAR_WEDGE_ESCALATED),
    ),
    (NEARPOLAR_NOSPLIT, 1e-12, "file", Pass(1, 1, 3, 3, 2)),
    // -- tests/fixtures/band/ftc11_uref_off.stp -----------------------
    (FTC11, 1e-9, "file", Refused(SEAM_HALFPLANE_DEFINITE)),
    (FTC11, 1e-9, "1e-6", Refused(TANGENT_PLANES_COINCIDE)),
    (FTC11, 1e-9, "1e-12", Refused(TANGENT_PLANES_COINCIDE)),
    (FTC11, 1e-6, "file", Refused(SEAM_HALFPLANE_ESCALATED)),
    (FTC11, 1e-6, "1e-6", Refused(PARAM_SPAN_ESCALATED)),
    (FTC11, 1e-6, "1e-12", Refused(PARAM_SPAN_ESCALATED)),
    (FTC11, 1e-12, "file", Refused(SEAM_HALFPLANE_DEFINITE)),
    (FTC11, 1e-12, "1e-6", Pass(1, 1, 6, 16, 12)),
    (FTC11, 1e-12, "1e-12", Pass(1, 1, 6, 16, 12)),
    // -- tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp ------------
    // The AMBIENT sweep only, at this file's own ε_in — the same shape
    // dm1 took in 2026-08-13, for the same reason and on the same kind
    // of evidence. It was nine cells until 2026-08-22; the six dropped
    // ones were the `1e-6` and `1e-12` ε_in tags, and each held the
    // SAME disposition, census and refusal message as the `file` tag
    // beside it at every band. This file is AMBIENT-sensitive and
    // ε_in-INVARIANT, and those are different axes: what moves it is
    // that the NIST inch translator prints ~12 significant digits, so
    // at ambient 1e-12 the file does not state itself finely enough —
    // a property of the file against the ambient band, which no
    // interpretation budget in this sweep touches. See [`EPS_IN_SWEPT`].
    (NIST09, 1e-9, "file", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-6, "file", Pass(1, 1, 158, 454, 300)),
    (NIST09, 1e-12, "file", Refused(ENDPOINT_START_MAPPED_CURVE)),
    // -- tests/fixtures/wild/stepcode/dm1-id-214.stp (#327) -----------
    // The AMBIENT sweep only, at this file's own ε_in: two cells at the
    // rational-flux stall, one at the ladder's `#389` gap.
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
    (DM1, 1e-9, "file", Refused(RATIONAL_FLUX_STALL)),
    // The coarse band reaches the GATE now and escalates there: the
    // enclosure lands ~1% under the loose `1024·ε` target, inside the
    // convergence predicate's ambiguity band. That masks — it does not
    // fix — the `#389` ladder gap that used to be this cell.
    (DM1, 1e-6, "file", Refused(QUAD_CONVERGED_ESCALATED)),
    (DM1, 1e-12, "file", Refused(RATIONAL_FLUX_STALL)),
    // -- tests/fixtures/poleguard/*.step (issue 896) ------------------
    // The AMBIENT sweep only, at the files' own ε_in (they state
    // themselves to full double precision). The near-pole feature is
    // 0.9e-9 m by construction, so which certification refuses is a
    // function of the ambient band alone: the span escalates in the
    // default band's indeterminate zone, certifies ZERO at 1e-6, and
    // at 1e-12 the spans clear and the rim/sphere near-tangency
    // refuses at adoption — `poleguard.rs` holds the route argument.
    (POLEBAND, 1e-9, "file", Refused(PARAM_SPAN_ESCALATED)),
    (POLEBAND, 1e-6, "file", Refused(INTERVAL_NOT_FORWARD)),
    (POLEBAND, 1e-12, "file", Refused(TANGENT_SECOND_ORDER_ZERO)),
    // The ε-relative sibling's cells mirror the twins' one band down:
    // its 5.65e-12 m span escalates exactly where the band is 1e-12
    // and certifies ZERO at both coarser bands.
    (POLEBAND12, 1e-9, "file", Refused(INTERVAL_NOT_FORWARD)),
    (POLEBAND12, 1e-6, "file", Refused(INTERVAL_NOT_FORWARD)),
    (POLEBAND12, 1e-12, "file", Refused(PARAM_SPAN_ESCALATED)),
    (POLEFRUSTUM, 1e-9, "file", Refused(PARAM_SPAN_ESCALATED)),
    (POLEFRUSTUM, 1e-6, "file", Refused(INTERVAL_NOT_FORWARD)),
    (
        POLEFRUSTUM,
        1e-12,
        "file",
        Refused(TANGENT_SECOND_ORDER_ZERO),
    ),
];

const FTC11: &str = "tests/fixtures/band/ftc11_uref_off.stp";
const NEARPOLAR_SPLIT: &str = "tests/fixtures/cert1-r1/nearpolar_split.step";
const NEARPOLAR_NOSPLIT: &str = "tests/fixtures/cert1-r1/nearpolar_nosplit.step";
/// The nearpolar twins' coarse-band sub-reason: the rim/plane wedge
/// angle's adoption certification, by predicate name, so a regression
/// that moves the refusal to another door fails these cells.
const NEARPOLAR_WEDGE_ESCALATED: &str = "predicate 'dihedral_wedge' indeterminate";
const DM1: &str = "tests/fixtures/wild/stepcode/dm1-id-214.stp";
const POLEBAND: &str = "tests/fixtures/poleguard/poleband.step";
const POLEBAND12: &str = "tests/fixtures/poleguard/poleband_eps12.step";
const POLEFRUSTUM: &str = "tests/fixtures/poleguard/polefrustum.step";
/// The poleguard twins' coarse-band sub-reason: the sub-band span
/// certifies zero, and the attachment gate refuses the degenerate
/// interval by name.
const INTERVAL_NOT_FORWARD: &str = "the stored parameter interval is not forward";
/// Their fine-band sub-reason: with the spans certified, adoption
/// refuses the rim/sphere near-tangency — the second-order arm's own
/// verdict, so a regression that moves the refusal to another door
/// fails these cells.
const TANGENT_SECOND_ORDER_ZERO: &str = "tangent_second_order) is exactly zero at sample 1";
/// dm1's fine-band sub-reason: the shared at-rest gate cannot compute
/// the exact-B-rep volume of a RATIONAL cylinder wall to target. The
/// quadrature converges there — it quarters cleanly per refinement
/// round — and what it runs out of is the FIXED round budget, inside a
/// factor of two. Named specifically so the gate's preamble (which a
/// tier-1/2 regression would also match) cannot stand in.
const RATIONAL_FLUX_STALL: &str = "the certified quadrature enclosure stalled at";
/// dm1's coarse-band sub-reason: the convergence predicate declines to
/// decide, by name, so a regression that turned this into a silent
/// answer (or into a different door) fails the cell.
const QUAD_CONVERGED_ESCALATED: &str = "predicate 'props_quad_converged' indeterminate";
/// dm1's coarse-band sub-reason: the ladder's own refusal on edge
/// `#389`, a two-point `QUASI_UNIFORM_CURVE` polyline that stays NURBS
/// and is offered zero candidates.
/// dm1's `#389` polyline gap. **No cell reaches it any more**: it was
/// the coarse band's first refusal until the patch-flux enclosure
/// tightened enough to escalate ahead of it. Kept, not deleted — the
/// gap is real, unfixed, and would become reachable again the moment
/// anything at the gate moves.
#[allow(dead_code)]
const LADDER_NO_DESCRIPTION: &str = "edge #389: no intensional description certifies";
const NIST09: &str = "tests/fixtures/wild/nist/nist_ftc_09_asme1_rd.stp";

/// The S58 iso-rectangle predicate, by name: *every rim sits at one of
/// the face's two extreme `v`-levels*. Naming the PREDICATE rather than
/// the shared "shared at-rest validation gate" preamble is what lets
/// these rows see a regression that re-widens the rule, as opposed to
/// one that merely moves the refusal somewhere else.
const ISO_RECTANGLE_PREDICATE: &str = "props_rim_level";

/// The seam carrier's residual is DECIDEDLY outside the band.
const SEAM_HALFPLANE_DEFINITE: &str =
    "SeamHalfplane residual at sample 0 definitely exceeds the tolerance band";
/// The same residual, IN the band: escalate-never-guess, by name.
const SEAM_HALFPLANE_ESCALATED: &str =
    "SeamHalfplane at sample 0 escalated: predicate 'carrier_in_seam_halfplane' indeterminate";
/// Coarse enough for the two walls to read as one: the Intersection
/// transversality precondition fails, and the ladder says which.
const TANGENT_PLANES_COINCIDE: &str = "tangent planes coincide at interior sample 1 — the Intersection transversality \
     precondition fails";
/// At ambient 1e-6 the file's own span decision is in-band too, and it
/// is reached first — at assembly, before any edge is adopted.
const PARAM_SPAN_ESCALATED: &str =
    "ParamSpan (not a sampled check) escalated: predicate 'interval_span_forward' indeterminate";
/// Naming the MAPPED-CURVE arm pins that BOTH candidates were tried and
/// both refused definite — the seam arm alone would match a prefix.
const ENDPOINT_START_MAPPED_CURVE: &str = "mapped curve: geometry attachment gate: certification: EndpointStart residual at sample 0 \
     definitely exceeds";

/// Every committed STEP file, with the disposition measured at M7-7.
/// Paths are relative to this crate's manifest directory (the `../`
/// rows are `step-export`'s corpus, which this crate imports from).
const CORPUS: [(&str, Disposition); 73] = [
    ("tests/fixtures/band/band_a.stp", Pass(1, 1, 2, 6, 4)),
    ("tests/fixtures/band/band_a180.stp", Pass(1, 1, 2, 6, 4)),
    ("tests/fixtures/band/band_b180.stp", Pass(1, 1, 2, 6, 4)),
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
    ("tests/fixtures/band/washer180.stp", Pass(1, 1, 3, 5, 4)),
    ("tests/fixtures/band/washer90.stp", Pass(1, 1, 3, 5, 4)),
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
        Pass(2, 2, 8, 14, 10),
    ),
    ("tests/fixtures/freecad/cone_apex.step", Pass(1, 1, 3, 4, 3)),
    (
        "tests/fixtures/freecad/cone_trunc.step",
        Pass(1, 1, 3, 3, 2),
    ),
    ("tests/fixtures/freecad/cylinder.step", Pass(1, 1, 3, 3, 2)),
    (
        "tests/fixtures/freecad/fuse_boxes.step",
        Pass(1, 1, 14, 32, 20),
    ),
    ("tests/fixtures/freecad/sphere.step", Pass(1, 1, 2, 2, 2)),
    ("tests/fixtures/freecad/torus.step", Pass(1, 1, 2, 4, 2)),
    (
        "tests/fixtures/freecad/twobody_importexport.step",
        Pass(2, 2, 8, 14, 10),
    ),
    // -- tests/fixtures/cert1-r1/ (reviewer probes, adopted) ----------
    // R1's adversarial near-polar variants of the halfcap generator:
    // `nearpolar_*` puts the rim 0.0208 rad off the pole; `polesplit_*`
    // is issue 723's body with the split vertex EXACTLY on the pole
    // (the pole-membership decide sits on its Zero through this door).
    // `cert1_r1_import_probes.rs` holds all four to the closed form.
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
        Pass(1, 1, 3, 3, 2),
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
    // volume.
    ("tests/fixtures/halfcap/halfcap.step", Pass(1, 1, 3, 4, 3)),
    // The near-pole split twins: the same solid with the ordinary
    // vertex 1e-6 / 1e-7 rad off the pole, landing the
    // pole-membership margin inside or beside the default band —
    // refused `Escalated` until the indeterminate outcome folded.
    (
        "tests/fixtures/halfcap/halfcap_eps6.step",
        Pass(1, 1, 3, 4, 3),
    ),
    (
        "tests/fixtures/halfcap/halfcap_eps7.step",
        Pass(1, 1, 3, 4, 3),
    ),
    (
        "tests/fixtures/halfcap/halfcap_nosplit.step",
        Pass(1, 1, 3, 3, 2),
    ),
    // -- tests/fixtures/iso-rect/ (S58 / #649) ------------------------
    // #649's own fixtures, committed with the fix. Both plus-domain
    // solids are geometrically VALID — manifold, closed, χ = 2 — and
    // both refuse here on the one iso-rectangle predicate, but they
    // arrive from opposite places. `cross` USED to import and then
    // MEASURE: 19% low with `pad = 0.0`, a certificate of exactness on
    // a wrong number, this gate green — it is the disposition S58
    // moved. `tee` was already refused before S58, by the span-sum
    // rule (`props_du_consistent`): only the reason in its refusal
    // string moved, which is why pinning the reason rather than the
    // disposition is what makes these rows able to see a regression.
    // `rect` is the control (a genuine iso-rectangle of the same Δu and
    // v extent) and `xsplit` is the same solid as `cross` authored with
    // rectangular sub-faces; both keep passing, and
    // `s58_iso_rectangle.rs` holds them to their EXACT volumes and runs
    // `merge_coplanar_faces` on `xsplit` — #649's second door.
    (
        "tests/fixtures/iso-rect/cross.step",
        Refused(ISO_RECTANGLE_PREDICATE),
    ),
    ("tests/fixtures/iso-rect/rect.step", Pass(1, 1, 6, 12, 8)),
    (
        "tests/fixtures/iso-rect/tee.step",
        Refused(ISO_RECTANGLE_PREDICATE),
    ),
    (
        "tests/fixtures/iso-rect/xsplit.step",
        Pass(1, 1, 18, 40, 24),
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
    // #653's import route: one D-prism, stated four ways. The two
    // `split_*` files state the cylindrical face's vertical boundary as
    // two collinear `EDGE_CURVE`s, which is what every exporter emits
    // when a vertex lands mid-side; the two `*_oblique` files place the
    // part by a general rotation. Tessellating them is
    // `split_iso_side.rs`'s row — here they are corpus like any other
    // file, and the census below is the pin that says the difference is
    // exactly the split and nothing else.
    //
    // TWO slots move, necessarily: `Pass` is (solids, shells, faces,
    // edges, vertices), and splitting one edge in two also mints the
    // vertex between them — 6 edges/4 vertices becomes 7/5. Solids,
    // shells and faces are identical, which is the part that matters:
    // the importer did not merge the sub-edges back together (which
    // would have shown as 6 edges) and did not adopt an extra face.
    // The `*_oblique` rows equal their `*_axis` counterparts exactly,
    // so the placement changes no count at all.
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
        Pass(1, 1, 4, 7, 5),
    ),
    (
        "tests/fixtures/split-iso/split_oblique.step",
        Pass(1, 1, 4, 7, 5),
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
        EpsSensitive,
    ),
    (
        "tests/fixtures/wild/nist/nist_ftc_11_asme1_rb.stp",
        Pass(1, 1, 6, 14, 10),
    ),
    (
        "tests/fixtures/wild/occ-oss/b123d_nema17_bracket.step",
        Refused("(SURFACE_CURVE) is outside the imported subset"),
    ),
    (
        "tests/fixtures/wild/occ-oss/cq_red_cube_blue_cylinder.step",
        Pass(2, 2, 9, 17, 12),
    ),
    (
        // Refuses in EVERY ambient-ε cell, but the refusal SITE and
        // sub-reason shift with ε (edge #170 Surface2Residual at
        // default/1e-12; edge #177 tangent-planes at 1e-6). This row's
        // fragment is the shared adoption-ladder preamble, so the shift
        // is invisible here BY DESIGN: pinning it would cost 9 more
        // EpsSensitive cells to buy site-pinning the pin design does
        // not offer anywhere (fragments pin sub-reason, not site). The
        // preamble still pins the refusal to the adoption-ladder class
        // — a drift to a parse error, crash, or Pass goes red.
        "tests/fixtures/wild/stepcode/TAIL_TURBINE.stp",
        Refused("no intensional description certifies"),
    ),
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
        // rational-flux stall above, but at ambient 1e-6 the ladder
        // stops earlier, on edge `#389`. Retiring #685 is what made
        // #389 reachable at all — it had been masked behind #685 at
        // every band — so the coarse cell is a PRE-EXISTING gap newly
        // exposed, not a movement of anything #327 built. Three cells
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
        Pass(1, 1, 2, 2, 2),
    ),
    (
        "../step-export/tests/fixtures/boss_union.step",
        Pass(1, 1, 10, 21, 14),
    ),
    (
        "../step-export/tests/fixtures/composed_die.step",
        Pass(1, 1, 89, 195, 129),
    ),
    (
        "../step-export/tests/fixtures/cone.step",
        Pass(1, 1, 4, 6, 4),
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
        Pass(1, 1, 48, 96, 71),
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
        // The touching two-solid assembly: since M9-2 the shared gate
        // is the tier-3′ form, so the UNDECLARED corner kiss refuses
        // typed here — and certifies WITH the import-side declaration
        // (the WITH/WITHOUT pair is pinned in
        // review_r1_tier_gate_probes.rs, the retired R1 finding).
        "../step-export/tests/fixtures/kiss_assembly.step",
        Refused("undeclared contact"),
    ),
    (
        "../step-export/tests/fixtures/lily_lantern.step",
        Pass(1, 1, 8, 14, 8),
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
        "../step-export/tests/fixtures/washer.step",
        Pass(1, 1, 4, 8, 4),
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
            topo::validate_geometric(&body, Tol::witness()),
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

/// Every corpus file's disposition, and the positive tier-validity of
/// every body that ships — at each tolerance in the sweep.
#[test]
fn every_corpus_import_passes_the_shared_gate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // File-major, so each file is read once and each file's own ε_in
    // sweep (see `eps_in_rows_for`) is visible at the loop head.
    for (rel, row) in CORPUS {
        let text = std::fs::read_to_string(root.join(rel))
            .unwrap_or_else(|e| panic!("reading {rel}: {e}"));
        for &(eps_tag, eps_in) in eps_in_rows_for(rel) {
            let options = ImportOptions {
                eps_in,
                ..ImportOptions::default()
            };
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
                        topo::validate_geometric(&body, Tol::witness()),
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
                    let msg = e.to_string();
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
}

/// The reader touches the kernel's validator at exactly ONE place (the
/// #260 ask: make skipping it structurally hard). The gate is asked
/// about several subjects — each solid alone, then the assembled body
/// — but always through `lib.rs`'s `gate`, which maps the verdicts to
/// a typed refusal and does nothing else. Import owns no validation
/// logic of its own: no second entry, no kind predicate deciding who
/// is asked, no verdict filter deciding which failures count. This
/// counts the validator calls in the crate's sources and pins the
/// count at one; a second call is not automatically wrong, but it is
/// exactly the shape the old band-only backstop had, so it must be
/// argued for here rather than appear.
#[test]
fn exactly_one_validation_call_site_in_the_reader() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sites = Vec::new();
    // The needle is a CALL, so comments and literal bodies are both
    // blanked: a commented-out door must not answer for a live one,
    // and prose naming the validator must not manufacture a site. The
    // blanked view keeps line structure, so the line number is real.
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).unwrap();
        for (i, line) in test_utils::source::code_only(&text).lines().enumerate() {
            if line.contains("validate_geometric(") || line.contains("validate_pseudomanifold(") {
                sites.push(format!("{}:{}", path.display(), i + 1));
            }
        }
    }
    // Two doors since M9-2, still zero opinions: the per-solid
    // tier-3 door (`gate`) and the aggregate tier-3′ door (`gate3`,
    // which consumes the import-side declaration channel). Anything
    // beyond these two is validation logic growing in the reader.
    assert_eq!(
        sites.len(),
        2,
        "the reader must call the kernel's at-rest validators at exactly the two \
         named doors (per-solid tier 3, aggregate tier 3′): {sites:?}"
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
    assert_eq!(
        errors.as_slice(),
        [topo::ValidationError::NegativeVolume],
        "the verdicts are the kernel's own, unfiltered and unrephrased"
    );
    let msg = e.to_string();
    for want in [
        "shared at-rest validation gate",
        "NegativeVolume",
        "signed volume is definitely negative",
    ] {
        assert!(msg.contains(want), "the message must name {want:?}: {msg}");
    }
}

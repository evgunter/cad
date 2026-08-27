//! **M7-2 acceptance: the FreeCAD-authored foreign corpus** — the
//! first geometry this kernel adopts that it did not write.
//!
//! Rows 1, 2, 4, 5, 6 and 7 of M7-2's acceptance list live here (row 3,
//! the own-corpus regression, is the M7-1 suites staying green in
//! `roundtrip.rs` / `parser.rs` / `review_probes.rs`; the three refusals
//! M7-2 is defined to retire flip there, named).
//!
//! **Comparison discipline** is `common`'s, unchanged: counts,
//! certified scalars, and structural invariants only — never arena
//! order against walk order.
//!
//! **Where the expectations come from**: `tests/fixtures/freecad/gen.py`
//! is committed beside the files as provenance, so every census and
//! every volume below is derived from the generator's own dimensions in
//! a comment, never read back from the importer.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::f64::consts::PI;

use common::{FREECAD_FIXTURES, census, freecad_fixture};
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

/// What a FreeCAD fixture must import to.
struct Expect {
    /// (solids, shells, faces, edges, vertices) — the KERNEL census,
    /// which is the file's own except where a structure normalization
    /// re-mints a face (`normalizations` below states the mapping).
    census: (usize, usize, usize, usize, usize),
    /// The generator's closed-form volume in **mm³** (FreeCAD's own
    /// unit), with its derivation in the table's comment.
    volume_mm3: f64,
    /// The **relative** dialect budget the file's own printed text
    /// forces on the volume, beyond quadrature pad and roundoff.
    ///
    /// Nonzero only for the cones. Their height is DERIVED from the
    /// printed semi-angle (`apex = base − axis·r/tan α`), and Open
    /// CASCADE prints α truncated to 12 significant digits, so the
    /// derived height carries `|dh/h| = |dα| / (sin α · cos α)` with
    /// `|dα| ≤ 5e-13` (half the last printed place — the
    /// `step_import::tolerance` budget). The volume is linear in `h`,
    /// so the same relative bound lands on it:
    ///   * `cone_apex`   α = π/4:        sin α cos α = 0.5 → 1.0e-12
    ///   * `cone_trunc`  α = atan(1/2):  sin α cos α = 0.4 → 1.25e-12
    ///
    /// Every other fixture's numbers are dyadics the file prints
    /// exactly, so their budget is zero and only roundoff applies.
    dialect_rel: f64,
    /// How many structure normalizations the import must report.
    normalizations: usize,
}

/// The corpus expectations, derived from `gen.py`'s dimensions
/// (FreeCAD's unit is mm; the kernel's is m, so each volume is scaled
/// by 1e-9 at the comparison).
fn expect(name: &str) -> Expect {
    // Shorthand: no dialect budget, no normalization.
    let plain = |census, volume_mm3| Expect {
        census,
        volume_mm3,
        dialect_rel: 0.0,
        normalizations: 0,
    };
    match name {
        // makeBox(1,1,1): a cube, 6 faces / 12 edges / 8 vertices.
        "box" | "box_importexport" => plain((1, 1, 6, 12, 8), 1.0),
        // makeCylinder(0.5, 1): the seam-unsplit wall (one face, its
        // seam edge used twice) plus two caps; V = π r² h.
        "cylinder" => plain((1, 1, 3, 3, 2), PI * 0.25),
        // makeCone(1, 0.5, 1): frustum, V = (π h / 3)(R² + R r + r²).
        "cone_trunc" => Expect {
            dialect_rel: 1.25e-12,
            ..plain((1, 1, 3, 3, 2), PI * (1.0 + 0.5 + 0.25) / 3.0)
        },
        // makeCone(1, 0, 1): a full cone, V = π R² h / 3. Its lateral
        // face is re-minted (the apex is a valence-1 vertex as stated),
        // so the census is the kernel's 2 lateral halves + 1 cap.
        "cone_apex" => Expect {
            dialect_rel: 1.0e-12,
            normalizations: 1,
            ..plain((1, 1, 3, 4, 3), PI / 3.0)
        },
        // makeSphere(1): one edge-free face in the file; the kernel's
        // canonical ball splitting after normalization. V = 4π r³ / 3.
        "sphere" => Expect {
            normalizations: 1,
            ..plain((1, 1, 2, 2, 2), 4.0 * PI / 3.0)
        },
        // makeTorus(1, 0.25): one fundamental-polygon face in the file;
        // the kernel's two half-faces after normalization.
        // V = 2π² R r².
        "torus" => Expect {
            normalizations: 1,
            ..plain((1, 1, 2, 4, 2), 2.0 * PI * PI * 0.0625)
        },
        // makeBox(2,2,1) cut by makeCylinder(0.5,1) through the centre:
        // 4 walls + 2 rings + the bore = 7 faces. V = 2·2·1 − π r² h.
        "box_hole" => plain((1, 1, 7, 15, 10), 4.0 - PI * 0.25),
        // makeBox(1,1,1) ∪ the same box at (0.5,0.5,0): the overlap is
        // 0.5·0.5·1. V = 1 + 1 − 0.25.
        "fuse_boxes" => plain((1, 1, 14, 32, 20), 1.75),
        // Unit box with ONE vertical edge filleted at r = 0.25: the
        // fillet replaces a square corner prism by a quarter cylinder
        // over the full height, removing r²(1 − π/4)·h.
        "box_fillet_edge" => plain((1, 1, 7, 15, 10), 1.0 - 0.0625 * (1.0 - PI / 4.0)),
        // Unit box with the THREE edges at the origin corner filleted
        // at r = 0.25. Away from the corner cube [0,r]³ each of the
        // three edges loses r²(1 − π/4) per unit length over its
        // remaining length (1 − r); inside the corner cube the solid
        // that survives is the sphere octant of radius r centred at
        // (r,r,r), i.e. π r³/6 of the cube's r³.
        "box_fillet_corner" => plain(
            (1, 1, 10, 21, 13),
            1.0 - 3.0 * 0.0625 * (1.0 - PI / 4.0) * 0.75 - (0.015625 - PI * 0.015625 / 6.0),
        ),
        // makeCompound([box, sphere(0.5) at (3,0,0)]): two solids under
        // a plain SHAPE_REPRESENTATION. V = 1 + 4π(0.5)³/3.
        // Import.export's two-body document is the same two solids
        // reached through the assembly layer (identity transforms).
        "compound_two" | "twobody_importexport" => Expect {
            normalizations: 1,
            ..plain((2, 2, 8, 14, 10), 1.0 + 4.0 * PI * 0.125 / 3.0)
        },
        other => panic!("no expectations stated for fixture {other}"),
    }
}

/// The corpus's smallest cylindrical feature radius in kernel metres:
/// `gen.py`'s `makeFillet(0.25, …)` — 0.25 mm. The bore and the
/// cylinder wall are twice that; nothing round in the corpus is
/// smaller.
const SMALLEST_ROUND_FEATURE_M: f64 = 2.5e-4;

/// The ambient-ε ceiling above which **this corpus is below the
/// kernel's own tolerance scale**, and the rows that certify it stop
/// being meaningful.
///
/// # The finding (reported, not worked around)
///
/// FreeCAD authors in millimetres, so the adopted bodies are about
/// **1000× smaller** than the kernel's own metre-scale corpus: the
/// whole cylinder fixture is 1 mm tall and 0.5 mm in radius. The
/// kernel's ambient ε is an ABSOLUTE length, and so are the K
/// ambiguity bands around every certification predicate. Raising ε to
/// 1e-6 m therefore does not make this corpus 1000× easier the way it
/// does the native one — it makes it 1000× *harder*, because ε is now
/// a fifth of a percent of the smallest round feature.
///
/// What the kernel does about that is exactly right, and is the reason
/// this is a ceiling and not a bug: it **refuses, typed**, at the
/// certification gates. Nothing is silently accepted.
///
/// **Re-measured at M6-3** (walk row 4). The pre-M6-3 table put the
/// ceiling at 1e-8: the 1e-7/1e-6 refusals came from the
/// `pcurve_chart_radial_moving` trilean, whose old metering weighted
/// the radial amplitude BY THE CHART RADIUS — an r²-scaled margin
/// that landed mm-scale rims in the ambiguity band three decades
/// early. With the amplitude metered as the displacement it is
/// (metres), the corpus certifies through 1e-5, and the gates that
/// finally refuse are the attachment/span trileans at the corpus's
/// own feature scale. Measured, end to end:
///
/// | ambient ε | outcome |
/// |---|---|
/// | 1e-12 … 1e-6, 1e-5 | all 13 import, all three tiers green |
/// | 1e-4 | ε is 40% of the smallest round feature: the corpus refuses typed across the attachment gates (`dihedral_arm`/`dihedral_wedge` in-band on edge certification, `interval_span_forward` on sub-band parameter spans, `tangent_second_order` escalations) — every refusal naming its predicate and band |
///
/// The ceiling is set at the finest ε measured to hold the whole
/// corpus (1e-5 m — 4% of the smallest round feature); the true
/// boundary lies between there and 1e-4. Above it the certifying rows
/// **skip loudly** and
/// [`sub_tolerance_geometry_is_refused_not_silently_imported`] takes
/// over, pinning the claim that actually matters at any ε: a body the
/// kernel cannot certify at the ambient tolerance is REFUSED, never
/// handed out wrong.
const CORPUS_EPS_CEILING: f64 = 1e-5;

/// Whether the ambient ε is fine enough for this millimetre corpus to
/// certify; prints a loud skip naming the numbers when it is not.
fn corpus_scale_gate(row: &str) -> bool {
    let eps = geom_core::Tol::witness().get().eps;
    if eps <= CORPUS_EPS_CEILING {
        return true;
    }
    println!(
        "{row}: above the ceiling — ambient ε {eps:e} m exceeds {CORPUS_EPS_CEILING:e} m for a \
         millimetre-authored corpus (ε is {:.1e} of the smallest round feature, \
         {SMALLEST_ROUND_FEATURE_M:e} m). Asserting the SUB-TOLERANCE obligation instead of \
         this row's certifying one.",
        eps / SMALLEST_ROUND_FEATURE_M
    );
    // **Not a no-op.** A row that returns early having asserted nothing
    // is a green tick for work not done, and a matrix full of those
    // rots into fiction while still reading as a pass. So above the
    // ceiling every certifying row asserts the obligation that IS
    // meaningful there — the corpus meets the kernel's gates and is
    // REFUSED, typed, rather than certified — and fails if it is not.
    assert_sub_tolerance_obligation(row);
    false
}

/// The obligation that holds at any ε: every fixture's outcome is
/// typed, nothing imports that the kernel then calls geometrically
/// false, and — above the ceiling — at least one fixture actually hits
/// the certification gates.
///
/// # What tier 3 is allowed to say
///
/// Below the ceiling, `Ok(())`. Above it, `Ok(())` **or an
/// escalation** — a K in-band / indeterminate classification, which is
/// the kernel declining to answer at a tolerance too coarse for the
/// geometry (the allowance stands for honestly-metered in-band
/// margins). What
/// is never allowed is a DEFINITE geometric falsehood
/// such as `NegativeVolume`: "cannot compute at this ε" is honest,
/// "computed, and it is inside out" is a body that should not have
/// been handed out. This is the row's whole claim, and it is asserted
/// rather than scoped away.
fn assert_sub_tolerance_obligation(row: &str) {
    let eps = geom_core::Tol::witness().get().eps;
    let mut refused = Vec::new();
    for name in FREECAD_FIXTURES {
        match import_step(
            &freecad_fixture(name),
            &ImportOptions::default(),
            Tol::witness(),
        ) {
            Ok(StepImport::Solid { body, .. }) => {
                assert_eq!(
                    topo::validate(&body),
                    Ok(()),
                    "{row}/{name}: tier 1 at ε {eps:e}"
                );
                assert_eq!(
                    topo::validate_closed(&body),
                    Ok(()),
                    "{row}/{name}: tier 2 at ε {eps:e}"
                );
                if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
                    assert!(
                        errs.iter().all(is_escalation),
                        "{row}/{name}: tier 3 at ε {eps:e} reports a definite geometric \
                         falsehood, not an escalation — a body like this must never have \
                         been imported: {errs:?}"
                    );
                    println!(
                        "{row}/{name}: tier 3 declines at ε {eps:e} (in-band escalation, \
                         not a wrong answer): {errs:?}"
                    );
                }
            }
            Ok(StepImport::Wireframe { .. }) => panic!("{row}/{name}: not a wireframe"),
            Err(e) => {
                // A refusal is fine; a refusal that says nothing is not.
                let text = e.to_string();
                assert!(
                    text.starts_with("step import:") && text.len() > 40,
                    "{row}/{name}: a refusal must name what it refused: {text}"
                );
                refused.push(format!("{name}: {text}"));
            }
        }
    }
    if eps <= CORPUS_EPS_CEILING {
        assert!(
            refused.is_empty(),
            "at ε {eps:e} the whole corpus must certify, but: {refused:?}"
        );
    } else {
        for r in &refused {
            println!("{row}: sub-tolerance refusal at ε {eps:e} — {r}");
        }
        assert!(
            !refused.is_empty(),
            "at ε {eps:e} — coarser than the stated ceiling — the millimetre corpus must \
             hit the kernel's certification gates; if nothing refuses, the ceiling is wrong"
        );
    }
}

/// Whether a tier-3 error is the kernel DECLINING to answer (a K
/// in-band / indeterminate classification) rather than answering
/// falsely.
fn is_escalation(e: &topo::ValidationError) -> bool {
    matches!(
        e,
        topo::ValidationError::VolumeUncomputable { .. }
            | topo::ValidationError::PlanarFaceEscalated { .. }
            | topo::ValidationError::PlanarBoundaryEscalated { .. }
            | topo::ValidationError::CensusEscalated { .. }
    )
}

/// **The scale finding's own row, live at every ε.** Whatever the
/// ambient tolerance, a fixture either imports as a body the kernel's
/// tiers accept — with tier 3 either green or DECLINING in band, never
/// answering falsely — or it refuses TYPED. What must never
/// happen — and is what a quiet gate-widening would produce — is a
/// body that imports and is then not a solid.
///
/// Below [`CORPUS_EPS_CEILING`] every fixture takes the first arm (the
/// full rows assert the rest). Above it, at least one must take the
/// second, or the ceiling is fiction.
#[test]
fn sub_tolerance_geometry_is_refused_not_silently_imported() {
    assert_sub_tolerance_obligation("sub_tolerance_geometry_is_refused_not_silently_imported");
}

/// Imports a FreeCAD fixture, panicking on refusal.
fn import_freecad(name: &str) -> StepImport {
    let text = freecad_fixture(name);
    import_step(&text, &ImportOptions::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("importing FreeCAD fixture {name}: {e}"))
}

/// The imported body plus its ε_in and reported normalizations.
fn freecad_body(
    name: &str,
) -> (
    topo::Body<f64>,
    f64,
    Vec<step_import::StructureNormalization>,
) {
    match import_freecad(name) {
        StepImport::Solid {
            body,
            eps_in,
            normalizations,
            ..
        } => (body, eps_in, normalizations),
        StepImport::Wireframe { .. } => panic!("{name} imported as a wireframe, expected a solid"),
    }
}

/// **Row 1 — the foreign-corpus row.** Every committed FreeCAD fixture
/// imports; census matches the stated expectation; certified volume
/// matches the generator's closed form within the quadrature's own
/// pad plus roundoff plus the file's own dialect budget; and the
/// validity ladder is green at default ε.
#[test]
fn foreign_corpus() {
    if !corpus_scale_gate("foreign_corpus") {
        return;
    }
    for name in FREECAD_FIXTURES {
        let (body, _eps, normalizations) = freecad_body(name);
        let e = expect(name);
        assert_eq!(census(&body), e.census, "{name}: census");
        assert_eq!(
            normalizations.len(),
            e.normalizations,
            "{name}: reported structure normalizations"
        );

        let props = topo::mass_properties(&body, Tol::witness())
            .unwrap_or_else(|err| panic!("{name}: {err}"));
        // mm³ → m³ (the generator's unit is FreeCAD's mm).
        let expected_m3 = e.volume_mm3 * 1e-9;
        // Roundoff: the volume is a fixed-order sum of per-face
        // contributions, each rounded, so the budget scales with the
        // number of summands (roundtrip.rs's own derivation).
        #[allow(clippy::cast_precision_loss)]
        let ulps = (e.census.2 as f64) * (expected_m3.next_up() - expected_m3);
        let dialect = expected_m3.abs() * e.dialect_rel;
        let tolerance = props.volume_pad + ulps + dialect;
        assert!(
            (props.volume - expected_m3).abs() <= tolerance,
            "{name}: volume {} m³ vs closed form {} m³ (tolerance {}: pad {} + ulps {} \
             + dialect {})",
            props.volume,
            expected_m3,
            tolerance,
            props.volume_pad,
            ulps,
            dialect
        );

        assert_eq!(topo::validate(&body), Ok(()), "{name}: tier 1");
        assert_eq!(topo::validate_closed(&body), Ok(()), "{name}: tier 2");
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "{name}: tier 3"
        );
    }
}

/// **Row 1's census-mapping pin.** The structure normalizations are
/// carried out as DATA, and this is the mapping table a reader needs:
/// what the file states for the re-minted region, against what the
/// kernel minted in its place. Nothing here is silent.
#[test]
fn structure_normalizations_are_reported_with_their_census_mapping() {
    if !corpus_scale_gate("structure_normalizations") {
        return;
    }
    use step_import::{FaceCensus, NormalizationKind};
    let cases = [
        // sphere.step #17: ADVANCED_FACE over the whole sphere, its
        // only bound a VERTEX_LOOP — 1 face, 0 edges, 1 vertex.
        (
            "sphere",
            NormalizationKind::EdgeFreeSphere,
            (1, 0, 1),
            (2, 2, 2),
        ),
        // compound_two.step #167 and twobody's sphere: the same shape.
        (
            "compound_two",
            NormalizationKind::EdgeFreeSphere,
            (1, 0, 1),
            (2, 2, 2),
        ),
        (
            "twobody_importexport",
            NormalizationKind::EdgeFreeSphere,
            (1, 0, 1),
            (2, 2, 2),
        ),
        // cone_apex.step #17: the lateral face, its seam generator
        // ending at a valence-1 apex — 1 face, 2 edges, 2 vertices.
        (
            "cone_apex",
            NormalizationKind::DegenerateApexCone,
            (1, 2, 2),
            (2, 4, 3),
        ),
        // torus.step #17: the fundamental-polygon face — 1 face, 2
        // edges (each used twice), 1 vertex.
        (
            "torus",
            NormalizationKind::FullPeriodTorus,
            (1, 2, 1),
            (2, 4, 2),
        ),
    ];
    let cens = |(faces, edges, vertices)| FaceCensus {
        faces,
        edges,
        vertices,
    };
    for (name, kind, file, kernel) in cases {
        let (_body, _eps, norms) = freecad_body(name);
        let [n] = norms.as_slice() else {
            panic!("{name}: expected exactly one reported normalization, got {norms:?}");
        };
        assert_eq!(n.kind, kind, "{name}: normalization kind");
        assert_eq!(n.file_census, cens(file), "{name}: the file's census");
        assert_eq!(n.kernel_census, cens(kernel), "{name}: the kernel's census");
        assert!(n.face > 0, "{name}: the normalization names a real entity");
    }
    // And nothing else in the corpus is re-minted: normalization is a
    // bounded repair of named cases, not a habit.
    for name in FREECAD_FIXTURES {
        assert_eq!(
            freecad_body(name).2.len(),
            expect(name).normalizations,
            "{name}: normalization count"
        );
    }
}

/// **Row 2 — the cross-dialect fixed point.** import(FreeCAD file) →
/// `step_export::step_string` → import again: censuses and certified
/// volumes bit-identical, and the SECOND export byte-identical to the
/// first.
///
/// The fixed point is in OUR dialect after one hop, and only there:
/// byte-identity with the FreeCAD source is impossible (different
/// units, different product structure, different entity order) and is
/// not claimed. What one hop must establish is that adopting a foreign
/// file lands on a body the writer round-trips exactly — that nothing
/// about the foreign dialect survives as a wobble.
#[test]
fn cross_dialect_fixed_point() {
    if !corpus_scale_gate("cross_dialect_fixed_point") {
        return;
    }
    for name in FREECAD_FIXTURES {
        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let (body1, _, _) = freecad_body(name);
        let export1 = step_export::step_string(&body1, &options, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-export 1: {e}"));
        let reimport = import_step(&export1, &ImportOptions::default(), Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-import of our own dialect: {e}"));
        let StepImport::Solid { body: body2, .. } = reimport else {
            panic!("{name}: re-import lost the solid");
        };
        assert_eq!(
            census(&body1),
            census(&body2),
            "{name}: census identical across the adoption pass"
        );
        let export2 = step_export::step_string(&body2, &options, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: re-export 2: {e}"));
        assert_eq!(
            export1, export2,
            "{name}: the second export must be byte-identical to the first"
        );
        let v1 = topo::mass_properties(&body1, Tol::witness())
            .unwrap()
            .volume;
        let v2 = topo::mass_properties(&body2, Tol::witness())
            .unwrap()
            .volume;
        // Bit-identity everywhere but ONE named fixture. Byte-identical
        // exports already prove both bodies carry the same stated
        // geometry, so any residue is arithmetic, not data — and the
        // one residue has a cause: STEP carries no trim parameters, so
        // an edge's carrier interval is DERIVED from its vertices on
        // every import. `cone_apex` is the only fixture whose derived
        // intervals are re-derived across a structure normalization
        // (its base circle is split at its own parameter midpoint on
        // the first import, and re-derived independently from the two
        // half-arcs' vertices on the second), and the closed-form cone
        // contribution reads that interval. The residue is one ulp,
        // asserted as one ulp — not widened, not waved away.
        let residue = if name == "cone_apex" {
            v1.abs().next_up() - v1.abs()
        } else {
            0.0
        };
        assert!(
            (v1 - v2).abs() <= residue,
            "{name}: certified volume across the adoption pass: {v1} vs {v2} \
             (allowed residue {residue})"
        );
    }
}

// ---- Row 4: the dialect rows ---------------------------------------

/// A FreeCAD fixture's text with one exact substitution applied (the
/// probe idiom: change one thing, name what it proves).
/// A body's certified volume in mm³ — the comparable scalar for the
/// S9 flip rows, where what matters is that two imports describe the
/// same solid (or a stated multiple of it).
fn volume_mm3(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass properties")
        .volume
        * 1e9
}

/// The least x over a body's points, in mm — the one scalar that
/// distinguishes a placed body from an unplaced one when every
/// rigid invariant, by construction, cannot.
fn min_x_mm(body: &topo::Body<f64>) -> f64 {
    body.points()
        .fold(f64::INFINITY, |acc, (_, p)| acc.min(p.x))
        * 1e3
}

/// Each solid's own x extent in millimetres, in body solid order — the
/// instrument a per-component placement needs (a whole-body extent
/// cannot tell "both moved 5 mm" from "one moved 10 mm").
fn solid_x_mm(body: &topo::Body<f64>) -> Vec<(f64, f64)> {
    body.solids()
        .map(|(sk, solid)| {
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for shell in &solid.shells {
                for &face in &body.get_shell(*shell).expect("a shell").faces {
                    let f = body.get_face(face).expect("a face");
                    for lk in std::iter::once(f.outer).chain(f.rings.iter().copied()) {
                        let topo::LoopBoundary::Cycle { first } =
                            body.get_loop(lk).expect("a loop").boundary
                        else {
                            continue;
                        };
                        for he in body.loop_cycle(first).expect("a cycle") {
                            let v = body.get_half_edge(he).expect("a use").start;
                            let p = body
                                .get_point(body.get_vertex(v).expect("a vertex").point)
                                .expect("a point");
                            lo = lo.min(p.x * 1e3);
                            hi = hi.max(p.x * 1e3);
                        }
                    }
                }
            }
            let _ = sk;
            (lo, hi)
        })
        .collect()
}

fn mutated(name: &str, from: &str, to: &str) -> String {
    let text = freecad_fixture(name);
    assert!(
        text.contains(from),
        "{name}: probe anchor {from:?} not present"
    );
    text.replace(from, to)
}

/// **Row 4 (a) — millimetre scaling against closed forms.** Every
/// length in a FreeCAD file is a millimetre, and the whole file passes
/// through one correctly-rounded multiply by 1e-3 on its way to kernel
/// metres.
///
/// Where the mm value is a dyadic the SCALED value need not be: 1e-3 is
/// not a binary fraction, so `1. mm` becomes the f64 nearest 1e-3 m and
/// not the real number 1/1000. That is stated rather than papered over
/// — the assertion below is bitwise against `1e-3`, i.e. against the
/// same correctly-rounded decimal the file's own text names, which is
/// the strongest claim the arithmetic supports.
#[test]
fn millimetre_lengths_scale_by_one_rounded_multiply() {
    let (body, _, _) = freecad_body("box");
    let mut coords: Vec<f64> = body
        .vertices()
        .filter_map(|(_, v)| body.get_point(v.point).map(|p| [p.x, p.y, p.z]))
        .flatten()
        .collect();
    coords.sort_by(f64::total_cmp);
    coords.dedup();
    // gen.py's makeBox(1,1,1): every coordinate is 0 mm or 1 mm.
    assert_eq!(coords.len(), 2, "the unit box has two distinct coordinates");
    assert_eq!(coords[0].to_bits(), 0.0f64.to_bits(), "0 mm is exactly 0 m");
    assert_eq!(
        coords[1].to_bits(),
        (1.0f64 * 1e-3).to_bits(),
        "1 mm is the f64 nearest 1e-3 m — one rounded multiply, no more"
    );
    // And the closed form lands within the roundoff that implies.
    let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
    assert!(
        (v - 1e-9).abs() <= 8.0 * f64::EPSILON * 1e-9,
        "unit cube volume {v} m³ vs 1e-9 m³"
    );
}

/// **Row 4 (b) — the base-placement cone's apex derivation.** Open
/// CASCADE places a cone at its BASE circle; the kernel's surface is
/// apex-placed, so the apex is derived as
/// `location − axis·(radius / tan α)`. The derivation is pinned against
/// the generator's closed form: `makeCone(1, 0, 1)` has its apex 1 mm
/// above the base plane, on the axis.
///
/// The budget is ε_in — the file's own declared uncertainty, scaled
/// (1e-7 mm → 1e-10 m). It has room to spare: Open CASCADE prints the
/// semi-angle truncated to 12 significant digits (`0.785398163397` for
/// π/4, a 4.5e-13 relative miss), which propagates to
/// `|dh| = |dα| · r / (sin α · cos α)` ≈ 1e-15 m here — five orders
/// under the flat budget. That headroom is the point: the truncation a
/// foreign writer introduces is RELATIVE, ε_in is ABSOLUTE, and for
/// any part smaller than about 100 m the absolute one dominates. A
/// model big enough to invert that fails adoption typed, and the
/// remedy is the per-call ε_in override.
#[test]
fn base_placement_cone_apex_matches_the_closed_form() {
    let (body, eps_in, _) = freecad_body("cone_apex");
    let apexes: Vec<_> = body
        .surfaces()
        .filter_map(|(_, s)| match *s {
            geom::Surface::Cone { apex, .. } => Some(apex),
            _ => None,
        })
        .collect();
    let [apex] = apexes.as_slice() else {
        panic!("cone_apex must carry exactly one conical surface, got {apexes:?}");
    };
    // gen.py: makeCone(1, 0, 1) — base circle radius 1 mm at z = 0,
    // apex at (0, 0, 1) mm, i.e. (0, 0, 1e-3) m.
    let closed_form = geom_core::Point3::new(0.0, 0.0, 1e-3);
    assert_eq!(eps_in, 1e-10, "the file's declared 1e-7 mm, scaled");
    let residual = (*apex - closed_form).norm();
    assert!(
        residual <= eps_in,
        "derived apex {apex:?} vs the generator's {closed_form:?} \
         (residual {residual}, budget {eps_in})"
    );
    // And the residual really is the printed semi-angle's truncation
    // showing through, not a coincidence: it is nonzero and of the
    // order |dα|·r/(sin α cos α) ≈ 1e-15 m.
    assert!(
        residual > 0.0 && residual < 1e-14,
        "the truncation residual {residual} should sit near 1e-15 m"
    );
}

/// **Row 4 (c) — `FACE_BOUND` orientation is honored, not ignored.**
///
/// Two probes, because the flag has two independent things to prove.
/// `box.step` states three faces with a `.F.` bound (#18, #98, #142 —
/// the x = 0, y = 0 and z = 0 faces); flattening those to `.T.` must
/// break the import, or the flag was being ignored. And the measured
/// COUNTEREXAMPLE class — a face whose `same_sense` is `.F.` while its
/// bound is `.T.` (`cone_apex`'s planar cap #43/#44) — proves the two
/// flags are independent: reversing that bound alone must break the
/// import too, which it could not do if `.F.` were merely the face
/// sense in disguise.
#[test]
fn face_bound_orientation_is_honored_independently_of_face_sense() {
    // (i) Ignoring the flag: flatten box.step's three reversed bounds.
    let text = freecad_fixture("box");
    let flattened = text
        .replace(
            "#18 = FACE_BOUND('',#19,.F.);",
            "#18 = FACE_BOUND('',#19,.T.);",
        )
        .replace(
            "#98 = FACE_BOUND('',#99,.F.);",
            "#98 = FACE_BOUND('',#99,.T.);",
        )
        .replace(
            "#142 = FACE_BOUND('',#143,.F.);",
            "#142 = FACE_BOUND('',#143,.T.);",
        );
    assert_ne!(text, flattened, "box.step must carry reversed bounds");
    let err = import_step(&flattened, &ImportOptions::default(), Tol::witness())
        .expect_err("a cube whose reversed bounds are flattened is not a closed shell");
    assert!(
        matches!(err, step_import::StepImportError::Topology { .. }),
        "expected a typed topology refusal, got: {err}"
    );

    // (ii) The counterexample class: cone_apex's cap is face .F. with
    // bound .T. — the pair the substrate measured four of. Reverse the
    // BOUND only.
    let probe = mutated(
        "cone_apex",
        "#44 = FACE_BOUND('',#45,.T.);",
        "#44 = FACE_BOUND('',#45,.F.);",
    );
    // (The owning face #43 is `.F.`; if the bound flag were redundant
    // with it, this edit would be a no-op.)
    assert!(freecad_fixture("cone_apex").contains("#43 = ADVANCED_FACE('',(#44),#47,.F.);"));
    let err = import_step(&probe, &ImportOptions::default(), Tol::witness())
        .expect_err("reversing one bound un-closes the shell");
    assert!(
        matches!(err, step_import::StepImportError::Topology { .. }),
        "expected a typed topology refusal, got: {err}"
    );
}

/// **Row 4 (d) — outerness inferred on the multi-ring fixture.**
/// `box_hole`'s top and bottom faces each carry two plain
/// `FACE_BOUND`s — an outer rectangle and the bore's circle,
/// syntactically indistinguishable. The inference must pick the
/// rectangle: exactly two faces have a ring, each ring is the bore's
/// single circular edge, and each outer loop is the four-sided
/// rectangle. (Row 1's volume is the independent check: reading the
/// bore as outer would make those faces disks instead of annuli.)
#[test]
fn outerness_is_inferred_on_the_multi_ring_fixture() {
    if !corpus_scale_gate("outerness_is_inferred_on_the_multi_ring_fixture") {
        return;
    }
    let (body, _, _) = freecad_body("box_hole");
    let cycle_len = |lk: topo::LoopKey| -> usize {
        let Some(topo::LoopBoundary::Cycle { first }) = body.get_loop(lk).map(|l| l.boundary)
        else {
            panic!("a finished face's loop must be a cycle");
        };
        let mut n = 0;
        let mut he = first;
        loop {
            n += 1;
            he = body.get_half_edge(he).expect("a live half-edge").next;
            if he == first {
                return n;
            }
        }
    };
    let mut ringed = 0;
    for (_, face) in body.faces() {
        if face.rings.is_empty() {
            continue;
        }
        ringed += 1;
        assert_eq!(face.rings.len(), 1, "each holed cap has exactly one ring");
        assert_eq!(cycle_len(face.outer), 4, "the outer bound is the rectangle");
        assert_eq!(
            cycle_len(face.rings[0]),
            1,
            "the ring is the bore's single closed circle"
        );
    }
    assert_eq!(ringed, 2, "the through-hole pierces exactly two faces");
}

/// **Row 4 (d, ambiguity) — an undecidable outerness refuses typed.**
/// Moving `box_hole`'s bore circle clear of its face leaves two
/// DISJOINT rings on one plane: neither contains the other, so there
/// is no outer bound to be had. D7 forbids guessing; the import
/// refuses, naming the face.
#[test]
fn ambiguous_outerness_refuses_typed() {
    // #135's circle is the bore's rim on the z = 1 cap; sliding its
    // centre 10 mm out in x puts it wholly outside the 2x2 rectangle.
    let text = freecad_fixture("box_hole");
    let anchor = text
        .lines()
        .find(|l| l.starts_with("#135 ="))
        .expect("the probe anchor line");
    println!("ambiguity probe anchors on: {anchor}");
    let probe = text.replace("(1.,1.,1.)", "(11.,1.,1.)");
    assert_ne!(text, probe, "the bore placement must actually move");
    let err = import_step(&probe, &ImportOptions::default(), Tol::witness())
        .expect_err("two disjoint rings have no outer bound");
    assert!(
        matches!(err, step_import::StepImportError::Topology { .. }),
        "expected a typed refusal naming the face, got: {err}"
    );
    assert!(
        format!("{err}").contains("outer") || format!("{err}").contains("bound"),
        "the refusal must say what it could not decide: {err}"
    );
}

/// **Row 4 (e) — negative zeros normalize to `+0.0`.** FreeCAD writes
/// `-0.` on nearly every placement (`DIRECTION('',(1.,0.,-0.))`). The
/// value is the same real number, but the bit pattern is not, and the
/// importer compares surface records bitwise to restore the writer's
/// key sharing. The proof is textual and end to end: the source carries
/// `-0.`, and nothing the importer produces does.
#[test]
fn negative_zeros_normalize_at_translation() {
    if !corpus_scale_gate("negative_zeros_normalize_at_translation") {
        return;
    }
    for name in FREECAD_FIXTURES {
        let text = freecad_fixture(name);
        assert!(text.contains("-0."), "{name}: the source dialect has -0.");
        let (body, _, _) = freecad_body(name);
        let export = step_export::step_string(
            &body,
            &step_export::StepOptions {
                product_name: name.to_owned(),
                ..step_export::StepOptions::default()
            },
            Tol::witness(),
        )
        .unwrap();
        // An exact token match: `-0.001` starts with `-0.0` and is a
        // genuine negative coordinate, not a negative zero.
        assert!(
            !export.contains("-0.0,") && !export.contains("-0.0)"),
            "{name}: an imported body must carry no negative zero"
        );
    }
}

// ---- Row 5: refusal preservation -----------------------------------

/// **Row 5 — the subset boundary still holds.** Relaxing the dialect
/// must not relax the SUBSET: every refusal M7-1 states that still
/// applies must still fire, typed and entity-named. The substrate
/// confirms FreeCAD never emits the first three, so they are the
/// subset's boundary rather than dead code — and each is exercised
/// here by planting it in a real FreeCAD file.
#[test]
fn refusals_survive_the_dialect_relaxations() {
    use step_import::StepImportError as E;

    // (a) **FLIPPED (M7-4 Leg E).** `EDGE_CURVE` `same_sense` `.F.`
    // used to refuse: the carrier runs against its own edge, and
    // reversing a carrier's parameterization would move bits the file
    // printed. Two wild files surfaced it, one of them an
    // imports-class target that reaches its oracle census only through
    // it, so the sense is now COMPOSED into the half-edge direction
    // instead — nothing about the carrier changes, and the box that
    // used to refuse here imports with the same census it has when the
    // same edge is stated `.T.`.
    let stated = import_step(
        &freecad_fixture("box"),
        &ImportOptions::default(),
        Tol::witness(),
    )
    .expect("the unmutated box imports");
    // The same edge, stated from its other end: the vertices swap,
    // the sense goes `.F.`, and each of the two `ORIENTED_EDGE`s that
    // use it flips to keep the loops walking the way they did. A
    // reader that composes the sense correctly cannot tell the two
    // files apart.
    let probe = mutated(
        "box",
        "#21 = EDGE_CURVE('',#22,#24,#26,.T.);",
        "#21 = EDGE_CURVE('',#24,#22,#26,.F.);",
    )
    .replace(
        "#20 = ORIENTED_EDGE('',*,*,#21,.F.);",
        "#20 = ORIENTED_EDGE('',*,*,#21,.T.);",
    )
    .replace(
        "#106 = ORIENTED_EDGE('',*,*,#21,.T.);",
        "#106 = ORIENTED_EDGE('',*,*,#21,.F.);",
    );
    let flipped = import_step(&probe, &ImportOptions::default(), Tol::witness())
        .expect("a .F. edge now composes");
    let (StepImport::Solid { body: a, .. }, StepImport::Solid { body: b, .. }) =
        (&stated, &flipped)
    else {
        panic!("both are solids");
    };
    assert_eq!(
        census(a),
        census(b),
        "the same edge said the other way round is the same edge"
    );
    assert_eq!(
        volume_mm3(a),
        volume_mm3(b),
        "and the same solid, to the bit"
    );

    // (b) **FLIPPED (M7-4 Leg C).** A non-unit `VECTOR` magnitude used
    // to refuse, because the file's line parameter would no longer be
    // the kernel's arc length. It still is not — but no trim parameter
    // crosses the wire, so the interval is re-derived from the two
    // vertices against the normalized direction, and the magnitude
    // simply has nowhere to land. ST-Developer writes `10.` on every
    // line it emits; that is 5 of the 8 imports-class fixtures.
    let probe = mutated(
        "box",
        "#28 = VECTOR('',#29,1.);",
        "#28 = VECTOR('',#29,2.);",
    );
    let rescaled = import_step(&probe, &ImportOptions::default(), Tol::witness())
        .expect("any positive magnitude now imports");
    let StepImport::Solid { body: c, .. } = &rescaled else {
        panic!("a solid");
    };
    assert_eq!(census(a), census(c), "the magnitude changes no topology");
    assert_eq!(volume_mm3(a), volume_mm3(c), "and moves no geometry");
    // What still refuses: a magnitude that is not a positive scale.
    for bad in ["0.", "-1.", "1.E400"] {
        let probe = mutated(
            "box",
            "#28 = VECTOR('',#29,1.);",
            &format!("#28 = VECTOR('',#29,{bad});"),
        );
        match import_step(&probe, &ImportOptions::default(), Tol::witness())
            .expect_err("a non-positive magnitude describes no line")
        {
            E::MalformedRecord { id, .. } => assert_eq!(id, 28, "the refusal names the VECTOR"),
            other => panic!("expected MalformedRecord, got: {other}"),
        }
    }

    // (c) B-spline geometry — the named M7 frontier. The corpus has
    // NONE (measured: zero B_SPLINE_* records in 13 files), which is
    // why the frontier is stated rather than exercised here; the
    // refusal itself is pinned in `parser.rs`
    // (`bspline_surface_refuses_typed`).
    for name in FREECAD_FIXTURES {
        assert!(
            !freecad_fixture(name).contains("B_SPLINE"),
            "{name}: the measured corpus has no B-spline geometry"
        );
    }

    // (d) **PARTLY FLIPPED (M7-4 Leg D).** A non-identity assembly
    // transform used to refuse outright. A RIGID one now places the
    // body through the kernel's own `transform_rigid` door — but only
    // when it places ALL of the file's content, because placing
    // components independently is assembly instancing and this crate
    // has no body graph to hold it. `twobody_importexport` has two
    // components with one `ITEM_DEFINED_TRANSFORMATION` each, so it
    // exercises both halves.
    let text = freecad_fixture("twobody_importexport");
    assert!(text.contains("#194 = ITEM_DEFINED_TRANSFORMATION('','',#11,#15);"));
    assert!(text.contains("#225 = ITEM_DEFINED_TRANSFORMATION('','',#11,#19);"));
    let unplaced =
        import_step(&text, &ImportOptions::default(), Tol::witness()).expect("identity transforms");
    let StepImport::Solid { body: base, .. } = &unplaced else {
        panic!("a solid");
    };
    // Both components displaced by the same 5 mm: one placement,
    // applied.
    let both = |x: &str| {
        text.replace(
            "#15 = AXIS2_PLACEMENT_3D('',#16,#17,#18);",
            &format!(
                "#15 = AXIS2_PLACEMENT_3D('',#9995,#17,#18);\n\
                 #9995 = CARTESIAN_POINT('',({x},0.,0.));"
            ),
        )
        .replace(
            "#19 = AXIS2_PLACEMENT_3D('',#20,#21,#22);",
            &format!(
                "#19 = AXIS2_PLACEMENT_3D('',#9996,#21,#22);\n\
                 #9996 = CARTESIAN_POINT('',({x},0.,0.));"
            ),
        )
    };
    let placed = import_step(&both("5."), &ImportOptions::default(), Tol::witness())
        .expect("one rigid placement over all of the file's content applies");
    let StepImport::Solid { body: moved, .. } = &placed else {
        panic!("a solid");
    };
    assert_eq!(
        census(base),
        census(moved),
        "a rigid placement moves a body, it does not re-shape one"
    );
    assert!(
        (volume_mm3(base) - volume_mm3(moved)).abs() <= 1e-9 * volume_mm3(base).abs(),
        "and volume is a rigid invariant"
    );
    assert!(
        min_x_mm(moved) - min_x_mm(base) > 4.9,
        "the body actually moved: {} → {}",
        min_x_mm(base),
        min_x_mm(moved)
    );
    // **FLIPPED AGAIN (M8 instancing).** Only ONE component placed
    // used to refuse here — one map had to cover all of a file's
    // content, because a placed body was placed by transforming the
    // FINISHED body once and there was nowhere to put a second frame.
    // The `REPRESENTATION_RELATIONSHIP` says which content each
    // transform places (`rep_1` is the component), so each component
    // now materializes under ITS OWN frame: the first body moves 5 mm,
    // the second stays exactly where the unplaced import put it.
    let one = text.replace(
        "#15 = AXIS2_PLACEMENT_3D('',#16,#17,#18);",
        "#15 = AXIS2_PLACEMENT_3D('',#9995,#17,#18);\n\
         #9995 = CARTESIAN_POINT('',(5.,0.,0.));",
    );
    let split = import_step(&one, &ImportOptions::default(), Tol::witness())
        .expect("per-component placement materializes per component");
    let StepImport::Solid { body: split, .. } = &split else {
        panic!("a solid");
    };
    assert_eq!(
        census(base),
        census(split),
        "placing one component of two re-shapes neither"
    );
    // The evidence that the frames landed on the RIGHT components:
    // per-solid x extents, against the unplaced import's own.
    let want: Vec<(f64, f64)> = solid_x_mm(base)
        .into_iter()
        .enumerate()
        .map(|(i, (lo, hi))| {
            if i == 0 {
                (lo + 5.0, hi + 5.0)
            } else {
                (lo, hi)
            }
        })
        .collect();
    let got = solid_x_mm(split);
    assert_eq!(got.len(), 2, "two components, two solids");
    for (i, ((glo, ghi), (wlo, whi))) in got.iter().zip(want.iter()).enumerate() {
        assert!(
            (glo - wlo).abs() <= 1e-9 && (ghi - whi).abs() <= 1e-9,
            "solid {i}: x extent [{glo}, {ghi}] mm, expected [{wlo}, {whi}]"
        );
    }
    // And two DIFFERENT frames, the case that has no single-map
    // reading at all: 5 mm and 12 mm, one file, two solids.
    let two = text
        .replace(
            "#15 = AXIS2_PLACEMENT_3D('',#16,#17,#18);",
            "#15 = AXIS2_PLACEMENT_3D('',#9995,#17,#18);\n\
             #9995 = CARTESIAN_POINT('',(5.,0.,0.));",
        )
        .replace(
            "#19 = AXIS2_PLACEMENT_3D('',#20,#21,#22);",
            "#19 = AXIS2_PLACEMENT_3D('',#9996,#21,#22);\n\
             #9996 = CARTESIAN_POINT('',(12.,0.,0.));",
        );
    let instanced = import_step(&two, &ImportOptions::default(), Tol::witness())
        .expect("two different component frames materialize as two placed solids");
    let StepImport::Solid {
        body: instanced, ..
    } = &instanced
    else {
        panic!("a solid");
    };
    assert_eq!(census(base), census(instanced), "still the same two bodies");
    let want: Vec<(f64, f64)> = solid_x_mm(base)
        .into_iter()
        .enumerate()
        .map(|(i, (lo, hi))| {
            let d = if i == 0 { 5.0 } else { 12.0 };
            (lo + d, hi + d)
        })
        .collect();
    for (i, ((glo, ghi), (wlo, whi))) in solid_x_mm(instanced).iter().zip(want.iter()).enumerate() {
        assert!(
            (glo - wlo).abs() <= 1e-9 && (ghi - whi).abs() <= 1e-9,
            "solid {i}: x extent [{glo}, {ghi}] mm, expected [{wlo}, {whi}]"
        );
    }
    // And a mirror is never a placement. A placement PAIR cannot state
    // one — ISO 10303-42 builds both frames right-handed whatever their
    // fields say — so the file has to reach for the operator form to
    // say it, and that is what refuses, by name.
    let operator = both("5.").replace(
        "#194 = ITEM_DEFINED_TRANSFORMATION('','',#11,#15);",
        "#194 = CARTESIAN_TRANSFORMATION_OPERATOR_3D('','',#13,#14,#12,-1.,$);",
    );
    match import_step(&operator, &ImportOptions::default(), Tol::witness())
        .expect_err("a mirroring/scaling operator must refuse")
    {
        E::Structure { id, what } => {
            assert_eq!(id, 194, "the refusal names the transform entity");
            assert!(what.contains("mirror"), "and says why: {what}");
        }
        other => panic!("expected Structure naming the transform, got: {other}"),
    }

    // (e) **FLIPPED (M7-4 Leg B).** A conversion-based length unit —
    // an inch file's normal form, with no `SI_UNIT` record on the
    // length at all — used to refuse. It now resolves through the
    // conversion expression THE FILE states: the box's millimetre unit
    // re-declared as an inch over that same millimetre comes out 25.4×
    // larger, and the factor comes from the record, not from a table
    // of what an inch is.
    let inch = mutated(
        "box",
        "#166 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );",
        "#166 = ( CONVERSION_BASED_UNIT('INCH',#9990) LENGTH_UNIT() NAMED_UNIT(#9991) );\n\
         #9990 = LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(25.4),#9992);\n\
         #9991 = DIMENSIONAL_EXPONENTS(1.,0.,0.,0.,0.,0.,0.);\n\
         #9992 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );",
    );
    let scaled = import_step(&inch, &ImportOptions::default(), Tol::witness())
        .expect("a conversion-based unit resolves from the file's own factor");
    let StepImport::Solid { body: big, .. } = &scaled else {
        panic!("a solid");
    };
    let ratio = volume_mm3(big) / volume_mm3(a);
    assert!(
        (ratio - 25.4_f64.powi(3)).abs() <= 1e-6 * 25.4_f64.powi(3),
        "the file's own 25.4 scaled every length: volume ratio {ratio}"
    );
    // What still refuses: a conversion whose factor is not a length.
    let bogus = mutated(
        "box",
        "#166 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );",
        "#166 = ( CONVERSION_BASED_UNIT('INCH',#9990) LENGTH_UNIT() NAMED_UNIT(#9991) );\n\
         #9990 = PLANE_ANGLE_MEASURE_WITH_UNIT(PLANE_ANGLE_MEASURE(25.4),#9992);\n\
         #9991 = DIMENSIONAL_EXPONENTS(1.,0.,0.,0.,0.,0.,0.);\n\
         #9992 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );",
    );
    match import_step(&bogus, &ImportOptions::default(), Tol::witness())
        .expect_err("a length declared over an angle is not a length")
    {
        E::UnsupportedUnit { id, found } => {
            assert_eq!(id, 166, "the refusal names the unit");
            assert!(found.contains("plane angle"), "and both sides: {found}");
        }
        other => panic!("expected UnsupportedUnit, got: {other}"),
    }

    // (f) And the orphan rule, NARROWED but not retired: a solid no
    // representation references is still refused rather than guessed
    // into the model. (The deliberate narrowing is that a plain
    // SHAPE_REPRESENTATION now counts as a reference — `compound_two`
    // and `twobody_importexport` import because of it.)
    let probe = mutated(
        "compound_two",
        "#10 = SHAPE_REPRESENTATION('',(#11,#15,#165),#177);",
        "#10 = SHAPE_REPRESENTATION('',(#11,#15),#177);",
    );
    match import_step(&probe, &ImportOptions::default(), Tol::witness())
        .expect_err("an orphan solid must refuse")
    {
        E::Structure { id, .. } => assert_eq!(id, 165, "the refusal names the orphan solid"),
        other => panic!("expected Structure naming the orphan, got: {other}"),
    }
}

/// **The A7 assembly record** (`docs/ASSEMBLY-DESIGN.md`): flattening
/// is the correct evaluation product, but flattening is not
/// forgetting. Every shipped solid carries a [`PlacedInstance`] saying
/// which component representation it came from, which
/// `MANIFOLD_SOLID_BREP`, which occurrence and transform stated it,
/// and the rigid map applied — so the association a body graph would
/// rebuild (component → instances → solid indices) survives the
/// flatten without re-parsing the file.
///
/// The fixture is the **dm1 class**, planted on real records: a third
/// `NEXT_ASSEMBLY_USAGE_OCCURRENCE` of `twobody_importexport`'s FIRST
/// component, so one representation is instanced twice and the file
/// ships three solids from two breps. That is the shape dm1 states
/// seven times over three, and the only shape where the record says
/// something a solid count cannot.
#[test]
fn the_assembly_record_retains_the_occurrence_structure() {
    let text = freecad_fixture("twobody_importexport");

    // Component 1 at +5 mm, component 2 at +12 mm (the same planted
    // frames row (d) checks), and a THIRD occurrence of component 1 at
    // +30 mm — new placement, new transform, new relationship, new
    // occurrence, all stated the way the file states its own two.
    let probe = text
        .replace(
            "#15 = AXIS2_PLACEMENT_3D('',#16,#17,#18);",
            "#15 = AXIS2_PLACEMENT_3D('',#9990,#17,#18);\n\
             #9990 = CARTESIAN_POINT('',(5.,0.,0.));",
        )
        .replace(
            "#19 = AXIS2_PLACEMENT_3D('',#20,#21,#22);",
            "#19 = AXIS2_PLACEMENT_3D('',#9991,#21,#22);\n\
             #9991 = CARTESIAN_POINT('',(12.,0.,0.));",
        )
        .replace(
            "#10 = SHAPE_REPRESENTATION('',(#11,#15,#19),#23);",
            "#10 = SHAPE_REPRESENTATION('',(#11,#15,#19,#9993),#23);\n\
             #9992 = CARTESIAN_POINT('',(30.,0.,0.));\n\
             #9993 = AXIS2_PLACEMENT_3D('',#9992,#17,#18);\n\
             #9994 = ITEM_DEFINED_TRANSFORMATION('','',#11,#9993);\n\
             #9995 = ( REPRESENTATION_RELATIONSHIP('','',#36,#10)\n\
             REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(#9994)\n\
             SHAPE_REPRESENTATION_RELATIONSHIP() );\n\
             #9996 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('3','BoxA_again','',#5,#31,$);\n\
             #9997 = PRODUCT_DEFINITION_SHAPE('Placement','third',#9996);\n\
             #9998 = CONTEXT_DEPENDENT_SHAPE_REPRESENTATION(#9995,#9997);",
        );

    let imported = import_step(&probe, &ImportOptions::default(), Tol::witness())
        .expect("three occurrences, two components");
    let StepImport::Solid { ref body, .. } = imported else {
        panic!("a solid");
    };
    let record = imported.instances();

    // One record per shipped solid, indexing them in order.
    assert_eq!(body.solids().count(), 3, "three occurrences, three solids");
    assert_eq!(record.len(), 3, "one record per materialized instance");
    for (i, r) in record.iter().enumerate() {
        assert_eq!(r.index, i, "the record indexes the shipped solids in order");
    }

    // component → instances: ONE representation instanced twice, and
    // both of its instances name the SAME `MANIFOLD_SOLID_BREP`. That
    // repetition IS the instancing — a reader that had flattened
    // without the record could not tell it from two distinct parts.
    assert_eq!(record[0].component, record[2].component, "same component");
    assert_ne!(record[0].component, record[1].component, "the other one");
    assert_eq!(record[0].solid, record[2].solid, "one brep, two copies");
    assert_ne!(record[0].solid, record[1].solid);

    // Every instance names its own occurrence, relationship and
    // transform — three distinct sites for three distinct copies.
    let sites: Vec<_> = record
        .iter()
        .map(|r| {
            (
                r.occurrence.expect("the file links a NAUO"),
                r.relationship.expect("stated by a relationship"),
                r.transform.expect("read from a transform"),
            )
        })
        .collect();
    assert_eq!(sites[0], (196, 193, 194), "the file's own first occurrence");
    assert_eq!(sites[1], (227, 224, 225), "and its second");
    assert_eq!(sites[2], (9996, 9995, 9994), "and the planted third");

    // placement → geometry: the map the record says was applied is the
    // map the shipped solid at that index actually sits under. Checked
    // against the UNPLACED import's own per-solid extents, so the
    // record is metered against the geometry rather than against
    // itself.
    let base =
        import_step(&text, &ImportOptions::default(), Tol::witness()).expect("the unplaced import");
    let StepImport::Solid { body: base, .. } = &base else {
        panic!("a solid");
    };
    let unplaced = solid_x_mm(base);
    let placed = solid_x_mm(body);
    // Which unplaced solid each record's `solid` id refers to: the
    // record's own component order is the resolution order.
    let source_of = [0_usize, 1, 0];
    for (r, &src) in record.iter().zip(source_of.iter()) {
        let map = r.placement.expect("a placed instance carries its map");
        let dx = map.translation.x * 1e3;
        let (lo, hi) = unplaced[src];
        let (glo, ghi) = placed[r.index];
        assert!(
            (glo - (lo + dx)).abs() <= 1e-9 && (ghi - (hi + dx)).abs() <= 1e-9,
            "solid {}: x extent [{glo}, {ghi}] mm, record says {dx} mm from [{lo}, {hi}]",
            r.index
        );
    }
    assert_eq!(
        record[2].placement.expect("placed").translation.x * 1e3,
        30.0
    );
}

/// The same record on a file that states **no assembly at all**: a
/// consumer never has to ask whether it exists. `compound_two` carries
/// two solids under one plain `SHAPE_REPRESENTATION` and not one
/// `NEXT_ASSEMBLY_USAGE_OCCURRENCE`.
#[test]
fn the_assembly_record_covers_a_file_that_places_nothing() {
    let imported = import_step(
        &freecad_fixture("compound_two"),
        &ImportOptions::default(),
        Tol::witness(),
    )
    .expect("imports");
    let StepImport::Solid { ref body, .. } = imported else {
        panic!("a solid");
    };
    let record = imported.instances();
    assert_eq!(record.len(), body.solids().count(), "one per shipped solid");
    assert_eq!(record.len(), 2, "compound_two ships two");
    for (i, r) in record.iter().enumerate() {
        assert_eq!(r.index, i);
        assert!(r.placement.is_none(), "nothing was placed");
        assert!(r.occurrence.is_none() && r.relationship.is_none());
        assert!(r.transform.is_none());
        assert_ne!(r.solid, 0, "and it still names the brep it came from");
    }
    assert_eq!(
        record[0].component, record[1].component,
        "one representation names both"
    );
    assert_ne!(record[0].solid, record[1].solid, "two distinct breps");
}

// ---- Row 6: the ε_in rows ------------------------------------------

/// **Row 6 — ε_in is the file's, scaled, and overridable.**
///
/// Every FreeCAD file declares `UNCERTAINTY_MEASURE_WITH_UNIT(
/// LENGTH_MEASURE(1.E-07), …)` against a `.MILLI. .METRE.` unit: 1e-7
/// mm, which is 1e-10 m. The declared uncertainty is a LENGTH, so it
/// scales with every other length — reading it raw would leave ε_in
/// three orders too loose.
///
/// The per-call override still wins over the file, unchanged from M7-1.
#[test]
fn eps_in_is_the_scaled_declaration_and_the_override_wins() {
    if !corpus_scale_gate("eps_in_is_the_scaled_declaration_and_the_override_wins") {
        return;
    }
    for name in FREECAD_FIXTURES {
        let text = freecad_fixture(name);
        assert!(
            text.contains("LENGTH_MEASURE(1.E-07)"),
            "{name}: the declared uncertainty is 1e-7 mm"
        );
        assert!(
            text.contains("SI_UNIT(.MILLI.,.METRE.)"),
            "{name}: in millimetres"
        );
        let (_, eps_in, _) = freecad_body(name);
        assert_eq!(eps_in, 1e-10, "{name}: 1e-7 mm scaled into kernel metres");

        let overridden = import_step(
            &text,
            &ImportOptions {
                eps_in: Some(2.5e-8),
                ..ImportOptions::default()
            },
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(
            overridden.eps_in(),
            2.5e-8,
            "{name}: the per-call override wins over the file"
        );
    }
}

/// **Row 6 (the truncation row).** The π-derived class — the cone
/// semi-angles Open CASCADE prints truncated to 12 significant digits
/// — adopts cleanly under the DEFAULT ε_in, with no per-literal
/// widening anywhere. This is the claim the flat budget has to earn on
/// the real corpus: `0.785398163397` misses π/4 by 4.5e-13 relative and
/// `0.463647609001` misses atan(1/2) by 4.7e-13, and both cones import,
/// certify, and pass the validity ladder against a budget of 1e-10 m
/// that knows nothing about their printed text.
#[test]
fn pi_derived_truncation_adopts_under_the_flat_budget() {
    if !corpus_scale_gate("pi_derived_truncation") {
        return;
    }
    for (name, printed, exact) in [
        ("cone_apex", "0.785398163397", std::f64::consts::FRAC_PI_4),
        ("cone_trunc", "0.463647609001", 0.5_f64.atan()),
    ] {
        let text = freecad_fixture(name);
        assert!(text.contains(printed), "{name}: the printed semi-angle");
        let miss = (printed.parse::<f64>().unwrap() - exact).abs();
        assert!(
            miss > 1e-13,
            "{name}: the truncation must be real, not hypothetical ({miss})"
        );
        let (body, eps_in, _) = freecad_body(name);
        assert_eq!(eps_in, 1e-10);
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "{name}: tier 3"
        );
        println!(
            "{name}: semi-angle {printed} misses its identity by {miss:e}, adopted at ε_in {eps_in:e}"
        );
    }
}

// ---- Row 7: the optional FreeCAD oracle (loud skip) -----------------

/// **Row 7 — the independent reader.** Every fixture makes the full
/// foreign round trip: FreeCAD authored it, this kernel adopted it, the
/// writer re-emitted it, and **FreeCAD reads it back** and reports a
/// valid solid of the expected volume. The kernel is not its own judge
/// here; a second CAD system is.
///
/// The oracle is located from the environment (`$FREECADCMD`, else the
/// documented local install) and the row **skips loudly** when it is
/// absent, so `cargo test` stays hermetic on a machine without FreeCAD.
/// Set `REQUIRE_FREECAD=1` to make absence a failure.
///
/// Counts are deliberately not compared: OCC re-adds the pole and seam
/// edges it dropped on export, and the kernel's own tessellation of the
/// re-minted shapes is its own — what a foreign round trip must
/// preserve is the SOLID and its volume, which is what is asserted.
/// (The export-side oracle, `scripts/check_step.sh`, checks a different
/// round trip and is untouched.)
#[test]
fn freecad_oracle_reads_back_every_reexported_fixture() {
    if !corpus_scale_gate("freecad_oracle") {
        return;
    }
    let freecadcmd = std::env::var("FREECADCMD").unwrap_or_else(|_| {
        format!(
            "{}/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd",
            std::env::var("HOME").unwrap_or_default()
        )
    });
    if !std::path::Path::new(&freecadcmd).is_file() {
        println!(
            "freecad_oracle: SKIP — no freecadcmd at '{freecadcmd}' \
             (set FREECADCMD to a FreeCAD headless binary to run this row)"
        );
        assert!(
            std::env::var("REQUIRE_FREECAD").unwrap_or_default() != "1",
            "REQUIRE_FREECAD=1 and freecadcmd is absent at '{freecadcmd}'"
        );
        return;
    }
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("oracle")
        .join("reexport_check.py");
    let dir = std::env::temp_dir().join(format!("step-import-oracle-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch directory for the re-exports");

    let mut ran = 0;
    for name in FREECAD_FIXTURES {
        let (body, _, _) = freecad_body(name);
        let text = step_export::step_string(
            &body,
            &step_export::StepOptions {
                product_name: name.to_owned(),
                ..step_export::StepOptions::default()
            },
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{name}: re-export: {e}"));
        let path = dir.join(format!("{name}.step"));
        std::fs::write(&path, &text).expect("writing the re-export");
        let e = expect(name);
        let out = std::process::Command::new(&freecadcmd)
            .arg(&script)
            .env("STEP_FILE", &path)
            .env("EXPECT_SOLIDS", e.census.0.to_string())
            .env("EXPECT_VOLUME_MM3", format!("{}", e.volume_mm3))
            // The oracle's own import tolerance plus the dialect budget
            // row 1 derives; 1e-9 relative is the export-side oracle's
            // standing choice for this comparison.
            .env("EXPECT_VOLUME_RTOL", "1e-9")
            .output()
            .unwrap_or_else(|err| panic!("running the FreeCAD oracle: {err}"));
        print!("{}", String::from_utf8_lossy(&out.stdout));
        assert!(
            out.status.success(),
            "{name}: the FreeCAD oracle refused the re-export\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        ran += 1;
    }
    assert_eq!(
        ran,
        FREECAD_FIXTURES.len(),
        "every fixture went to the oracle"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// **The corpus DIALECT facts two modules rest on, made mechanical**
/// (issue #667).
///
/// Three of this importer's design decisions are justified by a claim
/// about what the committed FreeCAD files literally say, and all three
/// were measured once, by hand, with nothing re-measuring them:
///
/// * `chart`'s module header — *"FreeCAD 1.1.2 never writes
///   `FACE_OUTER_BOUND` at all (0 occurrences in the 13 measured
///   files)"*. The entire geometric outerness-inference lane, with its
///   three typed refusals, exists **because** of it;
/// * `units`' header — *"FreeCAD 1.1.2 writes `SI_UNIT(.MILLI.,
///   .METRE.)` on every file it emits, and its declared uncertainty
///   (`1.E-07`) is in those millimeters too"* — the reason the prefix
///   table is data rather than a millimetre special case; and
/// * the same header's *"a `.MILLI. .RADIAN.` context is … absent from
///   every file measured"*, which is why the angle path refuses a
///   prefixed SI angle instead of folding in a second scale.
///
/// A fourteenth fixture, or a regenerated one from a writer whose
/// dialect moved, would leave all three sentences quietly false while
/// every acceptance row above stayed green — the inference and the
/// prefix table are still *correct*, they would just no longer be
/// answering the situation their prose describes. So the corpus is the
/// guard, and this row reads it: cheap (a text scan of committed bytes,
/// no import), and it pins the literal **13** as well as the contents,
/// because "the 13 measured files" is half of each claim. The count is
/// asserted against [`FREECAD_FIXTURES`]`.len()` directly, not only as a
/// correspondence with the fixtures directory: committing a fourteenth
/// file AND listing it would keep the two sides equal while falsifying
/// every sentence above, which is exactly the scenario this row exists
/// for.
///
/// **Every dialect check below is shaped as an ABSENCE test, not an
/// existence one**, because the corpus already contains the case that
/// separates them: `twobody_importexport.step` declares **three**
/// separate unit contexts. `text.contains("SI_UNIT($,.RADIAN.)")` would
/// pass on a file that carried a prefixed radian context *beside* an
/// unprefixed one — which is the failure the `units` header claims
/// cannot happen. So the rows enumerate every `SI_UNIT` and every
/// `LENGTH_MEASURE` occurrence and require **all** of them to agree,
/// and separately require at least one, so a file that dropped its
/// unit context entirely cannot pass vacuously.
///
/// Whitespace is stripped before scanning: ISO-10303-21 permits a line
/// fold anywhere outside a string literal, so a regenerated fixture
/// could split `FACE_OUTER_BOUND` across two lines and hide the marker
/// from a raw `contains`. Stripping also normalizes `SI_UNIT( $ , …)`
/// spacing. It flattens whitespace inside string literals too, which is
/// harmless here — nothing below scans a literal's text.
///
/// Deliberately NOT a claim about FreeCAD in general — it pins what the
/// committed corpus says, which is all either module ever had. If a
/// later FreeCAD moves, this goes red and the headers get rewritten with
/// the new corpus in hand.
///
/// **Sibling, same job, same crate:** `wild.rs`'s
/// `the_committed_corpus_still_carries_the_dialects_it_was_chosen_for`
/// pins the wild corpus's dialect legs the same way. It asks
/// `any(corpus contains X)` — right for its claim, which is *"each gap
/// is present in something committed"* — where this row asks
/// all-or-none per file, because `chart`'s and `units`' claims are
/// universally quantified over the corpus. Two shapes, one class; if a
/// third corpus claim appears, it belongs beside one of these.
#[test]
fn the_committed_freecad_corpus_still_says_what_chart_and_units_quote() {
    assert_eq!(
        FREECAD_FIXTURES.len(),
        13,
        "`chart` and `units` both quote \"the 13 measured files\" — if the corpus grew or \
         shrank, re-measure both headers against the new set and move this literal with them"
    );

    let dir: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", "freecad"]
        .iter()
        .collect();
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".step"))
        .collect();
    on_disk.sort();
    let mut named: Vec<String> = FREECAD_FIXTURES
        .iter()
        .map(|n| format!("{n}.step"))
        .collect();
    named.sort();
    assert_eq!(
        on_disk, named,
        "the committed corpus and `FREECAD_FIXTURES` disagree — both quoted claims are \
         scoped to \"the 13 measured files\", so a file in one and not the other leaves \
         them measured over the wrong set"
    );

    /// Every `SI_UNIT(<prefix>,<name>)` in whitespace-stripped STEP
    /// text, as `(prefix, name)`. `$` is the no-prefix slot.
    fn si_units(text: &str) -> Vec<(&str, &str)> {
        text.match_indices("SI_UNIT(")
            .filter_map(|(i, m)| {
                let rest = &text[i + m.len()..];
                let close = rest.find(')')?;
                rest[..close].split_once(',')
            })
            .collect()
    }

    /// Every `LENGTH_MEASURE(<value>)` argument, same normalization.
    fn length_measures(text: &str) -> Vec<&str> {
        text.match_indices("LENGTH_MEASURE(")
            .filter_map(|(i, m)| {
                let rest = &text[i + m.len()..];
                let close = rest.find(')')?;
                Some(&rest[..close])
            })
            .collect()
    }

    let mut wrong: Vec<String> = Vec::new();
    for name in FREECAD_FIXTURES {
        // Fold-proofing, per the doc above: whitespace is not
        // significant in an ISO-10303-21 exchange structure outside
        // string literals, so a fold can land mid-keyword.
        let text: String = freecad_fixture(name)
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        if text.contains("FACE_OUTER_BOUND") {
            wrong.push(format!(
                "{name}: states FACE_OUTER_BOUND (`chart`'s premise)"
            ));
        }

        let units = si_units(&text);
        let metres: Vec<&str> = units
            .iter()
            .filter(|(_, n)| *n == ".METRE.")
            .map(|(p, _)| *p)
            .collect();
        if metres.is_empty() {
            wrong.push(format!(
                "{name}: no SI length unit at all (`units`' premise)"
            ));
        }
        for prefix in &metres {
            if *prefix != ".MILLI." {
                wrong.push(format!(
                    "{name}: an SI length context is `SI_UNIT({prefix},.METRE.)`, not \
                     `.MILLI.` (`units`' premise)"
                ));
            }
        }

        // The prefixed SI ANGLE `units` refuses on sight, and says it
        // has never had to. ALL radian contexts must use `$`, not just
        // one of them: `twobody_importexport` carries three.
        for (prefix, _) in units.iter().filter(|(_, n)| *n == ".RADIAN.") {
            if *prefix != "$" {
                wrong.push(format!(
                    "{name}: a PREFIXED SI angle unit `SI_UNIT({prefix},.RADIAN.)` appeared \
                     (`units`' premise)"
                ));
            }
        }

        let uncertainties = length_measures(&text);
        if uncertainties.is_empty() {
            wrong.push(format!(
                "{name}: no declared uncertainty at all (`units`' premise)"
            ));
        }
        for value in &uncertainties {
            if *value != "1.E-07" {
                wrong.push(format!(
                    "{name}: a declared uncertainty is {value}, not 1.E-07 (`units`' premise)"
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a corpus dialect fact that `chart` and `units` quote has moved: {wrong:#?} — \
         re-measure and rewrite those headers; do not delete this row"
    );
}

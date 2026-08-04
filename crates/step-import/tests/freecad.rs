//! **M7-2 acceptance: the FreeCAD-authored foreign corpus** — the
//! first geometry this kernel adopts that it did not write.
//!
//! Rows 1, 2, 4, 5, 6 and 7 of `docs/M7-2-SPEC.md` §2 live here (row 3,
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

/// Imports a FreeCAD fixture, panicking on refusal.
fn import_freecad(name: &str) -> StepImport {
    let text = freecad_fixture(name);
    import_step(&text, &ImportOptions::default())
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
    for name in FREECAD_FIXTURES {
        let (body, _eps, normalizations) = freecad_body(name);
        let e = expect(name);
        assert_eq!(census(&body), e.census, "{name}: census");
        assert_eq!(
            normalizations.len(),
            e.normalizations,
            "{name}: reported structure normalizations"
        );

        let props = topo::mass_properties(&body).unwrap_or_else(|err| panic!("{name}: {err}"));
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
        assert_eq!(topo::validate_geometric(&body), Ok(()), "{name}: tier 3");
    }
}

/// **Row 1's census-mapping pin.** The structure normalizations are
/// carried out as DATA, and this is the mapping table a reader needs:
/// what the file states for the re-minted region, against what the
/// kernel minted in its place. Nothing here is silent.
#[test]
fn structure_normalizations_are_reported_with_their_census_mapping() {
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
    for name in FREECAD_FIXTURES {
        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let (body1, _, _) = freecad_body(name);
        let export1 = step_export::step_string(&body1, &options)
            .unwrap_or_else(|e| panic!("{name}: re-export 1: {e}"));
        let reimport = import_step(&export1, &ImportOptions::default())
            .unwrap_or_else(|e| panic!("{name}: re-import of our own dialect: {e}"));
        let StepImport::Solid { body: body2, .. } = reimport else {
            panic!("{name}: re-import lost the solid");
        };
        assert_eq!(
            census(&body1),
            census(&body2),
            "{name}: census identical across the adoption pass"
        );
        let export2 = step_export::step_string(&body2, &options)
            .unwrap_or_else(|e| panic!("{name}: re-export 2: {e}"));
        assert_eq!(
            export1, export2,
            "{name}: the second export must be byte-identical to the first"
        );
        let v1 = topo::mass_properties(&body1).unwrap().volume;
        let v2 = topo::mass_properties(&body2).unwrap().volume;
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
    let v = topo::mass_properties(&body).unwrap().volume;
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
            geom_surfaces::Surface::Cone { apex, .. } => Some(apex),
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
    let err = import_step(&flattened, &ImportOptions::default())
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
    let err = import_step(&probe, &ImportOptions::default())
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
    let err = import_step(&probe, &ImportOptions::default())
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

    // (a) EDGE_CURVE same_sense .F. — a carrier running against its own
    // edge. Never emitted by FreeCAD (0 of 100+ measured); planted.
    let probe = mutated(
        "box",
        "#21 = EDGE_CURVE('',#22,#24,#26,.T.);",
        "#21 = EDGE_CURVE('',#22,#24,#26,.F.);",
    );
    match import_step(&probe, &ImportOptions::default()).expect_err("a .F. edge is out of subset") {
        E::Topology { id, .. } => assert_eq!(id, 21, "the refusal names the EDGE_CURVE"),
        other => panic!("expected Topology, got: {other}"),
    }

    // (b) A non-unit VECTOR magnitude — the line parameter would no
    // longer be arc length. Always exactly `1.` in the corpus.
    let probe = mutated(
        "box",
        "#28 = VECTOR('',#29,1.);",
        "#28 = VECTOR('',#29,2.);",
    );
    match import_step(&probe, &ImportOptions::default())
        .expect_err("a non-unit vector is out of subset")
    {
        E::MalformedRecord { id, .. } => assert_eq!(id, 28, "the refusal names the VECTOR"),
        other => panic!("expected MalformedRecord, got: {other}"),
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

    // (d) A non-identity assembly transform. `twobody_importexport`'s
    // two components are placed by identity ITEM_DEFINED_TRANSFORMATIONs
    // (FreeCAD bakes placement into the exported shape); moving one
    // makes the stated geometry disagree with where the assembly puts
    // it, and importing it unplaced would silently misposition a body.
    let text = freecad_fixture("twobody_importexport");
    assert!(text.contains("#194 = ITEM_DEFINED_TRANSFORMATION('','',#11,#15);"));
    let probe = text.replace(
        "#15 = AXIS2_PLACEMENT_3D('',#16,#17,#18);",
        "#15 = AXIS2_PLACEMENT_3D('',#9995,#17,#18);\n\
         #9995 = CARTESIAN_POINT('',(5.,0.,0.));",
    );
    match import_step(&probe, &ImportOptions::default())
        .expect_err("a non-identity assembly transform must refuse")
    {
        E::Structure { id, what } => {
            assert_eq!(id, 194, "the refusal names the transform entity");
            assert!(what.contains("assembly"), "and says what it is: {what}");
        }
        other => panic!("expected Structure naming the transform, got: {other}"),
    }

    // (e) A conversion-based length unit (an inch file's normal form —
    // no SI_UNIT record at all). FreeCAD emits none for an mm-configured
    // document, so this too is a planted boundary probe.
    let probe = mutated(
        "box",
        "#166 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );",
        "#166 = ( CONVERSION_BASED_UNIT('INCH',#9990) LENGTH_UNIT() NAMED_UNIT(#9991) );",
    );
    match import_step(&probe, &ImportOptions::default())
        .expect_err("a conversion-based unit must refuse, not scale-lie")
    {
        E::UnsupportedUnit { id, .. } => assert_eq!(id, 166, "the refusal names the unit"),
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
    match import_step(&probe, &ImportOptions::default()).expect_err("an orphan solid must refuse") {
        E::Structure { id, .. } => assert_eq!(id, 165, "the refusal names the orphan solid"),
        other => panic!("expected Structure naming the orphan, got: {other}"),
    }
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
            },
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
        assert_eq!(topo::validate_geometric(&body), Ok(()), "{name}: tier 3");
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

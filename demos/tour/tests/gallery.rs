//! `demo_tour::gallery`'s tests.

#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use pncad::document::{BooleanValue, Evaluation, Node, RecipeNodeId, ValuePayload, VarId};
    use pncad::document::{
        CancelToken, CheckId, ChecksConfig, EvalOptions, ProfileDoc, evaluate, run_checks,
    };
    use pncad::geom_core::Tol;
    use pncad::topo::{Body, mass_properties};

    /// The placements whose copies stand in the world: the product's
    /// (a placement against another body joins that body's space).
    fn world_placements(doc: &ProfileDoc, _evaluation: &Evaluation<f64>) -> Vec<RecipeNodeId> {
        doc.placements()
            .into_iter()
            .filter(|&p| doc.node_space(p) == Some(VarId::WORLD))
            .collect()
    }

    /// What one gallery scene is expected to PRODUCE.
    struct Shape {
        name: &'static str,
        doc: ProfileDoc,
        /// World placements. One, unless the scene has a reason.
        placements: usize,
        /// The at-rest findings the advisory registry reports, loud and
        /// quiet, and why.
        at_rest: usize,
        /// The product's shape, where a row's `why` makes a claim
        /// about it: pinned so the claim goes red when the document
        /// changes under it.
        product: Option<Product>,
        why: &'static str,
    }

    /// The gathered product's solid and face counts.
    struct Product {
        solids: usize,
        faces: usize,
    }

    /// **A gallery document must denote what its scene means, or say
    /// why not.**
    ///
    /// The gallery's whole purpose is to be opened in the viewer, and
    /// the viewer draws the PRODUCT: the world, every copy a placement
    /// defines. A scene that authors a body for narration leaves it
    /// unplaced; a placement nobody meant as product is a second body
    /// in the picture.
    ///
    /// That is the bug this row exists for. `diefillet` once shipped
    /// the blank beside the composed die, the blank being the die's
    /// own outer shape with no pips cut, so the file drew one
    /// die-shaped thing with its pips filled in, its faces z-fighting,
    /// and twice the material (115 faces, V = 1.918146). It looked
    /// almost right, which is why it survived: every local battery
    /// passes, because each body is individually perfect.
    ///
    /// The row is a TABLE rather than a blanket "no findings", because
    /// one scene legitimately reports and hiding that would be the
    /// same silence in a different place. Changing any number here is
    /// a claim about what a scene produces, so it should cost a
    /// sentence in `why`.
    #[test]
    fn each_gallery_document_denotes_its_scene_or_says_why_not() {
        let tol = Tol::witness();
        let shapes = [
            Shape {
                name: "bracket",
                doc: demo_tour::bracket::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                product: None,
                why: "one placement: the extrude, split, its corner piece kept and that piece's \
                      chords chamfered; the offcuts live in the scene's wall probe, not in the \
                      document",
            },
            Shape {
                name: "checks",
                doc: demo_tour::checks::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                product: None,
                why: "one placement; its connectedness finding is the scene's own subject \
                      and is not an at-rest one",
            },
            Shape {
                name: "ring",
                doc: demo_tour::ring::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                product: None,
                why: "one revolve, one placement",
            },
            Shape {
                name: "diefillet",
                doc: demo_tour::diefillet::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                product: None,
                why: "the composed die alone — the blank is a narration body and \
                      nothing places it, which is what this row guards",
            },
            Shape {
                name: "heatsink",
                doc: demo_tour::heatsink::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                product: None,
                why: "one placement, and nothing in the document interpenetrates: the base \
                      is rounded by a Fillet after a Union folds the fin family into the \
                      base, so the whole part is in the recipe",
            },
            Shape {
                name: "teapot",
                doc: demo_tour::teapot::gallery_document(tol),
                placements: 4,
                at_rest: 42,
                product: None,
                why: concat!(
                    "FOUR placements, because the teapot is four solids and the operand ",
                    "gate has no arm for either join. FORTY-TWO at-rest findings, all ",
                    "loud and none refusing: ONE interference, the handle's torus ",
                    "standing in the vessel's sphere, unlocalised because the ",
                    "intersection cannot take it; FORTY-ONE could-not-look, the face ",
                    "pairs where a curved face of one part is within reach of the ",
                    "other (the spout's walls in the vessel among them) and the ",
                    "contacts the census met but no door recorded. The lifted lid's ",
                    "pairs are the ones the box rule PROVED apart: its dome is boxed ",
                    "by its latitude zone, not the whole ball. No mate is authored, ",
                    "so nothing declares the gap the render shows",
                ),
            },
            Shape {
                name: "impeller",
                doc: demo_tour::impeller::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                // The 24-gon hub's 26 faces, and each blade's three
                // outer walls plus its top and bottom.
                product: Some(Product {
                    solids: 1,
                    faces: 26 + 6 * 5,
                }),
                why: "the union of the hub and the blade group, at the scene's first count; \
                      the union fuses the blade family, so its six blades are one body",
            },
            Shape {
                name: "plate",
                doc: demo_tour::plate::gallery_document(tol),
                placements: 1,
                at_rest: 0,
                product: Some(Product {
                    solids: 1,
                    faces: 6,
                }),
                why: concat!(
                    "ONE placement, the blank's extrude, so the product is a six-face ",
                    "slab with no holes. The two hole extrudes exist only as the web ",
                    "measure's references and nothing places them, so the document ",
                    "denotes the study's numbers and not the two-hole plate it is ",
                    "about. The cut spelling places the cut part and that is its ",
                    "product (the_cut_plate_is_its_product); its wall is the certified ",
                    "drive, which certifies no box of it (work/tally/a-hole-wholly-",
                    "inside-its-target-ties-the-subtract-volume-bound.md)",
                ),
            },
            Shape {
                name: "chain",
                doc: demo_tour::chain::gallery_document(tol),
                placements: 10,
                at_rest: 10,
                // Each bar a six-face box; each pin a cylinder wall in
                // two halves and its two caps.
                product: Some(Product {
                    solids: 10,
                    faces: 4 * 6 + 6 * 4,
                }),
                why: concat!(
                    "TEN placements — four bars, five pins, and the target pin — and TEN ",
                    "copies: one bar extrude, each link placed against its parent, is ",
                    "four copies, each its own output with its own names; the target ",
                    "stands in the world because the tip measure reads it beside the ",
                    "tip pin's copy. TEN at-rest findings, all interference and none ",
                    "could-not-look, because the pins are separate extrudes rather ",
                    "than bores: each pin shares a half-disc of material with every ",
                    "bar end it stands in — the base pin with bar 1, joint pin k = 2..4 ",
                    "with bars k-1 and k, the tip pin with bar 4 — and the target, standing ",
                    "where the tip pin does at nominal, with bar 4 and wholly with the ",
                    "tip pin (that one unlocalised, the two boundaries coinciding). ",
                    "The bars' end caps only touch at the joints and are no at-rest finding. ",
                    "No mate declares a joint",
                ),
            },
        ];

        for shape in &shapes {
            let evaluation = evaluate::<f64>(
                &shape.doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                tol,
            );
            assert_eq!(
                world_placements(&shape.doc, &evaluation).len(),
                shape.placements,
                "{}: world placements ({})",
                shape.name,
                shape.why
            );
            if let Some(want) = &shape.product {
                let product = pncad::document::product(&shape.doc, &evaluation, tol)
                    .unwrap_or_else(|error| {
                        panic!("{}: the product gathers: {error:?}", shape.name)
                    });
                assert_eq!(
                    (product.solids().count(), product.faces().count()),
                    (want.solids, want.faces),
                    "{}: the product's (solids, faces) ({})",
                    shape.name,
                    shape.why
                );
            }
            let report = run_checks(&shape.doc, &evaluation, &ChecksConfig::default(), tol)
                .unwrap_or_else(|error| {
                    panic!(
                        "{}: the registry refused: {}",
                        shape.name,
                        error.spoken(&shape.doc)
                    )
                });
            let at_rest = report
                .findings
                .iter()
                .filter(|finding| finding.check == CheckId::AtRest)
                .count();
            assert_eq!(
                at_rest,
                shape.at_rest,
                "{}: at-rest findings ({}) — {}",
                shape.name,
                shape.why,
                report.spoken(&shape.doc)
            );
            // What a scene reports at rest is an overlap no assertion
            // could quiet or a pair it could not look at, outside what
            // its document can yet say, so it refuses at no severity
            // (CHECKS DS6): the teapot's handle in its vessel and the
            // chain's pins in its bars included.
            let error = ChecksConfig {
                at_rest: pncad::document::Severity::Error,
                ..ChecksConfig::default()
            };
            let refused = pncad::document::enforce_checks(&report, &error)
                .err()
                .map(|refusal| refusal.findings.len());
            assert_eq!(refused, None, "{}: refused at Error", shape.name);
        }
    }

    /// **The plate with its holes cut is its product.** The web
    /// measure reads the cut part's bore walls; the document places
    /// the part, and a reader of a placed body does not keep it out of
    /// the world. The product is one body, the cut part's own value.
    #[test]
    fn the_cut_plate_is_its_product() {
        use demo_tour::plate::{RADIUS_SIGMA, SPACING_HALF_WIDTH, WEB_BOUND, cut_plate};
        let tol = Tol::witness();
        let cut = cut_plate(SPACING_HALF_WIDTH, RADIUS_SIGMA, WEB_BOUND, tol);
        let evaluation = evaluate::<f64>(
            &cut.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let [placement] = world_placements(&cut.doc, &evaluation)[..] else {
            panic!(
                "the cut plate places one body in the world, the cut part: {:?}",
                world_placements(&cut.doc, &evaluation)
            )
        };
        let Some(Node::Place { source, .. }) = cut.doc.node(placement) else {
            unreachable!("the placement is a Place")
        };
        let part_node = cut
            .doc
            .operation_of(source.read)
            .expect("the placed body is live");
        assert!(
            matches!(cut.doc.node(part_node), Some(Node::Subtract { .. })),
            "the placed body is the second cut"
        );
        let product = pncad::document::product(&cut.doc, &evaluation, tol)
            .unwrap_or_else(|error| panic!("the cut plate gathers: {error:?}"));
        let part = match &evaluation
            .value(part_node)
            .expect("the cut part evaluated")
            .payload
        {
            ValuePayload::Body(b) => b.body().clone(),
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body.body().clone(),
            other => panic!("the cut part is a body, got {other:?}"),
        };
        assert_eq!(product.solids().count(), 1, "the product is one body");
        assert_eq!(
            digest(&product, tol),
            digest(&part, tol),
            "the product is the cut part's own value"
        );
    }

    /// A key-free digest of a body's value: its census, its mass
    /// properties and its points, by bits.
    fn digest(body: &Body<f64>, tol: Tol) -> (usize, usize, usize, u64, u64, Vec<[u64; 3]>) {
        let mass = mass_properties(
            &demo_tour::booleans::at_rest(body, tol),
            tol,
            demo_tour::printed(),
        )
        .expect("the body has mass properties");
        let mut points: Vec<[u64; 3]> = body
            .points()
            .map(|(_, p)| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
            .collect();
        points.sort_unstable();
        (
            body.faces().count(),
            body.edges().count(),
            body.vertices().count(),
            mass.enclosure.volume.to_bits(),
            mass.enclosure.surface_area.to_bits(),
            points,
        )
    }

    /// The die, by the numbers its own scene already knows.
    ///
    /// `diefillet::stops` asserts the composed BODY is 26 + 21·3 faces
    /// and tier-3 valid. This asserts the same thing one level up, of
    /// the document's PRODUCT — which is the thing the viewer draws
    /// and the thing that was wrong. The volume is the composed die's,
    /// not twice it.
    #[test]
    fn the_die_document_produces_one_die() {
        let tol = Tol::witness();
        let doc = demo_tour::diefillet::gallery_document(tol);
        let evaluation = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let product =
            pncad::document::product(&doc, &evaluation, tol).expect("the die document gathers");
        assert_eq!(
            product.faces().count(),
            26 + 21 * 3,
            "the product is the composed die's own face count, not it plus a blank"
        );
        let volume = mass_properties(
            &demo_tour::booleans::at_rest(&product, tol),
            tol,
            demo_tour::printed(),
        )
        .expect("the product has mass properties")
        .enclosure
        .volume;
        // MEASURED, not a closed form: the scene closes the BLANK's
        // volume in `diefillet::blank_volume` (core + slabs + quarter
        // cylinders + octants), and the composed die — 21 spherical
        // caps out, 21 torus bands in — has no such form written
        // anywhere, so this is a pin on the value the gather answers.
        // What makes it worth pinning is the defect's signature: with
        // the blank also in the product it answered 1.918146, which
        // is this number DOUBLED, because the same material was
        // gathered twice.
        let want = 0.952_914_984_014_647_f64;
        assert!(
            (volume - want).abs() < 1.0e-9 * want,
            "the product's volume is the composed die's: {volume} vs {want}"
        );
    }
}

//! **R1 adversarial review probes for PR #325 (assembly instancing).**
//!
//! Constructed multi-instance STEP files (mutations of the committed
//! `twobody_importexport` fixture) attacking the claims the PR makes
//! by execution: N-copy materialization through `graft_disjoint`,
//! per-instance frames on the right solids, the identity-detection
//! boundary at ε_in, the per-solid tier gate naming the right
//! `MANIFOLD_SOLID_BREP`, entity-id-deterministic ordering (D9), and
//! the silently-dropped-transform hole a nested assembly would fall
//! into.
//!
//! **Authorship**: rows 1–7 are the R1 reviewer's, adopted verbatim
//! into the fix pass (the file is theirs; only the nested-assembly row
//! below changed, from "refuse or compose" to the positive row the fix
//! earns). The nesting rows after it are the fix pass's own.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{census, freecad_fixture};
use geom_core::Tol;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

/// The base fixture: BoxA (rep #36, MSB #37) and SphereB (rep #205,
/// MSB #206), each placed once at the identity (#194, #225).
fn base_text() -> String {
    freecad_fixture("twobody_importexport")
}

fn solid_body(r: step_import::StepImport) -> topo::Body<f64> {
    match r {
        StepImport::Solid { body, .. } => body,
        StepImport::Wireframe { .. } => panic!("expected a solid"),
    }
}

fn import(text: &str) -> topo::Body<f64> {
    solid_body(import_step(text, &ImportOptions::default(), Tol::witness()).expect("import"))
}

/// One extra placed instance of `rep`, appended with fresh entity ids
/// starting at `id0`: a bare relationship complex + transform + frame,
/// translating by (tx, ty, tz) mm (rotation about z by `deg` degrees
/// via the ref direction when non-zero).
fn extra_instance(rep: u64, id0: u64, tx: f64, ty: f64, tz: f64, deg: f64) -> String {
    extra_instance_into(rep, 10, id0, tx, ty, tz, deg)
}

/// [`extra_instance`] with an explicit `rep_2` — the edge's OTHER end,
/// which is what a nested assembly varies (a component is placed into
/// a sub-assembly, not into the root).
fn extra_instance_into(
    rep: u64,
    into: u64,
    id0: u64,
    tx: f64,
    ty: f64,
    tz: f64,
    deg: f64,
) -> String {
    // Only 0° and 90° are used, so one-decimal formatting is exact.
    let (rx, ry) = (deg.to_radians().cos(), deg.to_radians().sin());
    let (rx, ry) = (format!("{rx:.1}"), format!("{ry:.1}"));
    format!(
        "#{r} = ( REPRESENTATION_RELATIONSHIP('','',#{rep},#{into}) \
         REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION(#{t}) \
         SHAPE_REPRESENTATION_RELATIONSHIP() );\n\
         #{t} = ITEM_DEFINED_TRANSFORMATION('','',#11,#{p});\n\
         #{p} = AXIS2_PLACEMENT_3D('',#{pt},#{pz},#{px});\n\
         #{pt} = CARTESIAN_POINT('',({tx:?},{ty:?},{tz:?}));\n\
         #{pz} = DIRECTION('',(0.,0.,1.));\n\
         #{px} = DIRECTION('',({rx},{ry},0.));\n",
        r = id0,
        t = id0 + 1,
        p = id0 + 2,
        pt = id0 + 3,
        pz = id0 + 4,
        px = id0 + 5,
    )
}

fn append(text: &str, block: &str) -> String {
    // The DATA section's ENDSEC (the last one; HEADER has one too).
    let at = text.rfind("ENDSEC;").expect("an ENDSEC");
    format!("{}{block}{}", &text[..at], &text[at..])
}

/// Every vertex point of each solid, per solid in body order, in mm,
/// sorted — the point-cloud instrument (exact per-instance placement
/// evidence a whole-body extent cannot fake).
fn solid_clouds(body: &topo::Body<f64>) -> Vec<Vec<[f64; 3]>> {
    body.solids()
        .map(|(_, solid)| {
            let mut pts = Vec::new();
            for shell in &solid.shells {
                for &face in &body.get_shell(*shell).expect("shell").faces {
                    let f = body.get_face(face).expect("face");
                    for lk in std::iter::once(f.outer).chain(f.rings.iter().copied()) {
                        let topo::LoopBoundary::Cycle { first } =
                            body.get_loop(lk).expect("loop").boundary
                        else {
                            continue;
                        };
                        for he in body.loop_cycle(first).expect("cycle") {
                            let v = body.get_half_edge(he).expect("use").start;
                            let p = body
                                .get_point(body.get_vertex(v).expect("vertex").point)
                                .expect("point");
                            pts.push([p.x * 1e3, p.y * 1e3, p.z * 1e3]);
                        }
                    }
                }
            }
            pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
            pts.dedup_by(|a, b| a == b);
            pts
        })
        .collect()
}

fn assert_cloud_eq(got: &[[f64; 3]], want: &[[f64; 3]], label: &str) {
    assert_eq!(got.len(), want.len(), "{label}: point count");
    for (g, w) in got.iter().zip(want) {
        for k in 0..3 {
            assert!(
                (g[k] - w[k]).abs() <= 1e-9,
                "{label}: {g:?} vs {w:?} (axis {k})"
            );
        }
    }
}

fn mapped(cloud: &[[f64; 3]], f: impl Fn([f64; 3]) -> [f64; 3]) -> Vec<[f64; 3]> {
    let mut out: Vec<[f64; 3]> = cloud.iter().map(|&p| f(p)).collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out
}

/// (a) + (d): THREE placed instances of one component (plus the other
/// component identity-placed), with high fresh entity ids. Census is
/// exactly the per-solid sum, every copy's point cloud sits at its own
/// frame, volume is 3×box + sphere against closed-form anchors.
#[test]
fn three_instances_of_one_component() {
    let base = import(&base_text());
    let base_clouds = solid_clouds(&base);
    assert_eq!(base_clouds.len(), 2);

    let text = append(
        &append(
            &base_text(),
            &extra_instance(36, 4_000_000_000_001, 30.0, 0.0, 0.0, 0.0),
        ),
        &extra_instance(36, 4_000_000_000_101, 60.0, 0.0, 0.0, 0.0),
    );
    let body = import(&text);
    let (s, sh, f, e, v) = census(&body);
    let (bs, bsh, bf, be, bv) = census(&base);
    // Box census from the base body's own solid 0 (6 faces, 12 edges,
    // 8 vertices, 1 shell): two extra copies add exactly that.
    assert_eq!(
        (s, sh, f, e, v),
        (bs + 2, bsh + 2, bf + 12, be + 24, bv + 16),
        "census must be the disjoint sum of the copies"
    );
    let clouds = solid_clouds(&body);
    assert_eq!(clouds.len(), 4, "box, sphere, box+30, box+60");
    // Instance order is relationship entity id ascending: #193 (box),
    // #224 (sphere), then the two appended relationships.
    assert_cloud_eq(&clouds[0], &base_clouds[0], "identity box");
    assert_cloud_eq(&clouds[1], &base_clouds[1], "identity sphere");
    assert_cloud_eq(
        &clouds[2],
        &mapped(&base_clouds[0], |[x, y, z]| [x + 30.0, y, z]),
        "box at +30",
    );
    assert_cloud_eq(
        &clouds[3],
        &mapped(&base_clouds[0], |[x, y, z]| [x + 60.0, y, z]),
        "box at +60",
    );
    // Volume, against closed forms: the box is an axis-aligned box so
    // its volume is its AABB product; the sphere is r = 0.5 mm.
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for p in &base_clouds[0] {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let v_box = (hi[0] - lo[0]) * (hi[1] - lo[1]) * (hi[2] - lo[2]);
    let v_sphere = 4.0 / 3.0 * std::f64::consts::PI * 0.5f64.powi(3);
    let got = topo::mass_properties(&body, Tol::witness())
        .expect("mass properties")
        .volume
        * 1e9;
    let want = 3.0 * v_box + v_sphere;
    assert!(
        (got - want).abs() <= 1e-6 * want,
        "volume {got} mm^3, expected {want}"
    );
}

/// (b): two DIFFERENT components each instanced (2 boxes + 2 spheres,
/// one of each placed away from the identity original).
#[test]
fn two_components_each_instanced() {
    let base = import(&base_text());
    let base_clouds = solid_clouds(&base);
    let text = append(
        &append(
            &base_text(),
            &extra_instance(36, 4_000_000_000_001, 0.0, 25.0, 0.0, 0.0),
        ),
        &extra_instance(205, 4_000_000_000_101, 0.0, -25.0, 0.0, 0.0),
    );
    let body = import(&text);
    let clouds = solid_clouds(&body);
    assert_eq!(clouds.len(), 4);
    assert_cloud_eq(&clouds[0], &base_clouds[0], "identity box");
    assert_cloud_eq(&clouds[1], &base_clouds[1], "identity sphere");
    assert_cloud_eq(
        &clouds[2],
        &mapped(&base_clouds[0], |[x, y, z]| [x, y + 25.0, z]),
        "box copy at +25y",
    );
    assert_cloud_eq(
        &clouds[3],
        &mapped(&base_clouds[1], |[x, y, z]| [x, y - 25.0, z]),
        "sphere copy at -25y",
    );
}

/// (2): a COMPOSED map — rotation 90° about z plus a translation — on
/// an extra instance of each component. The sphere's curved carriers
/// (seam, rims) must re-certify under the rotation, and the clouds
/// must land at the composed frame exactly.
#[test]
fn rotation_plus_translation_re_certifies() {
    let base = import(&base_text());
    let base_clouds = solid_clouds(&base);
    let text = append(
        &append(
            &base_text(),
            &extra_instance(36, 4_000_000_000_001, 7.0, 3.0, 2.0, 90.0),
        ),
        &extra_instance(205, 4_000_000_000_101, 7.0, 3.0, 2.0, 90.0),
    );
    let body = import(&text);
    let clouds = solid_clouds(&body);
    assert_eq!(clouds.len(), 4);
    let rot = |[x, y, z]: [f64; 3]| [-y + 7.0, x + 3.0, z + 2.0];
    assert_cloud_eq(&clouds[2], &mapped(&base_clouds[0], rot), "rotated box");
    assert_cloud_eq(&clouds[3], &mapped(&base_clouds[1], rot), "rotated sphere");
}

/// (2): the ε_in identity boundary. The file's uncertainty is 1e-7 mm;
/// a 5e-8 mm displacement must classify identity (nothing moves) and a
/// 2e-7 mm displacement must classify as a placement (the body moves by
/// exactly that much).
#[test]
fn identity_boundary_at_eps_in() {
    let base = import(&base_text());
    let base_clouds = solid_clouds(&base);
    // Below ε_in: identity.
    let below = base_text().replace(
        "#16 = CARTESIAN_POINT('',(0.,0.,0.));",
        "#16 = CARTESIAN_POINT('',(5.E-08,0.,0.));",
    );
    let clouds = solid_clouds(&import(&below));
    assert_cloud_eq(
        &clouds[0],
        &base_clouds[0],
        "sub-eps displacement is identity",
    );
    // Above ε_in: a real placement, applied exactly.
    let above = base_text().replace(
        "#16 = CARTESIAN_POINT('',(0.,0.,0.));",
        "#16 = CARTESIAN_POINT('',(2.E-07,0.,0.));",
    );
    let clouds = solid_clouds(&import(&above));
    assert_cloud_eq(
        &clouds[0],
        &mapped(&base_clouds[0], |[x, y, z]| [x + 2e-7, y, z]),
        "supra-eps displacement is a placement",
    );
    assert_cloud_eq(&clouds[1], &base_clouds[1], "the other solid does not move");
}

/// (1)(d) tier gate: with one component INVERTED (sense-flipped faces)
/// among instanced good copies, the per-solid gate's refusal must name
/// the inverted component's own MANIFOLD_SOLID_BREP (#206, SphereB) —
/// not the box, not the aggregate.
#[test]
fn per_solid_gate_names_the_inverted_instance() {
    let text = append(
        &base_text(),
        &extra_instance(36, 4_000_000_000_001, 40.0, 0.0, 0.0, 0.0),
    )
    .replace(
        "#208 = ADVANCED_FACE('',(#209),#213,.T.);",
        "#208 = ADVANCED_FACE('',(#209),#213,.F.);",
    );
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::TierInvalid { solid, .. }) => {
            assert_eq!(solid, Some(206), "the refusal names the inverted sphere");
        }
        Ok(_) => panic!("an inverted instance must not import"),
        Err(other) => panic!("expected TierInvalid naming #206, got {other}"),
    }
}

/// (6) D9: instance ordering and the whole materialized body are
/// deterministic — two imports of the same multi-instance text are
/// Debug-identical, byte for byte.
#[test]
fn multi_instance_import_is_deterministic() {
    let text = append(
        &append(
            &base_text(),
            &extra_instance(36, 4_000_000_000_001, 30.0, 0.0, 0.0, 90.0),
        ),
        &extra_instance(205, 4_000_000_000_101, 60.0, 0.0, 0.0, 0.0),
    );
    let a = format!("{:?}", import(&text));
    let b = format!("{:?}", import(&text));
    assert_eq!(a, b, "two imports of one text must be identical");
    // Cross-profile comparison: a stable fingerprint printed for the
    // reviewer to compare between debug and release runs.
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in a.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    println!(
        "multi_instance_debug_fnv1a: {hash:016x} ({} bytes)",
        a.len()
    );
}

/// **The R1 MAJOR's row, now positive.** An outer transform whose
/// `rep_1` is a CONTENTLESS intermediate `SHAPE_REPRESENTATION` — a
/// nested assembly. The reviewer's version accepted either a typed
/// refusal or the composed frame; the fix COMPOSES, so this row now
/// demands the composed frame and nothing else. Silently importing at
/// the inner frame alone (+10) is the regression it was written to
/// catch.
#[test]
fn nested_assembly_outer_transform_must_not_be_dropped() {
    // #205 (sphere) is re-pointed INTO an intermediate rep #9300 with
    // its own inner frame (+10 mm), and #9300 is placed into the root
    // #10 with an outer frame (+100 mm). Composed: +110 mm.
    let inner = "#9300 = SHAPE_REPRESENTATION('',(#11),#23);\n";
    let text = append(
        &append(
            &base_text().replace(
                "#224 = ( REPRESENTATION_RELATIONSHIP('','',#205,#10) ",
                "#224 = ( REPRESENTATION_RELATIONSHIP('','',#205,#9300) ",
            ),
            inner,
        ),
        // Outer: the intermediate rep #9300 into the root at +100.
        &extra_instance(9300, 4_000_000_000_001, 100.0, 0.0, 0.0, 0.0),
    )
    // Give the sphere's own relationship a +10 mm frame.
    .replace(
        "#20 = CARTESIAN_POINT('',(0.,0.,0.));",
        "#20 = CARTESIAN_POINT('',(10.,0.,0.));",
    );
    let body = import(&text);
    let clouds = solid_clouds(&body);
    let base = solid_clouds(&import(&base_text()));
    assert_eq!(clouds.len(), 2, "one box, one nested sphere");
    // The box is untouched; the sphere sits at the COMPOSED frame.
    assert_cloud_eq(&clouds[0], &base[0], "the box, not nested, unmoved");
    assert_cloud_eq(
        &clouds[1],
        &mapped(&base[1], |[x, y, z]| [x + 110.0, y, z]),
        "the sphere at inner(+10) THEN outer(+100)",
    );
}

/// **Three levels, with a ROTATION in the middle** — the case that
/// tells composition from addition. Hand-computed oracle, written out
/// as a closed form here and never read back from the importer:
///
/// ```text
/// component → sub-A : translate (+10, 0, 0)
/// sub-A     → sub-B : rotate +90° about z   (a,b,c) ↦ (−b, a, c)
/// sub-B     → root  : translate (+100, 0, 0)
///
/// composed  = T₁₀₀ ∘ R₉₀ ∘ T₁₀ :  (x, y, z) ↦ (100 − y, x + 10, z)
/// ```
///
/// Order matters and is the whole point: the additive reading
/// (+110 in x) and the wrong-order reading (R₉₀ applied first) both
/// give clouds this row rejects.
#[test]
fn three_level_nesting_composes_outermost_last() {
    let reps = "#9300 = SHAPE_REPRESENTATION('',(#11),#23);\n\
                #9400 = SHAPE_REPRESENTATION('',(#11),#23);\n";
    let text = append(
        &append(
            &append(
                &base_text().replace(
                    "#224 = ( REPRESENTATION_RELATIONSHIP('','',#205,#10) ",
                    "#224 = ( REPRESENTATION_RELATIONSHIP('','',#205,#9300) ",
                ),
                reps,
            ),
            // sub-A into sub-B: rotate +90° about z, no translation.
            &extra_instance_into(9300, 9400, 4_000_000_000_001, 0.0, 0.0, 0.0, 90.0),
        ),
        // sub-B into the root: +100 mm in x.
        &extra_instance_into(9400, 10, 4_000_000_000_011, 100.0, 0.0, 0.0, 0.0),
    )
    .replace(
        "#20 = CARTESIAN_POINT('',(0.,0.,0.));",
        "#20 = CARTESIAN_POINT('',(10.,0.,0.));",
    );
    let body = import(&text);
    let clouds = solid_clouds(&body);
    let base = solid_clouds(&import(&base_text()));
    assert_eq!(clouds.len(), 2);
    assert_cloud_eq(&clouds[0], &base[0], "the box, not nested, unmoved");
    assert_cloud_eq(
        &clouds[1],
        &mapped(&base[1], |[x, y, z]| [100.0 - y, x + 10.0, z]),
        "the sphere through three composed frames",
    );
    // And the composition is NOT the additive reading.
    assert!(
        mapped(&base[1], |[x, y, z]| [x + 110.0, y, z]) != clouds[1],
        "the rotation is load-bearing in this oracle"
    );
}

/// A stated placement that reaches NO component refuses typed. This is
/// the other half of the MAJOR's fix: composing chains means a
/// relationship may legitimately not start one, so "contributes
/// nothing" must be checked for rather than assumed benign.
#[test]
fn a_placement_no_chain_reaches_refuses_typed() {
    // An outer frame on a representation nothing is placed into.
    let text = append(
        &append(
            &base_text(),
            "#9300 = SHAPE_REPRESENTATION('',(#11),#23);\n",
        ),
        &extra_instance(9300, 4_000_000_000_001, 100.0, 0.0, 0.0, 0.0),
    );
    match import_step(&text, &ImportOptions::default(), Tol::witness())
        .expect_err("an unreachable placement")
    {
        StepImportError::Structure { id, what } => {
            assert_eq!(id, 4_000_000_000_001, "the refusal names the relationship");
            assert!(what.contains("no component's placement chain"), "{what}");
        }
        other => panic!("expected Structure, got {other}"),
    }
}

/// A placement chain that returns to a representation it already
/// places states no finite instance set — refused, never unrolled to a
/// guessed depth.
#[test]
fn a_cyclic_placement_chain_refuses_typed() {
    // sphere → #9300 → #9400 → #9300.
    let reps = "#9300 = SHAPE_REPRESENTATION('',(#11),#23);\n\
                #9400 = SHAPE_REPRESENTATION('',(#11),#23);\n";
    let text = append(
        &append(
            &append(
                &base_text().replace(
                    "#224 = ( REPRESENTATION_RELATIONSHIP('','',#205,#10) ",
                    "#224 = ( REPRESENTATION_RELATIONSHIP('','',#205,#9300) ",
                ),
                reps,
            ),
            &extra_instance_into(9300, 9400, 4_000_000_000_001, 1.0, 0.0, 0.0, 0.0),
        ),
        &extra_instance_into(9400, 9300, 4_000_000_000_011, 2.0, 0.0, 0.0, 0.0),
    );
    match import_step(&text, &ImportOptions::default(), Tol::witness()).expect_err("a cyclic chain")
    {
        StepImportError::Structure { what, .. } => {
            assert!(what.contains("returns to a representation"), "{what}");
        }
        other => panic!("expected Structure, got {other}"),
    }
}

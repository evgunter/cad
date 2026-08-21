//! R1 adversarial-review probes for PR #276 (M7-7, the shared at-rest
//! gate at import). **Authored on the review branch
//! (`m7/tier-review-probes`) and adopted verbatim-in-substance at the
//! fix pass**; the two that failed BY DESIGN as findings are now the
//! PINS of their fixes or of the residue they name:
//!
//! * C1's multi-solid smuggle (`r1_...multisolid...`) was the MAJOR —
//!   an inverted solid shipped when accompanied. It now asserts the
//!   per-solid refusal, and its companion pins that gating cannot
//!   regress to first-solid-only.
//! * C4's kiss-assembly probe (`r1_finding_kiss_assembly_...`) names a
//!   BANKED residue, not a fixed defect: imports declare no contacts,
//!   so a touching imported assembly gets tier 3 where its native twin
//!   carries 3'. It stays green while that is true and turns red the
//!   day import learns to declare contacts — which is the point.
//!
//! Fixtures are doctored IN CODE from committed ones so the
//! corpus-walk pin in `tier_gate.rs` stays untouched.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::PathBuf;

use step_import::{ImportOptions, StepImport, StepImportError, import_step};
use topo::ValidationError;
use geom_core::Tol;

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)).unwrap()
}

/// Flips a cube solid inside-out: reverse every loop (`FACE_OUTER_BOUND`
/// orientation) AND flip every `ADVANCED_FACE` `same_sense`, so windings
/// and sense bits stay mutually consistent (check 6 blind by design) and
/// only the volume invariant can see it.
fn invert(line: &str) -> String {
    if line.contains("FACE_OUTER_BOUND") || line.contains("ADVANCED_FACE") {
        line.replace(".T.", ".F.")
    } else {
        line.to_owned()
    }
}

/// Adds `tx` to the x coordinate of every CARTESIAN_POINT line.
fn translate_x(line: &str, tx: f64) -> String {
    let Some(start) = line.find("CARTESIAN_POINT('', (") else {
        return line.to_owned();
    };
    let open = start + "CARTESIAN_POINT('', (".len();
    let close = line[open..].find(')').unwrap() + open;
    let coords: Vec<f64> = line[open..close]
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    format!(
        "{}{}, {}, {}{}",
        &line[..open],
        coords[0] + tx,
        coords[1],
        coords[2],
        &line[close..]
    )
}

/// Renumbers every `#id` reference by `offset`.
fn renumber(line: &str, offset: u64) -> String {
    let mut out = String::new();
    let mut chars = line.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if c == '#' {
            let mut num = String::new();
            while let Some(&(_, d)) = chars.peek() {
                if d.is_ascii_digit() {
                    num.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            out.push('#');
            out.push_str(&(num.parse::<u64>().unwrap() + offset).to_string());
        } else {
            out.push(c);
        }
    }
    out
}

/// The id of an entity line (`#n = ...`), if it is one.
fn entity_id(line: &str) -> Option<u64> {
    let rest = line.strip_prefix('#')?;
    let end = rest.find(' ')?;
    rest[..end].parse().ok()
}

/// One solid of a doctored multi-solid file: the id offset its copy of
/// cube.step is renumbered by, and the per-line edit applied to it.
type Doctor<'a> = (u64, &'a dyn Fn(&str) -> String);

/// Rebuilds cube.step with the given per-solid doctors; each copy of
/// entities #1..=#150 is renumbered by its offset, the shared trailer
/// (#151..) is kept, and the shape representation lists every solid.
fn multi_cube(doctors: &[Doctor<'_>]) -> String {
    let src = fixture("../step-export/tests/fixtures/cube.step");
    let mut out: Vec<String> = Vec::new();
    let mut trailer: Vec<String> = Vec::new();
    for line in src.lines() {
        match entity_id(line) {
            Some(id) if id <= 150 => {
                for &(offset, doctor) in doctors {
                    out.push(renumber(&doctor(line), offset));
                }
            }
            Some(160) => {
                let breps: Vec<String> = doctors
                    .iter()
                    .map(|(o, _)| format!("#{}", 150 + o))
                    .collect();
                trailer.push(format!(
                    "#160 = ADVANCED_BREP_SHAPE_REPRESENTATION('', (#154, {}), #159);",
                    breps.join(", ")
                ));
            }
            Some(_) => trailer.push(line.to_owned()),
            None => {
                if trailer.is_empty() && out.iter().all(|l| entity_id(l).is_none()) {
                    out.push(line.to_owned()); // header lines through DATA;
                } else {
                    trailer.push(line.to_owned()); // ENDSEC, END-ISO
                }
            }
        }
    }
    out.extend(trailer);
    out.join("\n")
}

/// CONTROL: a single fully-inverted cube must refuse at the gate with
/// the kernel's own NegativeVolume verdict (the band_c180 shape, on a
/// planar body). One solid, so the subject is the assembled body.
#[test]
fn r1_control_single_inverted_cube_refuses_negative_volume() {
    let text = multi_cube(&[(1000, &invert)]);
    let e = import_step(&text, &ImportOptions::default()).unwrap_err();
    let StepImportError::TierInvalid { solid, errors } = &e else {
        panic!("want the gate's refusal, got: {e:?}");
    };
    assert_eq!(*solid, None, "a one-solid file's subject is the whole body");
    assert!(
        errors.contains(&ValidationError::NegativeVolume),
        "want NegativeVolume among the verdicts: {errors:?}"
    );
}

/// **R1 FINDING (C1), now the fix's pin.** A TWO-solid file whose other
/// solid is a fully inverted cube used to SHIP: all solids assemble
/// into one `Body`, the +V invariant (check 7) is the boundary flux
/// SUMMED over every shell, so +1 m^3 + (-1 m^3) = 0 is Zero-exempt,
/// and check 6 is blind by design to a consistent inversion of an
/// all-planar solid. Re-running the gate on the shipped body was
/// equally blind, so no re-gate row could have caught it.
///
/// The fix asks the gate about each `MANIFOLD_SOLID_BREP` on its OWN
/// body, before the solids are aggregated, and the refusal names which
/// one. Both orders are pinned: **an inverted solid must refuse
/// wherever it sits in the file**, which is also what kills a
/// regression to gating only the first solid — the reviewer's
/// `take(1)` mutation survived the old suite and must not survive this
/// one.
#[test]
fn r1_multisolid_cannot_smuggle_an_inverted_solid_at_any_position() {
    let normal: &dyn Fn(&str) -> String = &|l: &str| translate_x(l, 10.0);
    let inverted: &dyn Fn(&str) -> String = &invert;
    // (offsets, which offset carries the inverted cube)
    for (doctors, guilty) in [
        (vec![(1000, normal), (3000, inverted)], 3000u64),
        (vec![(1000, inverted), (3000, normal)], 1000u64),
    ] {
        let text = multi_cube(&doctors);
        let e = import_step(&text, &ImportOptions::default())
            .expect_err("an inverted solid must refuse whichever position it occupies");
        let StepImportError::TierInvalid { solid, errors } = &e else {
            panic!("want the gate's typed refusal, got: {e:?}");
        };
        // multi_cube renumbers the MSB (#150 in cube.step) by the offset.
        assert_eq!(
            *solid,
            Some(150 + guilty),
            "the refusal must name the guilty solid, not merely that one exists"
        );
        assert!(
            errors.contains(&ValidationError::NegativeVolume),
            "the per-solid subject exposes the +V verdict the sum hid: {errors:?}"
        );
    }
}

/// The other half of the per-solid claim: a multi-solid file of
/// HONEST solids still imports. A per-solid gate that refused valid
/// assemblies would be a false alarm, not a certification — and
/// `kiss_assembly`, `compound_two` and `twobody_importexport` in the
/// committed corpus already hold this from the other direction.
#[test]
fn r1_multisolid_of_honest_solids_still_imports() {
    let normal: &dyn Fn(&str) -> String = &|l: &str| translate_x(l, 10.0);
    let identity: &dyn Fn(&str) -> String = &|l: &str| l.to_owned();
    let text = multi_cube(&[(1000, identity), (3000, normal)]);
    let StepImport::Solid { body, .. } =
        import_step(&text, &ImportOptions::default()).expect("two honest cubes import")
    else {
        panic!("expected a solid");
    };
    assert_eq!(body.solids().count(), 2, "both solids shipped");
}

/// Moves the cube's (0,1,1) corner to (0,1,1+delta) and re-aims every
/// LINE whose edge ends at that corner so both endpoints stay exactly
/// ON their carriers (adoption certifies edges; it must stay green —
/// the GATE's planar-face Newell residual is the door under test).
/// Surgery: every CARTESIAN_POINT record holding (0,1,1) moves; every
/// EDGE_CURVE(v1, v2, line) with a moved endpoint gets its LINE's
/// origin point rewritten to v1's coords and its DIRECTION rewritten
/// to normalize(v2 - v1), full f64 precision.
fn perturb_corner_consistently(src: &str, delta: f64) -> String {
    use std::collections::HashMap;
    let mut ent: HashMap<u64, String> = HashMap::new();
    let mut order: Vec<Option<u64>> = Vec::new();
    for line in src.lines() {
        if let Some(id) = entity_id(line) {
            let body = line.split_once(" = ").unwrap().1.to_owned();
            ent.insert(id, body);
            order.push(Some(id));
        } else {
            order.push(None);
        }
    }
    let get_refs = |body: &str| -> Vec<u64> {
        let mut refs = Vec::new();
        let mut it = body.char_indices().peekable();
        while let Some((_, c)) = it.next() {
            if c == '#' {
                let mut n = String::new();
                while let Some(&(_, d)) = it.peek() {
                    if d.is_ascii_digit() {
                        n.push(d);
                        it.next();
                    } else {
                        break;
                    }
                }
                refs.push(n.parse().unwrap());
            }
        }
        refs
    };
    let parse_triple = |body: &str| -> Option<[f64; 3]> {
        let open = body.rfind('(')?;
        let close = body[open..].find(')')? + open;
        let v: Vec<f64> = body[open + 1..close]
            .split(',')
            .map(|s| s.trim().parse().ok())
            .collect::<Option<_>>()?;
        v.try_into().ok()
    };
    let fmt_triple = |kw: &str, p: [f64; 3]| -> String {
        format!("{kw}('', ({:.17e}, {:.17e}, {:.17e}));", p[0], p[1], p[2])
    };
    // 1. Move every CARTESIAN_POINT at (0,1,1).
    let mut moved_points: Vec<u64> = Vec::new();
    for (&id, body) in ent.clone().iter() {
        if body.starts_with("CARTESIAN_POINT") && parse_triple(body) == Some([0.0, 1.0, 1.0]) {
            ent.insert(id, fmt_triple("CARTESIAN_POINT", [0.0, 1.0, 1.0 + delta]));
            moved_points.push(id);
        }
    }
    // 2. Vertex coords: VERTEX_POINT(name, #point).
    let vertex_coords = |ent: &HashMap<u64, String>, vid: u64| -> [f64; 3] {
        let pref = get_refs(&ent[&vid])[0];
        parse_triple(&ent[&pref]).unwrap()
    };
    // 3. Re-aim every LINE of an EDGE_CURVE ending at a moved vertex.
    for body in ent.clone().values() {
        if !body.starts_with("EDGE_CURVE") {
            continue;
        }
        let refs = get_refs(body); // v1, v2, curve
        let (v1, v2, curve) = (refs[0], refs[1], refs[2]);
        if !ent[&curve].starts_with("LINE") {
            continue;
        }
        let (p1, p2) = (vertex_coords(&ent, v1), vertex_coords(&ent, v2));
        let touches_moved = [v1, v2].iter().any(|&v| {
            let pref = get_refs(&ent[&v])[0];
            moved_points.contains(&pref)
        });
        if !touches_moved {
            continue;
        }
        let line_refs = get_refs(&ent[&curve]); // origin point, vector
        let (origin, vector) = (line_refs[0], line_refs[1]);
        let dir_ref = get_refs(&ent[&vector])[0];
        let d = [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let d = [d[0] / n, d[1] / n, d[2] / n];
        ent.insert(origin, fmt_triple("CARTESIAN_POINT", p1));
        ent.insert(dir_ref, fmt_triple("DIRECTION", d));
    }
    order
        .iter()
        .zip(src.lines())
        .map(|(id, raw)| match id {
            Some(id) => format!("#{id} = {}", ent[id]),
            None => raw.to_owned(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// C3 route 1 (measured): an IN-BAND vertex defect never reaches the
/// gate — the ADOPTION ladder re-certifies carriers with the KERNEL
/// band (not eps_in) and refuses first, loudly, with an Escalated
/// certification error; above the band it refuses definite. Either
/// way: no in-band geometry ever ships silently, which is the C3
/// posture. (A gate-level escalated verdict was not constructible
/// through any route tried; adoption's certification sweep covers the
/// vertex/carrier defect classes first.)
///
/// The defect is sized FROM the ambient band, never from a literal:
/// "in band" means (ε, Kε) and "above band" means past Kε, so the
/// probe constructs the same two situations on every hosted ε row.
/// (At the default ε = 1e-9, K = 10 these are the recorded
/// 2/4/8e-9 and 1.5/3e-8 metres.) `eps_in` is pinned 1000× ABOVE the
/// ambient ε on every row, which is the point being made: an ε_in that
/// coarse still cannot loosen what adoption decides.
#[test]
fn r1_in_band_vertex_defect_refuses_loudly_at_adoption() {
    let src = fixture("../step-export/tests/fixtures/cube.step");
    let tol = geom_core::Tol::witness().get();
    let (eps, escalate) = (tol.eps, tol.k * tol.eps);
    for (delta, want_escalated) in [
        (2.0 * eps, true),
        (4.0 * eps, true),
        (8.0 * eps, true),
        (1.5 * escalate, false),
        (3.0 * escalate, false),
    ] {
        let doctored = perturb_corner_consistently(&src, delta);
        let options = ImportOptions {
            eps_in: Some(1000.0 * eps),
            ..ImportOptions::default()
        };
        match import_step(&doctored, &options) {
            Ok(shipped) => panic!("delta {delta:e}: SHIPPED silently: {shipped:?}"),
            Err(e) => {
                let dbg = format!("{e:?}");
                assert_eq!(
                    dbg.contains("Escalated"),
                    want_escalated,
                    "delta {delta:e}: wrong refusal class: {dbg}"
                );
            }
        }
    }
}

/// A thin triangular wedge prism, hand-emitted STEP: apex edge along z
/// at x=y=0, legs of length 1, apex angle `s` radians (small). Every
/// coordinate is exact and every vertex sits exactly on its planes, so
/// per-edge ADOPTION has nothing to refuse; the apex dihedral is the
/// only defect, and it is visible ONLY to the gate's check 4 (the
/// wedge classification) — adoption certifies carriers-on-surfaces,
/// not wedge angles. With s*lever in (1e-9, 1e-8) the classification
/// lands IN-BAND: the refusal, if any, must be an escalated verdict.
fn sliver_wedge_step(alpha: f64, leg: f64, h: f64) -> String {
    let (sin, cos) = alpha.sin_cos();
    let (qx, qy) = (-sin, cos); // outward normal of the tilted face
    let (cax, cay) = (-cos, -sin); // C->A direction (unit)
    let (cxx, cyy) = (leg * cos, leg * sin); // vertex C
    let db = ((cxx - leg), cyy); // B->C, unnormalized
    let dbn = (db.0 * db.0 + db.1 * db.1).sqrt();
    let (bcx, bcy) = (db.0 / dbn, db.1 / dbn); // B->C direction
    let (rx, ry) = (bcy, -bcx); // outward normal of far face R
    let mut e = Vec::new();
    let mut id = 0u64;
    let mut emit = |body: String, e: &mut Vec<String>| -> u64 {
        id += 1;
        e.push(format!("#{id} = {body}"));
        id
    };
    // -- vertices (6)
    let mut mk_v = |x: f64, y: f64, z: f64, e: &mut Vec<String>| {
        let p = emit(
            format!("CARTESIAN_POINT('', ({x:.17e}, {y:.17e}, {z:.17e}));"),
            e,
        );
        emit(format!("VERTEX_POINT('', #{p});"), e)
    };
    let a0 = mk_v(0.0, 0.0, 0.0, &mut e);
    let b0 = mk_v(leg, 0.0, 0.0, &mut e);
    let c0 = mk_v(cxx, cyy, 0.0, &mut e);
    let a1 = mk_v(0.0, 0.0, h, &mut e);
    let b1 = mk_v(leg, 0.0, h, &mut e);
    let c1 = mk_v(cxx, cyy, h, &mut e);
    // -- an edge: line through `p` with direction `d`, between v1 -> v2
    let mut mk_edge =
        |v1: u64, v2: u64, p: (f64, f64, f64), d: (f64, f64, f64), e: &mut Vec<String>| {
            let orig = emit(
                format!(
                    "CARTESIAN_POINT('', ({:.17e}, {:.17e}, {:.17e}));",
                    p.0, p.1, p.2
                ),
                e,
            );
            let dir = emit(
                format!("DIRECTION('', ({:.17e}, {:.17e}, {:.17e}));", d.0, d.1, d.2),
                e,
            );
            let vec = emit(format!("VECTOR('', #{dir}, 1.0);"), e);
            let line = emit(format!("LINE('', #{orig}, #{vec});"), e);
            emit(format!("EDGE_CURVE('', #{v1}, #{v2}, #{line}, .T.);"), e)
        };
    let ab = mk_edge(a0, b0, (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), &mut e);
    let bc = mk_edge(b0, c0, (leg, 0.0, 0.0), (bcx, bcy, 0.0), &mut e);
    let ca = mk_edge(c0, a0, (cxx, cyy, 0.0), (cax, cay, 0.0), &mut e);
    let ab1 = mk_edge(a1, b1, (0.0, 0.0, h), (1.0, 0.0, 0.0), &mut e);
    let bc1 = mk_edge(b1, c1, (leg, 0.0, h), (bcx, bcy, 0.0), &mut e);
    let ca1 = mk_edge(c1, a1, (cxx, cyy, h), (cax, cay, 0.0), &mut e);
    let aa = mk_edge(a0, a1, (0.0, 0.0, 0.0), (0.0, 0.0, 1.0), &mut e);
    let bb = mk_edge(b0, b1, (leg, 0.0, 0.0), (0.0, 0.0, 1.0), &mut e);
    let cc = mk_edge(c0, c1, (cxx, cyy, 0.0), (0.0, 0.0, 1.0), &mut e);
    // -- a face: plane (origin, normal, refdir) + oriented loop
    let mut mk_face = |uses: &[(u64, bool)],
                       o: (f64, f64, f64),
                       nrm: (f64, f64, f64),
                       refd: (f64, f64, f64),
                       e: &mut Vec<String>| {
        let po = emit(
            format!(
                "CARTESIAN_POINT('', ({:.17e}, {:.17e}, {:.17e}));",
                o.0, o.1, o.2
            ),
            e,
        );
        let dn = emit(
            format!(
                "DIRECTION('', ({:.17e}, {:.17e}, {:.17e}));",
                nrm.0, nrm.1, nrm.2
            ),
            e,
        );
        let dr = emit(
            format!(
                "DIRECTION('', ({:.17e}, {:.17e}, {:.17e}));",
                refd.0, refd.1, refd.2
            ),
            e,
        );
        let ax = emit(format!("AXIS2_PLACEMENT_3D('', #{po}, #{dn}, #{dr});"), e);
        let pl = emit(format!("PLANE('', #{ax});"), e);
        let oe: Vec<String> = uses
            .iter()
            .map(|&(edge, fwd)| {
                let flag = if fwd { ".T." } else { ".F." };
                format!(
                    "#{}",
                    emit(format!("ORIENTED_EDGE('', *, *, #{edge}, {flag});"), e)
                )
            })
            .collect();
        let lp = emit(format!("EDGE_LOOP('', ({}));", oe.join(", ")), e);
        let ob = emit(format!("FACE_OUTER_BOUND('', #{lp}, .T.);"), e);
        emit(format!("ADVANCED_FACE('', (#{ob}), #{pl}, .T.);"), e)
    };
    // bottom (outward -z): A0 -> C0 -> B0
    let f_bot = mk_face(
        &[(ca, false), (bc, false), (ab, false)],
        (0.0, 0.0, 0.0),
        (0.0, 0.0, -1.0),
        (1.0, 0.0, 0.0),
        &mut e,
    );
    // top (outward +z): A1 -> B1 -> C1
    let f_top = mk_face(
        &[(ab1, true), (bc1, true), (ca1, true)],
        (0.0, 0.0, h),
        (0.0, 0.0, 1.0),
        (1.0, 0.0, 0.0),
        &mut e,
    );
    // P, y=0 (outward -y): A0 -> B0 -> B1 -> A1
    let f_p = mk_face(
        &[(ab, true), (bb, true), (ab1, false), (aa, false)],
        (0.0, 0.0, 0.0),
        (0.0, -1.0, 0.0),
        (1.0, 0.0, 0.0),
        &mut e,
    );
    // R, the far face (outward ~+x): B0 -> C0 -> C1 -> B1
    let f_r = mk_face(
        &[(bc, true), (cc, true), (bc1, false), (bb, false)],
        (leg, 0.0, 0.0),
        (rx, ry, 0.0),
        (0.0, 0.0, 1.0),
        &mut e,
    );
    // Q, tilted (outward ~+y): A0 -> A1 -> C1 -> C0
    let f_q = mk_face(
        &[(aa, true), (ca1, false), (cc, false), (ca, true)],
        (0.0, 0.0, 0.0),
        (qx, qy, 0.0),
        (0.0, 0.0, 1.0),
        &mut e,
    );
    let shell = emit(
        format!("CLOSED_SHELL('', (#{f_bot}, #{f_top}, #{f_p}, #{f_r}, #{f_q}));"),
        &mut e,
    );
    let brep = emit(format!("MANIFOLD_SOLID_BREP('wedge', #{shell});"), &mut e);
    // context + rep, cube.step's shape
    let o = emit("CARTESIAN_POINT('', (0.0, 0.0, 0.0));".into(), &mut e);
    let z = emit("DIRECTION('', (0.0, 0.0, 1.0));".into(), &mut e);
    let x = emit("DIRECTION('', (1.0, 0.0, 0.0));".into(), &mut e);
    let ax = emit(format!("AXIS2_PLACEMENT_3D('', #{o}, #{z}, #{x});"), &mut e);
    let lu = emit(
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) );".into(),
        &mut e,
    );
    let au = emit(
        "( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($, .RADIAN.) );".into(),
        &mut e,
    );
    let su = emit(
        "( NAMED_UNIT(*) SI_UNIT($, .STERADIAN.) SOLID_ANGLE_UNIT() );".into(),
        &mut e,
    );
    let un = emit(
        format!(
            "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.0E-9), #{lu}, \
             'distance_accuracy_value', '');"
        ),
        &mut e,
    );
    let ctx = emit(
        format!(
            "( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{un})) \
             GLOBAL_UNIT_ASSIGNED_CONTEXT((#{lu}, #{au}, #{su})) \
             REPRESENTATION_CONTEXT('Context #1', '3D') );"
        ),
        &mut e,
    );
    emit(
        format!("ADVANCED_BREP_SHAPE_REPRESENTATION('', (#{ax}, #{brep}), #{ctx});"),
        &mut e,
    );
    format!(
        "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''), '2;1');\n\
         FILE_NAME('wedge.step', '1970-01-01T00:00:00', (''), (''), 'reviewer', '', '');\n\
         FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\nENDSEC;\nDATA;\n{}\n\
         ENDSEC;\nEND-ISO-10303-21;\n",
        e.join("\n")
    )
}

/// C3 route 2: the sliver wedge — the in-band defect adoption cannot
/// see (a wedge ANGLE, not a carrier residual). Whatever door refuses,
/// it must refuse LOUDLY; a shipped wedge would mean an in-band
/// dihedral passed the gate silently.
#[test]
fn r1_in_band_dihedral_wedge_never_ships_silently() {
    // Legs 10 m, apex edge h = 0.1 m. The dihedral margin is alpha*h,
    // so alpha is chosen FROM the ambient band to put it at 3/5/8·ε —
    // inside (ε, Kε) on every hosted ε row, not just the default one
    // (where these are the recorded 3/5/8e-8 rad). The far edge's
    // margin is alpha*leg = 100× that, decidedly positive throughout.
    let (leg, h) = (10.0_f64, 0.1_f64);
    let eps = geom_core::Tol::witness().get().eps;
    for alpha in [3.0 * eps / h, 5.0 * eps / h, 8.0 * eps / h] {
        let text = sliver_wedge_step(alpha, leg, h);
        match import_step(&text, &ImportOptions::default()) {
            Ok(StepImport::Solid { body, .. }) => panic!(
                "alpha={alpha:e}: the in-band wedge SHIPPED as a solid (gate re-run: {:?})",
                topo::validate_geometric(&body)
            ),
            Ok(other) => panic!("alpha={alpha:e}: unexpected non-solid {other:?}"),
            Err(err) => {
                println!("alpha={alpha:e}: refused: {err}");
                if let StepImportError::TierInvalid { errors, .. } = &err {
                    assert!(
                        errors
                            .iter()
                            .any(|v| matches!(v, ValidationError::SliverDihedral { .. })),
                        "alpha={alpha:e}: gate refusal without the sliver verdict: {errors:?}"
                    );
                    let msg = err.to_string();
                    assert!(
                        msg.contains("shared at-rest validation gate"),
                        "voice: {msg}"
                    );
                }
            }
        }
    }
}

/// **The R1 finding, RETIRED AS PINNED (M9-2)**: the finding recorded
/// that the shipped tier-3 gate passed the touching kiss the 3′(∅)
/// form would refuse — the substantive residue of the #260 one-gate
/// ruling. The shared gate is now the 3′ form over the import-side
/// declaration channel (D7 step 4), so the residue is GONE in exactly
/// the pinned direction: the kiss refuses UNDECLARED at import — the
/// old pin's inversion, exact — and certifies WITH the declared
/// corner, through the same `validate_pseudomanifold` a native
/// declared-contact body runs.
#[test]
fn r1_finding_retired_kiss_refuses_undeclared_and_certifies_declared() {
    let text = fixture("../step-export/tests/fixtures/kiss_assembly.step");
    // WITHOUT: the undeclared corner kiss is the census's finding.
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("the undeclared kiss refuses at the shared 3′ gate");
    let StepImportError::TierInvalid { errors, .. } = &err else {
        panic!("expected the gate's refusal, got {err:?}");
    };
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "the refusal is exactly the undeclared kiss: {errors:?}"
    );
    // WITH: the declared corner certifies — same file, same gate.
    let options = ImportOptions {
        declared_contacts: vec![step_import::ImportContact::VertexRest {
            at: [1.0, 1.0, 1.0],
        }],
        ..ImportOptions::default()
    };
    let StepImport::Solid { body, .. } = import_step(&text, &options).unwrap() else {
        panic!("the declared kiss imports as a solid");
    };
    assert_eq!(body.solids().count(), 2, "both kissing solids ship");
    // A wrong anchor is a typed refusal, never a silent drop.
    let bad = ImportOptions {
        declared_contacts: vec![step_import::ImportContact::VertexRest {
            at: [9.0, 9.0, 9.0],
        }],
        ..ImportOptions::default()
    };
    match import_step(&text, &bad) {
        Err(StepImportError::DeclarationUnresolved { found: 0, .. }) => {}
        other => panic!("an unresolvable anchor must refuse typed, got {other:?}"),
    }
}

/// D9 determinism: importing the same file twice yields byte-identical
/// bodies (Debug rendering compared, which prints every arena).
#[test]
fn r1_import_is_deterministic_for_a_gated_body() {
    for rel in [
        "../step-export/tests/fixtures/composed_die.step",
        "tests/fixtures/band/band_a.stp",
    ] {
        let text = fixture(rel);
        let a = import_step(&text, &ImportOptions::default()).unwrap();
        let b = import_step(&text, &ImportOptions::default()).unwrap();
        let (StepImport::Solid { body: ba, .. }, StepImport::Solid { body: bb, .. }) = (a, b)
        else {
            panic!("{rel}: expected solids");
        };
        assert_eq!(
            format!("{ba:?}"),
            format!("{bb:?}"),
            "{rel}: import not deterministic"
        );
    }
}

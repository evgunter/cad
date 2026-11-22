//! **Every hollow body round-trips through STEP**: a solid with
//! cavities leaves as one `BREP_WITH_VOIDS` (its voids
//! `ORIENTED_CLOSED_SHELL(.F.)`s, `step-export`'s crate docs) and comes
//! back, through the at-rest gate every import passes, as one solid
//! with the same shell roles, the same census and the same volume.
//!
//! **The volume budget.** Both sides are certified enclosures of the
//! same true volume, each read at the pinned resolution, so they
//! overlap: `|v_back − v_built| ≤ pad_back + pad_built`, plus a
//! fold-order slop of `1e-12·|v|` (the two sides may sum their faces'
//! terms in different orders). The closed form, where the shape has
//! one, is held to the built side's enclosure the same way.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use std::f64::consts::PI;

use common::arena_census;
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use step_export::{StepOptions, step_string};
use step_import::{StepImport, import_step};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::test_support::{brick, finished};
use topo::{AtRestBody, Body, BooleanResult, ShellRole};

/// Each solid's (outer, void) shell counts, in solid order.
fn roles(body: &AtRestBody<f64>, tol: Tol) -> Vec<(usize, usize)> {
    let rows = topo::classify_shells(body, tol).expect("the body's shells classify");
    body.solids()
        .map(|(solid, _)| {
            let of = |role| {
                rows.iter()
                    .filter(|row| row.solid == solid && row.role == role)
                    .count()
            };
            (of(ShellRole::Outer), of(ShellRole::Void))
        })
        .collect()
}

/// The certified volume `(value, pad)` at the pinned resolution.
fn volume(body: &AtRestBody<f64>, tol: Tol) -> (f64, f64) {
    let enclosure = body
        .measure(topo::test_support::pinned_resolution::<f64>(tol))
        .expect("the body measures")
        .enclosure;
    (enclosure.volume, enclosure.volume_pad)
}

/// Exports `body`, imports the text, and holds the import to the body:
/// one solid per solid with the same roles, census and volume (module
/// docs), and `closed_form` to the built volume. `voids` is the number
/// of cavities the shape is built with.
fn round_trips(name: &str, body: Body<f64>, voids: usize, closed_form: f64) {
    let tol = Tol::witness();
    let built = finished(name, body, tol);
    assert_eq!(
        roles(&built, tol),
        vec![(1, voids)],
        "{name}: the built roles"
    );
    let text = step_string(
        &built,
        &StepOptions {
            product_name: name.to_owned(),
            ..StepOptions::default()
        },
        tol,
    )
    .unwrap_or_else(|e| panic!("{name}: export: {e:?}"));
    assert_eq!(
        (
            text.matches("= BREP_WITH_VOIDS(").count(),
            text.matches("= ORIENTED_CLOSED_SHELL(").count()
        ),
        (1, voids),
        "{name}: one solid with voids, one oriented shell per cavity"
    );
    let back = match import_step(&text, &common::own_import_options(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        Ok(other) => panic!("{name}: imported as something other than a solid: {other:?}"),
        Err(e) => panic!("{name}: import: {e_at}", e_at = e.render()),
    };
    assert_eq!(
        roles(&back, tol),
        vec![(1, voids)],
        "{name}: the imported roles"
    );
    assert_eq!(
        arena_census(&back),
        arena_census(&built),
        "{name}: census (solids, shells, faces, edges, vertices)"
    );
    let ((v_built, pad_built), (v_back, pad_back)) = (volume(&built, tol), volume(&back, tol));
    let slop = 1e-12 * v_built.abs();
    assert!(
        (v_back - v_built).abs() <= pad_back + pad_built + slop,
        "{name}: imported volume {v_back} ± {pad_back} against built {v_built} ± {pad_built}"
    );
    assert!(
        (v_built - closed_form).abs() <= pad_built + slop,
        "{name}: built volume {v_built} ± {pad_built} against the closed form {closed_form}"
    );
}

/// `a` minus each of `cuts`, in turn.
fn subtracted(a: Body<f64>, cuts: &[Body<f64>], tol: Tol) -> Body<f64> {
    cuts.iter().fold(a, |a, b| {
        let a = finished("the minuend", a, tol);
        let b = finished("the cavity", b.clone(), tol);
        let BooleanResult::Body(result) = topo::subtract(&a, &b, tol).expect("the cut") else {
            panic!("a cavity cut leaves a body")
        };
        result.body.into_body()
    })
}

/// The revolve axis: the sketch plane's +y through the origin.
fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// `body` hollowed by the shell verb at `thickness`.
fn shelled(body: Body<f64>, thickness: f64, tol: Tol) -> Body<f64> {
    topo::shell(&finished("the operand", body, tol), thickness, tol)
        .expect("the body shells")
        .body
        .into_body()
}

#[test]
fn a_box_with_a_box_cavity_round_trips() {
    let tol = Tol::witness();
    let body = subtracted(
        brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol),
        &[brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol)],
        tol,
    );
    round_trips("box cavity", body, 1, 26.0);
}

#[test]
fn a_box_with_two_cavities_round_trips() {
    let tol = Tol::witness();
    let body = subtracted(
        brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol),
        &[
            brick((0.5, 1.0), (0.5, 1.0), (0.5, 1.0), tol),
            brick((2.0, 2.5), (2.0, 2.5), (2.0, 2.5), tol),
        ],
        tol,
    );
    round_trips("two cavities", body, 2, 27.0 - 2.0 * 0.125);
}

/// A curved cavity: the full revolve of an annulus, a torus with a
/// toroidal bore, `2π²R(rₒ² − rᵢ²)`.
#[test]
fn a_ring_with_a_toroidal_cavity_round_trips() {
    let tol = Tol::witness();
    let (r, ro, ri) = (5.0, 0.5, 0.35);
    let circle = |radius| profile::circle(Point2::new(r, 0.0), radius, tol).unwrap();
    let annulus = Profile::new(
        SketchPlane::xy(),
        vec![circle(ro).into(), circle(ri).into()],
    )
    .validate(tol)
    .unwrap();
    let ring = revolve(&annulus, axis_y(), Revolution::Full, tol)
        .unwrap()
        .body
        .into_body();
    round_trips(
        "hollow ring",
        ring,
        1,
        2.0 * PI * PI * r * (ro * ro - ri * ri),
    );
}

/// The shell verb's planar output: a `[0, 2]³` box hollowed at 0.25.
#[test]
fn a_shelled_box_round_trips() {
    let tol = Tol::witness();
    let body = shelled(brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol), 0.25, tol);
    round_trips("shelled box", body, 1, 8.0 - 1.5_f64.powi(3));
}

/// The shell verb's curved output: a cylinder of radius 1 and height 2,
/// revolved from its half-section, hollowed at 0.1 — a cylinder cavity
/// of radius 0.9 and height 1.8.
#[test]
fn a_shelled_cylinder_round_trips() {
    let tol = Tol::witness();
    let half_section = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 2.0),
        Point2::new(0.0, 2.0),
    ]);
    let section = Profile::new(SketchPlane::xy(), vec![half_section])
        .validate(tol)
        .unwrap();
    let cylinder = revolve(&section, axis_y(), Revolution::Full, tol)
        .unwrap()
        .body
        .into_body();
    let body = shelled(cylinder, 0.1, tol);
    round_trips("shelled cylinder", body, 1, PI * (2.0 - 0.9 * 0.9 * 1.8));
}

// ------------------------------------------------- what the gate refuses

/// The exported text of the brick `[lo, hi]³`.
fn brick_text(lo: f64, hi: f64, tol: Tol) -> String {
    let body = finished(
        "the brick",
        brick::<f64>((lo, hi), (lo, hi), (lo, hi), tol),
        tol,
    );
    step_string(&body, &StepOptions::default(), tol).expect("a brick exports")
}

/// The id of a `#id = record` line.
fn id_of(line: &str) -> u64 {
    line.strip_prefix('#')
        .and_then(|rest| rest.split_once(" = "))
        .and_then(|(id, _)| id.parse().ok())
        .unwrap_or_else(|| panic!("not a record line: {line}"))
}

/// A `BREP_WITH_VOIDS` spliced by hand: `outer`'s one solid, with
/// `cavity`'s one closed shell as its void, stated with `orientation`
/// (`.T.` or `.F.`). The cavity's topology and geometry records are
/// renumbered past `outer`'s, and nothing else of its file is kept.
fn spliced(outer: &str, cavity: &str, orientation: &str) -> String {
    let offset = outer
        .lines()
        .filter(|line| line.starts_with('#'))
        .map(id_of)
        .max()
        .expect("records");
    let renumbered = |line: &str| {
        let mut out = String::new();
        let mut rest = line;
        while let Some(at) = rest.find('#') {
            out.push_str(&rest[..=at]);
            rest = &rest[at + 1..];
            let digits = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            let id: u64 = rest[..digits].parse().expect("an id");
            out.push_str(&(id + offset).to_string());
            rest = &rest[digits..];
        }
        out + rest
    };
    // The writer emits a solid's topology and geometry before the solid
    // record, its closed shell last among them.
    let body: Vec<String> = cavity
        .lines()
        .filter(|line| line.starts_with('#'))
        .take_while(|line| !line.contains("= MANIFOLD_SOLID_BREP("))
        .map(renumbered)
        .collect();
    let shell = body.last().expect("the cavity's records");
    assert!(
        shell.contains("= CLOSED_SHELL("),
        "the cavity's last record is its shell"
    );
    let oriented = offset + body.len() as u64 + 1;
    let void = format!(
        "#{oriented} = ORIENTED_CLOSED_SHELL('', *, #{}, {orientation});",
        id_of(shell)
    );
    outer
        .lines()
        .map(|line| match line.split_once(" = MANIFOLD_SOLID_BREP(") {
            Some((id, args)) => {
                let args = args.strip_suffix(");").expect("a record");
                let mut lines = body.clone();
                lines.push(void.clone());
                lines.push(format!("{id} = BREP_WITH_VOIDS({args}, (#{oriented}));"));
                lines.join("\n")
            }
            None => line.to_owned(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The import's tier refusals.
fn refusals(text: &str, tol: Tol) -> Vec<topo::ValidationError> {
    match import_step(text, &common::own_import_options(), tol) {
        Err(step_import::StepImportError::TierInvalid { errors, .. }) => errors,
        Err(e) => panic!("refused, but not at the gate: {e_at}", e_at = e.render()),
        Ok(_) => panic!("the import shipped a body"),
    }
}

/// **The splice is sound**: a cavity stated as the standard states it —
/// the brick `[1, 2]³` written outward, `.F.` — imports as the box with
/// a box cavity, so the refusals below are about what the file says.
#[test]
fn a_hand_spliced_void_imports_as_its_cavity() {
    let tol = Tol::witness();
    let text = spliced(
        &brick_text(0.0, 3.0, tol),
        &brick_text(1.0, 2.0, tol),
        ".F.",
    );
    let back = match import_step(&text, &common::own_import_options(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the spliced box imports: {other:?}"),
    };
    assert_eq!(roles(&back, tol), vec![(1, 1)], "one solid, one cavity");
    let (v, pad) = volume(&back, tol);
    assert!(
        (v - 26.0).abs() <= pad + 1e-12 * 26.0,
        "the hollow box's volume: {v} ± {pad}"
    );
}

/// **A void stated with the wrong sense is a second piece of material**:
/// `.T.` keeps the cavity brick's faces pointing out of it, so it reads
/// as an `Outer` shell, and the gate refuses the solid.
#[test]
fn a_void_with_the_wrong_sense_refuses_at_the_gate() {
    let tol = Tol::witness();
    let text = spliced(
        &brick_text(0.0, 3.0, tol),
        &brick_text(1.0, 2.0, tol),
        ".T.",
    );
    let errors = refusals(&text, tol);
    assert!(
        matches!(
            errors[..],
            [topo::ValidationError::SolidOuterShells { outer: 2, .. }]
        ),
        "two outer shells under one solid: {errors:?}"
    );
}

/// **A void outside its outer shell bounds no cavity in the solid's
/// material**: the gate refuses it standing at the wrong winding.
#[test]
fn a_void_outside_its_outer_shell_refuses_at_the_gate() {
    let tol = Tol::witness();
    let text = spliced(
        &brick_text(0.0, 1.0, tol),
        &brick_text(5.0, 6.0, tol),
        ".F.",
    );
    let errors = refusals(&text, tol);
    assert!(
        !errors.is_empty()
            && errors
                .iter()
                .all(|e| matches!(e, topo::ValidationError::ShellWinding { .. })),
        "the void stands at the wrong winding: {errors:?}"
    );
}

/// **The committed hollow box refuses if its cavity is not reversed.**
/// Two mutants of `voided.step`: the cavity's shell written as stored
/// (every `same_sense` and bound flag after the outer shell flipped
/// back), and the cavity stated `.T.`. Either leaves the cavity's faces
/// pointing into the material, a second outer shell, which the gate
/// refuses — so the writer's reversal is what makes the file import.
#[test]
fn a_void_written_unreversed_or_stated_t_refuses() {
    let tol = Tol::witness();
    let text = common::fixture("voided", "step");
    // The cavity's records follow the outer shell's CLOSED_SHELL.
    let outer = text.find("= CLOSED_SHELL(").expect("the outer shell");
    let at = outer + text[outer..].find('\n').expect("a line end") + 1;
    let (head, tail) = text.split_at(at);
    let flip = |line: &str| {
        let oriented = ["= ADVANCED_FACE(", "= FACE_OUTER_BOUND(", "= FACE_BOUND("]
            .iter()
            .any(|k| line.contains(k));
        match (
            oriented,
            line.strip_suffix(".T.);"),
            line.strip_suffix(".F.);"),
        ) {
            (true, Some(rest), _) => format!("{rest}.F.);"),
            (true, _, Some(rest)) => format!("{rest}.T.);"),
            _ => line.to_owned(),
        }
    };
    let unreversed = format!(
        "{head}{}\n",
        tail.lines().map(flip).collect::<Vec<_>>().join("\n")
    );
    let stated_t = text.replace(
        ", .F.);\n#300 = BREP_WITH_VOIDS",
        ", .T.);\n#300 = BREP_WITH_VOIDS",
    );
    assert_ne!(stated_t, text, "the mutant states the cavity .T.");
    for (what, mutant) in [("unreversed", unreversed), ("stated .T.", stated_t)] {
        let errors = refusals(&mutant, tol);
        assert!(
            matches!(
                errors[..],
                [topo::ValidationError::SolidOuterShells { outer: 2, .. }]
            ),
            "{what}: two outer shells under one solid: {errors:?}"
        );
    }
}

/// **Open CASCADE's hollow solids import** (`tests/fixtures/occ_voids/`,
/// written by OCCT's own STEP writer, `gen_occ_voids.py`): its
/// `BREP_WITH_VOIDS` with `.F.` voids is read as one solid with its
/// cavities, the volume the generator's closed form (the files are in
/// millimetres; the import is in metres). The budget is the enclosure's
/// pad plus `1e-9·|V|` for the file's printed coordinates.
#[test]
fn open_cascade_hollow_solids_import_with_their_cavities() {
    let tol = Tol::witness();
    for (file, voids, mm3) in [
        ("box_cavity.step", 1, 26000.0),
        ("two_cavities.step", 2, 26750.0),
        ("cylinder_sphere_cavity.step", 1, 732.0 * PI),
    ] {
        let path = format!(
            "{}/tests/fixtures/occ_voids/{file}",
            env!("CARGO_MANIFEST_DIR")
        );
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let back = match import_step(&text, &common::own_import_options(), tol) {
            Ok(StepImport::Solid { body, .. }) => body,
            Ok(other) => panic!("{file}: imported as something other than a solid: {other:?}"),
            Err(e) => panic!("{file}: import: {e_at}", e_at = e.render()),
        };
        assert_eq!(roles(&back, tol), vec![(1, voids)], "{file}: the roles");
        let (v, pad) = volume(&back, tol);
        let want = mm3 * 1e-9;
        assert!(
            (v - want).abs() <= pad + 1e-9 * want,
            "{file}: volume {v} ± {pad} against the closed form {want}"
        );
    }
}

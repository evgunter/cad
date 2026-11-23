//! Second delta re-read probes for PR 7124 at `8e9af35fce3`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, RigidMotion, Tol, UnitVec3, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{Assembled, AtRestBody, Body, Built, Cell, Cited, Cites, Contact, ContactRecords};

fn tol() -> Tol {
    Tol::witness()
}

fn rod(cx: f64, cy: f64, r: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    let circle: ProfileLoop<f64> =
        bulge_loop(vec![(at(0.0), 1.0), (at(core::f64::consts::PI), 1.0)]);
    let profile = Profile::new(sketch_at(z0), vec![circle]).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance { depth: h, side: ExtrudeSide::Along },
        tol(),
    )
    .unwrap()
    .body
    .into_body();
    finished("rod", body, tol())
}

fn ell(z0: f64) -> AtRestBody<f64> {
    let corner = |x, y| (Point2::new(x, y), 0.0);
    let body: Body<f64> = sweep::test_support::prism_at(
        vec![
            corner(0.0, 0.0),
            corner(2.0, 0.0),
            corner(2.0, 1.0),
            corner(1.0, 1.0),
            corner(1.0, 2.0),
            corner(0.0, 2.0),
        ],
        z0,
        1.0,
        tol(),
    );
    finished("ell", body, tol())
}

fn names(findings: &[topo::ValidationError]) -> Vec<String> {
    let mut out: Vec<String> = findings
        .iter()
        .map(|f| {
            let s = format!("{f:?}");
            s.split(|c: char| c == ' ' || c == '{' || c == '(').next().unwrap().to_owned()
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Every pair of one cell of each of two bodies that touch nowhere
/// (`b` placed by `motion`), handed to the gate as the body's one
/// record: how many are held, by kind.
fn plant(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, motion: Option<RigidMotion<f64>>) -> usize {
    let mut join = Assembled::<f64>::new();
    let ka = join.graft_finished(a, None, tol()).unwrap();
    let kb = join.graft_finished(b, motion, tol()).unwrap();
    let done = topo::finish_assembled(join, tol()).unwrap();
    let minted = done.body.records().rows().len();
    let between = names(&done.findings);
    let body = topo::FinishedBody::into_body(done.body);
    let cells = |src: &AtRestBody<f64>, k: &topo::GraftKeys| {
        let mut out: Vec<Cell> = src.faces().filter_map(|(f, _)| k.face(f)).map(Cell::Face).collect();
        out.extend(src.edges().filter_map(|(e, _)| k.edge(e)).map(Cell::Edge));
        out.extend(src.vertices().filter_map(|(v, _)| k.vertex(v)).map(Cell::Vertex));
        out
    };
    let (ca, cb) = (cells(a, &ka), cells(b, &kb));
    let (mut held, mut total) = (0, 0);
    let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for &x in &ca {
        for &y in &cb {
            let Ok(contact) = Contact::new(&body, x, y) else { continue };
            total += 1;
            let mut records = ContactRecords::default();
            records.extend([Cited::new(contact, Cites::decided(0))]);
            let kind = |c: Cell| match c {
                Cell::Face(f) => format!("F:{:?}", body.face_surface(f).map(geom::Surface::kind)),
                Cell::Edge(_) => "E".to_owned(),
                Cell::Vertex(_) => "V".to_owned(),
            };
            let key = match topo::finish(Built::of(body.clone()).recording(records), tol()) {
                Ok(done) => {
                    held += 1;
                    format!("HELD {:?} {} x {}", done.bar(), kind(x), kind(y))
                }
                Err(e) => format!("refused {:?}", names(e.as_slice())),
            };
            *tally.entry(key).or_default() += 1;
        }
    }
    eprintln!("E-1 {what}: join minted {minted}, findings {between:?}; {held} of {total} planted held");
    for (k, n) in &tally {
        eprintln!("E-1 {what}:   {n} x {k}");
    }
    held
}

/// E-1: bodies that touch nowhere, close enough that their boxes
/// overlap.
#[test]
fn probe_e1_near_misses() {
    let a = rod(0.0, 0.0, 0.5, 0.0, 1.0);
    let z = UnitVec3::exact_z();
    let x = UnitVec3::exact_x();
    let quarter = core::f64::consts::FRAC_PI_4;
    let unit = || finished("block", brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    plant(
        "a block 0.2 from the wall, caps coplanar",
        &a,
        &finished("block", brick::<f64>((0.7, 1.7), (-0.5, 0.5), (0.0, 1.0), tol()), tol()),
        None,
    );
    plant(
        "a block turned 45 degrees, its corner 0.2 from the wall",
        &a,
        &unit(),
        Some(RigidMotion::rotation_then_translation(z, quarter, Vec3::new(0.7, 0.0, 0.0)).unwrap()),
    );
    plant(
        "a block turned 45 degrees and lifted 0.3",
        &a,
        &unit(),
        Some(RigidMotion::rotation_then_translation(z, quarter, Vec3::new(0.7, 0.0, 0.3)).unwrap()),
    );
    plant(
        "a rod tilted 30 degrees, 1.5 off",
        &a,
        &rod(0.0, 0.0, 0.3, 0.0, 1.0),
        Some(RigidMotion::rotation_then_translation(x, 0.5, Vec3::new(1.5, 0.0, 0.0)).unwrap()),
    );
    plant(
        "a rod across at right angles, 1.0 off",
        &a,
        &rod(0.0, 0.0, 0.3, -0.5, 1.0),
        Some(
            RigidMotion::rotation_then_translation(x, core::f64::consts::FRAC_PI_2, Vec3::new(1.0, 0.0, 0.5))
                .unwrap(),
        ),
    );
    plant("a rod in an L's notch, on the top's plane", &rod(1.5, 1.5, 0.25, 0.0, 1.0), &ell(-1.0), None);
    plant("a rod in an L's notch, 0.1 above the top's plane", &rod(1.5, 1.5, 0.25, 0.1, 1.0), &ell(-1.0), None);
    plant("a rod in an L's notch, beside it at one height", &rod(1.5, 1.5, 0.25, 0.0, 1.0), &ell(0.0), None);
    plant("a thin rod inside a wider rod's shadow, above it", &rod(0.0, 0.0, 0.2, 1.2, 1.0), &rod(0.0, 0.0, 0.5, 0.0, 1.0), None);
}

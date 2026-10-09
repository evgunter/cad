//! **The `shell` verb's operands** and the two role readers a shell row
//! runs over them: the vessel and the tube (a rectangular and an
//! annular meridian, each revolved a full turn), the hollow box, the
//! two-void box, the curved-mouth operands (a vessel under a spherical
//! cap, a hemisphere or a cone; the D-section; a dome sector), and the
//! readers that name a body's shells by the role the classifier decides.
//!
//! A shell row and its review twin are about THE SAME BODY only while
//! both build it here ([`super::cavity`]'s rule): a fixture that moves
//! reddens both at once instead of splitting the corpus. Body authoring
//! plus readers that evaluate no surface, so it routes here
//! ([`super`]'s routing rule), as [`super::latitude_seam`] does for the
//! same reason.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - [`super::operands`], which the vessel and the tube would join by
//!   kind (plain named bodies) and do not: they are the shell rows'
//!   operands and stay beside the role readers those rows run over
//!   them;
//! - `shell8_common`'s `outer_and_void_of`, the per-SOLID twin of
//!   [`outer_and_void`] on a multi-solid body — that tree is the SHELL-8
//!   family's own and is not a `common::` module;
//! - the polygon prism, which is [`sweep::test_support::prism`] over
//!   [`sweep::test_support::corners`] and needs no name here;
//! - the subtraction the two-void box is cut with, which is
//!   [`super::cavity::cut`];
//! - the box's closed-form volume `V(w, d, h)`, which is a truth
//!   derived without the kernel and so is
//!   [`super::oracles::box_volume`].

use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::test_support::{block, brick, corners, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis};
use topo::{Body, ShellKey, ShellRole, SolidKey};

use super::cavity::cut;
use sweep::test_support::finished;

/// **The vessel**: a rectangular meridian revolved a full turn about
/// the `y` axis — a solid cylinder of radius `r` and height `h`,
/// bounded by one cylinder wall and two planar caps. The perf fixture.
pub fn vessel(r: f64, h: f64) -> Body<f64> {
    revolved_about_y(
        corners(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)]),
        Revolution::Full,
        Tol::witness(),
    )
}

/// **The tube**: the annular meridian `[ri, ro] × [0, h]` revolved a
/// full turn — the curved two-shell shape the STEP gate is recorded on.
pub fn tube(ri: f64, ro: f64, h: f64) -> Body<f64> {
    revolved_about_y(
        corners(&[(ri, 0.0), (ro, 0.0), (ro, h), (ri, h)]),
        Revolution::Full,
        Tol::witness(),
    )
}

/// `lp` revolved a full turn about the `y` axis.
fn revolved_full(lp: ProfileLoop<f64>) -> Body<f64> {
    let tol = Tol::witness();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the meridian validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    sweep::revolve(&profile, axis, Revolution::Full, tol)
        .expect("the meridian revolves")
        .body
}

/// **The capped vessel**: a cylinder of radius `r` and height `h`
/// under a spherical cap whose meridian arc runs `deg` degrees from
/// the wall to the pole, so the sphere has radius `r / sin(deg)` and
/// meets the wall at an angle (a cap of `deg < 90` sits above its
/// equator; one of `deg > 90` bulges past it). Returns the body and
/// the sphere's radius and centre height.
pub fn capped_vessel(r: f64, h: f64, deg: f64) -> (Body<f64>, f64, f64) {
    let polar = deg.to_radians();
    let rho = r / polar.sin();
    let rise = rho * (1.0 - polar.cos());
    let body = revolved_full(bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r, 0.0), 0.0),
        (Point2::new(r, h), (polar / 4.0).tan()),
        (Point2::new(0.0, h + rise), 0.0),
    ]));
    (body, rho, h + rise - rho)
}

/// **The hollow capped vessel**: [`capped_vessel`]`(0.5, 0.6, 60)`
/// shelled sealed at `0.2`, so its void is the same vessel eroded — a
/// void with a pole-touching cap. Returns the body, the void cap's two
/// half-faces and the operand's sphere radius.
pub fn hollow_capped_vessel() -> (Body<f64>, Vec<topo::FaceKey>, f64) {
    let (vessel, rho, _) = capped_vessel(0.5, 0.6, 60.0);
    let hollow = topo::shell(
        &finished("the operand", vessel, Tol::witness()),
        0.2,
        Tol::witness(),
    )
    .expect("the capped vessel shells sealed")
    .body;
    let cap = hollow
        .faces()
        .filter(|(_, f)| {
            matches!(hollow.get_surface(f.surface),
                Some(geom::Surface::Sphere { radius, .. }) if (*radius - (rho - 0.2)).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    (hollow, cap, rho)
}

/// **The domed vessel**: a cylinder of radius `r` and height `h` under
/// a hemisphere of the same radius, tangent to the wall at the equator.
pub fn domed_vessel(r: f64, h: f64) -> Body<f64> {
    revolved_full(bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(r, 0.0), 0.0),
        (Point2::new(r, h), core::f64::consts::FRAC_PI_8.tan()),
        (Point2::new(0.0, h + r), 0.0),
    ]))
}

/// **The cone-tipped vessel**: a cylinder of radius `r` and height `h`
/// under a cone of height `k` whose apex is on the axis.
pub fn cone_tipped_vessel(r: f64, h: f64, k: f64) -> Body<f64> {
    revolved_about_y(
        corners(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h + k)]),
        Revolution::Full,
        Tol::witness(),
    )
}

/// **The D-section**: the half disc of radius `r` on `x ≥ 0` extruded
/// `h` along `z` — one half-cylinder face, its flat, and two ends.
pub fn d_section(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let d = bulge_loop(vec![
        (Point2::new(0.0, -r), 0.0),
        (Point2::new(0.0, r), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![d])
        .validate(tol)
        .expect("the half disc validates");
    sweep::extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the half disc extrudes")
    .body
}

/// **The dome sector**: [`sweep::test_support::dome`]'s bored zone
/// revolved `deg` degrees, so its sphere face is a window bounded by
/// two latitudes and two meridians, and touches no pole.
pub fn dome_sector(r: f64, deg: f64) -> Body<f64> {
    revolved_about_y(
        sweep::test_support::dome_profile(r),
        Revolution::Partial(deg.to_radians()),
        Tol::witness(),
    )
}

/// **The hollow box**: `block(2, 3, 4)` shelled at `0.25` — one solid,
/// two shells, the hollow operand every row that shells a hollow body
/// starts from.
pub fn hollow_box() -> Body<f64> {
    topo::shell(
        &finished(
            "the operand",
            block(2.0, 3.0, 4.0, Tol::witness()),
            Tol::witness(),
        ),
        0.25,
        Tol::witness(),
    )
    .expect("the first shell is the sealed row's own green")
    .body
}

/// **The two-void box**: a `6 × 4 × 4` box with two voids of
/// `1.2 × 2 × 2` side by side, `0.4` of material between them and at
/// least `1.0` to every outer wall — built as two subtractions, the way
/// a user would write it. Returns the body and the void gap.
pub fn two_void_box() -> (Body<f64>, f64) {
    let one = cut(
        "first void",
        &block(6.0, 4.0, 4.0, Tol::witness()),
        &brick((1.0, 2.2), (1.0, 3.0), (1.0, 3.0), Tol::witness()),
    );
    let two = cut(
        "second void",
        &one,
        &brick((2.6, 3.8), (1.0, 3.0), (1.0, 3.0), Tol::witness()),
    );
    assert_eq!(two.solids().count(), 1, "one solid");
    assert_eq!(two.shells().count(), 3, "outer plus two voids");
    (two, 0.4)
}

/// A one-hollow body's `(outer, void)` shell keys, decided through the
/// shell classifier on the body itself.
pub fn outer_and_void(body: &Body<f64>) -> (ShellKey, ShellKey) {
    let roles = topo::classify_shells(body, Tol::witness()).expect("the operand classifies");
    let pick = |role: ShellRole| {
        let hits: Vec<ShellKey> = roles
            .iter()
            .filter(|c| c.role == role)
            .map(|c| c.shell)
            .collect();
        assert_eq!(hits.len(), 1, "exactly one {role:?} shell");
        hits[0]
    };
    (pick(ShellRole::Outer), pick(ShellRole::Void))
}

/// Per SOLID, the shell roles sorted — grouped by `Shell::solid`, not
/// read as a flat multiset, because tier 3 does not check solid
/// membership and the grouping is pinned here.
pub fn roles_by_solid(body: &Body<f64>) -> Vec<(SolidKey, Vec<ShellRole>)> {
    let roles = topo::classify_shells(body, Tol::witness()).expect("the shells classify");
    body.solids()
        .map(|(solid, _)| {
            let mut kinds: Vec<ShellRole> = roles
                .iter()
                .filter(|c| c.solid == solid)
                .map(|c| c.role)
                .collect();
            kinds.sort_by_key(|r| format!("{r:?}"));
            (solid, kinds)
        })
        .collect()
}

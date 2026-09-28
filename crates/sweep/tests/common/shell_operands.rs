//! **The `shell` verb's operands** and the two role readers its rows
//! run over them: the vessel and the tube (a rectangular and an
//! annular meridian, each revolved a full turn), the hollow box, the
//! two-void box, and the readers that name a body's shells by the role
//! the classifier decides.
//!
//! `verbs_shell` authored these and every SHELL-5, SHELL-8, SHELL-9
//! and SHELL-10 suite shells them, so a row in one of those suites and
//! its twin in `verbs_shell` are about THE SAME BODY ([`super::cavity`]'s
//! rule): a fixture that moves reddens both at once instead of
//! splitting the corpus. Body authoring plus readers that evaluate no
//! surface, so it routes here ([`super`]'s routing rule), as
//! [`super::latitude_seam`] does for the same reason.
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
//! - the polygon prism `verbs_shell` and `shell5_r1_probes` build, which
//!   is [`sweep::test_support::prism`] over
//!   [`sweep::test_support::corners`] and needs no name here;
//! - the subtraction the two-void box is cut with, which is
//!   [`super::cavity::cut`];
//! - the box's closed-form volume `V(w, d, h)`, which is a truth
//!   derived without the kernel and so is
//!   [`super::oracles::box_volume`].

use geom_core::Tol;
use sweep::Revolution;
use sweep::test_support::{block, brick, corners, revolved_about_y};
use topo::{Body, ShellKey, ShellRole, SolidKey};

use super::cavity::cut;

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

/// **The hollow box**: `block(2, 3, 4)` shelled at `0.25` — one solid,
/// two shells, the hollow operand every row that shells a hollow body
/// starts from.
pub fn hollow_box() -> Body<f64> {
    topo::shell(&block(2.0, 3.0, 4.0, Tol::witness()), 0.25, Tol::witness())
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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use super::{SiteCarriers, SiteHalf, SiteRows, apply_site_rows};
use crate::test_support::arc_chain_over_the_jump;
use crate::{Body, FaceKey, LoopBoundary};

/// Every row and element `face`'s outer loop holds, in loop order.
fn rows(body: &Body<f64>, face: FaceKey) -> Vec<String> {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall's outer loop is a cycle")
    };
    let mut out: Vec<String> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| {
            let c = body.pcurve(he).unwrap();
            format!(
                "{he:?} {:?} {:?} {:?} {:?}",
                c.params(),
                c.pcurve(),
                c.certificate(),
                body.joint(he)
            )
        })
        .collect();
    out.sort();
    out
}

/// **A joint whose link the door changes is decided, not kept, even
/// between two images that stand.** The strut kill of the arc
/// chain's tip (`q → t` from `q`) re-links `s → q` onto `q → s`:
/// both stand, and the element stored on `q → s` is the turn from
/// `t → q`, a period, where the new joint's is the identity. The
/// site mint planned over the kill's after-loop (the loop with the
/// killed halves gone, as `kev_loops_after` hands it) decides that
/// joint and nothing else, and its rows, written over the kill, are
/// the pass's, elements included.
#[test]
fn a_re_linked_joint_between_standing_images_is_decided() {
    let tol = Tol::witness();
    let (body, face, s_q, q_t) = arc_chain_over_the_jump(tol);
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the wall's outer loop is a cycle")
    };
    let after: Vec<SiteHalf> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .filter(|he| !q_t.contains(he))
        .map(SiteHalf::Existing)
        .collect();
    let stale = body.joint(s_q[1]).unwrap();
    let plans = body
        .plan_site_mint(
            &[outer],
            |b, minted| {
                Ok(minted
                    .iter()
                    .map(|(f, _)| b.site_face(*f, &[(outer, after.clone())], None))
                    .collect())
            },
            SiteCarriers::Existing,
            tol,
        )
        .unwrap();
    let [SiteRows::Mint { images, joints }] = plans.as_slice() else {
        panic!("the wall is minted, not left or cleared")
    };
    assert!(images.is_empty(), "the kill creates no image");
    let mut pass = body.clone();
    pass.kev(q_t[0]).unwrap();
    crate::mint_pcurves_of(&mut pass, &[face], tol).unwrap();
    let decided = pass.joint(s_q[1]).unwrap();
    assert_ne!(stale, decided, "the premise: the stored element is stale");
    assert_eq!(
        joints.as_slice(),
        [(SiteHalf::Existing(s_q[1]), decided)],
        "the re-linked joint, and only it, is decided"
    );
    let mut killed = body.clone();
    killed.kev(q_t[0]).unwrap();
    killed.write_joint(s_q[1], None);
    apply_site_rows(&mut killed, plans, None);
    assert_eq!(
        rows(&killed, face),
        rows(&pass, face),
        "the rows are the pass's"
    );
}

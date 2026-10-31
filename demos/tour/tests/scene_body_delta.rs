//! `demo_tour::SceneBody`'s per-body chordal deviation.

use demo_tour::SceneBody;

fn body() -> SceneBody {
    SceneBody::plain("probe", [0.5, 0.5, 0.5], pncad::topo::Body::new())
}

#[test]
fn a_body_takes_its_scenes_delta_unless_it_asks_for_a_finer_one() {
    assert_eq!(body().delta(1e-2), 1e-2, "no per-body delta: the scene's");
    assert_eq!(
        body().finer(2e-3).delta(1e-2),
        2e-3,
        "a finer per-body delta"
    );
}

#[test]
#[should_panic(expected = "must be finer than its scene's")]
fn a_coarser_per_body_delta_is_refused() {
    body().finer(2e-2).delta(1e-2);
}

#[test]
#[should_panic(expected = "must be finer than its scene's")]
fn an_equal_per_body_delta_is_refused() {
    body().finer(1e-2).delta(1e-2);
}

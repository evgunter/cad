---
id: a-corner-touch-verdict-moves-with-the-planes-stored-origin
kind: issue
title: The plane × spline classifier's corner-touch verdict moves with where the plane's origin is stored
status: open
opened: 2026-11-03
priority: P1
cost: M
---

Found by the frame-readings unit (ONECOPY, PR 6463) and diagnosed by its review. Filed on SECTARMS's slate, the plane × spline section lane in `crates/geom-brep/src/pair_section/spline_implicit`.

**The defect.** The probe pair "a plane touching one corner" (`sweep/rows.rs` `probe_pairs`) stores the plane `x + z = 0` with normal `(1, 0, 1)`, normalized, and sets it against the unit wall `y = 0`, `x, z ∈ [0, 1]`. That plane touches the wall only at its corner `(0, 0, 0)`. `the_probes_pairs_are_classified_in_c5s_order` wants `Apart`. The verdict depends on how the same plane is represented:

- With the pivot `SplineImplicitPose::pivot` computes today (the ball centre's foot on the unit normal, `c − n·((c − o)·n)`), storing the plane's origin at `(3, 2, −3)` reads `Reaching(ChartReach { sides: [[true, false], [true, false]] })`. At `(0, 0, 0)`, `(0.25, 0, −0.25)`, `(0, 0.7, 0)` and others it reads `Apart`.
- Reading the pivot through `geom::Frame::foot` (`c − m·((c − o)·m / m·m)`) flips the origin-stored case too. The stored normal is already normalized, so `m·m = 0.9999999999999998`. The old pivot lands at `(−1.1e−16, 0, −1.1e−16)`, just outside the wall, and the new one lands exactly on the corner. Both are the same plane.

So the corner-touch verdict sits on a decision boundary that the plane's representation, not its geometry, decides. That is a latent defect in the classifier (its boundary pass or the pivot's charge), not a property of either foot formula. PR 6463 kept the old pivot formula so the row stays green; once this is fixed, the pivot can read `Frame::foot`.

**The reviewer's row**, red today and so not in the suite. It goes in `sweep/rows.rs` beside `the_probes_pairs_are_classified_in_c5s_order` once the classifier reads one verdict:

```rust
#[test]
fn a_corner_touch_reads_alike_wherever_the_planes_origin_is_stored() {
    let flat = || ruled(&[pt(0.0,0.0,0.0), pt(1.0,0.0,0.0)], &[pt(0.0,0.0,1.0), pt(1.0,0.0,1.0)]);
    let n = Vec3::new(1.0, 0.0, 1.0);
    let seen: Vec<String> = [pt(0.0,0.0,0.0), pt(0.25,0.0,-0.25), pt(0.0,0.7,0.0), pt(3.0,2.0,-3.0)]
        .into_iter().map(|o| format!("{:?}", got(&plane(o, n), &flat()))).collect();
    assert!(seen.windows(2).all(|w| w[0] == w[1]), "the verdict moved with the stored origin: {seen:?}");
}
```

**Fix.** The corner touch reads one verdict for one plane wherever its origin is stored. Then `pivot` reads `geom::Frame::foot` and the inline unit-normal foot goes.

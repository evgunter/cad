(reviewer lane, PR 7054) The three rows, written against `split_rulings.rs`' own helpers (`turned`, `cone_rulings`, `admits`, `split_and_placed`, `got_enclosing`, `tol`; row 3 also `validated`, `axis_y`, `revolve`, all already imported there). None needs a helper of mine. I ran all three appended to that file on the frozen head 64e55e4699 at the default ε, 1e-6 and 1e-12, then restored the file. What each printed is stated; the asserting forms below differ from what I ran only in turning the printed answers into `assert_eq!`, and were not re-run after that edit.

**Row 1: the literal-rule killer.** Body: wall `(1,0),(2+tan 26.6°,0),(2,1),(1,1)` revolved 0.05. Split: each cone ruling in turn, 1e-8 of its range from its start and from its end. δ 0.1, 0.01, 0.001. Must assert `Got::Meshed` in all 12 cells. Measured: 12 of 12 `Meshed` at default ε and at 1e-12; at 1e-6 every split is no split (0 cells). Under the "anchor always the neighbour before" mutant my grid's cell (cone ruling 1, split 1e-8 from its end, δ 0.01) answers `UnresolvedWinding` on the cone; I did not re-run the mutant against this exact function.

```rust
#[test]
fn a_run_at_a_corner_is_anchored_along_its_ruling() {
    let t = 26.6f64.to_radians().tan();
    let body = turned(
        &[(1.0, 0.0), (2.0 + t, 0.0), (2.0, 1.0), (1.0, 1.0)],
        Revolution::Partial(0.05),
    );
    let solid = split_and_placed(&body, &[], &[]);
    let mut cells = 0usize;
    for (k, ek) in cone_rulings(&body).into_iter().enumerate() {
        for fracs in [[1e-8], [1.0 - 1e-8]] {
            if !admits(&body, ek, &fracs) {
                eprintln!("SKIPPED at ε {:e}: ruling {k}, {fracs:?} is no split", tol().eps());
                continue;
            }
            let split = split_and_placed(&body, &[ek], &fracs);
            for delta in [0.1, 0.01, 0.001] {
                assert_eq!(
                    got_enclosing(&split, &solid, delta),
                    Got::Meshed,
                    "cone ruling {k}, split at {fracs:?}, δ {delta}"
                );
                cells += 1;
            }
        }
    }
    let want = if tol().eps() > geom_core::tolerance::DEFAULT_EPS { 0 } else { 12 };
    assert_eq!(cells, want, "the cells this row reads");
}
```

**Row 2: two runs anchored by the apex, at the default ε.** Body: cone `(0,0),(100·tan 10°,0),(0,100)` revolved 0.05 (half-angle 10°, height 100, through its apex). Split: BOTH cone rulings 1e-7 (a length, so `1e-7/len` of the range, `len = 100/cos 10°`) from their far end (fraction `1 − 1e-7/len`). δ 10, 1, 0.1. Must assert `Got::Meshed` and the cone patch has exactly 3 triangles. Measured: all three δ `Meshed` with 3 cone triangles at default ε and at 1e-12; at 1e-6 no split. (The same split at the near end, fraction `1e-7/len`, is no split at the default ε; at 1e-12 it also meshes with 3 triangles. I left it out.)

```rust
#[test]
fn two_runs_anchored_by_the_apex_mesh_on_a_large_cone() {
    let body = turned(
        &[(0.0, 0.0), (100.0 * 10f64.to_radians().tan(), 0.0), (0.0, 100.0)],
        Revolution::Partial(0.05),
    );
    let lines = cone_rulings(&body);
    assert_eq!(lines.len(), 2, "the wedge's cone face ends on two rulings");
    let len = 100.0 / 10f64.to_radians().cos();
    let fracs = [1.0 - 1e-7 / len];
    if !lines.iter().all(|&ek| admits(&body, ek, &fracs)) {
        assert!(tol().eps() > geom_core::tolerance::DEFAULT_EPS, "a split at the default ε");
        eprintln!("SKIPPED at ε {:e}: a split 1e-7 from the end is no split", tol().eps());
        return;
    }
    let split = split_and_placed(&body, &lines, &fracs);
    let solid = split_and_placed(&body, &[], &[]);
    for delta in [10.0, 1.0, 0.1] {
        assert_eq!(got_enclosing(&split, &solid, delta), Got::Meshed, "δ {delta}");
        let m = mesh::tessellate(&split, delta, tol()).unwrap();
        let cone = m
            .patches
            .iter()
            .find(|p| crate::reach_probes::kind_of(&split, p.face) == geom::SurfaceKind::Cone)
            .expect("the cone face's patch");
        assert_eq!(cone.triangles.len(), 3, "δ {delta}: the cone face's triangles");
    }
}
```

**Row 3: the needle refusal at the default ε.** Body: cone `(0,0),(1000,0),(0,1000)` revolved a full turn (45°, two half faces). Split: one cone ruling `1e-8` (a length; `len = 1000·√2`) from its far end. δ 100, 10, 1. Must assert: δ 100 and 10 tessellate and pass `check_mesh` and `check_winding`; δ 1 refuses `UnresolvedWinding` on the Cone face. Measured at default ε, both rulings: exactly that. At 1e-6 the split is no split. At 1e-12 the cone does not revolve (`turned`'s `unwrap` panics), so the row builds with `revolve` itself and skips. It cannot use `got_enclosing`: on this body `reach_probes::geometry_enclosing` panics at `pinned_volume` (reach_probes.rs:232) at the default ε, so the volume is not read. That panic is a thing I saw, not one I looked into.

```rust
#[test]
fn a_run_anchored_on_a_needle_refuses_unresolved_on_a_large_cone() {
    use mesh::TessellateError as E;
    let lp = ProfileLoop::polygon(
        [(0.0, 0.0), (1000.0, 0.0), (0.0, 1000.0)].map(|(x, y)| Point2::new(x, y)),
    );
    let Ok(revolved) = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol()) else {
        assert!(tol().eps() < geom_core::tolerance::DEFAULT_EPS, "the cone revolves at the default ε");
        eprintln!("SKIPPED at ε {:e}: the cone does not revolve", tol().eps());
        return;
    };
    let body = revolved.body.into_body();
    let fracs = [1.0 - 1e-8 / (1000.0 * 2f64.sqrt())];
    for (k, ek) in cone_rulings(&body).into_iter().enumerate() {
        if !admits(&body, ek, &fracs) {
            assert!(tol().eps() > geom_core::tolerance::DEFAULT_EPS, "a split at the default ε");
            eprintln!("SKIPPED at ε {:e}: a split 1e-8 from the end is no split", tol().eps());
            return;
        }
        let split = split_and_placed(&body, &[ek], &fracs);
        for delta in [100.0, 10.0] {
            let m = mesh::tessellate(&split, delta, tol())
                .unwrap_or_else(|e| panic!("ruling {k}, δ {delta}: {e:?}"));
            mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("ruling {k}, δ {delta}: {e:?}"));
            mesh::validate::check_winding(&m).unwrap_or_else(|e| panic!("ruling {k}, δ {delta}: {e:?}"));
        }
        match mesh::tessellate(&split, 1.0, tol()) {
            Err(E::UnresolvedWinding { face, .. }) => assert_eq!(
                crate::reach_probes::kind_of(&split, face),
                geom::SurfaceKind::Cone,
                "ruling {k}, δ 1"
            ),
            other => panic!("ruling {k}, δ 1: {:?}", other.map(|m| m.patches.len())),
        }
    }
}
```

Payload for the pin's text, from my probe at δ 1 on ruling 0: height 3.32e-13 against allowance 1.775e-12. Nothing committed or pushed; tree clean at the frozen head.

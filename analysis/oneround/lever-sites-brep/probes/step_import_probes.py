#!/usr/bin/env python3
"""Insert the reviewer's C4 probes into crates/step-import/src/build.rs's test module (they need the
private `choose` and the PR's `folded_discs` fixture). Usage: step_import_probes.py <worktree>
Run: cargo nextest run -p step-import --lib -E 'test(review_c4)' --no-capture"""
import sys
p = sys.argv[1] + '/crates/step-import/src/build.rs'
s = open(p).read()
anchor = "    /// **A wrap edge is the surface's own generator through the printed\n"
assert s.count(anchor) == 1
probe = r'''
    fn review_read(solid: &SolidSpec) -> Result<Chosen, StepImportError> {
        choose(solid, 1, &solid.edges[&1], (0, 1), Tol::witness(), FileBand::test(1e-6))
    }

    /// REVIEW C4 (exactness): the crease the 1e-5 fold builds, against the closed form. Plane 0 is
    /// `z = 0`; plane 1 passes through `(c0, 0, 0)` with normal `(sin f, 0, cos f)`: their meet is the
    /// line `x = c0, z = 0` along `y`, exactly.
    #[test]
    fn review_c4_the_built_crease_is_the_closed_form_line() {
        let c0 = (1.0_f64 - 0.01 * 0.01).sqrt();
        let chosen = review_read(&folded_discs(1e-5)).expect("the 1e-5 fold builds");
        let Some(Curve3::Line(line)) = chosen.carrier else { panic!("a line") };
        let o = line.origin;
        let d = line.dir;
        println!("REVIEW crease origin {o:?} dir {d:?}  dx={:e} z={:e} |dir x y|={:e}", o.x - c0, o.z, d.cross(Vec3::new(0.0, 1.0, 0.0)).norm());
        assert!((o.x - c0).abs() < 1e-9 && o.z.abs() < 1e-9, "off the closed form: {o:?}");
        assert!(d.x.abs() < 1e-12 && d.z.abs() < 1e-12);
    }

    /// REVIEW C4 (which bound): the fold swept through the file's band. The two faces stand at most
    /// `fold·(1 + c0) ≈ 2·fold` apart (disc 1's far point from plane 0); the lever is the cover's box
    /// diagonal, 4.47. Prints, per fold, what `choose` does.
    #[test]
    fn review_c4_fold_sweep() {
        for fold in [1e-7, 2e-7, 2.3e-7, 2.5e-7, 3e-7, 3.5e-7, 4e-7, 4.5e-7, 4.9e-7, 5e-7, 6e-7, 1e-6, 2e-6, 1e-5] {
            let what = match review_read(&folded_discs(fold)) {
                Ok(Chosen { description: Description::Intersection, carrier: Some(Curve3::Line(_)), .. }) => "BUILDS the crease".to_string(),
                Ok(_) => "builds something else".to_string(),
                Err(StepImportError::Interpretation { refusal: crate::interpret::Refusal::Touching { gap, .. }, .. }) => format!("refuses Touching, gap {gap:e}"),
                Err(e) => format!("refuses: {e}"),
            };
            println!("REVIEW fold {fold:e}: faces' true max separation {:e} (file band 1e-6): {what}", fold.sin() * (1.0 + (1.0_f64 - 1e-4).sqrt()));
        }
    }

    /// REVIEW C4 (the no-region arm): a sibling edge of one face with a non-finite interval leaves the
    /// pair with no region. What does edge 1 (whose own carrier and interval are sound) answer?
    #[test]
    fn review_c4_a_pair_with_no_region() {
        let mut solid = folded_discs(1e-5);
        solid.edges.get_mut(&2).unwrap().t0 = f64::NAN;
        match review_read(&solid) {
            Ok(c) => println!("REVIEW no-region: BUILDS {:?}", c.carrier),
            Err(e) => println!("REVIEW no-region: refuses: {e:?}  ||  display: {e}"),
        }
    }

'''
open(p, 'w').write(s.replace(anchor, probe + anchor))

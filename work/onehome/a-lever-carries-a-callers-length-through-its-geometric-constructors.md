---
id: a-lever-carries-a-callers-length-through-its-geometric-constructors
kind: issue
title: A Lever can carry any length a caller writes through its geometric constructors, against the README's 'never from a length a caller vouches for'
status: open
opened: 2026-11-03
priority: P1
cost: M
---


`crates/geom-core/README.md` ("Float rounding") says a lever is built only from its sources, and never from a length a caller vouches for. Ev ruled on PR 6380 (comment 6594257162): keep that text, and fix the violations.

**The violations.** PR 6391's FULL review built a `Margin::levered` arm from an arbitrary `len` six ways. `cargo check` and `scripts/gates/lever-sources.sh` both passed every one:
- `Lever::curvature_radius(len)`;
- `Lever::curvature(1.0 / len)`;
- `ExtentBall::of_sphere(p, len).lever()`, and likewise `of_circle`;
- `Reach::of_frame(p, None, (len, len)).lever_from(p)`;
- `ExtentBall::point(q).lever_from(p)` with a hand-made `q`;
- `about`-style folds (`.min(..)`/`.max(..)`) of any of these.

PR 6391's fix pass closed the doors that took bare numbers for no reason (`Reach::Face`, `of_overlap`, `edge_extent_*`'s chord). The ones above remain, because a geometric constructor takes its geometry's numbers.

**What fixing them means.** At a production call site, a lever's constructor reads the consumed geometry itself, not numbers the caller computed. Concretely:
- `curvature_radius` takes the surface or carrier it reads;
- `of_sphere`/`of_circle` take the carrier;
- `of_frame` takes the join frame's geometry.

The bare-number forms move to `geom_core::test_support` (behind the fixture gate), or become crate-private to the extent home. A gate then counts production calls of any bare-number form.

The 63 arms pinned `unsourced` are the other half of the violation, and they are filed on `unsourced-levers-await-a-source.md`. This row covers the constructor doors.

**Evidence:** PR 6391's review report (`onehome/one-extent`, head `083c51ce`), finding M1.

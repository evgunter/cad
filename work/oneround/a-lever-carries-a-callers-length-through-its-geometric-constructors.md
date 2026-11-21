---
id: a-lever-carries-a-callers-length-through-its-geometric-constructors
kind: issue
title: A Lever can carry any length a caller writes through its geometric constructors, against the README's 'never from a length a caller vouches for'
status: closed
opened: 2026-11-03
priority: P1
cost: M
branch: oneround/lever-doors
closed: 2026-11-20
pr: 7143
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

## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.

## Closed (2026-11-20, PR 7143)

`Lever`'s field is private, and a curvature arm is read off its carrier in one home (`geom_brep::curvature_arm` and its siblings): 40 of the 46 curvature arms that were written from a caller's radius. A gate, `scripts/gates/lever-doors.sh`, pins every named `Lever`, `ExtentBall` and `Reach` door that takes a caller's number or point, per door and file, and only falls. No record or function reads geometry back out of a `Lever`.

The README's "never from a length a caller vouches for" is not yet true, and the PR says so. What is left, each on its own row:

- `lever-doors-pinned-sites-await-a-geometry-read`: the 59 pinned references, classified by what each site's function holds, and the wrappers the pins cannot see. Its sites that hold their geometry are two units in flight (`the-lever-sites-that-hold-their-geometry-read-it-outside-topo`, `…-in-topo`); it is re-priced M for the rest.
- `unsourced-levers-await-a-source`: the records that hold a scalar and its lever as a pair (the half-fixes this unit lists there), headed for holding the carrier and deriving both.
- `sizes-are-read-back-out-of-extent-levers`: ten older sites that read a size out of an extent lever.
- `extent-reads-made-outside-the-extent-home`: the callers of `Lever::max` whose arms share no pivot.
- `work/shell/a-moved-rims-radius-is-positive-by-agreement-not-by-a-decision.md`: `transport_curve` adds an offset to a rim circle's radius with no decision, and `Circle::new` panics on a negative one.

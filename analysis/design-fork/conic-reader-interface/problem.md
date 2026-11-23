# Problem: what a reader of where a conic is asks the conic for

You are a designer lane. Read `docs/prompts/designer.md` in full first (it binds you), then `crates/geom-core/README.md`'s "Float rounding" section (above all the sentence beginning "a conic is one of a family of placements behind the one `Conic` reading" through "Every reading of where a conic is goes through the conic."), then the module doc and public surface of `crates/geom/src/curves/conic.rs`, then the code named below. Your worktree is a frozen checkout of the branch `oneround/conic-readers` at `bd07a867e7` (PR 7510: an implementation in review, NOT settled; treat what it built as evidence, not as a constraint); it is read-only for you (write nothing in it, build in your own target dir outside it if you build at all, and use only your own scratch directory). Do not read other designers' work or any directory under `~/.local/share/cad-work/` but your own worktree.

## Background

A conic carrier (`Circle`, `Ellipse`; the `Conic` trait) used to be stored as a centre, a frame and semi-axes, and every reader of "where the conic is" took `frame().origin()` or `center()` and did its own arithmetic from there. PR 7258 (Ev, 2026-11-22; `docs/DESIGN-FORK-LOG.md` row 296) decided that a conic is one of a family of placements: held about its centre, or about a point of the arc it serves, its construction fixing which. PR 7203 landed the second placement for the ellipse, private to `conic.rs` (`Held::Centre | Held::Arc { phase }`): the cone's plane section, whose centre can lie megametres from the arc, stores a point of the arc, and its centre is then a derived number rounded at the semi-axes' scale. More constructions are owed the same (`work/oneround/a-construction-whose-centre-runs-off-still-stores-a-centre.md`: the cylinder's tilted ellipse, `bisector_ellipses`, blend's oblique end face, the STEP import's circle), and the anchor is owed generalising to a point that is not a principal vertex (`work/oneround/a-section-conic-is-anchored-at-a-general-point-of-its-arc.md`).

The README's last clause, "Every reading of where a conic is goes through the conic", is the unit `work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md`: until a reader goes through the conic it reads the derived centre and loses the digits the arc-held form exists to keep.

## What was found

The implementer took a census with the compiler (the centre doors marked deprecated in a scratch build): outside `conic.rs`, production code holds 323 reads of a conic's centre or frame in 60 files, 3.5 times the row's estimate. Its report (a hypothesis; check it against the tree):

- 112 read a direction only (`frame().axis()`, `u_ref()`, `v_ref()`). PR 7510 gives `Conic` those three directly.
- 3 are the arc's cover and the extent bound. PR 7510 gives `Conic` two readings for them, `cover_over(window)` and `extent_along(window, origin, n)`, with the placement kept private; both needed, inside `conic.rs`, the stored point and the anchor's phase.
- 208 remain, in 142 functions and 57 files, each pinned by a gate (`clippy.toml`'s `disallowed-methods`, an `#[expect]` on each remaining function, `scripts/gates/conic-centre-reads.sh` and its `.counts`). The implementer groups them:
  - **the certified lane's harmonics** (about 30 functions: `geom-brep/src/implicit.rs`, `curve_pair/pose*.rs`, `ring_path.rs`, and more): each forms its own harmonic of the conic's position against a linear reading, `(c − o)·n + p·cos θ + q·sin θ` with `c` the centre, and then solves it, bounds it or encloses it over an interval, on f64 and on the `Interval` lane; some readers are not linear in the position (a distance about an axis; the torus residual);
  - **the symbolic scalar** (`swept.rs`);
  - **the cone section's chart image** (`derive_class_zero`, `pcurve.rs`, `certify.rs`): the image of the section in the cone's chart, stated today from the centre (`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`);
  - **the rebuild doors** (about 20 functions): code that makes a new conic from an old one, with a reversed axis or a transported frame, by reading the centre and frame and calling a constructor, which today turns an arc-held conic into a centre-held one;
  - **two statements of one circle compared by bit** (`mesh/sizing.rs`), and chart-frame readers of a circle leg (`torus_region.rs`);
  - **circle-only datum reads** (about 70 functions: coaxiality, rim reads, the STEP import): a circle's centre used as a datum. No construction mints an arc-held circle today.

The implementer stopped before those and asked what shape the conic's interface to them should take, since two readings on `Conic` served the cover and the extent but about thirty readers each do their own arithmetic on the position.

## What needs deciding

What does code outside `conic.rs` that needs to know where a conic is get to ask the conic, such that (1) "a conic is the curve its stored numbers state" holds for every reader on both placements, on f64 and on `Interval`; (2) the anchor can later move to a general point of the arc without each reader changing; (3) the wrong reading (a derived centre used as a position) cannot be taken by accident; and (4) a conic made from a conic keeps the placement it had. Decide it for each family above, or show that one answer serves them all, or that a family should not exist. Say what `conic.rs`'s public surface is in the final state, what happens to `Conic::frame` and `Conic::center`, and what the README and the module doc should say if either changes. Where a datum reader really does want a centre (a circle's axis of revolution, coaxiality), say what it should be handed and what that means for a conic whose centre is derived.

Check the framing first (`docs/prompts/designer.md` §1): the question as asked is "what interface"; it may be the wrong question one level up or down (what the readers are doing with a conic's position at all; what a conic stores; whether a harmonic of a conic is a thing the kernel should name).

## Where to look

- `crates/geom/src/curves/conic.rs` (the placement, the evaluator, what PR 7510 added), `crates/geom/src/curves/boxes.rs`, the cover home in `geom` that `Cover::arc` and `Cover::arc_of` live in.
- `scripts/gates/conic-centre-reads.counts` and the `#[expect(clippy::disallowed_methods…)]` sites: the list of what remains.
- `crates/geom-brep/src/implicit.rs`, `crates/geom-brep/src/curve_pair/pose.rs`, `crates/topo/src/ring_path.rs` for the harmonic readers; `crates/geom-brep/src/pcurve*.rs` and `certify.rs` for the chart image; `git grep -n 'Ellipse::new\|Circle::new' crates/*/src` for the rebuild doors.
- `work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md`, `the-arc-cover-misses-a-far-centred-arc…` and `a-conics-points-are-restated-in-centre-form-outside-its-evaluator.md` beside it, `work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`.
- Provenance of the ratified text: `git log --all -S'goes through the conic' -- crates/geom-core/README.md`; PR 7258's discussion is summarised in `docs/DESIGN-FORK-LOG.md` row 296. Ev's own words there include: "i definitely don't want it to have a weird cutoff for which form is used" and "i think rejecting ill-conditioned stuff is ok if we can do it as a panic, i.e., it is a kernel bug if we choose the wrong one".

## Deliver

Your report as your final message, in the two sections `docs/prompts/designer.md` §5 names (`## For Ev`, unsigned, leading with the decision; `## For the orchestrator`), at most 150 lines. Write no file outside your own scratch directory.

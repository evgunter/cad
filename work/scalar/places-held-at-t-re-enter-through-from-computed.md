---
id: places-held-at-t-re-enter-through-from-computed
kind: issue
title: Places a search or a walk evaluates at are held as T and re-entered through from_computed
status: open
opened: 2026-11-23
priority: P2
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found by `a-solved-value-the-body-stores-is-admitted-by-its-check` (PR 7651), whose gate `scripts/gates/from-computed-sites.sh` lists every `from_computed` caller; each site below is listed `owed` to this row in `scripts/gates/from-computed-sites.tsv` with its function, and its entry goes when the site does. The rule is `crates/geom-core/README.md` DL5.2.

A place (DL5.2's second row) is handed to the evaluator as an `f64` and never held as a `T`. These hold it as a `T` to read at, and decide on what they read; none stores it.

- `geom` `surfaces/projection.rs` and `curves/projection.rs` `project_from_seed`: Newton's iterate and the stagnation foot, the foot `project` returns (6).
- `topo::face_normal::spline_outward_normal_at` and `geom_brep::dihedral_side::foot_normal`: a foot `chart_point_normal` reads a normal at (4).
- `geom_brep::certify` `run_checks`: a row piece's ends, where its image is bracketed (2).
- `geom_brep::pair_section` `witness`: a tube run's window a foot is searched in (2); `polished`'s Newton iterates (2 of its 3 calls; the third, the kept value, is on `stored-solved-values-whose-check-differs-in-kind-are-not-behind-the-door`).
- `geom_brep::curve_pair::polish::pair_held`: a Gauss–Newton iterate re-centred at an enclosure scalar (2). Bounded `Decide`, so a `Dual<Interval>` takes a zero tangent there.
- `topo::link` `Link::lowest_on` and `Round::lowest_on`: a closed-form root at `T` read down to its point and re-entered (2). Reached by a dual; the comment's envelope argument is what makes a zero tangent right for the least, and the place belongs on the value channel.

Cost: M. Each moves to a place-taking evaluator (`geom/src/places.rs`) or reads on the value channel, as `a-place-is-an-f64-argument-answered-on-the-value-channel` did for its sites.

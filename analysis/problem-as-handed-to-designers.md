# Problem: ten readings decided through a lever door that are not levers

You are a designer lane. Read `docs/prompts/designer.md` in full first (it binds you), then `crates/geom-core/README.md`'s "Float rounding" section (the admission rule, the lever bullet, the doors), then the code named below. Your worktree is a frozen checkout of the branch `oneround/unsourced-stand-ins` at `ba6927b9e`; it is read-only for you (write nothing in it, build in your own target dir outside it if you build at all, and use only your own scratch directory). Do not read other designers' work or any directory under `~/.local/share/cad-work/` but your own worktree.

## Background

The kernel decides every sign through doors in `geom-core` (`Margin`, `Band`, `Lever`; `crates/geom-core/src/predicate.rs`). A dimensionless reading (a sine, a cosine) is turned into a length by a `Lever`, and the README's lever bullet says where a lever may come from: the geometry the site consumes, never a site's own number. The program ONEROUND is removing `Lever::unsourced(x)`, the door that lets a site hand in any number as a lever. Each remaining use is pinned by a gate (`scripts/gates/lever-sources.sh` and its counts file) and converted site by site.

## What was found

After this branch, ten references to `Lever::unsourced` remain in four families, and the implementer reports that NONE of them is a lever: each passes something else through the lever door because no door says what it is. As reported (check each against the code; the report is a hypothesis):

1. `geom-brep/src/dihedral.rs`, two sites (`wedge_reads_zero_at_the_arm` and its `_offer` twin, `Lever::unsourced(T::from_f64(band.escalate()))`): a sine (or tangent) of the dihedral angle compared against the band's own ratio, `sin θ ≤ 1/K`. The "lever" is the band's escalation threshold.
2. `topo/src/boolean/fate.rs`, `tip_wraps`: a count of half-turns (an integer plus the rounding of a sum of `atan2`s), decided Zero within ε at a unit arm.
3. `topo`'s join (`off_conic_slack`, `2ε·cot ψ`) and `geom-brep/src/intersect.rs`, five sites (`pn_apex_tangent_reach`, `pn_apex_point_reach`, `pair_bound`'s far, near and offset reads, `Margin::levered(1/x, Lever::unsourced(size))`): a stand-off (a length the site computes, such as `|apex_gap| + gap_err`) scaled by a dimensionless factor (one over a sine or a discriminant). An earlier review rejected spelling these as an inverse-levered margin (`levered_inv(size, x)`) as "the same number under another door".
4. `topo/src/validate.rs`, `resolved_as`: a rounding (a length) compared against the ε/K share of the band, `K·ρ ≤ ε`.

Facts the implementer states, to check: `Margin` is a length by its signature, and there is no dimensionless reading type; `Band::padded` and `Margin::less` take `f64`, so a `T`-valued stand-off cannot pad a band; `Band::linear_at` is a `doc(hidden)` instrument; the one production precedent for a band built from the band's own thresholds is `Band::new(band.zero()/2, band.escalate())` in `chart_region`.

## What needs deciding

What each of these ten readings IS, and what the kernel's decision vocabulary owes it, so that `Lever::unsourced` can be deleted without the ten reappearing as the same numbers under another door. The answer binds every later site of the same kinds, and it may touch the README's "Float rounding" text, which is ratified (check its provenance as designer.md says).

The options are yours to find. Start from whether the four families are the right cut, and whether each reading is needed at all (for each site: what it decides, who acts on which sign, and what breaks without it).

## Where to look

- The ten sites and their callers, by the names above.
- `crates/geom-core/src/predicate.rs` (`Margin`, `Band`, `Lever`, the `decide*` doors) and `crates/geom-core/README.md`.
- `scripts/gates/lever-sources.sh`, `scripts/gates/lever-doors.sh` and their counts files: what the gates hold and what they cannot see.
- `work/oneround/` on this branch: the unit's row and its parent row ("Read against the tree" sections), and `work/oneround/log.md` for how earlier lever questions were ruled.
- The seven conversions this branch did make (`git diff origin/main...HEAD` is large; the PR's per-site notes are in the unit row), three of which the implementer re-classified on reading (a "speed with no span" that was a turn at a curvature arm; a "lever" that was the least modulus of an `atan2` argument; a "rounding" that was a curvature arm plus a stand-off): evidence of how often a site's stated kind is wrong.

## The report

As `docs/prompts/designer.md` §5: `## For Ev` (unsigned; lead with the decision; define every term) and `## For the orchestrator`. At most 150 lines. Return it as your final message.

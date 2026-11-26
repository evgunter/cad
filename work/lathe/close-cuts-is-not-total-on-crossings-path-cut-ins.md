---
id: close-cuts-is-not-total-on-crossings-path-cut-ins
kind: issue
title: close_cuts is not total on crossings-path cut-ins: a sphere face carrying a first-pass crossing beside its interior loop refuses at the guard where the stack built it
status: open
opened: 2026-11-25
priority: P1
cost: H
refs: [the-main-merge-drops-mains-pole-form-and-pole-side-column-intents]
---

## What

On the crossings path, PR 7440 cuts in an interior loop that the
interior-loop guard proves on a sphere face, and the op re-enters on
the cut operands. Since main's cut-in framework (ORBIT) merged into the
one-band stack, the re-entered op's output stage closes every cut the
kernel made (`topo::boolean::slit::close_cuts`). That closing is total
only where two premises hold:

- **No join chord runs along a cut.** `ops.rs`'s `cuts_in_result` doc
  states it: "the cut-in runs only where the first pass found no
  crossing". The no-crossings path holds it by construction. The
  crossings path does not.
- **The chart's pole lies beyond one of a cut face's circles** where a
  cut's piece parts a face bounded by section curves alone
  (`slit::part_cut`, `slit::pole_beyond`: "the face holds no edge of
  its own boundary here"). `pole_beyond` reads this off circles only.

Where either fails, the re-entered op raised `JoinDesync`, a kernel
fault class, on poses the stack had built sound. Since the dual review
of the port at `3a0f5b84aa`, the crossings path decides both premises
before it re-enters (`interior_cuts::closing_holds`):
- no first-pass crossing names a cut face;
- each cut circle's plane leaves both chart poles decidedly on one
  side;
- a face with two or more cut-ins is cut across circles alone.

Where a premise fails, the op does not cut in, and the guard's refusal
(`CurvedPairUnsupported`, `InteriorLoopGuard`) stands, in either operand
order. These poses lose the builds they had on the stack. That is no
worse than main, and the refusal is loud.

## Witnesses

Each is pinned at the guard's refusal in all six ops, both orders, at ε
1e-9, 1e-6 and 1e-12, by
`sweep::all a_crossings_path_cut_in_is_made_only_where_its_closing_is_total::a_cut_in_whose_closing_is_not_total_refuses_alike_in_either_order`:

1. **The die's slab** `[−2, 2]² × [0, 0.7]` against the `y`-poled unit
   ball. The top cuts the ball's face in the interior circle
   `z = 0.7`. The bottom cuts it in `z = 0`, through both poles, where
   `part_cut` has no pole to read. 7440's row is now
   `m5_pr9c_sphere_doors::the_die_slab_cutting_the_ball_in_its_top_refuses_at_the_interior_loop_guard`.
2. **R2's probe**: the `y`-poled unit ball against `[−2, 2]² × [0.6, 2]`
   ∪ `[−0.2, 0] × [0.5, 2] × [0.05, 0.25]`. Before the check, `a ∩ b`
   raised `JoinDesync` and `b ∩ a` built.
3. **R1's probe**: the unit ball against `[−2, −0.9] × [−1.5, 1.5]²` ∪
   `[−2, −0.2] × [−2, −0.7] × [−2, 0]`. Before the check, `A ∩ B`
   raised `JoinDesync` and `B ∩ A` built. With the brick at `z > 0`, or
   reaching `x = 0.2`, 3 of 18 cells refused.

## What would close it

`close_cuts` extended to crossings-path cut-ins. Two things:
- A cut whose meridian a first-pass section crosses or runs along:
  the closing must read the join's chords on the cut and close across
  them.
- A face bounded by section curves alone, where the chart's poles lie
  on a circle's plane (the die slab), or on marched loops: the part that
  keeps the outline is decided some other way than `pole_beyond`'s pole.

Each premise then leaves `closing_holds` as the closing comes to hold
without it.

## Coupling

This is ORBIT's closing (`slit.rs`), read here by LATHE's crossings-path
cut-in (7440). A change to either side's premise moves this row. ORBIT
carries a seam note.

- 2026-11-26 — Evidence from the port's re-review (LATHE orchestrator).
  - Lost capability: the y-poled unit ball against (box `z ≥ 0.6`) ∪ box `x∈[0.3,0.5]`, `y∈[0.5,2]`, `z∈[−0.1,0]`. Its `z = 0` face's plane holds both poles, but its section lies far from the cut and the poles. Premise 2 refuses it, yet pre-fix `3a0f5b84aa` built it SOUND and maximal in all six ops at the default ε and 1e-6, so `close_cuts` was total there.
  - Premise 2 reads every partner circle on the cut face, not only those bounding the part `pole_beyond` reads.
  - Premise 1 refuses every partner kind other than a plane or a sphere (a cylinder, cone or torus on the cut face).
  - An annulus bounded only by authored rims (ball ∩ slab `z∈(0.5,0.8)` against a small ball plus a pin box) is cut in and closes soundly in all six ops at three ε. That is a premise set held, with `close_cuts` total.

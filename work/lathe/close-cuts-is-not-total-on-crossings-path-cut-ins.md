---
id: close-cuts-is-not-total-on-crossings-path-cut-ins
kind: issue
title: close_cuts is not total on crossings-path cut-ins: a sphere face carrying a first-pass crossing beside its interior loop refuses at the guard where the stack built it
status: closed
opened: 2026-11-25
closed: 2026-11-27
priority: P2
cost: H
refs: [the-main-merge-drops-mains-pole-form-and-pole-side-column-intents, a-cut-meridian-no-turn-clears-is-not-closed-across-the-joins-chords, a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read]
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

## 2026-11-27: the gate narrowed, the row shrunk (LATHE, `lathe/close-cuts-total`)

- **Premise 1, held by the cut's meridian.** A crossings-path circle cut-in no longer refuses where a first-pass section lies in its own meridian's plane: it runs in the first plane of a fixed sequence of turns about the axis that meets the circle decidedly twice, keeps both crossings on the cut's half, and is decidedly off every first-pass partner plane (a plane, or a sphere's radical plane) and every other cut-in circle's plane on the face (`interior_cuts::closing_meridians`). Witness 3 (R1's probe) and the box half of witness 2 build sound in either order. A loop's cut keeps its own meridian. A partner on a cut face that is no plane or sphere (a cylinder, a cone, a torus) is still not read, and the op does not cut in.
- **Premise 2, read by the closing itself.** The gate no longer reads every circle's plane against the poles. Where a cut's piece parts a face bounded by section circles alone and no part reads the pole beyond it, the closing answers `CutOutlineUndecided` (typed, not `JoinDesync`), and the crossings path drops the re-entry, so the op goes on uncut and the guard's refusal stands exactly as before. R2's lost pose builds sound in all six ops, in either order.
- **Witness 2's intersection** (the ball ∩ the cap box ∪ the pole box) builds in either order at the volume of its two parts built alone, and tier 3′ reads it only as the census's undecidable pair of curved parts (`work/reachhold/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`). It is census-undecidable, not unsound: the bypassed gate built this same correct body, which tier 3′'s census cannot separate.
- **Witness 1 (the die's slab)** is its own row now: `a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read`. Its union, and the ball less the slab, build sound. Its intersection, and the slab less the ball, refuse at the guard through `CutOutlineUndecided`.

**What is left (1b's generality)** moves to `a-cut-meridian-no-turn-clears-is-not-closed-across-the-joins-chords`, and this item closes. A cut whose meridian a first-pass section runs along where no turn of the meridian clears it: a partner whose section is no plane or sphere on a cut face, and a loop's cut, whose meridian is not turned. The closing would have to read the join's chords on the cut and close across them (ORBIT's `slit.rs`). No witness measured reaches it; the op refuses at the guard there.

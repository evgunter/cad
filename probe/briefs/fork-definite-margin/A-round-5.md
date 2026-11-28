Round 5 reply: fork-definite-margin

## For Ev

**Recommendation now (likely on the diagnosis, unsure until the next count is in).** Take the form reading out of the unit. The tool for enclosing a margin's range is a **mean-value reading on the expression graph**: the margin's value at one member of the box, plus its derivative enclosed over the box times the box's half-widths. The symbolic tier keeps the one job it does well, proving identities. Put no E12 text to you until the mean-value count moves the tour. What survives from round 4 is the E6 sentence on what a certified leaf promises; it holds for any reading tighter than the value's.

Terms:
- **V**: the value's enclosure, computed op by op over the box.
- **Form**: the polynomial normal form the symbolic tier builds.
- **Graph**: the expression DAG recorded beside each value, in which each parameter is one symbol.
- **Mean-value reading (MV)**: m(c) + Σᵢ ∂m/∂pᵢ(box)·(pᵢ − cᵢ), with c a member of the box, m(c) enclosed with its rounding, and each derivative enclosed over the whole box.

**What moved.** The count shows the tour's margins are not cancellations. They are true functions of the parameters whose spread over the box is about a hundredth of their enclosure (`curve_carrier_clear`: members 3.87e-3 to 3.93e-3, enclosure [0, 1.39e-2]). Every enclosure the kernel has, V and the form alike, is a zeroth-order evaluation, so it loses to repeated occurrences of a parameter however the occurrences are spelled. A first-order (centred) evaluation is the standard remedy. M16b is consistent with that; no count has yet tested it.

**1. What the defect is.**
- **The margins** are smooth, or Lipschitz through `abs`/`min`/`max`, functions of a few parameters. They are computed by deep graphs: carrier clearances through roots, projections and normalizations, with each parameter entering many times. Interval evaluation inflates their range by a factor that grows with the number of occurrences, and the inflation is first order in the box width. Only centring removes first-order inflation. This is the general case of `work/tier/real-margin-dependency-widening`; the cancelling constant of M15 was its easiest instance. (likely)
- **Where it starts: at what a decision encloses.** Every enclosure available to a decision is zeroth-order. It does not start at the readers. Re-spelling a reader cuts its occurrence count (PR 7724: 1,496 undecided down to 212) and works locally, but it is the per-site remedy E12 rejected and it never generalizes.
- **The form is the wrong object to enclose a range.** Normal form expands the graph: it multiplies occurrences, loses the factored structure, and freezes under its budget, which is exactly the 15,200 frozen ring nodes. It is the right object for proving identities (canonical, exact) and the wrong one for a range. Opened through its frozen nodes, it encloses no tighter than V (sure, from M16b's traced representatives).
- **Stored values** that are wide before any margin is formed (PR 7724's unbounded roots, the span hulls) are a separate class: a value that is not a decision. A reading over the graph back to the parameters goes through a stored value. It does not go through a hull, an opaque value or a free variable, and those sit in the graphs of `period_near_mark`, `azimuth_near_cut_side` and `curve_carrier_gap`.

**2. The round-4 agreement shrinks.**
- **E6 sentence kept:** "Every decision of a certified leaf holds of its margin's real value at every member, and the leaf takes the decisions the witness took; a member's own `f64` build agrees wherever its decisions are posed as `[simple.conditioned]` requires." MV, like any tighter enclosure, needs it.
- **The form-range reading (V ∩ F) is withdrawn** as the mechanism. It decides the fixtures, the slab and the verb-built body at ± 0.05, but MV on the graph should decide the same cases, because forward-mode derivatives cancel exactly where the form cancels linearly:
  - the cap residual: ∂/∂top of nz·(p.z − o.z) = nz·(1 − 1) = 0 exactly, so MV is the point reading P, in the band at ε;
  - `0.912·|top|`: the exact range;
  - the parallel constant 0.41: derivative 0.

  At the least band, MV on the cap residual straddles, as members do, so round 3's dispute disappears. Keep the form reading only if the next count shows asks it decides that MV does not, with members agreeing.

**MV as a design:**
- **What is evaluated:** the session's graph, walked once per leaf with a memo, as PR 7712's point reading walks it. Each node carries a point enclosure at c, and an interval derivative over the box per parameter of the box. It reads the graph and the brackets only: no lane value, no form, no freezing (a graph has no budget).
- **Soundness:** the mean-value theorem. Through kinks it is Lebourg's generalized version, with the Clarke straddle hulls the dual lane already carries (`[scalars]`). The result is a certified enclosure of the margin's real value, intersected with V. It tightens, and never refuses: where it declines, V stands, so E9 ("tangent poison never refuses") and E4 ("never for refusal decisions") hold literally.
- **Ratified text it touches:**
  - E7's heading "duals accelerate, never decide". Here derivatives enclose a value; they never decide a branch, which is what value-part delegation protects.
  - E12's rejection of mean-value bounds, which is right for identities, whose derivative enclosure keeps the dependency, and should say it is scoped to them.

  The provenance of both should be checked before anything is proposed.
- **Where it declines:**
  - a `Select` whose condition straddles over the box (a jump);
  - `atan2` over its cut;
  - a hull, unless its two branches meet at the tie;
  - opaque and free values;
  - a denominator, or a `sqrt` argument, enclosing 0 (the derivative is unbounded, so the reading is undecided, never wrong).
- **Cost:** one graph pass with (1 + n) channels per node per leaf, memoized, with n the box's parameters. That is about the point reading's 0.2 to 1 ms per ask times (1 + n), against the ladder's 29 ms. (unsure; measure)
- **Order at an undecided decision:** V; plain form, where zero gives a proven `Zero`; MV, deciding on V ∩ MV; the point reading, gating the rest of the ladder (PR 7712); the ladder.
- **One mechanism, and it is the rule:** decide on the intersection of the certified enclosures the tier holds, which are V and MV. The symbolic tier is the identity prover, not a range enclosure.

**3. The site work: keep holding, one count longer, sorted as round 2 sorted it.**
- **Continue** what fixes a value (PR 7724's roots), states an identity, or applies `[scalars]` (the fixed ray, the one-plane reading).
- **Hold** re-spellings whose only reason is occurrence count, because MV is designed to make them unnecessary.

What the top five are owed:
- `curve_carrier_clear` and `curve_face_pose_disc`: MV. Their members agree, and their graphs hold no hull or free value.
- `period_near_mark`, `azimuth_near_cut_side` and `curve_carrier_gap`: first, a trace of why a hull or a free value is in their graph. The compound readers `Period::near` and `Azimuth::near` read at the least band. If a span hull or a free identity is what blocks, that is value-layer site work, and it should be released for those three.

**4. What the mean-value count must show, and what I conclude if the tour still does not move.**
- **On asks:** MV decides most of the 22,812 straddles, and the margins that have no hull or free value in their graph. Members agree, apart from the least-band class. No empty intersection.
- **On leaves:** the tour's first drive moves, with refused leaves certified or fewer splits. Report, per leaf, how many decisions remain undecided, since a leaf moves only when all of them decide.
- **On cost:** a few ms per ask at most, and leaf time no worse than today's.
- **Against the form reading:** report the asks the form reading decides that MV does not, on the fixtures, the slab and the verb-built body. If there are none, the form reading is not built.
- **If the asks decide and the leaves still do not move**, find the first undecided decision in each leaf. If members disagree there, it is a real flip: the drive has reached its geometry, and the refusals are correct as reported. If it is a hull, an opaque or a free value, or a value that is not a decision, the wall is the stored values, and that is site work at the value layer, released.
- **If MV decides little:** the margins are not first-order. Either w² curvature dominates (a second-order or subdivided enclosure is needed), or the derivative enclosures carry the same dependency. Then I would conclude that no general reading at the decision moves this tour. Site re-spelling is the remedy after all and should be released wholesale, and E12 gets no change from this fork.

## For the orchestrator

- **I am withdrawing what both designers agreed in round 4 (V ∩ F).** The other reply may not have, so expect a difference here.
- **The derivative-cancels claim is unchecked.** That MV reproduces the form's wins on M15/M16 (the cap residual, the parallel constant) is my algebra, not run. Ask M16c to report V ∩ F against V ∩ MV, ask by ask, on population 1, the slab and the verb-built body.
- **On what M16c is measuring:** "the derivative enclosed over the box" must be the graph's forward derivative with interval-valued intermediates, not the derivative at a point. The latter is not sound.
- **No dual-lane replay for MV.** MV belongs on the graph walk, not on a `Dual<Interval>` replay of the document: a second replay is a parallel path. And a lane at `Dual<Sym<Interval>>` admits no meaning at the tier.
- **Provenance to check before any text is proposed:** E7's "duals accelerate, never decide" and E12's "Rejected — mean-value / Taylor bounds", with `git log -S`. The shallow checkout may need the PR tree.

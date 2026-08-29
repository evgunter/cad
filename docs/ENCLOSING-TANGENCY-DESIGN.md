# The enclosing (ρ < 0) fillet tangency — should anything ever emit it?

**Status: DESIGN CONVERSATION (S-BLEND, issue 827) — awaits Evan's
sign-off. Sign-off affordance: 👍 the PR comment.** Either answer
below closes the issue; the point of this doc is that one gets
chosen on purpose. Mechanics are measured (file:line), not assumed.

## Settled ground (cited, not re-litigated)

- The lattice door's boundary is deliberate and pinned: per the
  merge record of `6b8205ef` and Evan's S71 ruling, the enclosing
  class was not dropped by accident and is not a regression.
  `crates/profile/tests/review_s2.rs` pins it both ways —
  `the_lattice_door_never_emits_an_enclosing_tangency`, and the
  mined twin `enclosing_fillet_swallows_both_leg_carriers`; the
  suite's own header derives the reachability algebra
  (ρ = R − σ·τ·r < 0 ⟺ σ·τ = +1 and r > R) and builds the
  enclosing rows as fixtures precisely because no sampled corner
  can reach them.
- The construction machinery is live and correct:
  `crates/profile/src/sugar.rs` (the signed offset radii, the
  antipodal tangent-point flip, the measured-spoke projection whose
  error argument is written out at the tangent-point doc around
  `:715-770`). It is the substrate the boundary is defined
  *against*, exercised by the pins.
- No shipped door reaches the class: the lattice derives corners
  from anchored carriers and ranks survivors, and the pair's other
  crossing always carries a cheaper ordinary candidate the signed
  gates cannot exclude. Only a door that AUTHORS the corner could
  emit the enclosing tangency, and there is no such door.

## The open question

For a corner with σ·τ = +1 on both legs and r > R, the enclosing
tangency is the fillet the geometry demands. The kernel can compute
it; a user has no spelling for it; nothing is scheduled either way.
Should anything ever emit the class?

## The measured precedent this starts from (the VERBS handoff)

LILYWELD's junction gate, pinned live at `demos/tour/src/lily.rs`
`review_probes::the_globes_tangent_cone_neck_is_refused_by_the_junction_gate`:
a leg whose departure is spelled in COORDINATES and lands within ε
of the incoming tangent refuses `PathError::JunctionTangent` — a
vanishing turn margin (1.6e-17 measured at the handoff; the pin
bounds it below 1e-15) on a real lever arm — and the refusal names
its one recourse: say the tangency STRUCTURALLY, with the
`.tangent()` verb, which is exact by construction rather than by
arithmetic that happens to agree. The precedent generalizes: when a
special configuration is what the user means, the algebra does not
infer it from numbers; it demands a spelling that declares it.

## Recommendation — R1: yes in principle, structurally spelled, consumer-gated

1. **The class stays emittable only by a door that authors the
   corner** — a structural spelling of the enclosing intent, the
   `.tangent()` posture one level up — and NEVER by the ranking
   ladder. A ladder that starts picking enclosing candidates when
   r > R would be inferring a drastic intent (the enclosing fillet
   swallows both leg carriers — the twin pin's subject) from a
   magnitude comparison; that is the exact shape the junction gate
   exists to refuse.
2. **No unit schedules until a consumer exists.** Building a
   reviewed door with no caller is the dead-code pattern this
   repo's reviews punish (the A3-3 posture, verbatim). The gate is
   explicit — a named consumer arriving — not a vague "someday";
   that explicitness is what answers the reviewer-style Q6 charge
   that "recorded as a pickup" is not a schedule.
3. **The two pins stay boundary findings with a named deliberate
   flip**, not permanent properties; their doc prose keeps the
   hedge and names the door that would flip them.
4. **`sugar.rs`'s candidate machinery has a stated purpose either
   way**: today, the substrate the boundary pins measure against;
   under the door, the construction it consumes. Not dead, not
   deleted.
5. One honest sub-question rides the future unit, not this doc:
   whether an r > R request at a σ·τ = +1 corner should refuse
   loudly naming the unbuilt door, rather than the ladder serving
   the other crossing's ordinary candidate. What the ladder
   actually serves there is a measurement, and per the F7 rule
   (candidate predicates run against the corpus before building),
   it gets measured at the unit's opening, not asserted here.

## The honest counterargument — R2: permanent no

Every consumer met so far wants ordinary fillets. The enclosing
fillet does not round a corner; it replaces the corner's whole
neighborhood, swallowing both leg carriers — arguably a different
verb wearing the fillet's name, and one nobody has asked for. A
permanent NO turns the pins into permanent properties, drops the
hedge from their prose, and closes the question at zero machinery
cost — the S71/A3-2 shape: replace a promise with a true statement.
Under R2 the substrate keeps its measured purpose (the pins), so
nothing is deleted either; the cost is that large-radius enclosing
intent becomes permanently unspellable, a capability hole accepted
by name.

**Asked of Evan: R1 as stated, or R2.** R1 is recommended because
it prices both errors asymmetrically: if R1 is wrong, an unbuilt
door with an explicit gate costs a paragraph of hedged prose; if R2
is wrong, un-ruling a ratified "never" costs a design reversal.

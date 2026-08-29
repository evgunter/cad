# The enclosing (ρ < 0) fillet tangency — ruled out; a demanding request refuses

**Status: RATIFIED (Evan, in-chat, 2026-08-29).** The ruling,
verbatim: *"we should never allow this geometry; it is not a fillet
of that corner and should be a refusal."* The enclosing class is
permanently unreachable by design — no door, shipped or future,
emits it — and a request whose radius demands it is answered by a
typed refusal, not by machinery. Issue 827 closes at the
implementing unit's merge, and this decision folds into DESIGN.md's
companion table then. (The rejected alternative — a consumer-gated
corner-authoring door — is recorded in this PR's conversation;
rejected on exactly the ground below.)

## Settled ground the ruling builds on

- The lattice door's boundary was already deliberate and pinned:
  `crates/profile/tests/review_s2.rs`'s
  `the_lattice_door_never_emits_an_enclosing_tangency` and its
  mined twin `enclosing_fillet_swallows_both_leg_carriers`; the
  suite's header derives the class algebra
  (ρ = R − σ·τ·r < 0 ⟺ σ·τ = +1 and r > R).
- The construction machinery is live and correct
  (`crates/profile/src/sugar.rs` — signed offset radii, the
  antipodal tangent-point flip, the measured-spoke projection). It
  is the substrate the boundary is defined against, exercised by
  the pins.

## The ruling's argument

Tangency with these senses is INTERNAL tangency, and for r > R the
only internal tangency between the blend circle and a radius-R
carrier puts the carrier wholly inside the blend circle: C, O, t
collinear with O between, tangent point on the carrier's far side
(the antipodal flip). The corner lies on both carriers, hence
strictly interior to the blend circle — **the arc cannot reach the
corner it was asked to blend**. In the review fixture the solution
family even has a gap: the offset traces cross only while
2·|R − r| ≥ |O_a − O_b|, so ordinary solutions end (largest
inscribed fillet, tangent at the waist), no tangent circle with
these senses exists at all for an interval of r, and the enclosing
branch opens already touching at the fixture's outermost points —
detached from the corner from its first member. A construction that
cannot touch the corner is not a fillet OF that corner; any door
emitting it would serve a wrong answer wearing a fillet's name.

## What follows (the closing unit)

1. **A typed refusal for the enclosing-demanding request.**
   Measured-first, per the corpus-before-building rule: establish
   what the lattice door actually serves today for r past the
   ordinary branch on the pinned fixtures — where the honest answer
   is "no fillet of this corner exists at this radius", the refusal
   says so, naming the bound the legs' carriers impose and the
   reachable recourse (a smaller radius). The gate site is decided
   by that measurement, not assumed here.
2. **The pins become permanent properties**: both `review_s2` pins
   drop the boundary-finding hedge; their doc prose cites this
   ruling as the reason the boundary is permanent.
3. **`sugar.rs`'s candidate machinery gets its purpose stated at
   the site**: the substrate the pins measure the permanent
   boundary against, and the natural classifier for the refusal's
   "this radius demands the enclosing class" test.

## What this doc does not decide

The refusal's exact gate site and payload shape (the unit's opening
measurement decides); anything about the 3D blend verbs
(`crates/sweep` — the S-BLEND units proper); the fate of any other
`sugar.rs` path.

# OVOID — an ellipse arc in the profile, and the spheroid (plan)

The build spec is each unit's own body; the ruling is the parent row's
(`an-ovoid-cannot-be-authored`, "Weighed" and "Cut") and `docs/DESIGN.md`
`[surfaces]`.

## The slate, in dependency order

1. **`a-spheroid-is-a-surface-kind-that-nothing-mints-yet`** (H, open):
   the variant, its chart core and poles, every other arm refusing
   typed, nothing minting it. Settles the riskiest unknown first: which
   sphere readers compose with the stretch and which do not.
2. **`a-profile-segment-is-an-ellipse-arc`** (H, open): the segment
   kind, its checks at validate, its lift; every pair with it and every
   sweep refuse typed. Runs beside 1.
3. **`ellipse-to-draws-the-stretched-circle-from-a-point-tip`** (H, on
   2): the leg, the lift back, the program form.
4. **`profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader`**
   (M, on 2 and on PROFILE's planar-reader build): wiring the segment to
   the reader Ev ruled on PR 7290. This is the critical path.
5. **`the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis`**
   (H, on 1 and 4): the mint and the at-rest readers.
6. Beside each other, on 5: **`a-spheroid-face-tessellates-within-its-own-sizing-theorem`**
   (M), **`a-spheroid-face-measures-its-closed-form-volume-and-area`**
   (M), **`a-spheroid-writes-exact-step-as-a-surface-of-revolution`**
   (E), **`a-plane-cuts-a-spheroid-by-pulling-the-plane-back-through-the-stretch`**
   (H).
7. **`the-lily-bud-is-an-ovoid-and-wall-4-is-deleted`** (M, on 3, 6 and
   8).

Off this slate: part 3 is SPLINEWALL's (`work/splinewall/plan.md`).

## Exit shape

Every unit merged: the designers' bar for parts 1 and 2 (an
axis-centred ellipse-arc revolve builds full and partial, is at rest,
measures `4/3·π·a²·c` inside its certified bound, tessellates within
its theorem, writes exact STEP, replays at every lane scalar and is cut
by a plane; every other door refuses typed). The parent row stays open
on SPLINEWALL. No exit criteria are set, so no walk is owed.

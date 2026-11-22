# SPLINEWALL — the spline leg and the exact NURBS wall (plan)

The build spec is each unit's own body; the ruling is OVOID's parent
row (`work/ovoid/an-ovoid-cannot-be-authored.md`, "Weighed" and "Cut")
and `docs/DESIGN.md` `[surfaces]`.

## The slate, in dependency order

1. **`measure-what-an-exact-nurbs-wall-of-revolution-does-today`** (M,
   open): the designer's probe on hosted CI, plus a full-turn variant;
   nothing merges. Runs at once.
2. **`a-profile-segment-is-a-rational-spline-and-the-nurbs-legs-lower-to-it`**
   (H, on OVOID's ellipse segment and PROFILE's planar-reader build).
3. **`the-revolve-mints-the-exact-nurbs-wall-where-no-kind-fits`** (H,
   on 1 and OVOID's pair-rows unit).
4. **`extrude-loft-and-sweep-mint-the-wall-of-an-ellipse-or-spline-segment`**
   (H, `design`, on 1 and OVOID's ellipse segment): the elliptic
   cylinder is weighed first.
5. **`the-egg-and-the-elliptic-prism-meet-the-bar`** (X, 24, on 3 and
   4): re-cut into units when 1 reports.

## Exit shape

Every unit merged and the parent row closed: an egg and an elliptic
prism meet the bar parts 1 and 2 meet. No exit criteria are set, so no
walk is owed.

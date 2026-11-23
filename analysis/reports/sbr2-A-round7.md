1. **Edit 2: one sentence that passes both tests.**
   > A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' far distances from that point, each read from below and from above. A direction drawn between them has no extent of its own and takes that lever.
   - It keeps "far distances", which is the bullet's own definition of a lever. So no word is defined by itself, and a far distance stays one true number.
   - It drops "alike" and attaches "each read from below and from above" to the reading, which is B's point: what is read from each side is the far distance. This matches the lever bullet's "reads the extent from below" and edit 3's "read from below and from above".
   - Neither test has to give way.

2. **The tie's witness: one sentence.**
   > A tie's witness is the edge's point at the lesser of the face's radii of curvature and the edge's arc length, each read off what the bound holds (its arc, or a line's two points); the parting is levered at that point's distance from the vertex.
   - It meets B's test: every length is in the bound's hands, the face's radii are bare numbers, and no lever is read back as a length.
   - It meets my test: it does not say "extent", which this README uses for a region.
   - The arc length needs no bound from below. `ConicArc::at` advances by `l` over the greatest speed and stops at the far end, so any length gives a point of the edge.

3. **The fan's slack: B's side. Take "with no hand-written slack" out of the text for Ev.**
   - **Test:** the text for Ev states only what is decided.
   - The admission rule covers "a short, division-free chain of operations on in-box coordinates".
   - The rim as written divides by `cos(π/8)`. It also turns a direction through products with `1/√2`, which adds a few roundings. Strictly, that is not the chain the rule names.
   - The `1.01` is also a hundredth, orders of magnitude past those roundings. So it is a hand-written slack, not a rounding charge. The phrase would claim more than the reading of the rule shows.
   - **For the unit's brief, not for Ev:** "re-pose the rim with the constant `sec(π/8)` as a product, so its chain is division-free, short and in the box, and drop the `1.01`. The face cover's far distance is already a bound from above, so no geometric slack is owed."
   - Confidence: likely. I did not count the turns' operations against `SHORT_CHAIN_ULPS`.

For Ev: as round 6, with these three sentences. Item 5's last bullet now reads: "The link's face-tie fan stays a named construction: a polygon past the face, sized by the face cover's far distance, read as a length."

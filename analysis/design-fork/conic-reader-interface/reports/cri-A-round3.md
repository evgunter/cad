## For the orchestrator

My recommendation is unchanged, so there is no For Ev section. All three answers below are the other report's position.

1. **The sentence: I sign the other report's version as written.**
   > A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

   Reason: "offsets from that point" is true for a conic held at its centre only in an obscure sense (`cos θ` and `sin θ` read as offsets from the unit-circle centre). "Two functions of its parameter" is plain, is true on both placements, and leaves those functions private as they should be.

2. **`u_ref` and `v_ref`: remove them from `Conic` in this decision, and ask the principal frame through `major_frame()` / `section_frame()`.**

   Reason: reading the seam direction is pairing it with the parameter, since `θ = 0` lies along it. A type that states the vertex's parameter enforces what the other two reports leave to convention. Deferring it would leave the general-anchor row to find every such read again.

   One thing to add to the classification: count how many of the 112 direction reads use `axis` alone (which stays) and how many want an in-plane basis with no parameter. The second kind should get a frame with no seam meaning, not `u_ref`.

3. **The order of the circle reads: yes, the condition makes it a sequencing choice, and the orchestrator's, with one amendment.**

   The caller list must cover both of `Centre`'s doors, `axis_line()` as well as the coordinate door. A `Line` carries distance readings, so a coaxiality decision can be made through `axis_line()` as easily as through the coordinates.

   With both doors listed (each caller giving its reason), a forgotten read shows up on the list. The unit that mints the first arc-held circle is then accepted when that list holds only reporters and mints, and Ev need not rule on the order.

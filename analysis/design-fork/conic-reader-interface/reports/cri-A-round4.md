1. **The sentence: I sign (y) as written.**
   - On the page it goes on: the README's own words for the placement are "held about its centre or about a point of the arc it serves with its parameter zero there".
   - In `conic.rs` the word "offsets" names only the arc-held pair (the private `offsets`: "the two offsets an arc-held ellipse's point is summed from"). So (x) would carry the README a word the module doc uses for one placement only, and `cos θ` is not an offset from a centre that is not on the curve.
   - "Two functions of its parameter" is true on both placements and under the general anchor. The gloss naming the functions belongs in the module doc, which already has it.
   - If a splice is wanted, I would also sign this one; it changes only the first clause, borrowing the README's "held about":
     > A conic's position is the point it is held about plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

2. **`u_ref` and `v_ref`: the end state is (x), and the unit that does it is the general-anchor row's.**
   - The fact that decides it is already ratified. The README holds an arc-held conic "with its parameter zero there", so the general anchor moves parameter zero off the principal vertex. From then on `u_ref` no longer means "where θ = 0 lies", and pairing it with the parameter reads wrong.
   - Until that row lands, the parameter is still the eccentric anomaly from `u_ref`, every pairing is correct, and removing them now would design their replacement ahead of that row's choice. That is the stronger argument for (y)'s timing, and I accept it.
   - So Ev is shown one line, not asked a question: "`u_ref`/`v_ref` stay for now; the general-anchor row removes or renames them (never re-documents them under the same name), because moving parameter zero changes what they mean."
   - That line goes onto `work/oneround/a-section-conic-is-anchored-at-a-general-point-of-its-arc.md` as owed.
   - The one thing that would reopen this is that row choosing to keep the parameter as the eccentric anomaly from a principal `u_ref`, with the anchor's phase kept private. Then `u_ref` keeps its meaning and could stay. Ev would rule on that in that row, not here.

## For Ev

**Recommendation (likely). Store the radius signed, read the radial, and leave the symbolic tier alone.**
- **What is stored.** An arc's radius is the document's quantity, carrying the sign its construction decided as a literal:
  - `r` for the circle verbs and for a fillet whose radius is gated positive;
  - `σ·ρ` for the tangent arc and the bulge lowering, where `σ` is the ±1 of the side or turn the verb already decided.
  No arc constructor takes `abs` of a radius.
- **How the arc is read.** A reader of an arc that records its radials evaluates `centre + radius·radial`. No reader gets a radius or a direction back from a vertex through a square root or an angle.
- **The readers that still do are re-pointed** at what the construction holds:
  - the profile's own read-back of "start on its carrier", which today computes `rim(start) − radius`;
  - the door's quotient `u_ref = (a − c)/‖a − c‖`;
  - the cylinder chart's angle at the seam vertex, `atan2(0, r)`. The sweep built that cylinder's frame from the radial, so it should state the edge's image there instead.
- **Every registration this question produced goes:** `Arc2::register_radials`, the rim facts for arcs that record radials, and the doors' per-check additions.

This follows your ruling on #3186, which spells the bulge's span `4·atan(σ·b)` from the decided turn rather than `4·atan|b|`. The radius is the one arc quantity that does not yet do this.

**This replaces my first recommendation.** I first proposed that each `Sym` session record every sign a decision proved definite, and that the tier's `abs` and `√(f²)` simplifications read that record. I now hold that only in reserve, for three reasons.
1. **The kernel already has a ruled way for a decided sign to reach the tier (sure).** On #3186 you chose the literal in the spelling over both a registration and a read in the tier. `literal_sign` (`seg.rs`) and `arc_span` use it. `Arc2::from_chord_radials` is already handed `σ` and uses it in both radials, then takes `abs` for the radius three lines earlier.
2. **You declined the nearest relative of my proposal on #3283 (sure).** That was a read that simplifies `abs` and `sqrt` wherever the argument's sign is certified over the box; you wanted a structural look first.
   - Mine is narrower: it reads only signs a decision actually took, never an arbitrary bound. That matches what `[coincidence]` already calls a structural zero: "a theorem for every value at which the construction takes the decisions it took here".
   - But it has the effect you rejected. It changes expressions across the whole document, and its zeros hold only where those decisions hold.
   - Its gains are not one-way: a rule change can lose theorems as well as add them. Route B itself cost R2's link four.
   - Here a structural answer exists, so the read has no case.
3. **With the radius signed, nothing conditional is left (likely).**
   - For the circle, `centre + r·x` is the vertex node for node, for every `r`, negative included.
   - For the tangent arc and the bulge it is a polynomial identity, because `σ² = 1`.
   - Every fact is then one function of the parameters, which is E12's strict wording. Nothing is left for a registration or a sign record to supply.

**What it makes true.** Worked example, the tour's hole:
- The vertex is `a = c + r·x`, the stored radius is `r`, and the door's `u_ref` is `x`, placed.
- The carrier at `t` is `c + r·R(t)·x`. The edge's description, `a` turned about `c`, is the same expression.
- So start, end and every interior sample simplify as algebra, with no registration. Today one side reads `|r|` and the other `r`.

By lane:
- `f64`, `Interval` and `Dual`: not a bit changes wherever the gate decided positive, since `abs` of such a value is the value.
- `Sym<f64>` and `Sym<Interval>`: the expressions change, and nothing else does.
- Eager and deferring sessions: they agree, because nothing is registered or read.
- A box that straddles the band: the gate refuses as today, and the driver splits the box.

**What it leaves possible.** A future reader could again take `√` of an `r`-scaled quantity and need the sign. Nothing in the code prevents that; only review catches it. The reserve is the answer if such a reader appears with nothing structural to read instead (unsure how often that will happen).

**Final states compared:**
- **Signed radius only (recommended):** identities everywhere, no tier change, three readers re-pointed.
- **Session sign record only:** magnitudes stay stored and every reader simplifies, including the ones that divide. E12 would have to admit conditional premises. It is the route #3283 declined in kind, and its effect on documents is unmeasured and can go either way.
- **Both:** the signed radius makes the record unnecessary for arcs. The record would stand only as a reserve behind a switch left off, which is where #3283 left it ("a structural look before any read is proposed again").

**Ratified text.** No decision changes. One wording in E12:
- Its Decision paragraph says a zero holds "for every parameter value". `[coincidence]` and `Sym::register_equal`'s doc say "wherever the construction takes the decisions it took".
- E12 should say which it means. A conditional registration (`register_radials`, "wherever the door decided `r` positive") was written under the strict wording, and nobody could tell it was outside it. Under this recommendation arcs need only the strict form (likely).
- E12's passage on the plate's chart angle (`atan2(0, r²/sqrt(r²))`) describes the quotient `u_ref`. It is re-worded with the code, as description.

**Confidence:**
- Recommendation: likely.
- The circle and the hole simplify with no registration: likely. Derived by hand, and it depends on the vertices and `Arc2::on_radial` building the same node.
- Bulge and tangent identities through `σ² = 1`: likely.
- The reserve is not needed for today's documents: unsure until the doors' measurements arrive.

## For the orchestrator

1. **What moved me.** A's precedent argument, checked in the tree:
   - `rule-d-reaches-the-unit-bulge-only.md`, "Route B": the ruling chose the spelling over `register_equal(abs(b), σ·b)` and over rule C;
   - `the-apothems-sign-is-a-value-read.md`: #3283 declined the narrowed rule C;
   - `seg.rs` `read_back_arc`: the site A names, `Reading::Difference(arc.rim(a) - arc.radius)`, exists.

   What did not move me: the record is still sound. What A answered is the case for building it, not its soundness.
2. **A's four points against my first report:**
   - (i) True.
   - (ii) My record differs in what it reads (decisions taken, not bounds) but is the same in effect: zeros that hold only on the branch, everywhere in the document, with gains that can reverse.
   - (iii) True as a class. I did not verify A's "ten" on R1's boss; the route-B entry in `rule-d-reaches-the-unit-bulge-only.md` records four lost on R2's link.
   - (iv) True for the circle by node identity, provided the circle kernels build the vertex with the same operand order as `Arc2::on_radial` (`center.x + radius * c` against `centre + radial * radius`). If not, it is still a polynomial identity and still simplifies, just not node for node. True for the bulge and tangent arc by polynomial identity.
3. **A correction to my first report.** I said E12 needed a new premise clause. `[coincidence]` (DESIGN, the structural paragraph) and `register_equal`'s doc already state the conditional form; only E12's own Decision paragraph disagrees with them. I withdraw the `[intent.structural]` change.
4. **A correction to A.** A wrote "eight radial facts per arc"; `register_radials` returns four per arc, so eight only holds if a placed copy is counted too. I found nothing else in A's report false against the tree.
5. **Implementer notes beyond A's:**
   - `Arc2::from_chord(..)` passes sign `1`: drop it, or make it take the turn.
   - `Arc2.radius`'s doc should say it is the construction's signed quantity, positive where the constructor's gate decided.
   - What the doors' measurements would overturn: the recommendation, if storing `r` and re-pointing the read-back still leaves start, end or mapped-source unsimplified; the reserve's case, if the untraced `pcurve_map_residual` turns out to be a reader with no structural alternative.

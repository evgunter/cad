## For Ev

**Recommendation (likely).** Two layers, and I recommend the first now and the second as its own measured question.

1. **The spelling (both designers agree on its core).** An arc's radius is the document's quantity carrying the sign its construction decided, as a literal: `r` for the circle and the fillet, `σ·ρ` for the tangent arc and the bulge lowering (σ = the literal ±1 the verb already holds). No `abs`. The verb registers nothing about the sign. Readers of an arc evaluate `centre + radius·radial` and, where they check a point against the circle, use the kernel's existing sign-free residual `(d² − R²)/(2R)`; none takes `‖vertex − centre‖` and compares or divides by it. This is your route B ruling (#3186, `4·atan(σ·b)` not `4·atan|b|`) applied to the radius.
2. **A record of decided signs in the symbolic tier (the other designer's proposal; unsure).** When a decision proves a quantity definitely positive or negative, the session keeps that, and the tier's `abs`, `√(f²)` and `atan2(0, f)` simplifications may use it. It is sound and I no longer call it a second kind of fact. I would not bundle it with this change; reasons below.

Terms. *Symbolic tier*: the certifier's algebra that proves a residual zero as an expression in the document's variables, at any box width. *Box*: the range a study gives a variable; a *leaf* is one piece of it the drive certifies. *Gate*: a decision whose other answers refuse. *Rim*: `‖vertex − centre‖`. *Radial*: the unit direction centre → vertex the profile records. *Registration*: a site telling the tier two expressions are equal, an axiom resting on that site's proof (E12).

**Premise check.**
- The kernel already has a ruled way for a decided sign to reach the tier: a literal in the spelling (`arc_span`, `literal_sign`). The radius is where it is not used; `Arc2::from_chord_radials` is handed σ, uses it for the radials and takes `abs` for the radius. (sure)
- `|r|` is stored for the certifier's syntax (the verb's comment says so); at `f64` and `Interval` it is `r` bit for bit. (sure)
- With `r` stored, `centre + r·x` is the vertex at every value of `r`, so the fact is unconditional and needs no registration. (sure for the circle; likely for the bulge and tangent arc, worked by hand)

**What moved me, and what did not.**
- *Moved.* A definite verdict at the interval lane is a proof about every point of the leaf's box, the same class of premise the tier already uses when it says a denominator is non-zero "wherever the expression has a value". So the record is a proof, not an axiom, and needs no allowlist. I had under-counted the readers: about ten production sites read the rim at an arc's own vertex (the profile's read-back and on-carrier checks, the radial minted for an arc that records none, `carrier_end`, two sweep sites), not three. And a table arc that records no radials has nothing but its points and radius to read, so "read the radial instead" does not cover it.
- *Not moved.* Each of those ten has a sign-free form that reads only what it already holds: a difference `rim − radius` becomes `(d² − R²)/(2R)`, the form the kernel's cylinder and sphere residuals already use; a quotient `(a − c)/rim` becomes `(a − c)/radius`. So storing `r` does not "move the gap" to the dividing readers once they divide by the radius. The claim that cylinder radius checks read the rim is not what the tree shows: they are the squared form. (sure)
- *Not moved.* For the tangent arc and the bulge the other proposal keeps `abs` and relies on the tier reading signs factor by factor through a quotient, with a verdict recorded on each factor. The bulge's sign is read by a spelling choice (`literal_sign`), not obviously a recorded verdict, and that factor-wise fold does not exist. The literal σ is already in hand at both sites. (likely)

**The record, weighed.**
- For: it closes every reader at once, including ones not yet written and the chart phase `atan2(0, r)` (my guess at the one untraced refusal), which the spelling alone answers only by having the sweep door state the edge's image. The rim is the natural thing to write, and a rule against writing it is a discipline, not a type.
- Against, as properties of the finished code:
  - Which Zeros it proves depends on which decisions were taken before the question is asked. An eager session asks at once; a deferring session asks later with more signs recorded. So the two can differ in what they prove (never in soundness). The spelling has no such dependence.
  - Today the tier is consulted only where the numeric check is not definite. Recording the expression of every definite decision adds tier work to the common case unless it is made lazy; the earlier sign read cost +39% on the bracket's leaf (#3282).
  - Its reach is complete for an authored radius and unproven for derived ones.
  - It changes E12 and `[intent.structural]` (what the `unproven-coincidence` lint stays quiet about).
  - You retired rule C, declined the certified-sign read on #3283, and asked for a structural look before another read is proposed. This record is narrower than both (it uses only a sign a decision actually proved), but it is the same family.

**The three final states.**
- *Spelling only.* Verbs store `r` / `σ·ρ`; about ten rim readers move to the squared residual or divide by the radius; `register_radials`, the rim registrations and the doors' added ones retire; the tier and ratified text are untouched. Leaves possible: a future reader that takes a root of an `r`-scaled quantity refuses over a wide box until re-posed, and the chart phase needs the door to state its image.
- *Record only.* Circle and fillet store `r`; the rest keep `abs`; readers stay as written; correctness of every arc over a box rests on the record firing, including the unbuilt factor-wise fold. The spelling the verbs hold (σ) goes unused.
- *Both.* Spelling as above, readers re-posed or not as convenient, the record as a net under them. Best coverage; carries the record's order dependence, cost and text changes.

I recommend spelling only for this item, and putting the record to you separately with its measurements (what it proves and re-labels on the measured documents, its cost, eager against deferring), as #3186 and #3283 were put. If the doors' measurement shows a reader that needs the sign and holds nothing sign-free to read, I would take both. Confidence: spelling `likely`; leaving the record out of this change `unsure`.

**Ratified text.** Under spelling only, no decision changes. E12 says a symbolic Zero holds "for every parameter value" and a registration is "one function of the parameters"; DESIGN.md's structural-coincidence clause says "for every value at which the construction takes the decisions it took here (E12)". They differ, and E12 should say which it means whichever layer is taken. E12's description of the plate's chart phase is re-worded with the code.

## For the orchestrator

**1. What moves me.** Above. Net: I withdraw "second source of sign" as an objection and my count of three readers; I keep the recommendation's order.

**2. B's three claims.**
- (i) True of readers that divide by the rim; false once they divide by the stored radius, which is what a reader holding centre, radius and a vertex can do.
- (ii) Partly true: the rim is read at ~10 production sites on `boxfinish/doors-read`. By site, with what each holds:
  - `seg.rs` on-carrier (≈632) and the pair at ≈702/710: point, centre, radius → squared residual.
  - `seg::read_back_arc` Start and ThroughPoint (≈1059, 1087): same.
  - Its Decided-landing projection (≈1071) and the foot at ≈2698: `(p − c)·radius/rim`, a genuine projection of a point not on the circle, a real margin; leave.
  - `mint_radial` (≈913, used ≈895 and `validate.rs` ≈1439): `(a − c)/radius`.
  - `Arc2::carrier_end` (arc.rs ≈244) and `register_endpoints`' rim pair (≈331/335): retire for arcs with radials, radius-quotient otherwise.
  - `swept.rs` `register_rigidity` (≈744): retire if nothing reads the placed rim.
  - `swept.rs` ≈1529 (radials from quotient): take recorded radials or radius-quotient (I did not read its context).
  - `seg.rs` ≈1289, 2136, 2376 read the rim at OTHER points (`q`, `foot`): real distances, not identities.
  - "Cylinder radius checks" is false: `geom-brep/src/implicit.rs` `implicit_residual` is `(|w|² − R²)/(2R)`. The door's `u_ref` no longer divides on doors-read.
  - Angle readers: one, the chart phase (my unverified guess).
- (iii) Agreed as soundness.

**3.** In For Ev.

**4. Tree claims.**
- B: record "at `sign_within`, which every decision passes through", storing the margin's form digest. The numeric channel answers first and forms are lazy (`deferred.rs`: a symbolic Zero "is only ever offered where the numeric channel did not call the sign definite"), so this computes a form per definite decision unless node ids are recorded and resolved on demand. Not false, but a cost B does not state.
- B: "Deferring sessions … record identically. Discharging a deferred Zero then reads the record." At discharge the record holds decisions taken after that Zero; an eager session asked with fewer. Reach differs between the two; B presents them as the same.
- B: PR 6993 "untouched". Its structural question is specified as "a function of the expression DAG alone" with value-reading rules off; a record of verdicts is path-dependent, so it would have to be listed among the rules switched off there.
- B's fillet "if" is answerable from the tree: `fillet_arc`'s doc says "the authored radius as its magnitude `|r|`, as `circle_loop` stores it".
- B's bulge item says "the caller already reads `x`'s sign": via `seg::literal_sign` → `k_stats::choose`, which I did not confirm passes through `sign_within`; and it returns `None` at a tie.
- Mine shown short by B's reading: the reader count, and that table arcs without radials have no radial to read. Nothing I asserted about the tree was shown false.
- B's report does not cite #3186 or #3283, and its alternative list omits the certified-sign read Ev declined, which is the nearest precedent for the record.

**Measurements.** Unchanged predictions:
1. The untraced residual is the chart phase `atan2(0, r)`. B predicts `|r|` vs `r` through the cylinder's radius, which the probe's `|r| ≡ r` registration should already have closed.
2. With `r` stored the first refusal is the verb's own read-back Start read, a rim reader. B and I both predict this, so it does not by itself discriminate. What discriminates is whether, with the rim readers in squared or radius-quotient form, anything is left that needs the sign.

## For Ev

**Recommendation (likely).** Read a shell's role, outer boundary or cavity wall, as the side of infinity its faces leave. Read it with one function that every caller shares. That function reads the certified volume sign first, where the sign is definite. Otherwise it casts a ray from outside the shell, along the inward normal of one of its faces, and reads the nearest crossing. Check 7, D1's orientation check, moves onto the same function, so an inside-out thin body refuses at its own door. D10's "in-band shell" clause is deleted: once the role is read this way it adds no test beyond the pairs.

**Terms.**
- **Side at infinity:** whether a point far from the faces is inside or outside the material they bound. Outside means the shell is outer; inside means it is a cavity.
- **Aimed ray:** a ray through a point strictly inside one face, read to its nearest crossing.
- **In band:** a reading whose margin is inside the sliver band at this ε (the tolerance), so a smaller ε might decide it.

**The reader, reader by reader (likely).**
- **Volume first, then the ray.** A certified sign is a certified orientation, so wherever the volume decides, it decides correctly. It also covers faces the ray cannot cross yet (NURBS). When nothing decides, the refusal names the ray's reading first, and the volume's in-band V/A only where no ray could be cast.
- **Aim direction.** Each ray is aimed along the inward normal of the face it targets, from beyond the shell's reach, not from one fixed far point. So at the aimed face the ray crosses head-on, and the only margins left are distances between crossings.
- **Why that removes the in-band shell test.** The distance along a ray between two crossings is at least the distance between the two faces. So an in-band reading here implies an in-band pair, which the boolean already refuses.
  - A ray from one fixed far point can fail without any thin pair. Example: a slab lying in the plane x = y, with the far point on the hull's (1,1,1) diagonal. That point sits in the slab's own plane, so every ray to its big faces grazes them, and the ray reads in band although no pair of faces is.
- **Check 10, the pieces sort, `shell.rs`'s cavity read, the census's role gate, `classify_shells`, and STEP export** all call this one function.
  - `brick ∖ slab(15ε)` builds, with the cavity read as a cavity.
  - `slab ∪ far brick` sorts into two solids and builds, in either order.
- **A solid's only shell is outer by arity.** Nothing reads it, as D10 already says and `shell.rs` already does. Its orientation is check 7's job, and check 7 now uses the same reader. So editor-core's component count on a thin lone body works.

**Worked example.** `brick (−1,2)³ ∖ slab(15ε)`, ε 1e-9.
1. The cavity's V/A is −7.5e-9, which is in band, so the volume does not decide.
2. A ray comes down from above along the inward normal of the cavity's top face. It crosses that face first, and there the face's outward normal points along the ray, away from where it came from. So the start reads as inside: infinity is inside, and the shell is a cavity.
3. The next crossing is the bottom face, 15ε further. That is the top–bottom pair the census already reads as definite.

**Check 7 (coupled, likely; it is your call whether to take it now).** Today D1's orientation check reads V/A for the whole solid and exempts the band. So an inside-out slab 10–19ε thick finishes as validated, and the next operation's operand gate catches it by ray and blames that operand.
- **Option 1 (I lean here):** check 7 reads the same function. The inside-out slab refuses at its own door, and three readers of one fact become one. `AtRestBody::slivers` and `gate_sliver_orientation` retire. D1's "zero and escalated exempt" becomes "a solid whose side no reading decides refuses". Its stated reason, that tightening ε never flips valid to invalid, is already false under V/A.
- **Option 2:** leave check 7 for now. That is consistent too, and Option 1 is still one call away later.

**D10 text.** Delete "and no in-band shell, a shell of a solid of several shells whose role is certified in band". Keep "a solid's only shell is its outer shell and poses no role question".
- **The other version** keeps the clause, reworded as "whose role no reading decides", as check 10's completeness condition. That reads the same in substance, but it would label a capability limit (no face a ray can cross) as the operands' ill-conditioning (`Escalated`), which it is not. Check 10's completeness belongs in check 10's own doc.
- **Wording choice for you:** delete, or reword with the limit carved out.
- **Provenance:** both D10 phrases are text agents wrote that you approved in passing on PR 4690 ("sounds good!", commit `a865953bf`).

**Residue (sure).** Aim points exist only on planar faces today. So a thin shell with no planar face, such as a lens made of two sphere caps, still refuses on its V/A, and the refusal says so.
- The design is "aim at any face the ray can cross". Curved aim points are CLEAVE's open row, so this is a question of order, not a second design.
- The pairs-only claim is fully true only once that row lands.

**Confidence.**
- Recommendation: likely. Role = side at infinity: sure.
- An in-band aimed reading implies an in-band pair: sure for the order margin under any aim; likely for the normal-aimed scheme's angle margin. It fails for the fixed far point (the slab in x = y above): likely.
- The worked example: not measured, so unsure until it is pinned.

## For the orchestrator

**Point by point against the other report:**
1. **Volume first:** moved. The NURBS argument is real: without the volume, a spline face no ray can cross would lose its role read. My "never decides" was wrong. The refusal ranking stays ray-first.
2. **The clause:** not moved. Rewording routes capability limits to `Escalated`. My argument is not answered. In part this is Ev's preference.
3. **Check 7:** moved. I had left it to a separate fork. The three-readers argument and the false "never flips" reason convince me. I give it as an option because it touches D1.
4. **Curved aims:** partly moved. It is sequencing to CLEAVE's row. I keep it in the final state's description.
5. **Only shell by arity:** not moved. With check 7 on the shared reader, reading a lone shell again duplicates check 7, and D10 already says arity.
6. **Band claim:** I correct my round-1 statement. The 1/|cos θ| stretch makes the order margin larger, so it never breaks the implication. The fixed far point's real flaw is the angle margin at the aimed face (the slab in x = y). That part of the other report's scheme does not hold. My worry about the far point is answered on the order margin and stands on the angle margin.
7. **Rows neither report named:**
   - EXPORT `step-export-reclassifies-shell-roles-with-its-own-planar-flux`: closed by the export calling `classify_shells_of`, which now reads through the shared reader; its own `volume.rs` flux and the raw f64 sign go. That makes slab-cavity bodies exportable too.
   - TALLY `the-shell-role-refusal-locates-no-piece`: mostly retires. An in-band role reading becomes a pair refusal, and locating a pair is the pair refusal's own concern. Re-scope it to the V/A residue.
   - TALLY `…quotes-the-walks-f64-margin…`: narrows to the residual volume path, and its fix (quote the certified reading) still applies there.

**Probes to pin, not measured:**
- `brick ∖ slab` and `slab ∪ far brick`, at t ∈ {10, 12, 15, 19}ε and 9.9ε, at ε 1e-9 and 1e-6. At 9.9ε they still refuse, now on the pair.
- An x = y slab under the far-corner scheme, to confirm the graze.
- Check that `Margin::levered` on the ray's d·n uses a band consistent with the census's pair band.

**Rows that close** under check-7 Option 1: RESTFRONT `an-inside-out-slab-thin-enough…` and FLUSH `an-inside-out-sliver-result-passes-its-own-door`. Add this fork as evidence to CLEAVE's planar-aims row.

**Off the question:**
- `shell_winding_errors`' doc ("silent", "not counted") contradicts its `ShellRoleUndecided` push.
- `pieces_of`'s quiet return should say that check 10 is the door that answers for an unread role.
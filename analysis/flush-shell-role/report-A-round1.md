## For Ev

**Recommendation (likely):** read a shell's role (outer boundary or cavity wall) off one ray aimed at the shell from outside it, and stop reading it off the sign of its volume. Then a role can only read in band where a pair of faces does, so D10's "in-band shell" half adds nothing and should be deleted, leaving thinness judged by pairs alone. A solid with one shell reads no role anywhere: that shell is its outer shell.

**The premise, corrected.** "Is the role certified in band" is the wrong question. A role is an orientation: whether the shell's stored normals point away from the region it encloses (outer) or into it (cavity). Orientation has no thickness. Today it is read through a magnitude, V/A (signed volume over area, an average half-thickness), so a thin shell reads in band whatever its geometry. That ruler is the one PR 4690 retired for thinness, and it has survived in the role reader. The bug is the reader, not where the line between "role in band" and "role decided" is drawn.

**The reader (A, recommended).** Choose a point well inside one face of the shell. Cast a ray toward it from beyond the shell's reach, along the face's normal there. Read the nearest crossing with the shell's own faces. The ray starts outside, so the sign of d·n at that crossing (d the ray direction, n the face normal) is the role: d·n < 0 means outer, d·n > 0 means cavity. Every margin this ray reads is one of two kinds:
- **Transversality** (d·n at a crossing). At the aimed face it is about ±1. Elsewhere a graze just means trying the next aim point.
- **Order between two crossings.** The distance along the ray between crossings on two different faces is at least the distance between those faces. So it reads in band only if that pair of faces is in band.

The ray therefore fails only where a pair has already failed, or where there is no face it can aim at. That is why the "in-band shell" test reduces to pairs.

- *Worked example.* Brick (−1,2)³ minus a slab 15ε thick. The cavity's top face has its normal pointing down, into the slab. A ray from far above aimed down along it crosses that face first, with d·n = +1, so the shell reads as a cavity. The next crossing is the bottom face, 15ε further on. That is the same distance as the slab's top–bottom pair, which the census already reads as definite (the slab alone builds). So the shell reads as a cavity and the result builds. Slab ∪ far brick: the slab reads as an outer shell, the sort moves it into its own solid, and the result builds as two solids.
- *What a refusal names.* When no aim point decides, the refusal is the ray's own evidence. Either it is an in-band pair, named by its two faces and their gap (the same `Escalated` pair refusal the boolean already makes), or it is a capability limit (no face the ray can aim at or cross), typed as a limit. Neither is a "too thin shell" verdict.
- *One reader for one fact, every caller:* check 10, the result's pieces sort, `shell`'s cavity read, the census's `gate_role`, `classify_shells`, the missed-ray side in `solid_contain`, and CLEAVE's sliver-orientation operand gate. The last two answer the same question today with a volume-then-aimed-ray fallback.
- *Needed with it:* aim points on curved faces, not only planar ones (a point inside the face's chart, aimed along its normal). The ray walk already crosses cylinders, cones, spheres and tori. This closes CLEAVE's open row that says the aimed ray offers planar faces only. Spline faces stay a limit refusal, as they already are for containment.
- *`classify_shells`:* a solid's only shell is outer by arity, with no read (`shell.rs` already does this). That fixes editor-core's component count on a thin lone body. Volume and area are still reported beside the role, but no longer decide it.
- *Check 7 is unchanged:* it stays the orientation probe D10 ratified, and its in-band exemption still covers a thin lone body.
- *Reversible:* yes. It is one reader behind one function.

**B: volume first, ray only where the volume is in band** (the shape `missed_side` has today). This is sound, because a certified volume sign is a certified orientation, and it can be cheaper on fat shells. But it keeps two certificates for one fact. Which one appears in the verdict log then depends on thickness. And the refusal still has to come from the ray, so the volume becomes a cache that is never the reason for anything. I'd choose this only if measurement shows the ray costs noticeably more. Equally reversible.

**C: keep V/A and keep the "in-band shell" clause** (status quo). Rejected. It refuses definite geometry (all three measured cases at 10–20ε) through the average-thickness ruler PR 4690 retired.

**Not an option: carry roles through a boolean by lineage** instead of reading them. A union of two C-shapes turns two outer shells' faces into a cavity wall, so lineage does not determine role.

**Ratified text I would change.** In D10, "Booleans", I would drop "and no in-band shell, a shell of a solid of several shells whose role is certified in band". I would keep "a solid's only shell is its outer shell and poses no role question", and add that a shell's role among several is read off an aimed ray, so it is in band only where a pair is. Provenance: commit a865953bf (fork log row 113) wrote it, and you approved it on PR 4690 ("sounds good!"). The wording was written by agents, and it defined the term so that the V/A reader stayed as it was. Changing it is a design choice, so it needs your sign-off.

**Confidence:**
- Recommendation: likely.
- "Ray order margin ≥ the pair's census distance": likely. It holds for planar pairs. It is unchecked for how the census measures curved pairs, and it assumes the ray's order decision and the census use the same band.
- "The three measured cases build under A": unsure. Argued, not measured.

## For the orchestrator

- **Not measured.** I ran no probe. Before the `[ev]` PR, measure a single-shell `SolidFaces` selection read by `aimed_side`, with `from` set along the face normal, on the brick∖slab cavity and on slab ∪ far brick, at t ∈ {10, 15, 20}ε, ε 1e-9.
- **Compare the two bands.** Check the band `bool_point_in_solid_order` decides on against the census pair band. If the order decision's band is wider, the claim that A reduces to pairs fails at the margin, and the fix is to share the band.
- **Today's aim directions are wrong for this use.** `aimed_side` aims from a fixed `from` on the missed ray. A role reader should aim along each face's normal from beyond the selection's reach. Otherwise d·n can be small and the order distance gets stretched by 1/|cos θ|.
- **The brief's list of readers is short by four.** It misses `solid_contain` `at_infinity_side`/`missed_side`, `reduce.rs` `gate_sliver_orientation` (CLEAVE row `a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face`, finding 1, which this design closes), `classify_shells_structural`, and `role_at_target`.
  - For a validated solid's whole face set, the side at infinity is Out by validity, so `at_infinity_side` matters only for shell-scoped selections and inside-out detection.
  - That row's finding 2 (the silent `continue` on `SolidFaces::of`) is separate and still stands.
- **What retires under A:** `InBandShell`, `CertifiedSliver` on `ShellRoleUndecided`, `SHELL_ROLE`'s "thickness" size, and `BooleanDecision::ShellRole`'s V/A margin text in `refusal_routes.rs`. `shell.rs`'s `unreachable!` stays sound only if check 10 and the classification share the one reader, so keep them on it.
- **Doc drift in `validate.rs`.** `shell_winding_errors`' doc says a shell whose role is undecided "is not counted" and the check is "silent". The code pushes `ValidationError::ShellRoleUndecided`. Worth an issue, or fix it with the change.
- **Check 7 is left alone.** Folding it into the same reader would be a second fork, because D10 ratifies it as an orientation probe.
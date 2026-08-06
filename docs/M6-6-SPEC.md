# M6-6 spec — the curved sense-flip tier gate (binding)

Mandate: the RATIFIED unit (Evan 👍 on #184's triage) — the
curved counterpart of tier 3's loop-role gate. Substrate: the
executed inventory at `~/.local/share/cad-work/m6-6-substrate/
inventory.md` — read FIRST. Its truth table is the unit's
justification, sharper than the ratifying thread knew: NO gate
catches any single-face curved sense flip (cylinder, cone,
rim-bearing sphere, torus all bit-identical; rimless sphere
Zero-exempt), and fully inside-out washer/cone/donut/lily
certify GREEN with positive volume. This spec is binding;
deviations REPORTED, never improvised.

## 1. The gate

Factor the props lanes' material-side derivation into a public
`boundary_material_sign(surface, outer_loop, band) ->
Encoded(sign) | Unencoded` (geom-brep props — the s_f
sub-derivations at curved.rs:294/:641/:839 reuse it; behavior
byte-identical, the factoring proven by the census). A CURVED
arm of check 6 in `validate_geometric`: for each curved face,
compare the derived material sign against `face.sense_sign()`;
definite disagreement refuses (the curved LoopRoleInverted
sibling, own variant + honest message per the S11 voice);
escalation/Zero posture inherited (the rimless V=0 ball stays
exempt — documented residual, gap 1). Fires before check 7.
**No comparand exists** — the check is combinatorial (two exact
±1s); the sign derivation reuses the already-length-metered
named decides (props_rim_side / props_circle_axis_class /
props_meridian_orient), satisfying the margin convention by
reuse. S11's honest `.F.` faces (washer bore, die_pips dimples)
MUST pass — that is the gate's defining negative control.

## 2. Import parity rider (step-import, small, executed gap)

The torus normalization's inversion check (normalize.rs:582)
has no cylinder/cone counterpart — flipped cylinder and
cone_apex files IMPORT GREEN today (substrate-executed). With
the kernel gate live these imports go tier-3 red instead;
adoption must surface the same typed story: extend the
normalization inversion check to the cylinder/cone forms (the
torus check's pattern), so import refuses pre-body with the
orientation-inverted diagnosis rather than building a body the
tier ladder then rejects. The torus refusal COEXISTS with the
kernel gate (it fires pre-body; import returns certified
bodies) — only its justification text updates ("refused until
the kernel can hold…" retires; the refusal stands on the
pre-body ground). The sphere arm keeps NegativeVolume/the gate.

## 3. Pins (the substrate's matrix, verbatim)

Native, via `flipped_face_sense_for_tests`, both directions
per kind where the corpus offers them: washer outer T→F AND
bore F→T (cylinder); notched concave/convex; cone() lateral +
lily pucker (cone T→F; the F→T direction door-verified or a
countersink minted — implementer's call, reported); lily zones
+ die_pips dimples (sphere both directions); ball (rimless
residual pinned AS residual); donut (torus). Whole-body
inversion: washer/cone/donut flip from certify-green to the
new refusal (the headline pins); ball stays NegativeVolume.
Import rows: the f6 unflip and a1 controls flip to their
anticipated arms; cylinder/cone files refuse pre-body after
the rider. Planar check-6 control unchanged.

## 4. Constraints

Margin convention (no new comparand — assert none appears);
S9 for any retiring justification text; the M6-3-era claims
about sense handling get the stale-claims sweep if any go
false; fail-loud voice. Local battery: geom-brep/topo/
step-import targeted rows foreground; hosted CI per the
Actions-outage posture (local is the reporting gate; hosted
re-verifies on recovery — the standing waiver).

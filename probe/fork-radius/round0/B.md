## For Ev

**Recommendation (likely).** When a gate decides a value definitely positive or negative, the symbolic tier should know that sign. It is proven, not registered.
- **The mechanism.** When the numeric check decides a margin `m` Positive or Negative inside a `Sym` session, the session records `m`'s sign. The tier's `abs` simplification reads that record alongside its existing test ("manifestly positive"). Rule G already turns `√(f²)` into `|f|`, so `|r|` and `√(r²)` both become `r` wherever a gate decided `r > 0`.
- **What arcs store.** The radius the construction means: the authored `r` for the circle verbs and the fillet. `abs` stays only where the construction's own value can carry either sign (the tangent arc's `ρ`, the bulge's signed radius), and the same record simplifies it there.
- **What retires.** No verb, door or reader registers `|r| ≡ r` or anything derived from it. `Arc2::register_radials`, the rim half of `register_endpoints`, and the registrations the doors' lane added check by check all go.

**Premise check.** The item says the fact "holds only on the valuations a gate accepts". It asks whether E12's registration covers it: an axiom saying "two DAG nodes denote one function of the parameters". Both framings are slightly off.
- **The fact is not conditional, and not an axiom (sure).**
  - At `Sym<Interval>`, Positive means the certified bounds on `r` over the leaf's box lie above the band. So `r > 0` at every point of that box, whether or not the verb goes on to accept.
  - At `Sym<f64>` the session is a single point, and the verdict is a fact about that point.
- **The tier already reasons this way (sure).** Its theorems hold at "every point of the box". Rule E's soundness rests on "clause 1": wherever a denominator vanishes, the numeric channel divided by zero and the box was already refused (see the header of `sym/quotient.rs`; `sym/manifest.rs` argues the same way). A decided sign is a second certificate of that kind: the numeric channel's proof about the box, used as a premise.
- **So a registration is the wrong tool.**
  - `|r| ≡ r` is not one function of the parameters: it is false for `r < 0`.
  - The registration check cannot catch a misuse, because the two sides' bounds always overlap.
  - Each verb would restate by hand what its gate already proved, so one fact would be stated in two places ([one-way]).
  - Registering `|r| ≡ r` at each gate would work leaf by leaf, but it is exactly that duplication. The doors' lane was already adding one registration per refusing check.

**What the issue is evidence of (likely).** The defect is not the `abs` in `circle_loop`. The tier throws away a sign the numeric channel has already certified. Fixing it at the verb (store `r` and register) only moves the gap:
- It fixes the readers that multiply the radius by a direction, where the vertex is `c + r·x`.
- It breaks the readers that divide by the rim, such as `u_ref = r·x/√(r²)`.

Only a fix in the tier fixes both, because `√(r²)` then becomes `r`.

**The final state for each reader**, over a wide box on `r` with the gate Positive:
- **Rim.** `‖a − c‖ = √(r²)` becomes `|r|`, then `r`, which is the stored radius. It is a theorem, so the rim registration retires.
- **Door's `u_ref`.** `(a − c)/rim` becomes `r·x/r`, which becomes `x` (rule E cancels the common factor). It is exact, so it no longer matters whether the door reads the rim or the recorded radial.
- **Recorded radial.** `c + r·x` is the vertex exactly as it was built, so `register_radials` retires.
- **Edge description against its carrier.** With `a − c = r·x`, `a + (R − I)(a − c)` is `c + r·R·x`, which equals the carrier. `carrier_matches_mapped_source` and `carrier_endpoint_*` become identities (likely).
- **Pcurve map.** I predict the same `|r| − r` in another form (unsure; it is untraced).
- **Placed copy.** Placement is linear, so the carrier is unchanged.
- **A box that straddles `r = 0`.** The gate is undecided and the driver splits the box, as today. Each piece decided Positive simplifies.
- **By lane:**
  - `f64` and `Interval` without `Sym`: nothing changes. `abs` of a value certified positive is that value bit for bit.
  - `Sym<f64>` and `Sym<Interval>`: the record is made at `sign_within`, which every decision passes through.
  - Deferring sessions: they take every decision numerically, so they record identically. Discharging a deferred Zero then reads the record.

**The other sites that store a magnitude (likely).** For these the simplification has to read signs factor by factor across a quotient: `|N/D| = N/D` when every factor of `N` and `D` has a known sign, either visible in the expression or decided. Non-negative is enough for `abs`. Strictly positive matters only for `copysign`'s signed-zero edge, which `manifest.rs` already handles.
- **Bulge** (`Arc2::from_chord_radials`): `|L(1+x²)/(4x)|`. `L` is a square root, so non-negative; `1+x²` is positive; and the caller already reads `x`'s sign.
- **Tangent arc** (`rho.abs()`): `ρ = (d·d)/(2·across)`, and the side `σ` decided the sign of `across`. Same rule.
- **Fillet** (`carrier.radius.abs()`): if that value is the authored radius under a positivity gate, store it without `abs`. If it is a signed offset, it keeps `abs` and simplifies.

**Ratified text to change (needs your sign-off).**
1. **E12 "Soundness" and "Mechanism".** Say that a symbolic Zero is a theorem at every point of the leaf's box, resting on the numeric channel's certificates there: clause 1 (defined everywhere) and the session's definite verdicts. The Decision paragraph's "for every parameter value" already claims more than clause 1 allows. Add that a decided sign is a certificate, not a registration, so it needs no allowlist entry.
2. **`[intent.structural]`** currently says "zero as a polynomial in the document's variables". With decided signs as premises, a coincidence can count as structural *on the branch the document's verdicts take*. That is the honest reading, since off that branch the document does not build. But it widens what the `unproven-coincidence` lint stays quiet about, so it is your call. I think it is right (likely), but I am unsure whether you want it.
3. **The circle verb's doc comment and `Arc2`'s "radius (positive)"** are agent-written and not ratified (the circle text came in through PR 6890), so they change with the code.

**Alternatives weighed.**
- **(B) Register `|r| ≡ r` at each gate.** It works leaf by leaf, but it uses an axiom where a proof exists, restates the fact in every verb, and stretches E12's "one function". Rejected.
- **(C) Store `r` and have no reader use the rim**, only the recorded directions. The rim is the natural reading, and endpoint facts, cylinder radius checks and the door all use it. This also leaves the bulge and the tangent arc unsolved. Rejected.
- **(D) Declare radius variables positive in the document.** This fails for derived radii such as `w/2 − t`, and it restates what the gate decides ([intent.statements]). Rejected.

**Reversibility.** The record is a new `SymRules` switch, like `registered`. Turning it off restores today's tier, and comparing the two measures what the record buys.

**Confidence:**
- The recommendation: likely.
- A decided sign is a sound premise: sure, provided each leaf has its own session.
- The cut plate stops refusing on `u_ref` and on the `|r| − r` checks: likely.
- The circle needs no other registration: likely.

## For the orchestrator

- **Where the hook goes.** `Sym::sign_within` / `decide_sign` (`sym.rs`) records the margin's form digest and its sign on a definite verdict.
  - `manifest::positive` / `negative`, which `fold_abs` and `magnitude` call with `sess`, consult the record per indeterminate, and for whole-argument and factored matches.
  - A record that changes the set must clear `forms_door` and the memos exactly as `register_equal` does.
  - The D9 argument is the registry's own: one fixed walk per leaf.
- **Check before relying on it:** that no `Sym` session spans two boxes.
  - The registry already assumes this ("lives in the per-leaf session"). If an eager or editor-core session is reused across subdivided children, the record must be scoped like the registry.
  - The `sym-revalue-testing` instrument re-evaluates DAG nodes at other points. The simplifications live in forms, not the DAG, so it is unaffected. But a test that asserts a symbolic Zero at a re-evaluated point off the branch would now be asserting a theorem that holds only on the branch.
- **What the doors' three measurements would confirm or overturn:**
  1. **The untraced `pcurve_map_residual`.** I predict `|r|` against `r` through the cylinder's stored radius. If it is anything else (a trig identity of the sweep, `4·atan`), it is not this question and needs its own row.
  2. **Storing `r` with no `abs` and no door registration.** I predict the first refusal is a rim or `u_ref` reader (`√(r²)` against `r`), which confirms the fix belongs in the tier, not the verb. If it is a check that never reads the rim, my claim that the dividing readers are what fails first is overturned.
  3. **The site list with its gates.** This confirms or overturns my fillet guess, and whether each signed site's sign comes from a definite verdict on a factor of the `abs` argument. A site whose sign comes from a literal read off `f64` with no verdict on that value would need its own verdict, which is already a defect under [scalars].
- **PR 6993** (how the kernel asks whether a value is exactly zero) is next to this but untouched. The record never decides Zero itself, and the numeric channel still runs first. If 6993 changes the order inside `sign_within`, the hook moves with it.
- **The doors' added registrations should not merge.** `register_radials` and the rim facts become identities, so their allowlist rows go, which the allowlist gate will flag.

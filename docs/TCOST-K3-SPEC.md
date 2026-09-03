# TCOST-K3 — the tier-3 gate's discarded certificate (spec)

**Program:** S-TCOST (`work/tcost/plan.md`, kernel-logic track). **Track:**
kernel change — the standard v6 unit (binding spec, drawn implementer arm,
cross-model dual review, union fix pass, record-at-merge; §Review there).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — a small local edit (three functions, one hook type, one
  `step-import` field) behind a door with ~478 workspace call sites, plus
  a digest instrument and a hosted before/after on both lanes.
- **STRUCTURAL** — no predicate, bound, tolerance or margin moves; the
  only numeric obligation is an IDENTITY, so not `docs/MODEL-AB-LOG.md`
  §*Task-class field*'s ambiguous case.

## The claim

**A body that is gated at rest and then measured pays two certified quadratures
by rule.** The tier-3 doors compute a full certified `MassProperties` to decide
check 7 (the +V invariant) and return `()`, so a caller that also wants the
number runs the identical computation again; on rational-walled bodies that
second run is 7.8–8.8 s (PR 1621, measured). The unit is **one certificate per
body per gate**: the gate returns the enclosure it computed, and the rows below
and the STEP import path consume it instead of recomputing it.

Chain, cited. `topo::validate_geometric` (`crates/topo/src/validate.rs:2217`) →
`validate_geometric_declared` (`:2350`) → the structural half (`:2355`; no-op
check-7 hook, no certificate) then `validate_geometric_certified` (`:2356`,
defined `:2308`), whose body is
`plus_v_invariant(mass_properties_certified(body, band, tol), band)`
(`:2316`–`:2317`). `plus_v_invariant` (`:2401`) reads `volume`, `volume_pad`,
`surface_area` and **drops the `MassProperties`**. `validate_pseudomanifold`
(`:3921`) and `contact_marks` (`:2593`) reach check 7 through
`tier3_local_checks` (`:2371`), hook
`plus_v_invariant(mass_properties_with(..))` (`:2385`, twin `:2628`) — the same
drop; hook type `PlusVCheck` (`:2644`); check 7 runs only on a clean battery
(`:3506`). The public measurement door is `topo::mass_properties`
(`props.rs:190`): `Band::linear(tol)` then `mass_properties_with` (`:200`).

**The two are the same computation for every scalar that can reach the
certified door.** `mass_properties_with` dispatches to `T::quad_cut_face`; the
certified half names `quad_lane::cut_face` directly (`props.rs:221`–`:241`).
`PropsQuadLane` has exactly four impls: `f64` (`:839`), `Probe` (`:874`),
`Interval` (`:915`) — each `quad_cut_face` **is**
`quad_lane::cut_face(..).map(Some)` — and `Dual` (`:954`), which answers
`Ok(None)` and cannot form the certified call (`compile_fail`, `:2207`). Same
band, same face order (`props.rs:255`): the gate's value is the caller's value
bit for bit, which is what D9 asks of this lever.

### One correction to the finding, read from the code

The finding names `validate_geometric` as the door the import path pays. **On a
single-solid import it is not:** `import_step` skips the per-solid `gate` as an
identity at one instance (`crates/step-import/src/lib.rs:682`) and gates the
aggregate through `gate3` → `validate_pseudomanifold` (`:762`; `gate` at
`:788`). Both fixtures below are single-solid, so changing only
`validate_geometric` cuts **zero** certificates from the two import rows. The
mechanism is identical (the finding stands in substance), but the lever must
cover the tier-3′ door too. Each row's OTHER certificates are not redundant:
the native quadratures (`nurbs_import.rs:264`, `cert5_r1_import_probes.rs:237`)
run on a body that is not the imported one, and the re-import gate
(`nurbs_import.rs:383`) has no partner.

| row | certs | the redundant pair |
|---|--:|---|
| `sweep::m8_3_rational_volume::tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion` | 2 | `mass_properties(&loft)` (`crates/sweep/tests/m8_3_rational_volume.rs:181`) then `validate_geometric(&loft)` (`:203`) — same body, same `Tol::witness()`, 22 lines apart |
| `step-import::cert5_r1_import_probes::own_rational_wall_roundtrips_through_the_import_door` | 3 | the gate inside `import_step` (`crates/step-import/tests/cert5_r1_import_probes.rs:307`) then `mass_properties(&body)` (`:311`) on the body it just gated |
| `step-import::nurbs_import::arc_loft_natively_computes_its_rational_volume` | 4 | the gate inside `import_step` (`crates/step-import/tests/nurbs_import.rs:301`) then `mass_properties(&body)` (`:309`); the row's comment (`:247`–`:257`) names all four and says they cannot be shared "without a kernel API change" |

## Phase 1 — measure before touching anything

`memories/refusal-text-is-not-cause.md`: measure-first is mandatory, and a
count read off a PR body is a sentence, not a measurement.

**Instrument** (local, uncommitted; nothing that ships prints): a thread-local
counter in `mass_properties_impl` (`props.rs:255`) logging per call the wall,
the face count, `volume.to_bits()`, `volume_pad.to_bits()` and a caller tag
threaded from the four entry points (`mass_properties`,
`mass_properties_certified`, the check-7 hook, `mass_properties_closed_form`).
Each row alone (`--test-threads=1`, CI's `opt-level = 1` env) at ε = 1e-9 /
1e-6 / 1e-12, on default features and `--features interval`. **Report in the PR
body**, per named row and for `import_step` on both fixtures plus
`dm1-id-214.stp` (the non-test consumer): how many certificates run, from which
entry point, on which body, the wall each costs, and whether the collapsed pair
is **bit-identical in both fields**.

**Stop clause.** If the pair is not bit-identical, or the calls are on
different bodies or `tol`s, or the second certificate is a small fraction of
the row (< 15 %), the redundancy is not what the finding says: **stop at a
report** and file it as an issue (`CLAUDE.md` §*Filing an issue*).

## Phase 2 — the lever: the gate returns what it computed

1. **Widen the existing door** (`validate_geometric` →
   `Result<MassProperties<T>, Vec<ValidationError>>`). Rejected: ~478 call
   sites, nearly all `assert_eq!(validate_geometric(..), Ok(()))`, forced
   to name a value they do not want.
2. **A consuming door** (`validate_geometric_with(body, props, tol)`).
   **Rejected on soundness:** nothing in `MassProperties<T>`
   (`props.rs:53`) ties a value to the body, band or ε it came from, so
   the gate would bless a claim it did not derive, against tier 3's
   never-trust posture (`PropsQuadLane::recertify_approx`); a provenance
   token is more machinery than the saving.
3. **A returning sibling door — PICKED.** Additive:
   `validate_geometric_certificate` and its `_declared` twin return
   `Result<MassProperties<T>, Vec<ValidationError>>` at the same bound
   (`PropsQuadLane + CertifiedBounds`); `validate_geometric[_declared]`
   become that call `.map(|_| ())` — one certificate, no existing caller
   edited, the `compile_fail` guarantee (`:2207`) preserved by the shared
   bound. Same shape for the tier-3′ door (`validate_pseudomanifold`),
   without which the import rows do not move. (A `topo`-side memo cache is
   the fourth option, refused: D9 replay becomes a cache property.)

**The returned value must be THE computation, never a second one.**
`validate_geometric_certified` already holds it at `:2317` and throws it away —
return it. For the hook path, change `PlusVCheck` (`:2644`) from "returns
errors" to "returns `Option<Result<MassProperties<T>, MassPropsError>>`" and
let the battery call `plus_v_invariant` itself (collapsing `:2385`/`:2628`);
the structural half's no-op hook becomes `None` — honest, that door computes no
certificate. Any spelling that computes twice fails this unit. `import_step`
then hands back the enclosure its aggregate gate computed: a field on
`StepImport::Solid` (`lib.rs:763`) documented as *the gate's own value, not a
second computation*, `None` where the gate did not reach check 7.
`gate`/`gate3` keep their no-opinion contract (`:788`): more returned, nothing
filtered.

## Constraints, binding

- **No gate is weakened** — same rejections, same typed verdicts, same
  order. Proven twice: (a) a **planted-failure row** per refusal class the
  doors reach (an inverted body → `NegativeVolume`; a quadrature refusing
  on budget → `VolumeUncomputable`; a body failing a structural check),
  asserted through BOTH doors with identical `Vec<ValidationError>`;
  (b) a **mutant** — drop the `?` sequencing
  structural-before-certified (`:2355`) and show a named row reds. A
  refusing arm returns no properties: a refusal carries no blessed number.
- **No digest change on certifying bodies.** The instrument is K1's —
  `docs/TCOST-K1-SPEC.md` §*Constraints*, "Every face that certifies today
  certifies with a bit-identical bracket": roster + FNV digest of every
  certified `FaceCutBounds` over the shipped corpus, the STEP fixtures and
  every probe suite, at the three ε rows on both lanes, md5-identical
  across merge base and head. **Extended here** to every `MassProperties`
  a tier-3 door produces (four fields as raw bits) — direct evidence.
- **Every re-baselined pin carries its reason.** No pin should move, so
  one that DOES is a finding, not a chore: name it, say what moved and
  why, never adjust a number to preserve a green
  (`docs/prompts/implementer-discipline.md` §3); adopted rows keep their
  claims by NAME, not number
  (`memories/output-stability-as-justification.md`), and their assertions
  (`nurbs_import.rs:320`–`:336`, the overlap at
  `cert5_r1_import_probes.rs:318`) survive VERBATIM — that they still pass
  IS the identity claim.
- **Nothing else changes** — not the quadrature, the schedule, the band,
  check 7's margin (`:2401`), or the face walk's order.

## Acceptance

- The Phase 1 table in the PR body, from the merge base.
- Hosted, both lanes drawn or asked for (`CI-Config: lane=interval` on one
  head, `lane=default` on another, at `eps=1e-12` where the rational
  schedule exhausts), green; the digest identical across builds; every
  re-baselined pin carrying its reason.
- The three named rows before/after in the hosted cost report: one
  certificate gone from each (≈ −1/2, −1/3, −1/4 of the row's quadrature
  time, largest at ε = 1e-12).
- The unit's own suite, one labelled assertion each: the new door's value
  is bit-identical to `mass_properties` on the same body at the same tol
  (a RATIONAL-walled body, so the claim is about the quadrature lane); the
  door runs exactly ONE certificate; the refusing arm returns none; the
  planted-failure rows above.

## Out of scope

The cost of one certificate (TCOST-K1 owns it); the
`AtRestPolicy`/`AtRestOutcome` surface (`props.rs:1041`–`:1130`) and
`editor-core`'s gather gate, which measures nothing today and so pays nothing
twice — its own unit if a measurement says otherwise; `contact_marks`' marks
channel; multi-instance imports' N per-solid gates (`lib.rs:682`), certifying N
different bodies; test-content change beyond adopting the new door in the three
rows.

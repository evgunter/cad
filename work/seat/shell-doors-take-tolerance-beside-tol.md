---
id: shell-doors-take-tolerance-beside-tol
kind: issue
title: shell / shell_open take a raw tolerance: f64 beside tol: Tol, and the acceptance that no verb takes a Band beside a Tol has no mechanical guard
status: closed
opened: 2026-08-31
closed: 2026-09-05
github: 1409
refs: [1399, LIB-G17, shell-needs-shellnaming-birth-channel]
---

## From GitHub issue 1409

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Two related findings from SEAT-1's dual review (PR #1399), filed as one issue because both are about the verb doors' tolerance vocabulary.

**1. The instance.** `topo::shell` and `topo::shell_open` still take a raw `tolerance: f64` parameter beside the `tol: Tol` witness (`crates/topo/src/shell.rs`, the doors' signatures). That is the exact shape SEAT-1 just removed for `Band`: a second tolerance-flavored argument whose relationship to the committed global is the caller's problem. Whether this `tolerance` is genuinely independent (a per-call offset fitting budget, say) or another derivable-from-`Tol` value needs measuring before any change — SEAT-1's rule was "drop the parameter only after proving every caller passes the canonical derivation". Out of SEAT-1's ratified scope (VERB-SEAT-DESIGN §1 S4 names `Band` only), so recorded here rather than widened into that unit.

**2. The guard gap.** SEAT-1's acceptance clause — *no public kernel verb takes a `Band` beside a `Tol`* — is now a measurement with no mechanical guard: a future verb could reintroduce the parameter and nothing goes red. If the parameter-lint machinery (`docs/PARAM-LINT-SPEC.md` / `tools/k-lint`) grows signature rules at some point, this is a candidate row; until then this issue is the durable record that the invariant is unguarded by design, not by oversight.

## Home

`work/seat/` — the verb doors' tolerance vocabulary is SEAT's §1 ground (band derivation at operation entry) and both halves are SEAT-1's own disclosed residue.

## Ruled (Ev, PR 1904, 2026-09-05): (i) — derive at the door, and ε never travels as an `f64`

The shell doors drop `tolerance: f64`; the fit target IS ε_precision,
the one global ε D4 names. Ev's refinement binds the whole chain: **the
tolerance is passed as the ZST witness `Tol`, never as an `f64`** — so
it shows in every signature between the shell door and the one site
that classifies the residual, and no arithmetic can be done on it that
would make two callers effectively use different epsilons. The one
`tol.eps()` read lives at the classification site inside the fit
engine. The measurement that answered Ev's question (tier-3 validation
re-certifies every `Approx` face against the RUN's ε, so a looser mint
is a typed refusal, never loose geometry; every caller passed `1e-6`
into analytic offsets that ignore it) is in this file's history at the
`[ev]` PR. Unit: SEAT-9 (`docs/SEAT-9-SPEC.md`), block SEAT-B3. The
NURBS fit's COST at ε ≈ 1e-9 is the offset-fit owner's measurement,
reported by the unit, not gated on.

## Closed (2026-09-05, SEAT-9)

**Half 1, the instance, is executed, and the ruling's letter is
delivered.** `topo::shell` and `topo::shell_open` no longer take a
`tolerance: f64`: the witness is the only tolerance either takes, and it
travels the whole chain — `replace_face_offset`,
`replace_faces_offset`, `mint_offset`, `offset_charts_together`,
`PropsQuadLane::{approx_offset_surface, recertify_approx}` and
`geom-brep`'s five production fit doors (`fit_offset`,
`certify_offset`, `certify_offset_over`, `approx_offset_surface`,
`recertify_approx`) — with the value read once, at
`geom_brep::offset_fit::precision_target`. No signature between the
shell door and the classification carries an `f64` epsilon.

The fit ENGINE keeps a numeric-target form of each door, `#[doc(hidden)]`
and suffixed `_at`, because the only way to measure a refinement loop,
a round budget, a stall guard and a limb classification is to run them
at targets chosen for the measurement (1e-2 through 1e-18, and bounds
derived from a measured residual) — which one committed ε cannot
express. Twenty-two of the twenty-four `FIT_TOL` constants retired with
the parameter; the two that remain were renamed `ENGINE_FIT_TARGET` to
say what they now are. One production caller reaches an `_at` routine —
the transform lane's `remap_certificate`, which classifies a mapped pair
against the tolerance the SURFACE's claim was made at — and that is
censused by name rather than left to be discovered.

**Half 2, the guard gap, is answered in kind rather than by the
parameter lint.** `crates/topo/tests/shell_tolerance_chain.rs` is a
source census over the chain in three rows: no signature on it may take
an `f64` whose name reads as a tolerance (every `name: type` pair on a
line, not just the first, so a one-line signature cannot hide one); the
ε reads it holds must be exactly the ones it declares, per stretch,
with what each is for; and no production file outside the transform
lane may reach the numeric-target instrument. Each reds on its own
planted defect (a one-line `fn f(d: f64, tolerance: f64)`; a second
`let _ = tol.eps();`; `topo::props` calling `recertify_approx_at`).
Its blind spots are stated in its own header, including the ones the
first version of it hid. **The compiler is still the primary guard** —
a caller has no number to pass — and these rows are future-edit
coverage over a property the types already hold.
`docs/PARAM-LINT-SPEC.md` / `tools/k-lint` still grow no signature
rules, and this census is the narrower thing that exists instead — it
guards the one chain the finding was about, not the invariant in
general.

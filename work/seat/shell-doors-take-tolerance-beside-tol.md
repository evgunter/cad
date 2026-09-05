---
id: shell-doors-take-tolerance-beside-tol
kind: issue
title: shell / shell_open take a raw tolerance: f64 beside tol: Tol, and the acceptance that no verb takes a Band beside a Tol has no mechanical guard
status: open
opened: 2026-08-31
github: 1409
refs: [1399, LIB-G17, shell-needs-shellnaming-birth-channel]
needs_ev: true
---

## From GitHub issue 1409

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Two related findings from SEAT-1's dual review (PR #1399), filed as one issue because both are about the verb doors' tolerance vocabulary.

**1. The instance.** `topo::shell` and `topo::shell_open` still take a raw `tolerance: f64` parameter beside the `tol: Tol` witness (`crates/topo/src/shell.rs`, the doors' signatures). That is the exact shape SEAT-1 just removed for `Band`: a second tolerance-flavored argument whose relationship to the committed global is the caller's problem. Whether this `tolerance` is genuinely independent (a per-call offset fitting budget, say) or another derivable-from-`Tol` value needs measuring before any change — SEAT-1's rule was "drop the parameter only after proving every caller passes the canonical derivation". Out of SEAT-1's ratified scope (VERB-SEAT-DESIGN §1 S4 names `Band` only), so recorded here rather than widened into that unit.

**2. The guard gap.** SEAT-1's acceptance clause — *no public kernel verb takes a `Band` beside a `Tol`* — is now a measurement with no mechanical guard: a future verb could reintroduce the parameter and nothing goes red. If the parameter-lint machinery (`docs/PARAM-LINT-SPEC.md` / `tools/k-lint`) grows signature rules at some point, this is a candidate row; until then this issue is the durable record that the invariant is unguarded by design, not by oversight.

## Home

`work/seat/` — the verb doors' tolerance vocabulary is SEAT's §1 ground (band derivation at operation entry) and both halves are SEAT-1's own disclosed residue.

## Decision for Ev (2026-09-05; joint SEAT / SHELL / offset-fit owner — the `[ev]` PR carries it)

**Measured** (main `089c9715`): `topo::shell` / `shell_open`'s
`tolerance: f64` is the offset FIT budget handed to
`replace_faces_offset` (`crates/topo/src/replace_face.rs:1003`) for
the fitted offset surfaces of the cavity walls. Every caller in the
tree passes a constant (`FIT_TOL`, `1e-6`), and every caller is a test:
the door has no production caller, because `Node::Shell` (LIB-G17) is
parked. `FIT_TOL` is not `Band::linear(tol)`'s epsilon by inspection.
The question is forced now because SEAT-9 (the shell arm on `Verb`,
`docs/SEAT-9-NOTE.md`) cannot carry an `f64` beside a `T` in a
`Verb<T>` payload — that is the shape SEAT-1 removed for `Band`.

- **(i) Derive it at the door and drop the parameter.** SEAT-1's rule:
  only after proving every caller passes the canonical derivation —
  here that means the offset-fit owner (`geom-brep/src/offset_fit.rs`,
  S-CERT's then PROPS') says what the fit budget IS a function of. If
  it is a function of the committed tolerance, the parameter goes
  and the two shell doors read `tol` alone, like every other verb.
- **(ii) It is a genuine per-call budget: type it `T`, make it a
  slot.** Then it is document-visible vocabulary on LIB-G17's
  `Node::Shell` (a `SlotId` with a dimension and a display unit), the
  Verb arm carries it as a scalar parameter with an explicit empty
  flow row, and the kernel door keeps a typed parameter for it.
- **(iii) Keep `f64` at the kernel door, carry it opaquely.** Rejected:
  it is the raw-tolerance-beside-`Tol` smell itself.

**Recommendation: (i) if the offset-fit owner confirms the budget
derives from the committed tolerance; (ii) otherwise.** SEAT does not
own `shell.rs` (SHELL's) or `offset_fit.rs`; this PR asks you to rule
or to route the derivation question to those owners. SEAT-9's spec is
cut on the answer; nothing else in block SEAT-B3 waits on it.

Ev's ruling lands here in place.

**Measured after Ev's question on PR 1904 ("could we be handing back
geometry more than ε off?"): no — and the parameter has one value.**
O3's certificate claim is `sup ‖S_fit − (S + d·n)‖ ≤ ε_precision`, and
D4 names ε_precision as THE global ε. Tier-3 validation re-derives every
`Approx` face's certificate per call (O5: the stored one is never read)
and classifies it against the RUN's `tol.eps()`, not the mint's
tolerance (`topo/src/validate.rs:3126`; `ValidationError::
ApproxCertification`'s doc: "so does one minted looser than the ε this
run demands"); `shell` validates last and `replace_faces_offset` adopts
its clone only after validation. A fit minted looser than ε is a typed
refusal, never loose geometry. So the `tolerance` parameter is the
fit's refinement TARGET, and any value above ε guarantees a refusal
while any value below ε is ε plus wasted refinement: it has exactly
one value that can succeed, `tol.eps()`. Every caller in the tree
passes `1e-6` and (those read) shells analytic bodies, where the value
is ignored — which is why the suite is green at the 1e-12 lane. This
sharpens the recommendation to **(i) unconditionally**: derive at the
door, drop the parameter; the only open question is the NURBS fit's
COST at ε ≈ 1e-9 (reach it or refuse, D4 either way), which is the
offset-fit owner's measurement (S-CERT's then PROPS'), not a gate on
the verb arm. Ruling pending Ev's reply to the PR comment.

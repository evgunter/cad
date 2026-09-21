# STACK — the plan

what a certified enclosure CLAIMS: the stackup's hull, the non-real contract and the forgeable certificate family

Opened 2026-09-20 by ENCL's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**12.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `certified-hull-padding-is-the-leaf-width-not-the-lane` | H | a stackup's certified worst-case hull pads by the LEAF width: a tier that certifies a study in fewer leaves reports a wider hull at the same leaf budget |
| P2 | `certified-lane-non-real-contract-audit` | D | Contract - what a certified enclosure lane owes when a value goes non-real (poison absorbs vs widens) |
| P2 | `contribution-bounds-via-dual-interval` | D | E5 contribution bounds from Dual<Interval> derivative enclosures (M10-4 deviation 3) |
| P3 | `certificate-types-have-public-fields-and-are-forgeable` | D | MassProperties and the certificate family (PcurveCertificate, certify::Certificate, SsiCertificate, OffsetCertificate, CertifiedLeaf) have public fields — a downstream crate can forge a certified claim nothing computed |

## Order

`certificate-types-have-public-fields-and-are-forgeable` first,
because it is the cheapest and it is the one row whose fix
(constructors, private fields) makes the other three easier to state:
once a certificate can only be minted by the lane that earns it, what
the lane OWES is a question with a place to live.

Then `certified-lane-non-real-contract-audit`, which is that question
written down, and the two frontier rows behind it.

## Review posture

OPEN, for this program's first dispatch. ENCL inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.

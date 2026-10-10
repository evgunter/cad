---
id: an-unattributed-contact-at-rest-is-a-finding
kind: issue
title: D10 stage 4 PR J: A5's hard error on an unattributed contact becomes an unproven-coincidence finding; a contact between copies no mate attributes is recorded at the door
status: parked
priority: P0
cost: M
blocked_on: [carriers-compare-in-canonical-form]
opened: 2026-10-09
---

INTENT stage 4, PR J. Spec: `docs/INTENT-STAGE4-SPEC.md` §8b.

Cut out of I (`mates-declare-no-contact`) by the orchestrator on 2026-10-09. Stage 3's placement unit (`a-placement-is-the-bundle-of-mates`) drops today's declaring mates. Without this cut, the contacts those mates declared would meet A5's hard error and refuse the product, and I cannot come first: it waits on H, which waits on stage 3.

The at-rest census records a coincidence between copies that no mate attributes as a `Coincidence { site: CensusAtRest }` row on the product, proven at the door. `ValidationError::UndeclaredContact` for those classes becomes that row. `Attribution::Unattributed` goes, and so does A5's "Undeclared contact between instances is a hard error, never blessed". This builds D10's ratified text: a contact beyond what the mates fix is "recorded and linted, and its recourse is an assertion".

A pierce or a same-side crossing stays interference evidence (stage 5 B). A mate's own contact is still minted as a declaration until I.

Waits on stage 4 B (the door) and C (rung 2). Stage 4 D (rung 3) has merged. The contacts between differently placed copies, which stage 3 B's dropped declaring mates used to declare, stay unproven findings until H, because C compares a placement chain as one opaque atom. The interim is loud and refuses nothing.

## Vocabulary this PR inherits (CONTACTHOLD, 2026-10-10)

The census and tier-3′ validator still name every contact record a
"declared record" and a record-less coincidence an "undeclared
contact", in doc comments, symbols and runtime strings: `census.rs`
(the "declared-record confirm arm", `confirm_declarations`, `Recorded`,
the "declared contact" refusal string in the corner-only arm),
`validate.rs` (`ValidationError::UndeclaredContact` and its siblings'
docs, the "no declared contact behind them" / "is an undeclared
contact" messages), and `props.rs`. A boolean's records are cited, not
declared, since stage 4 B2. `contacthold/declared-wording` left these
alone because this PR retires `UndeclaredContact` and reshapes that
relation; rename them with it.

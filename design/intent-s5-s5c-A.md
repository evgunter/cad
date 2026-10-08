# FORK-S5C — the at-rest census as a check (designer A)

## For Ev

**Recommendation (likely).** Nothing refuses at rest. The census stops being a gate and becomes something the
product computes and checks read. Its contacts are findings of the one `unproven-coincidence` lint, which also
covers booleans' glue. Interference, and the pairs the census could not judge, are findings of one new resident,
`AtRest`, with a full `Off/Warn/Error` knob. An assertion is how a document answers any of these findings. It
replaces DS6's separate "waiver record". `Separation` retires into `AtRest`, and `assemble` goes.

**Premise check (sure).** "Where does the census refuse" assumes a refusal at rest has something to stop. It has
nothing. The census's result does not reach export (D10: export reads the world and refuses only an empty one, or a
placement whose body is gone, #4220). A "refusal" therefore reaches only the viewer badge, the tour and an exception
in Python, all of which are reports. The one real effect is harmful: today the viewer drops the product's body when
A5 refuses (`badge` keeps `None`). In addition, `enforce_checks`, the registry's one refusing path, has no caller
outside tests. So the fork is about what a reader is told, and "refuse" at rest can only mean "a caller asked
`enforce_checks` to fail on this". Read that way, D10's "neither refuses where the census has a lane" is a leftover
of the gate model: it implies a refusal where the census has no lane, and nothing remains at rest to do that refusing.

**Ratified text I would change.**
- **D10, Assertions.** Change "…and interference is a finding of its own; neither refuses where the census has a
  lane." to "…and interference is a finding of its own. Nothing at rest refuses: where the census has no lane for
  a pair, that is a finding too, saying it could not look." (D10 is the redesign, but this clause is the old gate
  model in its words.)
- **ASSEMBLY A5** ("The at-rest gate … Undeclared contact between instances is a hard error, never blessed.
  `AssemblyError::AtRest` is a verdict against the document"; *Interference.*: "…refused typed"). It becomes "The
  at-rest check": per space, the census records the contacts it finds at the coincidence door and reports
  interference and could-not-look pairs to `AtRest`. Nothing refuses, and `assemble` is gone.
- **DISCIPLINES DS6.** The rule "may offer `error` **iff it ships a waiver vocabulary**: a per-finding,
  stable-name-keyed acknowledgment record … there is nothing to verify — only to match, and to stale" changes.
  A record of intent *about a finding* is a third way of saying intent, and D10 says "Nothing else carries
  dependency or intent". Proposed: "A check may offer `error` iff the document can answer each finding it raises
  the D10 way, by constructing the coincidence or by an assertion that quiets it. A quiet finding is listed with the
  assertion that quiets it. A check's own could-not-look finding refuses at `error`: an error gate that passes what
  it could not see would be certifying a guess." Ev's own words in round 3 ("blocking is fine if exceptions are
  declarable") are kept. Only the agent-written shape of the exception changes. DS6 is "WIP, provisionally
  accepted", so the bar for this change is low. The "Second resident SHIPPED" paragraph is rewritten too
  (`Separation` → `AtRest`). DS9's "framework provision" of generic acknowledgment records goes the same way.

### 1. What the census becomes

- **The census is a producer, not a check.** It runs over the product per space, at the moment a reader asks for
  the product (as the registry's gather already does). It yields three things: the contacts it decided, which are
  recorded at stage 4's door like any value-decided coincidence (D10 names "a contact the at-rest census finds");
  the interference verdicts, as in stage 5's PR B; and the pairs it could not judge (`CensusUnsupported`,
  `…Undecidable`, `…Escalated`, sliver band, malformed body).
- **Two residents read it.** Contact findings go to the `unproven-coincidence` lint, which is D10's own word for
  them ("contact between copies is an `unproven-coincidence` finding"). That keeps one home for every
  non-structural coincidence: a boolean's glue and two blocks that happen to touch. Interference and could-not-look
  findings go to `AtRest`. Quiet findings are listed with the assertion that quiets them.
- **No lane.** A pair the census has no lane for is a loud `AtRest` finding with the census's own evidence (DS6:
  "a check that could not look says so as a FINDING"). At `Warn` it is reported. At `Error`, `enforce_checks` refuses
  it, and the caller asked for that.
- **Optional extension (unsure; it changes D10's quieting sentence).** A could-not-look pair is quiet under a
  holding assertion that the two copies' clearance is above some positive bound. That proves the copies neither
  touch nor overlap, by a lane other than the census's. Every `AtRest` finding could then be answered by an
  assertion, so `Error` would refuse only what the document has not answered. Whether the clearance measure has
  lanes where the census lacks them is not checked.

### 2. Assertions and the registry
An assertion is what answers a finding. It is not a waiver in DS6's sense, and it is better than one:
- it is document data with an author;
- it is a geometric claim that is checked, rather than "nothing to verify, only to match";
- it carries its own staleness. If the copies stop touching, a `Gap = 0` is `Violated`, and that reports more than
  a "stale waiver" flag would.

So DS6 changes as quoted above, and assertions become the only acknowledgment vocabulary. Severity stays a per-run
argument (`ChecksConfig`, nothing persisted). A refusal is still the caller's choice at `enforce_checks`; what the
document can do is quiet findings.

### 3. `Separation`
`Separation` retires (sure). Its finding is the denial of a box certificate, never "these overlap". It is a strict
superset of `AtRest`'s precise findings: it also flags every correctly touching pair, and its declared-contact
suppression reads seats that stage 4 deletes. Its box test, `topo::SolidSeparation`, becomes the census's pair
pre-filter. The obligation it discharged (`graft_disjoint_all_keyed` asserts nothing) is discharged precisely by
`AtRest`, plus could-not-look for every pair the census does not decide.

### 4. Callers
- **Viewer badge.** It shows counts from the report: loud, quiet and could-not-look, summed over `AtRest` and the
  at-rest part of the lint. The product's body is always kept. A gather refusal stays in its own channel
  (`badge_site`), as ruled in fork 25.
- **Python.** `assemble`, `Assembly` and `AtRestFinding` go. Readers call `product(...)` plus `Doc.checks()`, which
  return `AtRest` and lint findings, each with `quiet_by`. A CI script that wants a gate passes `AtRest=Error` to
  `enforce_checks`.
- **Tour.** The gallery rows read `AtRest { loud, quiet, unseen }`. The heat sink's fins, which sit 1/16 inside the
  base by design, read as loud interference until gap assertions quiet them.

### Alternatives (each a coherent final state)
- **B. One `AtRest` resident owns contact and interference** (the spec's shape). What it makes true: one resident
  answers "is the world sound at rest", so the badge reads one thing. What it costs: `unproven-coincidence`
  findings live in two homes (the lint for booleans, the resident at rest), and the lint's name means two things.
  It changes D10's sentence too. I lean against it (likely). It is easy to reverse either way, because only the
  report's partition differs.
- **C. A5 survives as a door that refuses only no-lane pairs** and reports findings on `Ok`. This is D10's text
  taken literally. What it makes true: no reader can see `Ok` over a pair the census did not judge without opting
  in. What it costs: a refusal that blocks nothing, the viewer losing the body, and `Refused` meaning both a gather
  failure and "could not look" (fork 25's defect again). DS6 is unchanged. I reject it (likely).
- **A′. `AtRest` as `Advisory` (Off/Warn), never refusing.** This needs no DS6 change, but it gives up the CI gate
  that Ev's round-3 rule allows once exceptions can be declared. Under D10 they can be (assertions), so I lean
  `Severity` (likely).

**Worked example.** Two copies of a block are placed by numbers so that their faces touch at the current values.
1. The census decides the contact Zero and records it at stage 4's door. The lint reports `unproven-coincidence`
   with the edit that makes the contact one construction (a mate).
2. If the author writes `Assert { Gap(a.top, b.bottom), =, 0 }` instead, the finding is listed as quiet under that
   assertion.
3. If the author later moves block b up by 1 mm, the contact finding disappears. The assertion is then `Violated`,
   and that is the staleness signal.
4. If the blocks were a torus and a cone that the census has no arm for, `AtRest` lists one loud could-not-look
   finding. Nothing refuses unless a caller asks for `Error`.

**Confidence.** That nothing refuses at rest: likely. Contact going to the one lint: likely. The DS6 rewrite:
likely. `Separation` retiring: sure. The could-not-look clearance extension: unsure.

## For the orchestrator

- **Spec reading.** I read spec §0–§10 only, not §11.
- **Provenance not traced.** `git log -S` hits only shallow-graft merges (`b804bcc1c`) for D10's sentence, the
  plan line and DS6's waiver text, so I could not tell whether Ev wrote "where the census has a lane" or approved it
  in passing. The fork log, row 62, records D10's model as ratified on #3990. DS6's round-3 line cites Ev's
  "blocking is fine if exceptions are declarable" as Ev's own words.
- **Off-question finding: `enforce_checks` has no non-test caller in the tree** (`grep`: only re-exports and
  `dsc_checks.rs`). Every claim that something "refuses" through the registry is hypothetical today.
- **Off-question finding: `ChecksConfig::expected_components` is per-run caller configuration keyed by
  `(root, output)`.** It is not a document record, so DS6's "carried with provenance like any declaration" was never
  true of the shipped acknowledgment. Under D10 it is intent stated outside the document, a candidate for an
  assertion on a component count. This is worth an issue (owner: the checks registry's program). I did not file it,
  because this lane changes nothing.
- **Stage 4 boundary.** My answer to (1) puts at-rest contacts in stage 4's lint. Spec §10 should then say that the
  census's contacts enter the door like a boolean's, and that the quieting rule's contact half lands in the lint. D10
  does not say whether the quieting rule also quiets a boolean's non-structural glue; that is worth asking when
  stage 4's spec lands.
- **Ergonomics.** Quieting is per site, so the 160-fin heat sink needs an assertion per fin unless a measure can read
  a `Faces` set (D10 allows sets). Not designed here.
- **Assumption.** Assumption: stage 3's spaces exist, and copies are compared only within one space.

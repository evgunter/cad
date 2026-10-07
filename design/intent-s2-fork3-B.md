# INTENT stage 2, FORK-3: what a selection is in the document (designer B)

## For Ev

### Round 2 (supersedes round 1 where they differ)

**Recommendation (likely): a selection is a variable definition,
`VarDef::Select { body, name }`, holding ONE name.**
- **Interned.** The document holds at most one select per `(body, name)`. Authoring a
  selection that already exists returns the existing variable.
- **Many singletons.** A fillet's edges are many `Edge` variables in a list slot. There are
  no set kinds.
- **Repaired by variable.** `Rebind { var, to }` redefines the one select. When `(body, to)`
  already exists, the two are merged.
- **`target` stays.** The blends and the shell keep their `target`. Each edge's body is
  checked against it at the door (`SelectionOffTarget`).
- **Definition, not node.** Both designers agree on this.

Round 1 recommended one set-valued variable per fillet, distinct-by-authoring, and a
name-addressed `Rebind`. Three things moved me.

1. **Name-addressed repair is wrong, and the set kind needed it (sure).**
   - **What `Rebind { from, to }` does today.** It rewrites every site naming `from` in
     every node, whatever body that site resolves in (`edit.rs`, the `Rebind` arm).
   - **How that strands a reader.** Say a boolean trims edge N into pieces. A reader
     downstream of the boolean loses N; a reader upstream still resolves N. Repairing the
     downstream reader to a piece rewrites the upstream one too, and strands it.
   - **Why round 1 cannot fix it.** A repair is a fact about a *denotation*: "what N means
     in this body". Round 1's set selections are authored per site, so they cannot be
     interned, and a repair has no single home. It has to fall back to rewriting names,
     which is the defective form.
   - **Why singletons can.** A singleton select is exactly a denotation. Interned, it has
     one address, and the repair is one edit that is true for every reader.
2. **Interning does not break VR1/VR8 (likely).**
   - **What round 1 relied on.** Round 1's argument was VR8's: two typed `5 mm` are
     distinct, so that editing one cannot move the other.
   - **Why that does not carry over.** VR8 protects *free* variables, whose values diverge
     by later edits. A select has no free arm. Changing which edge a fillet blends is a
     slot edit (point the slot at another select). It is never a redefinition.
   - **What remains.** The only redefinition is the repair, which should move every reader.
     Identity is still minted (VR1); the door finds rather than re-mints.
   - **The cost.** Interning is a load-door uniqueness check, like VR2's names.
   - **Reversibility.** It relaxes to the offer model by deleting that check.
3. **Sharing a single edge is not narrow (likely).** A measure, a datum face frame or a
   mate naming an edge or face a fillet also blends is ordinary. Under interning they share
   one variable, one name ("mouth") and one repair, for free. A set kind forbids that.

**Where round 1 still stands, and what it costs.** One argument from round 1 still holds:
the body is stated N+1 times, so a fillet whose edges belong to a different body than its
`target` can be represented. That needs a door refusal. I accept it, because:
- **It refuses loudly.** It can never act silently.
- **It is one rule for all three verbs.** A shell that opens no face still needs `target`,
  so dropping `target` from the blends would give the blends and the shell different rules.
- **Order and the content key.** `Shell.open`'s designation order is the slot's own fact.
  A fillet's canonical form is "sorted by its selects' names", so its content key is
  unchanged.

**Ratified text.**
- **D10.** Name the selection as a definition arm: "…, by an `Expr`, by a selection of a
  `Body` variable by `StableName`, or as an output of an operation".
- **SELECT-DESIGN §4.** Restate the one-type rule: the GUI's `Vec<StableName>` pick set is
  the *authored* form of a list of selects, as a `Formula` is of an `Expr` (VR6).
  Committing it finds or mints one select per name.
- **Wording provenance.** Per the other report's check, D10's selection wording is
  agent-drafted on #3990, and Ev's transcripts are silent on it.

**Confidence.**

| Claim | Confidence |
|---|---|
| Definition over node | likely |
| Singletons over sets | likely |
| Interning | likely |
| Repair by variable, merge on collision | sure, given interning |
| Keep `target` with the off-target check | unsure (close call; dropping it for the blends only is the alternative) |

### Round 1 (record; full text in this branch's first commit)

Round 1 recommended:
- a definition `Select { body, names: Vec<StableName> }`;
- set kinds `Faces`/`Edges`, so that a blend reads one variable and drops `target`;
- distinct-by-authoring identity;
- a name-addressed `Rebind`.

Its sole decisive argument was the N+1 body statements. Round 2 keeps that as the cost
above and drops the rest.

## For the orchestrator

- **Round 2 checks.**
  - **Latent defect.** Confirmed by reading `edit.rs`'s `Rebind` arm: it loops over every
    node's `rebind_payload_names` with no body test, so a site where `from` still resolves
    is rewritten too. The defect is today's (declared pairs and the appearance store).
    E retires it for selects. It stays live for declared pairs until stage 4: worth an issue
    file if E slips, as A says.
  - **A's empty-shell question.** `Node::Shell` docs: "Empty `open` is the sealed hollow —
    Legal, and not a refusal". A's lean does not flip.
- **Agreed with A.** E needs D's mid-evaluation binding whatever FORK-5 rules: selects are
  a third customer of it.
- **Round 1 orchestrator notes** (canonical order, empty blend refuses) carry over: the
  fillet's key is over the selects' names, sorted.

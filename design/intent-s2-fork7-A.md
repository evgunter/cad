# FORK-7 — must every variable have a name? (designer A)

## For Ev

**Recommendation (likely): require a name exactly where a variable is shared.**
The rule: *a variable that is read at more than one place must have a name. An
unnamed variable is read at exactly one place: one slot, or one spot in one
formula.* The person or the caller supplies the name, at the moment they make
the second read. The kernel never invents one. Outputs of operations are
exempt, because their operation already names them ("the body of Extrude
"base plate"").

To the person, this means a **variable** always has a name, and a number typed
into one field is a **value**. The kernel still stores that value as an unnamed
variable, because D10 lets no literal stand in a slot. But nobody ever sees an
unnamed variable that two things depend on.

**Why not require a name on every variable?** The kernel could do it, and it
would be coherent. But it collides with D10 itself, and it charges for names
where they buy nothing:
- **It fights D10's typing rule (sure).** D10 says typing a value *mints* a new
  free variable. If every variable needs a name, every retype of a depth asks
  for a fresh name. And the old variable, now named, is no longer cleaned up
  automatically, so it lingers as clutter until someone deletes it. The only
  way out is to make typing overwrite the value in place, and #4247 removed
  exactly that because it contradicts D10.
- **Every written number needs a name (sure).** That covers every sketch
  dimension, every count, and every `5 mm` inside a formula: `w + 5 mm` becomes
  invalid text until `5 mm` is declared under a name. Python's
  `extrude(depth=12*mm)` needs a `name=`. Names must be unique in the document,
  so five extrudes need five depth names. What people type under that pressure
  is `d1`, `d2`: the uninspiring scheme you rejected, now typed by hand.
- **Inlining gets worse (sure).** Inline refuses a carried name the host already
  holds. Inlining a part twice, or inlining a part that uses `depth` into a host
  that also uses `depth`, would refuse on every variable instead of on the few
  that were deliberately named.
- **Outputs and selections would be exempt anyway (sure).** No one will name
  every body and every fillet's edge set. So "every variable" really means
  "every free and formula variable".
- **What it would buy:** every variable could be read in a formula and listed in
  the panel, and the cleanup rule (VR7) would go away. Under the recommendation
  every *shared* variable is already speakable, and a variable read once is
  spoken unambiguously by the one place that reads it.

**What is actually wrong today (the premise check).** Anonymity itself is not
the defect. The defect is an unnamed variable that several places read. That
is a dependency between those slots that nothing can show or say. #4247
choice 9 shows nothing for it. Its choice 5 has a drag silently moving every
slot that reads one. The range probe's documentation describes widening "an
anonymous one several slots read". D10 makes reading a variable the one way to
say "this depends on that". A dependency should be visible, and a name is what
makes it visible. The recommendation makes that state impossible to represent,
instead of finding ways to show it. (sure on the diagnosis; likely on the
remedy.)

### What each source of variables does under the recommendation
- **Typed in the GUI:** mints an unnamed variable read by that slot, as today.
  No name is asked for.
- **The offer:** accepting an **unnamed** candidate opens the empty **name…**
  field for it. Committing names that variable and points the slot at it, as one
  undo step: `RenameVar`, then the slot edit, each valid on its own. Escape
  cancels both. Accepting a **named** candidate is unchanged. The candidate's
  button can still read "the distance of Extrude …", and that label is now
  always unambiguous, because the variable has exactly one reader.
- **A quantity in a formula (VR6):** `w + 5 mm` still stores `w + v`, with `v`
  unnamed and read once. No change.
- **Python and the Rust façade:** a written quantity mints for that call, as
  today. Passing a `Var` handle is still how two slots share one variable. If
  the handle names an unnamed variable that something already reads, the door
  refuses with a typed error ("name it first: `RenameVar`"). `declare_var`
  already requires a name.
- **Operation outputs:** no name is required, because they are named by their
  node and port. A name may still be set on one (VR2).
- **Selections** (`Face`/`Edge` sets): treated like free and formula variables.
  A selection shared between a fillet and a measure must be named. (unsure:
  this is the one place the rule may chafe. The alternative is to exempt
  selections, as outputs are, but two selections authored apart can read
  identically, so "spoken by their definition" is ambiguous.)
- **Split:** an unnamed variable has one reader, so it always travels with that
  reader. `UncutVarReference` (the refusal when a variable's readers are on both
  sides of the cut) can then only fire on named variables. FORK-6's rule is
  unchanged.
- **Inline:** unnamed variables are carried under new ids, as today. Named ones
  still refuse a name the host holds (FORK-6). The refusal now lands only on
  names someone chose. The recourse is unchanged: rename one side.

### Ratified text that changes
- **VR2** (orchestrator-written, #4002/#4062; no ratification by Ev found):
  "The kernel mints no name; the GUI proposes one and stores it only when the
  person commits it." → "The kernel mints no name, and the GUI proposes none: a
  name is typed by the person or given by the caller. A variable read at more
  than one site has a name; an unnamed one is read at exactly one site and reads
  as its value and that reader." This also folds in #4247's choice 7.
- **VR7:** "An anonymous variable is read by something" → "…is read at exactly
  one site". Add: "Clearing a name refuses unless exactly one site reads the
  variable, and a door that would give an unnamed variable a second reader
  refuses." The removal of a variable when its last reader goes stays.
- **VR9:** "passing a variable is how two slots share one" gains "(a named
  one)". The load door's anonymous-is-read check becomes read-exactly-once.
- **VR6:** unchanged.
- **D10:** unchanged ("Typing a value … mints … and offers" holds; optionally
  add "accepting an unnamed one names it").
- **#4247:** choice 9 becomes moot and choice 5's consequence cannot arise
  (sharing is always named); the accept button gains the naming step.

### Migration
Nothing is released. A file that holds an unnamed variable read at two sites
refuses at load, with a typed error and the regenerate recourse (VR9's
pattern). Corpus files and fixtures that share an unnamed variable get a name,
chosen by whoever edits the fixture. This mainly covers the fixtures INTENT-LITERALS
§9 converted to sharing, and any test that passes one anonymous `VarId` to two
slots. (likely: few. I did not count them.)

### Alternatives, as final states
1. **Shared ⇒ named (recommended).** Shared-and-unnamed cannot be represented,
   every dependency is speakable, and VR6 and D10's typing are untouched. The
   cost is one invariant, one refusal at the doors and at load; reversible.
2. **Every free and formula variable named.** Fully speakable, with no
   anonymous cleanup. It conflicts with D10's mint-on-type, forbids `w + 5 mm`
   as written, adds a name argument to every Python slot, and makes inline
   refuse broadly. Outputs and selections are exempt regardless. Reversing it
   later means re-introducing anonymity.
3. **The GUI asks for a name at typing; the kernel allows unnamed.** Friction in
   the GUI, while Python and formulas still share unnamed variables.
4. **Today's rule.** Keeps the invisible shared dependency (#4247 choices 5, 9).

## For the orchestrator

- **Brief breach, disclosed:** fetching #4247's comments returned the
  orchestrator's two comments along with Ev's, and I read them before I noticed.
  The spectrum they list overlaps option 2 above. The recommendation (shared ⇒
  named, the exactly-once invariant) is not among the options they sketch.
- **Checked:** `DocEdit::DeclareVar` already requires a `VarName`
  (`edit.rs`), so today anonymity arises only from slot lowering, formula
  quantities and outputs. Shared-anonymous is documented as intended in
  `range.rs` (`RangeField::Slot` doc) and in INTENT-LITERALS row 289. `RenameVar`
  already refuses clearing a name with zero readers (`AnonymousVarUnread`); the
  new arm extends this to two or more. Inline's `VarNameConflict` recourse is
  "rename one side" (`refactor.rs`).
- **Provenance:** VARIABLES-DESIGN entered under orchestrator authority (#4002;
  #4062's body says no ratification by Ev). The D10 sentence on typing and
  offering is Ev's redesign text and does not change.
- **Not checked:** how many tests or corpus files share an anonymous variable,
  and whether `lower_slot` gives a `Var` leaf any reader check today (I believe
  not). Build lane: add a `Doc` read-site count, with walks over slots and over
  definitions' leaf occurrences. Note that a formula leaf `v * v` with `v`
  anonymous is two read sites, so the façade must refuse it.
- **Placement:** an `[ev]` PR editing VR2, VR7, VR9; also rewrite
  INTENT-LITERALS row 289 and line 191 (`UncutVarReference` "now for anonymous
  variables as well"). The optional D10 clause waits on Ev only if added.

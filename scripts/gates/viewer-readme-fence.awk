# viewer-readme-fence.awk — the CommonMark fence tracker both viewer
# gates read `crates/viewer/README.md` through. It is awk source, not a
# gate: `viewer-vocab-declared-once.sh` and `viewer-module-kinds.sh`
# each load it into `$FENCE_AWK` and prepend it to their own awk
# programs, the way `lib.sh` prepends its own record splitter.
#
# THIS TEXT IS PART OF EVERY PROGRAM IT IS PREPENDED TO, comments
# included, and a caller's self-test kills one awk stage by matching a
# fragment of that stage's program text. So a word written here can
# collide with a shim's key and kill the wrong stage; the collision is
# loud (the case wants a stage by name and gets another one), and this
# line is why the sentence above says "record splitter" rather than
# spelling the function.
#
# ONE COPY, AND THIS FILE IS WHY IT IS A FILE. Two readers answering
# "is this line markdown structure" differently is a divergence nothing
# would catch, which is the argument `viewer-vocab-declared-once.sh`
# already makes for prepending ONE helper to both of its own programs;
# it holds with more force across two gates than within one, because
# nothing reads the two files together. The shared home is a sidecar
# rather than a sourced `.sh` because `gate-roster.sh` derives the gate
# roster from `scripts/gates/*.sh` and excludes exactly one member by
# name (`lib.sh`): a second sourced fragment there would read as a gate
# that runs nowhere, and teaching that roster about it means editing
# it. A `.awk` file cannot be mistaken for a gate by any glob in this
# repo, and its extension says what it is.
#
# CALLED ONCE PER LINE, held by convention. `md_fence` MUTATES
# `FENCE_CHAR`/`FENCE_LEN`, so a second call on the same line advances
# the tracker twice. Each caller's first rule is the one call, where a
# reader can see it.
#
# THE RULE IS COMMONMARK'S AND NOT A TOGGLE, because a toggle is wrong
# in a way that is quiet. A fence opens on three or more backticks or
# tildes indented at most three spaces; it closes only on the SAME
# character, at least as long, with nothing but whitespace after it. A
# bare toggle would let a ``` line inside a ~~~ block close it and hand
# the rest of the block back to the heading rule — so the tilde case is
# planted by both callers. Two further CommonMark rules are encoded
# here because each is a way to be wrong and silent: a backtick fence
# whose info string contains a backtick is NOT a fence (it is ordinary
# text), and a fence-shaped line inside a fence that does not close it
# is content.
#
# THREE ANSWERS AND NOT A BOOLEAN, and the third one is load-bearing.
# `md_fence` returns `open`, `inside`, `close` or the empty string, and
# a caller that only wants "is this line markdown structure" tests
# `!= ""`: a delimiter is not structure either — a ``` line is not a
# heading, not a table row and not a bullet — so a reader asks that of
# EVERY rule it has rather than only of `^#`.
#
# THAT BOOLEAN IS NECESSARY AND NOT SUFFICIENT, and which answer a rule
# needs is the rule's own question. A predicate asking *did the
# previous line END a block* gets it backwards: an OPENING delimiter
# starts a block, so the line under it is content, while a CLOSING
# delimiter ends one, so the line under it BEGINS a paragraph.
# `viewer-vocab-declared-once.sh`'s `opens` is such a predicate and
# reads `fence == "close"`; `viewer-module-kinds.sh` asks the other
# direction of the same distinction — a table body ENDED by an opening
# delimiter is a table a fenced block interrupts, and its rows below
# the fence are not the roster. A boolean answered both of those wrong,
# and shipped a false GREEN over an unratified fourth kind.
#
# NO `(` IMMEDIATELY AFTER AN INTERVAL, and this is the same class as
# this directory's no-backslash rule: a spelling one awk accepts and
# the other dies on. `mawk` 1.3.4 aborts its regex compiler outright —
# `REcompile() - panic: values still on machine stack` — when an
# interval is followed DIRECTLY by an opening parenthesis. The boundary
# is exactly that adjacency, derived rather than guessed: `/^ {0,3}(a)/`
# and `/^a{2}(b)/` both panic, while `/^ {0,3}-(a)/` and `/^(a){2}/`
# compile. So both the natural spelling of "three or more"
# (` /^ {0,3}(`{3,}|~{3,})/ `) and the plainer ` /^ {0,3}(```|~~~)/ `
# take a gate down with an exit 100 and a reader that decided nothing —
# reported honestly by the caller's guard, which is the only reason it
# was a diagnosis rather than a mystery. `gawk` compiles all of them
# happily, so the hosted runner would never have shown it.
#
# EVERY LIVE INTERVAL IN THIS FILE IS CLEAR OF IT, and the sweep rule
# is `grep -nE '[{][0-9]+,[0-9]*[}]' scripts/gates/viewer-readme-fence.awk`
# read by hand rather than a count carried in prose. It returns FIVE
# LINES, three of them inside the paragraph above — which is where the
# only occurrence of the fatal spelling in this file sits, and that
# paragraph forbids it. The two LIVE lines carry THREE intervals: the
# backtick and tilde tests share a line and the `sub` has the third,
# and none of them puts `(` next to the `}` — all three are followed by
# a literal backtick or tilde or by `, ""`. Each caller owes the
# same sweep over its OWN awk programs. Derived under `gawk` 5.2.1 and
# `mawk` 1.3.4, both green over both gates' fixtures and the real tree.
#
# WHAT IT DOES NOT DO. It is a fence tracker, not a markdown parser:
# indented (four-space) code blocks, HTML blocks and block quotes are
# not modelled, so a `#` at column zero inside one of those still ends
# a caller's section. Four-space indentation cannot put a `#` at column
# zero by construction, and neither of the other two has ever appeared
# in the scanned regions; a reader who adds one meets the same red this
# tracker removes, which is the residue and is stated rather than
# implied.
function md_fence(line,   s, ch, n, rest) {
  if (line ~ /^ {0,3}```/ || line ~ /^ {0,3}~~~/) {
    s = line
    sub(/^ {0,3}/, "", s)
    ch = substr(s, 1, 1)
    n = 0
    while (substr(s, n + 1, 1) == ch) n++
    rest = substr(s, n + 1)
    if (FENCE_CHAR == "") {
      if (ch == "`" && index(rest, "`") > 0) return ""
      FENCE_CHAR = ch
      FENCE_LEN = n
      return "open"
    }
    if (ch == FENCE_CHAR && n >= FENCE_LEN && rest ~ /^[[:space:]]*$/) {
      FENCE_CHAR = ""
      FENCE_LEN = 0
      return "close"
    }
    return "inside"
  }
  return (FENCE_CHAR != "") ? "inside" : ""
}

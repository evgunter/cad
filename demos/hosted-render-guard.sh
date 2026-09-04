# shellcheck shell=bash
# HOSTED IS THE DEFAULT RENDERER. Sourced by every local render entry
# point (render.sh, render-wild.sh, render-uv.sh) as its first act.
#
# WHY A GUARD AND NOT A README LINE. A render is the heaviest thing this
# repo asks of a developer box (render.sh's header measures a single
# scene at 106 s on a loaded host, and memories/freecad-render-lane.md
# records freecadcmd wedging mid-pass), and the frames it produces are
# COMMITTED artifacts — so an off-hand local pass does not just cost an
# hour, it can put this box's GL stack into the repo's tracked pixels.
# `.github/workflows/render.yml` is the sanctioned renderer; a pointer
# in a README is advice, and advice loses to muscle memory. This exits
# nonzero instead.
#
# THE OVERRIDE IS A SENTENCE, DELIBERATELY. `CAD_RENDER_LOCAL_OVERRIDE`
# must equal one of the exact strings below — not "1", not "yes", not
# "true". Those are values an agent or a developer reaches for
# reflexively when a script complains about an environment variable; a
# sentence naming what you are accepting is one nobody types by accident,
# and one that reads as an admission in the shell history that produced
# the frames.
#
# TWO SENTENCES, BECAUSE THERE ARE TWO ACCEPTORS. A developer on a box is
# accepting drift and must not commit what the pass draws. The hosted
# workflow is the opposite case: it IS the renderer whose frames get
# committed, so every line of the local warning is false for it, and the
# last line instructs the reader against exactly what the job is about to
# do — which misleads precisely when someone is reading a render log to
# diagnose something. So each acceptor declares WHICH it is and gets the
# message that is true of it. The two sentences are an enumeration, not a
# parse: any other value, and unset, still refuses.
#
# THE RULE IS STRUCTURAL, NOT SNIFFED. CI does not get an exemption for
# being CI: `render.yml` and `local-scripts/ci-local.sh` each set this
# variable in the file, at the step that renders. (`ci.yml` renders by
# CALLING render.yml, so that file makes the declaration for the gate
# too.) There is no
# GITHUB_ACTIONS check here on purpose — a sniffed exemption is invisible
# at the call site and grows silently (every new runner, every act-like
# local emulator), whereas an env line in the workflow is reviewable
# where the render is requested. The hosted sentence keeps that property
# whole: it is a declaration made in the file by the acceptor, not a
# guess the guard makes about the environment it woke up in. The
# workflows are also the most *informed* acceptors of a sentence:
# render.yml's own step summary is the measurement of exactly the drift
# the local sentence names.
#
# WHY THE NAME STILL SAYS "LOCAL". The variable outlived its adjective —
# one of its values now says "I am not local". Ev sanctioned either
# spelling (2026-08-19) and the name was kept: it is the string already
# written into every shell history, README line and workflow file that
# names it, and a rename buys nothing the sentences do not say out loud
# at each call site. This paragraph exists so the mismatch is not filed
# a second time.
CAD_RENDER_LOCAL_OVERRIDE_SENTENCE='i-accept-local-render-drift'
CAD_RENDER_HOSTED_SENTENCE='i-am-the-hosted-renderer'

# $1: the entry point's name, for the message. Returns (0) only when the
# override is set to one of the exact sentences above, printing the
# message that belongs to the acceptor that set it; on anything else,
# unset included, it prints the pointer and EXITS nonzero from the
# calling script.
require_hosted_render() {
    local entry="$1"
    local got="${CAD_RENDER_LOCAL_OVERRIDE:-}"

    if [ "$got" = "$CAD_RENDER_LOCAL_OVERRIDE_SENTENCE" ]; then
        echo "[$entry] LOCAL RENDER OVERRIDE in effect — this pass is PREVIEW ONLY." >&2
        echo "[$entry]   Frames it publishes carry THIS box's renderer/GL stack." >&2
        echo "[$entry]   The committed tree is refreshed by CI, which re-baselines" >&2
        echo "[$entry]   every lane on a push — do NOT commit what this pass draws." >&2
        return 0
    fi

    if [ "$got" = "$CAD_RENDER_HOSTED_SENTENCE" ]; then
        echo "[$entry] HOSTED RENDER declared — this pass IS the canonical renderer." >&2
        echo "[$entry]   Frames it publishes carry the runner's renderer/GL stack," >&2
        echo "[$entry]   which is the stack the committed tree is baselined on." >&2
        echo "[$entry]   On main this run COMMITS what it draws; on a PR it reports" >&2
        echo "[$entry]   the drift against the committed lane. None of it is preview." >&2
        return 0
    fi

    {
        echo
        echo "REFUSING: renders are hosted now. $entry is not the default path."
        echo
        echo "THE DEFAULT WAY TO RE-RENDER IS TO LET CI DO IT."
        echo "ci.yml renders all four lanes on every push. A lane that no longer"
        echo "matches is RE-BASELINED for you — you never hand-commit cells:"
        echo
        echo "  git push          # CI renders and posts a neutral (\"!\") drift"
        echo "                    #   check naming the cells that differ"
        echo "  <merge the PR>    # main's own run commits the new cells"
        echo "  git pull          # on main, the frames are there"
        echo
        echo "A drift check is NOT a failure: if the render is what you intended"
        echo "it is a pass, needing no re-run and no second commit. PRs report"
        echo "and main commits — a bot commit on a PR branch would strand every"
        echo "other check on the parent commit, so it is deliberately not done."
        echo
        echo "To LOOK at the new cells before merging, take the run's artifact:"
        echo
        echo "  local-scripts/render-hosted.sh --lane <kernel|freecad|uv|wild>"
        echo
        echo "If the branch has no CI run yet (not pushed, no PR), render on"
        echo "demand instead:"
        echo
        echo "  local-scripts/render-hosted.sh --on-demand --lane <kernel|freecad|uv|wild|all>"
        echo
        echo "That triggers .github/workflows/render.yml on your PUSHED branch"
        echo "and polls it; that run re-baselines too, so it also ends in a"
        echo "pull. See demos/README.md, \"Off-box: the hosted lanes\"."
        echo
        echo "For preview-only local iteration (a scene you are still shaping,"
        echo "frames you do NOT intend to commit):"
        echo
        echo "  CAD_RENDER_LOCAL_OVERRIDE=$CAD_RENDER_LOCAL_OVERRIDE_SENTENCE $entry"
        echo
        echo "That is the sentence for a box. The hosted workflows declare a"
        echo "different one, in the file, at the step that renders; it says the"
        echo "pass IS the canonical renderer, and it is not for a box to type."
        echo
        if [ -n "$got" ]; then
            echo "(CAD_RENDER_LOCAL_OVERRIDE is set to '$got', which is neither"
            echo "accepted sentence. They are spelled out in full on purpose —"
            echo "see demos/hosted-render-guard.sh.)"
            echo
        fi
    } >&2
    exit 1
}

#!/usr/bin/env bash
# ONE COMMAND FOR A RENDER: dispatch -> poll -> install.
#
#   scripts/render-hosted.sh                      # all lanes, this branch
#   scripts/render-hosted.sh --lane uv            # one lane
#   scripts/render-hosted.sh --run 12345678       # pull an existing run
#
# Renders are hosted (`.github/workflows/render.yml`); the local entry
# points refuse without an explicit override (demos/hosted-render-guard.sh).
# This is the front end that makes that refusal reasonable: it triggers
# the workflow on your branch, prints per-job progress until the run
# settles, and — on success — downloads each lane's artifact and
# INSTALLS it back into the working tree at the committed path, so the
# frames land exactly where a local pass would have put them and are
# reviewed and committed the ordinary way.
#
# IT RENDERS THE PUSHED TREE, SO IT REFUSES AN UNPUSHED HEAD. A runner
# checks out a ref from the remote; it cannot see your working tree and
# it cannot see unpushed commits. Rendering "your branch" while the
# remote is three commits behind is the failure mode this exists to make
# impossible — the check is a hard refusal, not a warning, because the
# result of getting it wrong is a plausible-looking set of frames drawn
# from the wrong scenes. Uncommitted changes are a warning by the same
# logic one step down: they were definitely not rendered, but you can
# see them in `git status` next to the frames.
#
# BYTE-EXACTNESS IS THE CONTRACT. The whole design rests on an artifact
# being the file, not a rendering of the file: the committed PNGs carry
# provenance `tEXt` chunks that demos/check_render_provenance.py gates
# on, and a pipeline that re-encoded or re-stamped anything would launder
# them. actions/upload-artifact zips and `gh run download` unzips, both
# lossless — and `--verify` proves it end to end rather than asserting
# it, by round-tripping the wild lane (matplotlib Agg, pinned, no GL
# anywhere, so byte-identity is a real expectation) and diffing what
# came back against what is committed.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

WORKFLOW=render.yml
LANE=all
RUN_ID=""
REF=""
SCENE_TIMEOUT=""
INSTALL=1
VERIFY=0
# Outer stop. The FreeCAD legs are capped at 90 min by the workflow, the
# tour at 45; queueing on top of that is the slack.
POLL_BUDGET_MIN=150
POLL_INTERVAL=20

usage() {
    cat <<'EOF'
usage: scripts/render-hosted.sh [options]

  --lane <kernel|freecad|uv|wild|all>   which lane(s) to render (default: all)
  --ref <branch|tag|sha>                what to render (default: current branch)
  --run <id>                            pull an existing run; no new render
  --scene-timeout <seconds>             FreeCAD per-scene budget (default: 300)
  --no-install                          download to a temp dir, do not touch the tree
  --verify                              round-trip proof: assert the pulled bytes
                                        equal the committed ones (wild/uv lanes)
  --budget-min <n>                      give up polling after n minutes (default: 150)
  -h, --help

Artifacts land at their committed paths:
  kernel  -> demos/renders/        freecad -> demos/renders-freecad/
  uv      -> demos/renders-uv/     wild    -> demos/renders-wild/
EOF
}

die() { echo "render-hosted: $*" >&2; exit 1; }
say() { echo "==> $*"; }

while [ $# -gt 0 ]; do
    case "$1" in
        --lane) LANE="${2:?--lane needs a value}"; shift 2 ;;
        --ref) REF="${2:?--ref needs a value}"; shift 2 ;;
        --run) RUN_ID="${2:?--run needs a value}"; shift 2 ;;
        --scene-timeout) SCENE_TIMEOUT="${2:?--scene-timeout needs a value}"; shift 2 ;;
        --budget-min) POLL_BUDGET_MIN="${2:?--budget-min needs a value}"; shift 2 ;;
        --no-install) INSTALL=0; shift ;;
        --verify) VERIFY=1; shift ;;
        -h|--help) usage; exit 0 ;;
        *) usage >&2; die "unknown argument: $1" ;;
    esac
done

case "$LANE" in
    kernel|freecad|uv|wild|all) ;;
    *) die "--lane must be one of kernel, freecad, uv, wild, all (got '$LANE')" ;;
esac
command -v gh >/dev/null || die "gh is not installed (https://cli.github.com)"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated — run: gh auth login"

# Lane -> (artifact name, committed directory). One table, used by the
# download, the install and the verify, so a new lane is one line.
artifact_for() {
    case "$1" in
        kernel) echo "renders-kernel" ;;
        freecad) echo "renders-freecad" ;;
        uv) echo "renders-uv" ;;
        wild) echo "renders-wild" ;;
    esac
}
dir_for() {
    case "$1" in
        kernel) echo "demos/renders" ;;
        freecad) echo "demos/renders-freecad" ;;
        uv) echo "demos/renders-uv" ;;
        wild) echo "demos/renders-wild" ;;
    esac
}
lanes_of() {
    if [ "$1" = all ]; then echo "kernel freecad uv wild"; else echo "$1"; fi
}

# ---------------------------------------------------------------- dispatch

# The ref a run renders is the ref it was dispatched on; `gh run list
# --branch` is how we find it again, so both must agree.
if [ -z "$RUN_ID" ]; then
    if [ -z "$REF" ]; then
        REF="$(git rev-parse --abbrev-ref HEAD)"
        [ "$REF" != HEAD ] || die "detached HEAD — pass --ref explicitly"
    fi

    # THE PUSH CHECK. Fetch first so "already pushed" is a fact about the
    # remote right now, not about a stale remote-tracking ref.
    say "checking $REF is pushed"
    git fetch -q origin "$REF" 2>/dev/null \
        || die "origin has no ref '$REF' — push it first: git push -u origin $REF"
    local_head="$(git rev-parse HEAD)"
    remote_head="$(git rev-parse FETCH_HEAD)"
    if [ "$REF" = "$(git rev-parse --abbrev-ref HEAD)" ] && [ "$local_head" != "$remote_head" ]; then
        {
            echo
            echo "REFUSING: your HEAD is not what origin/$REF points at."
            echo
            echo "    local  HEAD       $local_head"
            echo "    origin/$REF   $remote_head"
            echo
            echo "The runner checks out the PUSHED tree — it cannot see local"
            echo "commits, so this render would draw scenes you are not looking at."
            echo
            echo "    git push origin $REF"
            echo
        } >&2
        exit 1
    fi
    if ! git diff --quiet HEAD -- || [ -n "$(git ls-files --others --exclude-standard)" ]; then
        echo "render-hosted: WARNING — the working tree is dirty." >&2
        echo "render-hosted:   Uncommitted changes are NOT in the render; the run" >&2
        echo "render-hosted:   draws $remote_head." >&2
    fi

    # Remember the newest existing run so the one we are about to create
    # is identifiable without racing on timestamps.
    before="$(gh run list --workflow "$WORKFLOW" --limit 1 --json databaseId \
        --jq '.[0].databaseId // 0')"

    say "dispatching $WORKFLOW (lanes=$LANE) on $REF"
    dispatch=(gh workflow run "$WORKFLOW" --ref "$REF" -f "lanes=$LANE")
    [ -z "$SCENE_TIMEOUT" ] || dispatch+=(-f "scene_timeout=$SCENE_TIMEOUT")
    "${dispatch[@]}"

    # The dispatch API is asynchronous: the run exists a moment later.
    for _ in $(seq 1 30); do
        RUN_ID="$(gh run list --workflow "$WORKFLOW" --branch "$REF" --limit 1 \
            --json databaseId --jq '.[0].databaseId // 0')"
        [ "$RUN_ID" = "$before" ] || [ "$RUN_ID" = 0 ] || break
        sleep 2
    done
    [ -n "$RUN_ID" ] && [ "$RUN_ID" != 0 ] && [ "$RUN_ID" != "$before" ] \
        || die "dispatched, but no new run appeared within 60s — check: gh run list --workflow $WORKFLOW"
fi

say "run $RUN_ID  ($(gh run view "$RUN_ID" --json url --jq .url))"

# -------------------------------------------------------------------- poll

# Per-job status lines, printed on CHANGE. Silence-aware: a run that is
# genuinely working prints nothing for many minutes (a FreeCAD leg is
# ~20 scenes at up to 300 s each), so an unchanged poll still emits a
# heartbeat on a slower cadence — the difference between "wedged" and
# "working" is visible without drowning the log.
deadline=$(( $(date +%s) + POLL_BUDGET_MIN * 60 ))
last_signature=""
last_print=0
while :; do
    now=$(date +%s)
    [ "$now" -lt "$deadline" ] \
        || die "gave up after ${POLL_BUDGET_MIN}m — the run is still going: gh run view $RUN_ID"

    view="$(gh run view "$RUN_ID" --json status,conclusion,jobs)"
    status="$(jq -r .status <<<"$view")"
    signature="$(jq -r '[.jobs[] | "\(.name)=\(.status)/\(.conclusion // "-")"] | join(",")' <<<"$view")"

    if [ "$signature" != "$last_signature" ] || [ $(( now - last_print )) -ge 300 ]; then
        printf '    [%s]\n' "$(date +%H:%M:%S)"
        jq -r '.jobs[] | "      \(.conclusion // .status | ascii_upcase)  \(.name)"' <<<"$view"
        last_signature="$signature"
        last_print="$now"
    fi

    [ "$status" != completed ] || break
    sleep "$POLL_INTERVAL"
done

conclusion="$(jq -r .conclusion <<<"$view")"
if [ "$conclusion" != success ]; then
    echo >&2
    echo "render-hosted: run $RUN_ID concluded '$conclusion'. Failing jobs:" >&2
    jq -r '.jobs[] | select(.conclusion != "success" and .conclusion != "skipped" and .conclusion != null)
           | "      \(.conclusion | ascii_upcase)  \(.name)"' <<<"$view" >&2
    echo >&2
    echo "render-hosted: --- last 60 lines of failing-step logs ---" >&2
    gh run view "$RUN_ID" --log-failed 2>/dev/null | tail -n 60 >&2 || true
    echo "render-hosted: --- full log: gh run view $RUN_ID --log ---" >&2
    die "hosted render failed"
fi
say "run $RUN_ID succeeded"

# ---------------------------------------------------------------- download

staging="$(mktemp -d)"
# shellcheck disable=SC2064  # $staging must expand now, not at trap time
trap "rm -rf '$staging'" EXIT

have=""
for lane in $(lanes_of "$LANE"); do
    art="$(artifact_for "$lane")"
    if gh run download "$RUN_ID" -n "$art" -D "$staging/$lane" 2>/dev/null; then
        say "pulled $art ($(find "$staging/$lane" -type f | wc -l) files)"
        have="$have $lane"
    else
        echo "render-hosted: no '$art' artifact on run $RUN_ID (lane skipped?)" >&2
    fi
done
[ -n "$have" ] || die "run $RUN_ID produced none of the requested lanes' artifacts"

# ------------------------------------------------------------------ verify

# THE ROUND-TRIP PROOF. Not a claim that hosted pixels match local ones
# — the FreeCAD lanes' do not, and render.yml says so — but that the
# ARTIFACT PATH is lossless: a byte-reproducible lane (wild: matplotlib
# Agg, pinned deps, no GL; uv: stdlib text) that came back through
# upload/zip/download/unzip must be byte-identical to what is committed,
# stamp chunks and all. If that ever stops holding, the provenance guard
# is being fed laundered files and every other lane's pull is suspect.
if [ "$VERIFY" = 1 ]; then
    checked=0
    for lane in $have; do
        case "$lane" in uv|wild) ;; *) continue ;; esac
        dir="$(dir_for "$lane")"
        say "verify: $lane — pulled bytes vs committed $dir/"
        while IFS= read -r rel; do
            src="$staging/$lane/$rel"
            dst="$dir/$rel"
            [ -f "$dst" ] || { echo "    NEW      $rel"; continue; }
            if cmp -s "$src" "$dst"; then
                echo "    identical $rel  ($(sha256sum <"$src" | cut -c1-16))"
                checked=$(( checked + 1 ))
            else
                echo "    DIFFERS  $rel" >&2
                die "byte-exactness broken on $dir/$rel — the artifact path is not lossless, \
or this lane's render is not reproducible. Do not trust a pulled tree until this is explained."
            fi
        done < <(cd "$staging/$lane" && find . -type f -printf '%P\n' | sort)
    done
    [ "$checked" -gt 0 ] || die "--verify had nothing to check (needs the uv or wild lane)"
    say "verify: $checked file(s) byte-identical through upload -> zip -> download"
fi

# ----------------------------------------------------------------- install

if [ "$INSTALL" = 0 ]; then
    keep="${staging}"
    trap - EXIT
    say "--no-install: artifacts left in $keep"
    exit 0
fi

for lane in $have; do
    dir="$(dir_for "$lane")"
    mkdir -p "$dir"
    # Copy, do not sync: a file present locally but absent from the
    # artifact is REPORTED, never deleted. A lane publishes wholesale, so
    # a leftover is a real signal (a retired scene, or a lane that only
    # half-ran) and silently deleting tracked files on the way in is the
    # wrong default for a convenience script.
    while IFS= read -r rel; do
        mkdir -p "$dir/$(dirname "$rel")"
        cp -p "$staging/$lane/$rel" "$dir/$rel"
    done < <(cd "$staging/$lane" && find . -type f -printf '%P\n' | sort)
    while IFS= read -r rel; do
        [ -e "$staging/$lane/$rel" ] \
            || echo "render-hosted: NOTE — $dir/$rel is committed but not in this run's artifact" >&2
    done < <(git ls-files "$dir" | sed "s|^$dir/||")
    say "installed $lane -> $dir/"
done

# The guard the committed tree is held to. Running it here means a pull
# that laundered or mixed provenance fails at the pull, not at review.
if command -v python3 >/dev/null; then
    say "provenance guard over the working tree"
    python3 demos/check_render_provenance.py
fi

echo
say "what moved (review, then commit normally):"
git -c color.status=always status --short -- demos/renders demos/renders-freecad \
    demos/renders-uv demos/renders-wild

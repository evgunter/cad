# Shared by this action and render.yml's `render-head`, which source it.
# Both need GH_TOKEN and GITHUB_REPOSITORY in the environment.
# shellcheck shell=bash

# What lies between a rendered commit ($1) and a branch tip ($2), as one word:
# `identical`; `bot` when every commit past the rendered one was authored by
# github-actions[bot] (the lanes' re-baselines); else `moved:<status>[:<shas>]`
# naming the compare status and the commits from anyone else.
past_rendered() {
  local cmp status foreign
  cmp="$(gh api "repos/${GITHUB_REPOSITORY}/compare/$1...$2" \
    --jq '[.status, ([.commits[] | select(.author.login != "github-actions[bot]") | .sha[:9]] | join(","))] | join(" ")')" || return 1
  read -r status foreign <<<"$cmp"
  if [ "$status" = identical ]; then
    echo identical
  elif [ "$status" = ahead ] && [ -z "${foreign:-}" ]; then
    echo bot
  else
    echo "moved:${status}${foreign:+:$foreign}"
  fi
}

# A completed failing check run: post_failure <name> <sha> <title> <summary>.
post_failure() {
  gh api -X POST "repos/${GITHUB_REPOSITORY}/check-runs" \
    -f "name=$1" -f "head_sha=$2" -f status=completed -f conclusion=failure \
    -f "output[title]=$3" -f "output[summary]=$4" --silent
}

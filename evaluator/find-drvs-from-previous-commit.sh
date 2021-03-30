#!/usr/bin/env nix-shell
#!nix-shell -i bash -p git nix jq

set -euo pipefail

echoerr() {
        echo -e $@ >&2
}

usage() {
        local name=$(basename $0)
        echoerr "$name Create derivations from a given commit"
        echoerr ""
        echoerr "USAGE:"
        echoerr "\t$name <commit-ish>"
        echoerr ""
        echoerr "ARGS:"
        echoerr "\tcommit-ish>\t\t  git commit-ish from nixpkgs repo"
        echoerr ""
        echoerr "EXAMPLE:"
        echoerr "\t$ $name abcdefg"
        exit 1
}

[ "$1" == "--help" -o "$1" = "-h" ] && usage

# validate environment
if [ -z "${NIXPKGS_PATH:-}" ]; then
        echoerr "error: Please set NIXPKGS_PATH before running script"
        echoerr ""
        usage
fi

if [ -z "${1:-}" ]; then
        echoerr "No rev was given, please give a valid rev"
        echoerr ""
        usage
fi
PR_NUMBER="$1"

#if test $(git cat-file --path="$NIXPKGS_PATH" -t commit) != commit; then
#        echoerr "The rev \'$rev\' was not a valid commit"
#        echoerr ""
#        usage
#fi

set -x

# create workdir
worktreedir=$(mktemp -d)

# remove tempdir, incase something fails
#trap "rm -rf $worktreedir" EXIT
pushd "$NIXPKGS_PATH"

git -c fetch.prune=false fetch --force https://github.com/NixOS/nixpkgs pull/$PR_NUMBER/head:refs/evaluator

git worktree add $worktreedir refs/evaluator

cd $worktreedir

list_drvs() {
        nix-env -I nixpkgs="$NIXPKGS_PATH" -f $worktreedir -qaP --no-name --drv-path --show-trace | sort
}

list_drvs > new_drvs.txt

git checkout HEAD^

list_drvs > old_drvs.txt

comm -23 old_drvs.txt new_drvs.txt > old_diff_drvs.txt
comm -13 old_drvs.txt new_drvs.txt > new_diff_drvs.txt


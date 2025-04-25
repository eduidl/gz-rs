#!/bin/bash

set -euo pipefail

function main() {
    local -ra branches=(
        ign-msgs8
        gz-msgs9
        gz-msgs10
        gz-msgs11
    )

    cd "$(dirname "$0")"

    for branch in "${branches[@]}"; do
        pushd ./crates/gz-msgs/3rdparty/"${branch}"
        git fetch
        git checkout "${branch}"
        git pull
        popd
    done

    cargo build -p gz-msgs --features generate
}

main

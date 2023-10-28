#!/bin/bash

set -euo pipefail

function main() {
    local -rA crate_branches=(
        ["gz-msgs8"]=ign-msgs8
        ["gz-msgs9"]=gz-msgs9
        ["gz-msgs10"]=gz-msgs10
    )

    cd "$(dirname "$0")"

    for crate in "${!crate_branches[@]}"; do
        pushd ./crates/"${crate}"/3rdparty/gz-msgs
        git fetch
        git checkout "${crate_branches[${crate}]}"
        git pull
        cargo build
        popd
    done
}

main

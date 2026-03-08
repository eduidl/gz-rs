#!/bin/bash

set -euo pipefail

function main() {
    declare -rA TAGS=(
        ["ign-msgs8"]="ignition-msgs8_8.7.1"
        ["gz-msgs9"]="gz-msgs9_9.5.1"
        ["gz-msgs10"]="gz-msgs10_10.3.2"
        ["gz-msgs11"]="gz-msgs11_11.1.0"
        ["gz-msgs12"]="gz-msgs12_12.0.1"
    )

    local repo="https://github.com/gazebosim/gz-msgs"
    local outdir="crates/gz-msgs/3rdparty"

    cd "$(dirname "$0")"

    for series in "${!TAGS[@]}"; do
        local tag="${TAGS[$series]}"
        echo "Downloading ${series} @ ${tag}..."
        rm -rf "${outdir}/${series}"
        mkdir -p "${outdir}/${series}"
        curl -sL "${repo}/archive/refs/tags/${tag}.tar.gz" \
            | tar -xz --strip-components=1 -C "${outdir}/${series}"
    done

    cargo build -p gz-msgs --features generate
}

main

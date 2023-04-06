#!/bin/bash

cd "$(dirname "$0")" || exit 1

for f in ./proto/gz/msgs/*.proto; do
    module=$(basename "$f" | sed s/.proto//)
    mapfile -t typenames < <(cat "$f" | grep -E '^message ' | sed -E 's/message ([a-zA-Z0-9_]+).*/\1/')

    for original in "${typenames[@]}"; do
        echo "use msgs::${module}::${original};"
    done
done

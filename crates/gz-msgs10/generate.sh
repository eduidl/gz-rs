#!/bin/bash

cd "$(dirname "$0")" || exit 1

for f in ./3rdparty/gz-msgs/proto/gz/msgs/*.proto; do
    module=$(basename "$f" | sed s/.proto//)
    mapfile -t typenames < <(grep -E '^message ' <"$f" | sed -E 's/message ([a-zA-Z0-9_]+).*/\1/')

    for original in "${typenames[@]}"; do
        echo "pub use msgs::${module}::${original};"
    done
done

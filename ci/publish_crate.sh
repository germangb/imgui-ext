#!/bin/bash
set -x

if [[ ${CI} -ne "" ]] || [[ ${TRAVIS} -ne "" ]]; then
    echo > ~/.cargo/credentials
    cat > ~/.cargo/credentials << EOF
[registry]
token = "$CRATESIO"
EOF
fi

pushd imgui_derive && cargo publish && popd

# wait for crates-io index to update...
sleep 4

cargo publish

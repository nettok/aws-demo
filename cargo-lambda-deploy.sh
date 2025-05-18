#!/usr/bin/env bash
set -e

if [ -z "$1" ]; then
  cargo lambda deploy
  exit 1
fi

if [ -n "$2" ]; then
  lambda=$2
else
  lambda=$1
fi

pushd $lambda > /dev/null
echo $(pwd)
cargo lambda deploy --binary-name $lambda
popd > /dev/null

#!/usr/bin/env bash
set -e

lambdas=("demo-lambda-axum" "demo-lambda-tasks")

for dir in "${lambdas[@]}"; do
  pushd $dir > /dev/null
  echo $(pwd)
  cargo lambda build --release --arm64
  mkdir -p ../target/lambda
  cp -r dist/$dir ../target/lambda/
  rm -rf dist
  popd > /dev/null
done

#!/bin/sh

cargo build --target wasm32-unknown-unknown --release

cp -r assets web/
cp target/wasm32-unknown-unknown/release/sus-runner.wasm web/sus-runner.wasm

cd web
git add . && \
git commit -m "Deploy web build" && \
git push -u origin gh-pages

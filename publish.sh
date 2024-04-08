#! /bin/bash

# https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/packaging-and-publishing.html

rm -rf ./pkg

wasm-pack build --scope mzzya --target nodejs

# wasm-pack login

cd pkg
npm publish --access=public --registry=https://registry.npmjs.org
cd ..
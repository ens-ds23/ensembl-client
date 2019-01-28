#! /bin/sh

set -ev

SRC=$HOME/ruemoglasm/demo1
DEST=$HOME/ensembl-new

cargo +nightly web deploy --target=wasm32-unknown-unknown --release
cp $SRC/target/deploy/hellostdweb.wasm $DEST/public/hellostdweb.wasm
cp $SRC/target/deploy/hellostdweb.js $DEST/src/scripts/browser/hellostdweb.js
sed -i -e 's/"hellostdweb.wasm"/"\/public\/hellostdweb.wasm"/g' $DEST/src/scripts/browser/hellostdweb.js

#!/bin/bash  
set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

[ ! -d "./wat/" ] && echo "Run at crate root please" && exit -1;


RUSTFLAGS=('' '' 'RUSTFLAGS="-C link-arg=--export-table" ' );
FLAGS=('--target=wasm32-unknown-unknown' '--target=wasm32-unknown-unknown' '--target=wasm32-unknown-unknown');

readonly RUSTFLAGS;
readonly FLAGS;


for i in "${!FLAGS[@]}"; do 
  flags="${FLAGS[$i]}";
  rust_flags="${RUSTFLAGS[$i]}";

  save_location="'./wat/${rust_flags}cargo build ${flags}.wat'";
  
  exec_command="${rust_flags}cargo build ${flags} > ${save_location}";
  echo "$exec_command";
  bash -c "${exec_command}";
done

#!/bin/bash

read -p "Enter the Rust source file path: " rust_file
read -p "Enter the Rust function type (e.g., async or leave empty): " func_type
func_type=${func_type:-}

if [[ -n "$func_type" ]]; then
    grep -nE "^\s*pub\s+$func_type\s+fn|^\s*$func_type\s+fn" "$rust_file"
else
    grep -nE "^\s*pub\s+fn|^\s*fn" "$rust_file"
fi


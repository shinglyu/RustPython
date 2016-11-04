#!/usr/bin/env bash
# Usage: test.sh <tests/test_case.py>

set -e

TESTCASE=$(basename ${1})
#TMP_FILE="test_${TESTCASE}.bytecode"
TMP_FILE="${1}.bytecode"

python compile_code.py "${1}" > "${TMP_FILE}"
cd RustPython 
cargo run "../${TMP_FILE}"

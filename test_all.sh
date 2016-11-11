#!/usr/bin/env bash
# set -e 

source venv/bin/activate
#TESTCASE=tests/variables.py
# TESTCASE=tests/variables.py
#TESTCASE=tests/minimum.py
fails=0
oks=0
fail_titles=$""
for TESTCASE in $(find tests -name \*.py -print)
do
  echo "TEST START: ${TESTCASE}"
  echo "--------------------------------"
  python compile_code.py $TESTCASE > $TESTCASE.bytecode
  cd RustPython
  cargo run ../$TESTCASE.bytecode
  if [ $? -ne 0 ]; then
    echo "== FAIL =="
    let fails=fails+1
    fail_titles=$"${fail_titles}\n${TESTCASE}"
  else
    echo "==  OK  =="
    let oks=oks+1
  fi
  cd ..
  echo "--------------------------------"

done

echo "Summary"
echo "================"
echo "${fails} fails, ${oks} passes"
echo ""
echo "Fails:"
echo "$(printf ${fail_titles})"
echo "================"

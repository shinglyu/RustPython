#!/usr/bin/env bash
# set -e 

source venv/bin/activate
#TESTCASE=tests/variables.py
# TESTCASE=tests/variables.py
#TESTCASE=tests/minimum.py
unexpected_count=0
expected_count=0
fail_titles=$""
for TESTCASE in $(find tests -name \*.py -print)
do
  echo "TEST START: ${TESTCASE}"
  echo "--------------------------------"
  FILENAME="$(basename ${TESTCASE})"
  xfail=false
  if [ "${FILENAME:0:6}" = "xfail_" ]; then
    echo "Expected FAILLLLLl"
    xfail=true
  fi


  python compile_code.py $TESTCASE > $TESTCASE.bytecode
  cd RustPython
  cargo run ../$TESTCASE.bytecode


  if [[ $? -ne 0 ]]; then
    if [ "${xfail}" = true ]; then
      echo "== FAIL as expected  =="
      let expected_count=expected_count+1
    else
      echo "== expect PASS, found FAIL =="
      let unexpected_count=unexpected_count+1
      fail_titles=$"${fail_titles}\n${TESTCASE}\texpected PASS, found FAIL"
    fi
  else
    if [ "${xfail}" = true ]; then
      echo "== expect FAIL, found PASS=="
      let unexpected_count=unexpected_count+1
      let unexpected_count=unexpected_count+1
      fail_titles=$"${fail_titles}\n${TESTCASE}\texpect FAIL, found PASS"
    else
      echo "== OK as expected  =="
      let expected_count=expected_count+1
    fi
  fi
  echo "${fail_titles}"
  cd ..
  echo "--------------------------------"

done

echo "Summary"
echo "================"
echo "${unexpected_count} unexpected, ${expected_count} expected"
echo ""
echo "unexpected results:"
printf "${fail_titles}"
echo ""
echo "================"

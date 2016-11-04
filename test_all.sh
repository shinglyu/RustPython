set -e 
#TESTCASE=tests/variables.py
# TESTCASE=tests/variables.py
#TESTCASE=tests/minimum.py
for TESTCASE in $(find tests -name \*.py -print)
do
  echo "TEST START: ${TESTCASE}"
  echo "--------------------------------"
  python compile_code.py $TESTCASE > $TESTCASE.bytecode
  cd RustPython
  cargo run ../$TESTCASE.bytecode
  cd ..
  echo "--------------------------------"
done

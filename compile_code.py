import bytecode
import sys
import json
import types


class CodeEncoder(json.JSONEncoder):
    def default(self, obj):
        if (isinstance(obj, types.CodeType)):
            return (
                {
                    "co_consts": obj.co_consts,
                    "co_names": obj.co_names,
                    "co_code": parse_co_code_to_str(obj)
                }
            )
        return json.JSONEncoder.default(self, obj)


def parse_co_code_to_str(code):
    c = bytecode.Bytecode().from_code(code)
    return list(
        map(lambda op: (op.name, op.arg if op.arg != bytecode.UNSET else None),
            c.to_concrete_bytecode())
    )


def main():

    filename = sys.argv[1]
    with open(filename, 'rU') as f:
        code = f.read()

    code = compile(code, filename, "exec")

    print(CodeEncoder().encode(code))

if __name__ == "__main__":
    main()

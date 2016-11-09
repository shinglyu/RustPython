import bytecode
import sys
import json
import types


class CodeEncoder(json.JSONEncoder):
    def default(self, obj):
        if (isinstance(obj, types.CodeType)):
            return (
                {
                    "co_consts": consts_to_rust_enum(obj.co_consts),
                    "co_names": obj.co_names,
                    "co_code": parse_co_code_to_str(obj)
                }
            )
        return json.JSONEncoder.default(self, obj)


def consts_to_rust_enum(consts):
    def capitalize_first(s):
        return s[0].upper() + s[1:]

    def const_to_rust_enum(const):
        return {capitalize_first(str(type(const).__name__)): const}
    return list(map(const_to_rust_enum, consts))


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

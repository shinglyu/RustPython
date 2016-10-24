# import dis
import byteplay
# from pprint import pprint

filename = "test.py"
with open(filename, 'rU') as f:
    code = f.read()

code = compile(code, filename, "exec")
print("CONSTS: {0}".format(code.co_consts))
print("VARNAMES: {0}".format(code.co_varnames))
# print(code.co_varnames)
# print(list(bytearray(code.co_code)))
# for elem in list(bytearray(code.co_code)):
#     print(dis.opname[elem])
# dis_output = dis.dis(code)

c = byteplay.Code.from_code(code)
# pprint(c.code)
# print(list(c.code))

for op in c.code:
    print("{0}, {1}".format(op[0], op[1]))

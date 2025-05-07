import json

groups = [
    "control/br",
    "x8/rsb",
    "x8/alu",
    "x16/lsm",
    "control/misc",
    "x16/alu",
    "x8/lsm",
]

unprefixedFunctionBodies = {}
prefixedFunctionBodies = {}

for group in groups:
    unprefixedFunctionBodies[group] = []
    prefixedFunctionBodies[group] = []

unprefixed = ""
unprefixedInstructionCycles = []
unprefixedFunctionDecl = []
unprefixedFunctionPtrs = []

prefixed = ""
prefixedInstructionCycles = []
prefixedFunctionDecl = []
prefixedFunctionPtrs = []

mapping = {
    "BIT": {"func_name": "bit_test_bit", "macro": "alu_flags_op"},
    "RES": {"func_name": "bit_reset_bit", "macro": "alu_u8_reg"},
    "SET": {"func_name": "bit_set_bit", "macro": "alu_u8_reg"},
}


def getFunctionBody(val):
    split = val["Name"].split(" ")

    if len(split) != 2:
        return ""

    operands = split[1].split(",")

    if len(operands) == 0:
        return ""

    arguments = (
        f"{operands[0].lower()}"
        if len(operands) == 1
        else f"{operands[0].lower()}, {operands[1].lower()}"
    )
    return f"{mapping[split[0]]}!(self, {mapping[split[0]]}, {arguments})"

    """match split[0]:
        case "BIT":
            return f"alu_flags_op!(self, bit_test_bit, {operands[0]}, {operands[1].lower()});"
        
        case "RES" or "SET":
            
        
        case [op]:
            pass
            #print("one")"""


# https://izik1.github.io/gbops/index.html
with open("dmgops.json", "r") as f:
    data = json.loads(f.read())

for i, val in enumerate(data["Unprefixed"]):
    name = val["Name"]
    group = val["Group"]
    length = str((int(val["Length"]) - 1))
    opcode = "0x{0:0{1}x}".format(i, 2)
    cycles = str(val["TCyclesNoBranch"])

    unprefixed += '{{ "{0}", {1} }}, // {2}\n'.format(name, length, opcode)

    if name == "UNUSED":
        unprefixedFunctionPtrs.append(f"{opcode} => self.opcode_unused(),")
        unprefixedInstructionCycles.append("-1")
    else:
        unprefixedFunctionPtrs.append(f"{opcode} => self.opcode_{opcode}(),")
        unprefixedFunctionDecl.append(f"auto opcode_{opcode}() -> void; // {name}")

        unprefixedFunctionBodies[group].append(
            f"/// {name}\n"
            f"pub(super) opcode_{opcode}(&mut self) {{\n"
            f"    \n"
            f"}}\n"
        )
        unprefixedInstructionCycles.append(cycles)

for i, val in enumerate(data["CBPrefixed"]):
    name = val["Name"]
    group = val["Group"]
    length = str((int(val["Length"]) - 1))
    opcode = "0x{0:0{1}x}".format(i, 2)
    cycles = str(val["TCyclesNoBranch"])

    prefixed += f'{{ "{name}", {length} }}, // {opcode}\n'

    if name == "UNUSED":
        prefixedFunctionPtrs.append(f"{opcode} => self.opcode_cb_unused(),")
        prefixedInstructionCycles.append("-1")
    else:
        prefixedFunctionPtrs.append(f"{opcode} => self.opcode_cb_{opcode}(),")
        prefixedFunctionDecl.append(f"auto opcode_cb_{opcode}() -> void; // {name}")

        getFunctionBody(val)
        prefixedFunctionBodies[group].append(
            f"/// {name}\n"
            f"pub(super) opcode_cb_{opcode}(&mut self) {{\n"
            f"    {getFunctionBody(val)}\n"
            f"}}\n"
        )
        prefixedInstructionCycles.append(cycles)


# Format stuff.
def formatStructure(list):
    for i in range(len(list)):
        if i != len(list) - 1:
            list[i] += ", "

        if ((i + 1) % 16) == 0:
            list[i] += "\n"


# formatStructure(unprefixedFunctionPtrs)
formatStructure(unprefixedInstructionCycles)

# formatStructure(prefixedFunctionPtrs)
formatStructure(prefixedInstructionCycles)

# Finally write them to their respective files.
with open("out/Unprefixed.txt", "w") as f:
    f.write(unprefixed)

with open("out/CBPrefixed.txt", "w") as f:
    f.write(prefixed)

with open("out/gen_CPU.h", "w") as f:
    f.write("#include <array>\n\n")
    f.write("class CPU {\n")

    f.write("auto opcode_UNUSED() -> void;\n\n")

    f.write("\n".join(unprefixedFunctionDecl))

    f.write("\n\n")
    f.write("\n".join(prefixedFunctionDecl))

    f.write("\n\n")
    f.write("typedef void (CPU::*Instruction)(void);\n\n")

    f.write("std::array<Instruction, 256> unprefixedInstructions = {\n")
    f.write("\n".join(unprefixedFunctionPtrs))
    f.write("};")

    f.write("\n\n")
    f.write("std::array<Instruction, 256> prefixedInstructions = {\n")
    f.write("\n".join(prefixedFunctionPtrs))
    f.write("};")

    f.write("\n\n")
    f.write("// Cycles for unprefixed instructions\n")
    f.write("std::array<int, 256> unprefixedCycles = {\n")
    f.write("".join(unprefixedInstructionCycles))
    f.write("};")

    f.write("\n\n")
    f.write("// Cycles for prefixed instructions\n")
    f.write("std::array<int, 256> prefixedCycles = {\n")
    f.write("".join(prefixedInstructionCycles))
    f.write("};")

    f.write("\n};\n")

for group in groups:
    fileName = "out/gen_cpu_" + group.replace("/", "_") + ".rs"

    if len(unprefixedFunctionBodies[group]) > 0:
        fileName = "out/gen_cpu_" + group.replace("/", "_") + ".rs"

        with open(fileName, "w") as f:
            f.write("\n".join(unprefixedFunctionBodies[group]))

    if len(prefixedFunctionBodies[group]) > 0:
        fileName = "out/gen_cpu_" + group.replace("/", "_") + "_prefixed.rs"

        with open(fileName, "w") as f:
            f.write("\n".join(prefixedFunctionBodies[group]))

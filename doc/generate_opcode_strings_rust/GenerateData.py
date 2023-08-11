import json

groups = [
    "control/br",
    "x8/rsb",
    "x8/alu",
    "x16/lsm",
    "control/misc",
    "unused",
    "x16/alu",
    "x8/lsm",
]

functionBodies = {}

for group in groups:
    functionBodies[group] = []

unprefixed = ""
unprefixedInstructionCycles = []
unprefixedFunctionDecl = []
unprefixedFunctionPtrs = []

prefixed = ""
prefixedInstructionCycles = []
prefixedFunctionDecl = []
prefixedFunctionPtrs = []


# https://izik1.github.io/gbops/index.html
with open("dmgops.json", "r") as f:
    data = json.loads(f.read())

for i, val in enumerate(data["Unprefixed"]):
    name = val["Name"]
    group = val["Group"]
    length = str((int(val["Length"]) - 1))
    opcode = "0x{0:0{1}X}".format(i, 2)
    cycles = str(val["TCyclesNoBranch"])

    unprefixed += '{{ "{0}", {1} }}, // {2}\n'.format(name, length, opcode)

    if name == "UNUSED":
        unprefixedFunctionPtrs.append("{0} => self.opcode_unused(),".format(opcode))
        unprefixedInstructionCycles.append("-1")
    else:
        unprefixedFunctionPtrs.append("{0} => self.opcode_{0}(),".format(opcode))
        unprefixedFunctionDecl.append(
            "auto opcode_{0}() -> void; // {1}".format(opcode, name)
        )

        functionBodies[group].append(
            "auto CPU::opcode_{0}() -> void {{\n  // {1}\n  // {2}\n}}\n".format(
                opcode, name, length
            )
        )
        unprefixedInstructionCycles.append(cycles)

for i, val in enumerate(data["CBPrefixed"]):
    name = val["Name"]
    group = val["Group"]
    length = str((int(val["Length"]) - 1))
    opcode = "0x{0:0{1}X}".format(i, 2)
    cycles = str(val["TCyclesNoBranch"])

    prefixed += '{{ "{0}", {1} }}, // {2}\n'.format(name, length, opcode)

    if name == "UNUSED":
        prefixedFunctionPtrs.append("{0} => self.opcode_cb_unused(),".format(opcode))
        prefixedInstructionCycles.append("-1")
    else:
        prefixedFunctionPtrs.append("{0} => self.opcode_cb_{0}(),".format(opcode))
        prefixedFunctionDecl.append(
            "auto opcode_CB_{0}() -> void; // {1}".format(opcode, name)
        )
        functionBodies[group].append(
            "auto CPU::opcode_CB_{0}() -> void {{\n  // {1}\n  // {2}\n\n}}\n".format(
                opcode, name, length
            )
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
with open("Unprefixed.txt", "w") as f:
    f.write(unprefixed)

with open("CBPrefixed.txt", "w") as f:
    f.write(prefixed)

with open("gen_CPU.h", "w") as f:
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

for group in functionBodies:
    fileName = "gen_CPU_" + group.replace("/", "_") + ".cpp"
    with open(fileName, "w") as f:
        f.write('#include "gen_CPU.h"\n\n')
        f.write("\n".join(functionBodies[group]))

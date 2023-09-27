from pathlib import Path

current_dir = Path(__file__).parent
test_roms_dir = current_dir.joinpath("../external/gameboy-test-roms")

mooneye_tests_dir = test_roms_dir.joinpath("mooneye-test-suite")

to_markdown = ""
to_tests = ""

for item in sorted(mooneye_tests_dir.rglob("**/*")):
    if item.is_dir():
        to_markdown += f"## {item.relative_to(mooneye_tests_dir)}\n\n"
        to_markdown += "| Test | State | Comment |\n"
        to_markdown += "| - | - | - |\n"

        to_tests += f"// {item.relative_to(mooneye_tests_dir)}\n"
        to_tests += "testcases! {\n"


        for file in sorted(item.iterdir()):
            if file.is_dir() or (file.suffix != ".gb" and file.suffix != ".gbc"):
                continue

            name = file.stem

            to_markdown += f"| {name} | :no: |  |\n"
            to_tests += f"    {name.lower().replace('-', '_')}(\"{file.relative_to(mooneye_tests_dir)}\");\n"

        to_markdown += "\n"
        to_tests += "}\n\n"


print(to_tests)

# possibly useful: ls -aUv -w1 *.gb | sed -e 's/\.gb$//'

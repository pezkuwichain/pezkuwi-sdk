import os

cargo_path = "bizinikiwi/utils/pezframe/benchmarking-cli/Cargo.toml"

with open(cargo_path, 'r') as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if line.strip().startswith("pezsc-chain-spec ="):
        if "workspace = true" in line:
             new_line = line.replace("pezsc-chain-spec", "sc-chain-spec", 1)
             new_line = new_line.replace("{", '{ package = "pezsc-chain-spec", ')
             new_lines.append(new_line)
        else:
             new_line = line.replace("pezsc-chain-spec", "sc-chain-spec", 1)
             if "package =" not in new_line:
                 new_line = new_line.replace("{", '{ package = "pezsc-chain-spec", ')
             new_lines.append(new_line)
    else:
        new_lines.append(line)

with open(cargo_path, 'w') as f:
    f.writelines(new_lines)

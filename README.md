# Oracli

Oracli is a tool for accessing the KU EECS 665 oracle with a CLI frontend. It will send 0 or more input files to a specific version of the oracle, and write the stdout and stderr returned by the oracle to files in the working directory. For `file.jeff`, oracli will create output files of the form `file.out.expected` and `file.err.expected`.

# Installation & Dependencies

### Ensure that cargo is installed:

Linux/MacOS: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Other: see https://www.rust-lang.org/tools/install


### On Linux, activate cargo for the current shell:

`source "$HOME/.cargo/env"`

### Lastly, install oracli with one of:

`cargo install --git https://github.com/DerrickQuinnKU/oracli.git`

`cargo install --git ssh://git@github.com/DerrickQuinnKU/oracli.git`

This will take a little while as it builds all dependencies from scratch


# Usage

Specify an oracle version: (e.g., `--o4`) as a command line arg and any number of file names ending in `.jeff`:

`oracli --o4 final.jeff final1.jeff final_final.jeff final_final_final.jeff`

`oracli --o4 *.jeff`

`oracli --o4 ../*.jeff`

Note that stdout and stderr files will be saved to the directory in which you run oracli.

# Issues

If you have any problems, email me: derrickquinn [at] ku [dot] edu, or talk to me, or create an issue on github.

# Misc

Why rust? could use `curl -F ...` - Answer (mostly Cargo):
- Cargo allows this to be run on systems without the exact same utilities (curl, grep, etc...), unlike bash. Oracli should work nearly anywhere that LLVM does.
- Cargo allows simple installation without root privileges (works on cycle server)
- Cargo allows me to reuse existing tools unlike C/C++/others

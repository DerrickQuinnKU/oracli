# Oracli

oracli will send 0 or more input files to a specific version of the oracle, and write the stdout and stderr returned by the oracle. For `file.jeff`, oracli will create output files of the form `file.out.expected` and `file.err.expected`.

## Installation

Ensure that cargo is installed:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

To activate cargo for the current shell:

`source "$HOME/.cargo/env"`

Lastly, install oracli with one of:

`cargo install --git https://github.com/DerrickQuinnKU/oracli.git`

`cargo install --git ssh://git@github.com/DerrickQuinnKU/oracli.git`

This will take a little while as it builds all dependencies from scratch


## Opertaion

Specify an oracle version: `--o[0-9]+` and any number of file names ending in `.jeff`:

`oracli --o4 final.jeff final1.jeff final_final.jeff final_final_final.jeff`

`oracli --o4 *.jeff`

`oracli --o4 ../*.jeff`

Note that stdout and stderr files will be saved to the directory in which you run oracli.

## Issues

If you have any problems, email me: derrickquinn [at] ku [dot] edu, or talk to me, or create an issue on github.

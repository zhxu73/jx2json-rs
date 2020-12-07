# jx2json-rs
[![Build Status](https://travis-ci.com/zhxu73/jx2json-rs.svg?token=tkdr7Uxjegk2FwKqbMny&branch=main)](https://travis-ci.com/zhxu73/jx2json-rs)

`jx2json` in rust

## JX
JX is a workflow language that is developed by [The Cooperative Computing Lab](https://ccl.cse.nd.edu/) at University of Norte Dame.
It is a part of [Makeflow](https://ccl.cse.nd.edu/software/makeflow/), a workflow system used in many scientific applications.

### Language Reference
https://cctools.readthedocs.io/en/latest/jx/jx/

## jx2json
`jx2json` is a utility shipped with the cctools (which includes Makeflow). It translates JX workflow into a JSON representation of the workflow.

The original implementation is in C. Link to the entrypoint of the utility [dttools/src/jx2json.c](https://github.com/cooperative-computing-lab/cctools/blob/master/dttools/src/jx2json.c)

## jx2json-rs
jx2json-rs is an attempt at implementing the `jx2json` utility in rustlang.

Repo to the translator https://github.com/zhxu73/jx2json-rs

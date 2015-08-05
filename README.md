# Seax Utilities

[![Join the chat at https://gitter.im/hawkw/seax](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/hawkw/seax?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)


[![Build Status](https://img.shields.io/travis/hawkw/seax_util/master.svg?style=flat-square)](https://travis-ci.org/hawkw/seax_util)
[![Coverage](https://img.shields.io/codecov/c/github/hawkw/seax_util/master.svg?style=flat-square)](http://codecov.io/github/hawkw/seax_util?branch=master)
[![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax_util/)
[![Latest release](https://img.shields.io/crates/v/seax_util.svg?style=flat-square)](https://crates.io/crates/seax_util)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/hawkw/seax/LICENSE)

Utility code for building other Seax platform libraries.

This crate contains the following modules:
 + `cell`: Contains the definitions of all Seax Virtual Machine cell types, including instruction, atom, and list cells
 + `list`: Contains the singly-linked list and stack implementations used by the Seax VM internally
 + `bytecode`: Contains functions for encoding and decoding Seax VM cells to and from Bytecode
 + `compiler_tools`: Contains reusable code for implementing compilers targeting Seax, including traits for abstract syntax nodes and symbol tables.

Contributing
------------

Seax is an open-source project and contributions are happily welcomed. For more information on how to contribute to Seax, please see the [CONTRIBUTING](https://github.com/hawkw/seax/blob/master/CONTRIBUTING.md) document on the main Seax repository.

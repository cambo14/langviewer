# langviewer
A tool to examine languages and convert them between representations (CFG, DFA, NFA, Regex, ...)

Langviewer is currently very early in implementation with only a full DFA implementation currently being worked on with other forms to follow

# Documentation
Documentation for langviewers source code can be viewed [here](https://cambo14.github.io/langviewer/)

# Licensing
langviewer is licensed under the [GNU AGPLv3](./LICENCE)

# Contributing
Information about contributing to langviewer can be found in [CONTRIBUTING.md](./CONTRIBUTING.md)

## Custom Commands
The Cargo project for langviewer has the following custom `cargo` commands
 * `own-docs`: generates documentation for langviewer using RustDoc only generating documentation for langviewer, linking to external documentation hosts for any external crates used.
# Contributing

### Note:
langviewer is currently very early in its development and as such the information & processes contained here, as well as the intended features and roadmap of langviewer are subject to change at any time with very little if any notice.

## Overview:
langviewer is intended to be an application for viewing and analysing various grammars and languages, across various representations. At the moment the current direction for development is to finish implementing a complete DFA viewer/editor, then implementing a way to analyse and save the DFA. Next either categorisation of parser category (LR(k), LL(k), SLR(k), LALR(k)), and generation of parse tables, or support for other grammars and languages, like CFGs, NFAs, Regex, TMs, and so on is to be implemented

## How to contribute:
Feel free to create a PR for any issue currently listed in the repository (please first confirm exact requirements/specifications if they are unclear or not appropriately detailed). If you wish to implement a new feature not listed in any active issue, first create an issue with the enhancement label. If it is then marked as accepted then feel free to work on implementing it and create an appropriate PR

## Pull Requests:
When creating a PR to contribute to langviewer, ensure that a corresponding issue exists and is assigned to you. Also ensure that the following elements are included in the PR:

 * A link to the issue the corresponds to the PR
 * An explanation of what you have done and why
 * Screenshots or screen recordings demonstrating the PR actually implements what is described in the issue
 * Appropriate tests for anything new implemented or functionality that is changed
 * Full valid RustDoc style documentation for any code added

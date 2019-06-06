# 6. Coding Style

Date: 2018-09-05

## Status

Accepted

## Context

Coding style is one of the hot debating topics among members in any team. However, there needs to be a common consensus on what the style of coding needs to be, whether it be JavaScript, Python or CSS.

Since the frontend uses TypeScript and SCSS (SASS) for coding, there needs to be coding style rules enforced for these two languages. This would make commits in the future to not to have too many coding style changes, be it adding new lines or formatting with different tab spaces.

## Decision

Three tools are used to enforce this:

1. [ESLint](https://eslint.org/): for JavaScript and TypeScript rules
2. [Stylelint](https://stylelint.io/): for SASS/SCSS rules (Stylelint is originally for CSS but can be used for SASS too)
3. [Prettier](https://prettier.io/): the code formatting tool which will automatically format code in development mode

Supplementary tools such as extensions/plugins for IDEs can be used too. The recommended extensions for VS Code are:

1. [vscode-eslint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
2. [vscode-stylelint](https://marketplace.visualstudio.com/items?itemName=shinnn.stylelint)
3. [Prettier - Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)

## Consequences

The coding style can be enforced by the build tools itself during development. This should reduce commits necessary to format the code and also the team will get used to the coding style rules.
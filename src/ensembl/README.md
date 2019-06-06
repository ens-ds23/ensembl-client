# Ensembl 2020 Frontend

## Setup

The tools that need setting up are Node.js and the IDE. To setup Node.js you only need to run `npm install`. This will install all the necessary NPM packages for app development and the build.

### Build Scripts

There are several script commands that have been baked into the NPM configuration. They have been briefly described below:

- `npm run copy-dotenv` - Makes a copy of `.env.example` file named as `.env` in case it doesn't exist. This command will always run before `serve:dev`.
- `npm start` - This should start the development server via `webpack`. There is no need to do any compilation manually, as `webpack` will take care of it.
- `npm run serve:dev` - This does the same as `npm start`. The latter is actually an alias to this script.
- `npm run serve:prod` - This runs the built production site locally using `http`.
- `npm run serve:prod:secure` - Runs the build production site locally securely using `https`. You will need to run `certify` before running this, in case you already haven't generated an SSL certificate.
- `npm run build` - Runs the production build. It will initially delete the existing local production build and replace it with the new one.
- `npm run deploy` - Runs `deploy.js` file to deploy the production build into the master machine (`ves-hx2-70`). You will need to pass the full address of the machine name along with your username as an argument. 
- `npm run certify` - Runs `setup-ssl.js` to create a local SSL certificate to run the production build on `HTTPS`. There are two files that are created for this: `localhost.crt` and `localhost.key`.
- `npm run lint` - Runs both `lint:scripts` and `lint:style`. There is no need to run any of these lint scripts during development. This is because, each time a change is made to the code, the linters run along with the build.
- `npm run lint:scripts` - Runs ESLint to check code errors in TypeScript and React. Also, suggests improvements to the code.
- `npm run lint:styles` - Does the same as above through `stylelint` for the SASS code.
- `npm run storybook` - Starts the Storybook application
- `npm run deploy-docs` — Builds the Storybook application and deploys it, along with the docs on Genome browser, to Github Pages ([link](https://ensembl.github.io/ensembl-client))
- `npm run check-types` – Runs typescript compiler to check for type correctness
- `npm jest` - Runs `jest`in non-verbose mode.
- `npm test` - Runs `jest` to check whether the unit tests pass in verbose mode.
- `npm test:watch` - Runs `jest` on watch mode.
- `npm run coverage` - Updates the `jest` coverage of the React.js code, and shows the test coverage.
- `npm run snapshot` - Updates the `jest` snapshots. You will sometimes need to run this after a change is made to the React.js code. However, first check whether the unit tests are passing after updating them. If you get any errors related to `jest` snapshots, you will then need to run this script.


### IDE Setup

The IDE setup may change depending on which one you have chosen. This document will describe setting VS Code for development. The default setup of VS Code is sufficient for development, as it has great support for TypeScript and React.js. However, its functionality can be extended further with these VS Code extensions:

- [GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens): Extends the existing support for `git`.
- [Prettier - Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode): Automated code formatting. Prettier is built into `webpack`, however it is not fit for formatting the code on the fly. It is currently disabled in `webpack`, so using this extension is recommended.
- [SCSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=mrmlnc.vscode-scss): The support for SCSS, the alternate syntax of SASS, is not great in VS Code. This extension adds this support.
- [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint): Adds ESLint support to VS Code.
- [stylelint](https://marketplace.visualstudio.com/items?itemName=shinnn.stylelint): Adds `stylelint` support to VS Code.
- [SVG Viewer](https://marketplace.visualstudio.com/items?itemName=cssho.vscode-svgviewer): By default, VS Code will show SVG images only as a code file. This extension will add functionality, to let you view the SVG files as images.
- [snapshot-tools](https://marketplace.visualstudio.com/items?itemName=asvetliakov.snapshot-tools): Enable linking of Jest generated snapshots with the Jest unit tests.
- [vscode-icons](https://marketplace.visualstudio.com/items?itemName=robertohuertasm.vscode-icons): By default, VS Code does not have icon support for many of the configuration files and formats used in the code. This extension adds those missing icons and more.

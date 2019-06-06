# 4. Unit Tests

Date: 2018-08-30

## Status

Accepted

## Context

When changes are made after the initial implementation of components, there is a chance of code breakage. This needs to be mitigated.

## Decision

Write unit tests using Jest and Enzyme.

[Jest](https://jestjs.io/) is used as the testing framework and [`ts-jest`](https://kulshekhar.github.io/ts-jest/) is used as the TypeScrpit preprocessor to run them..

[Enzyme](https://airbnb.io/enzyme/) is a testing utility used to test the output of React components. A recent and promising alternative library for unit-testing React is react-testing-library. Its philosophy differs from Enzyme in that it offers the developer a very limited API and encourages testing of components only after they have been mounted in the DOM.

Enzyme's strong points are historically larger community (although `react-testing-library` is steadily getting traction in React community and even is first on the list of unit testing libraries suggested by React docs) and extensive API with numerous convenience methods. Its weakness is that it provides an extra level of abstraction over React, and therefore lags behind React in terms of supported features (as of this writing, hooks are not supported for shallow rendering; React.Suspense, React.lazy and React.memo are not supported at all). Some may argue that the extensive API is also a weakness, because it offers the developer multiple ways of doing a single thing, and thus may be confusing.

The strength of `react-testing-library` is that it is a very thin wrapper over React's own test renderer, and therefore supports almost all features of modern React. It has powerful wait and waitForElement utilities that are very useful for testing of asynchronous effects. Some may argue that its minimalist API is its strength rather than its weakness, because it discourages the practice of testing implementation details rather than behaviour of a component common among Enzyme users.

Despite the strengths of `react-testing-library`, we are currently sticking with Enzyme.

## Consequences

Unit tests can act as code documentation. Also, they can prevent code breakage. However, this would prolong development of the application and will need to be enforced to get the full benefit of testing.

Jest is developed by Facebook Inc. and used by many React developers. The same can be said of Enzyme, which is developed by Airbnb. This should enable the team to find help easily, as there is already a community built around these tools.

All unit tests are written in TypeScript and this adds some complexity to the testing code. Also, setting up the code and fixing issues related to TypeScript integration with the tools can be tricky.

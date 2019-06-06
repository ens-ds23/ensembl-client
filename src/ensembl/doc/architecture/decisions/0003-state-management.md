# 3. State Management

Date: 2018-08-30

## Status

Accepted

## Context

The state of data and UI will grow in the application as the development goes on. There needs to be a good state management mechanism to handle this.

## Decision

There are various ways to do state management in a React application. React itself provides two mechanisms:

- component local state (useful for tracking transient localised state that most parts of the application are not concerned with)
- React context (useful for sharing state between unrelated components)

Both of these mechanisms, while useful, are generally insufficient for tracking state in a large application, which resulted in proliferation of numerous third-party libraries for state management. Of those, the two most notable are:

- Redux
- MobX

These two libraries represent two different philosophies of state management (and programming in general). Redux follows the functional programming approach, in which state itself is immutable, and state changes are a series of new copies of state produced in response to message events (actions) dispatched within the store. In MobX, state is a mutable object that is changed via getters and setters (in latest implementations, proxies), which also notify subscriptions of the change.

While the MobX approach is generally regarded as easier to use, we feel that it also makes this library more **magical** and therefore more difficult to debug as compared to Redux. Also, Redux's reliance on events makes it a convenient target for plugging various kinds of middleware (i.e. analytics middleware, redux-saga or redux-observable) for better encapsulation of state-dependent logic.

## Consequences

Redux does not use any magic and instead uses functions and objects to create its various components i.e. actions, reducers, etc. This should make it straightforward to implement it. However, it does contain a lot of boilerplate.

Using `typesafe-actions` will help reduce the boilerplate, especially for action creators. Also, it provides typesafe utilities for Redux code. `redux-action` was another utility library for Redux that was considered. However, the TypeScript support of it was not great, hence why it was not chosen over `typesafe-actions`.

Selectors will be used to improve maintenance and managing of state. Also, `reselect` can be used for complex reducers and should improve performance through memoization.

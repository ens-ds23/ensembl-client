import { applyMiddleware, createStore } from 'redux';
import thunk from 'redux-thunk';
import { composeWithDevTools } from 'redux-devtools-extension';
import { createBrowserHistory } from 'history';
import { routerMiddleware } from 'connected-react-router';
import { StateType } from 'typesafe-actions';

import createRootReducer from './root/rootReducer';
import { analyticsMiddleWare } from './analyticsMiddleware';

export const history = createBrowserHistory();

const composeEnhancers = composeWithDevTools({});

const rootReducer = createRootReducer(history);

export type RootState = StateType<typeof rootReducer>;

export default function configureStore(preloadedState?: any) {
  const store = createStore(
    rootReducer,
    preloadedState,
    composeEnhancers(
      applyMiddleware(routerMiddleware(history), thunk, analyticsMiddleWare)
    )
  );

  return store;
}

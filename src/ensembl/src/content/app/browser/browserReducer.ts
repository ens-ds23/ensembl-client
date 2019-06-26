import { combineReducers } from 'redux';
import { ActionType, getType } from 'typesafe-actions';

import { RootAction } from 'src/objects';
import * as browserActions from './browserActions';
import * as drawerActions from './drawer/drawerActions';
import * as trackPanelActions from './track-panel/trackPanelActions';
import {
  BrowserState,
  defaultBrowserState,
  BrowserOpenState,
  BrowserLocationState,
  defaultBrowserLocationState,
  BrowserNavState,
  defaultBrowserNavState,
  TrackConfigState,
  defaultTrackConfigState,
  BrowserEntityState,
  defaultBrowserEntityState
} from './browserState';

export function browserInfo(
  state: BrowserState = defaultBrowserState,
  action: ActionType<RootAction>
): BrowserState {
  switch (action.type) {
    case getType(browserActions.updateBrowserActivated):
      return { ...state, browserActivated: action.payload };
    case getType(trackPanelActions.toggleTrackPanel):
      return {
        ...state,
        browserOpenState: action.payload
          ? BrowserOpenState.SEMI_EXPANDED
          : BrowserOpenState.EXPANDED
      };
    case getType(drawerActions.toggleDrawer):
      return {
        ...state,
        browserOpenState: action.payload
          ? BrowserOpenState.COLLAPSED
          : BrowserOpenState.SEMI_EXPANDED
      };
    default:
      return state;
  }
}

export function browserEntity(
  state: BrowserEntityState = defaultBrowserEntityState,
  action: ActionType<typeof browserActions>
): BrowserEntityState {
  switch (action.type) {
    case getType(browserActions.updateBrowserActiveGenomeId):
      return { ...state, activeGenomeId: action.payload };
    case getType(browserActions.updateBrowserActiveEnsObjectId):
      return { ...state, activeEnsObjectId: action.payload };
    default:
      return state;
  }
}

export function browserNav(
  state: BrowserNavState = defaultBrowserNavState,
  action: ActionType<typeof browserActions>
) {
  switch (action.type) {
    case getType(browserActions.toggleBrowserNav):
      return { ...state, browserNavOpened: !state.browserNavOpened };
    case getType(browserActions.updateBrowserNavStates):
      return { ...state, browserNavStates: action.payload };
    default:
      return state;
  }
}

export function browserLocation(
  state: BrowserLocationState = defaultBrowserLocationState,
  action: ActionType<typeof browserActions>
) {
  switch (action.type) {
    case getType(browserActions.updateChrLocation):
      return { ...state, chrLocation: action.payload };
    case getType(browserActions.updateDefaultChrLocation):
      return {
        ...state,
        defaultChrLocation: action.payload
      };
    case getType(browserActions.toggleGenomeSelector):
      return { ...state, genomeSelectorActive: action.payload };
    default:
      return state;
  }
}

export function trackConfig(
  state: TrackConfigState = defaultTrackConfigState,
  action: ActionType<typeof browserActions>
) {
  switch (action.type) {
    case getType(browserActions.updateCogList):
      return { ...state, browserCogList: action.payload };
    case getType(browserActions.updateCogTrackList):
      return { ...state, browserCogTrackList: action.payload };
    case getType(browserActions.updateSelectedCog):
      return { ...state, selectedCog: action.payload };
    case getType(browserActions.updateApplyToAll):
      return { ...state, applyToAll: action.payload };
    case getType(browserActions.updateTrackConfigNames):
      return {
        ...state,
        trackConfigNames: {
          ...state.trackConfigNames,
          [action.payload[0]]: action.payload[1]
        }
      };
    case getType(browserActions.updateTrackConfigLabel):
      return {
        ...state,
        trackConfigLabel: {
          ...state.trackConfigLabel,
          [action.payload[0]]: action.payload[1]
        }
      };
    default:
      return state;
  }
}

export default combineReducers({
  browserInfo,
  browserEntity,
  browserLocation,
  browserNav,
  trackConfig
});

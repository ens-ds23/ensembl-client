import { ActionType, getType } from 'typesafe-actions';

import { TrackPanelState, defaultTrackPanelState } from './trackPanelState';
import * as trackPanelActions from './trackPanelActions';

export default function trackPanel(
  state: TrackPanelState = defaultTrackPanelState,
  action: ActionType<typeof trackPanelActions>
): TrackPanelState {
  switch (action.type) {
    case getType(trackPanelActions.toggleTrackPanel):
      const trackPanelOpened =
        action.payload === undefined ? !state.trackPanelOpened : action.payload;

      return {
        ...state,
        trackPanelOpened
      };
    case getType(trackPanelActions.openTrackPanelModal):
      return {
        ...state,
        trackPanelModalOpened: true,
        trackPanelModalView: action.payload
      };
    case getType(trackPanelActions.closeTrackPanelModal):
      return {
        ...state,
        trackPanelModalOpened: false,
        trackPanelModalView: ''
      };
    case getType(trackPanelActions.selectBrowserTab):
      return {
        ...state,
        selectedBrowserTab: Object.assign(
          {},
          state.selectedBrowserTab,
          action.payload
        ),
        trackPanelModalOpened: false,
        trackPanelModalView: ''
      };
    default:
      return state;
  }
}

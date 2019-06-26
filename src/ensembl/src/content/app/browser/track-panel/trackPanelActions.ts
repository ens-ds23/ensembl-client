import { createAction } from 'typesafe-actions';
import { ThunkAction } from 'redux-thunk';
import { Action, Dispatch, ActionCreator } from 'redux';

import { RootState } from 'src/store';
import { getTrackPanelAnalyticsObject } from 'src/analyticsHelper';
import { TrackType } from './trackPanelConfig';
import browserStorageService from '../browser-storage-service';
import { getBrowserActiveGenomeId } from '../browserSelectors';

export const toggleTrackPanel = createAction(
  'track-panel/toggle-track-panel',
  (resolve) => {
    return (trackPanelOpened?: boolean) =>
      resolve(
        trackPanelOpened,
        getTrackPanelAnalyticsObject('User Interaction')
      );
  }
);

export const selectBrowserTab = createAction(
  'track-panel/select-browser-tab',
  (resolve) => {
    return (selectedBrowserTabForGenome: { [genomeId: string]: TrackType }) => {
      return resolve(selectedBrowserTabForGenome);
    };
  }
);

export const selectBrowserTabAndSave: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (selectedBrowserTab: TrackType) => (
  dispatch: Dispatch,
  getState: () => RootState
) => {
  const activeGenomeId = getBrowserActiveGenomeId(getState());
  const selectedBrowserTabForGenome = {
    [activeGenomeId]: selectedBrowserTab
  };

  dispatch(selectBrowserTab(selectedBrowserTabForGenome));
  browserStorageService.updateSelectedBrowserTab(selectedBrowserTabForGenome);
};

export const openTrackPanelModal = createAction(
  'track-panel/open-track-panel-modal',
  (resolve) => {
    return (trackPanelModalView: string) =>
      resolve(
        trackPanelModalView,
        getTrackPanelAnalyticsObject('User Interaction')
      );
  }
);

export const closeTrackPanelModal = createAction(
  'track-panel/close-track-panel-modal',
  (resolve) => {
    return () => resolve(undefined, getTrackPanelAnalyticsObject('Navigation'));
  }
);

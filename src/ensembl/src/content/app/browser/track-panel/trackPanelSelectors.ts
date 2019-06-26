import { RootState } from 'src/store';

export const getTrackPanelModalOpened = (state: RootState) =>
  state.trackPanel.trackPanelModalOpened;

export const getTrackPanelModalView = (state: RootState) =>
  state.trackPanel.trackPanelModalView;

export const getSelectedBrowserTab = (state: RootState) =>
  state.trackPanel.selectedBrowserTab;

export const getTrackPanelOpened = (state: RootState) =>
  state.trackPanel.trackPanelOpened;

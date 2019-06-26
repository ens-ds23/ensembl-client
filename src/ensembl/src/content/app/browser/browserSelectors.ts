import { RootState } from 'src/store';
import { BrowserOpenState, BrowserNavStates, CogList } from './browserState';
import { getQueryParamsMap } from 'src/global/globalHelper';

export const getBrowserActivated = (state: RootState): boolean =>
  state.browser.browserInfo.browserActivated;

export const getBrowserOpenState = (state: RootState): BrowserOpenState =>
  state.browser.browserInfo.browserOpenState;

export const getBrowserActiveGenomeId = (state: RootState): string =>
  state.browser.browserEntity.activeGenomeId;

export const getBrowserActiveEnsObjectId = (state: RootState) =>
  state.browser.browserEntity.activeEnsObjectId;

export const getBrowserQueryParams = (
  state: RootState
): { [key: string]: string } => getQueryParamsMap(state.router.location.search);

export const getBrowserNavOpened = (state: RootState): boolean =>
  state.browser.browserNav.browserNavOpened;

export const getBrowserNavStates = (state: RootState): BrowserNavStates =>
  state.browser.browserNav.browserNavStates;

export const getChrLocation = (state: RootState) =>
  state.browser.browserLocation.chrLocation;

export const getDefaultChrLocation = (state: RootState) =>
  state.browser.browserLocation.defaultChrLocation;

export const getGenomeSelectorActive = (state: RootState): boolean =>
  state.browser.browserLocation.genomeSelectorActive;

export const getBrowserCogList = (state: RootState): number =>
  state.browser.trackConfig.browserCogList;

export const getBrowserCogTrackList = (state: RootState): CogList =>
  state.browser.trackConfig.browserCogTrackList;

export const getBrowserSelectedCog = (state: RootState): string =>
  state.browser.trackConfig.selectedCog;

export const getTrackConfigNames = (state: RootState): any =>
  state.browser.trackConfig.trackConfigNames;

export const getTrackConfigLabel = (state: RootState): any =>
  state.browser.trackConfig.trackConfigLabel;

export const getApplyToAll = (state: RootState): boolean =>
  state.browser.trackConfig.applyToAll;

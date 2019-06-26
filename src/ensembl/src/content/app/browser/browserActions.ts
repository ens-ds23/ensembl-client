import { createAction } from 'typesafe-actions';
import { Dispatch, ActionCreator, Action } from 'redux';
import { ThunkAction } from 'redux-thunk';

import config from 'config';

import { BrowserNavStates, ChrLocation, CogList } from './browserState';
import {
  getBrowserActiveGenomeId,
  getBrowserActiveEnsObjectId,
  getDefaultChrLocation,
  getChrLocation
} from './browserSelectors';
import { getBrowserAnalyticsObject } from 'src/analyticsHelper';
import browserStorageService from './browser-storage-service';
import { RootState } from 'src/store';

export const updateBrowserActivated = createAction(
  'browser/update-browser-activated',
  (resolve) => {
    return (browserActivated: boolean) =>
      resolve(browserActivated, getBrowserAnalyticsObject('Default Action'));
  }
);

export const activateBrowser = (browserEl: HTMLDivElement) => {
  return (dispatch: Dispatch) => {
    const { protocol, host: currentHost } = location;
    const host = config.apiHost || currentHost;
    const activateEvent = new CustomEvent('bpane-activate', {
      bubbles: true,
      detail: {
        'config-url': `${protocol}${host}/browser/config`,
        key: 'main'
      }
    });

    browserEl.dispatchEvent(activateEvent);

    dispatch(updateBrowserActivated(true));
  };
};

export const updateBrowserActiveGenomeId = createAction(
  'browser/update-active-genome-id',
  (resolve) => {
    return (activeGenomeId: string) =>
      resolve(activeGenomeId, getBrowserAnalyticsObject('Navigation'));
  }
);

export const updateBrowserActiveGenomeIdAndSave: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (activeGenomeId: string) => (dispatch: Dispatch) => {
  dispatch(updateBrowserActiveGenomeId(activeGenomeId));
  browserStorageService.saveActiveGenomeId(activeGenomeId);
};

export const updateBrowserActiveEnsObjectId = createAction(
  'browser/update-active-ens-object-id',
  (resolve) => {
    return (activeEnsObjectId: { [objectId: string]: string }) =>
      resolve(activeEnsObjectId, getBrowserAnalyticsObject('Navigation'));
  }
);

export const updateBrowserActiveEnsObjectIdAndSave: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (activeEnsObjectId: string) => {
  return (dispatch: Dispatch, getState: () => RootState) => {
    const currentActiveEnsObjectId = getBrowserActiveEnsObjectId(getState());
    const updatedActiveEnsObjectId = { ...currentActiveEnsObjectId };
    const genomeId = activeEnsObjectId.split(':')[0];
    updatedActiveEnsObjectId[genomeId] = activeEnsObjectId;

    dispatch(updateBrowserActiveEnsObjectId(updatedActiveEnsObjectId));

    browserStorageService.updateActiveEnsObjectId(updatedActiveEnsObjectId);
  };
};

export const toggleBrowserNav = createAction(
  'browser/toggle-browser-navigation',
  (resolve) => {
    return () => resolve(undefined, getBrowserAnalyticsObject('Navigation'));
  }
);

export const updateBrowserNavStates = createAction(
  'browser/update-browser-nav-states',
  (resolve) => {
    return (browserNavStates: BrowserNavStates) =>
      resolve(browserNavStates, getBrowserAnalyticsObject('Navigation'));
  }
);

export const updateChrLocation = createAction(
  'browser/update-chromosome-location',
  (resolve) => {
    return (chrLocationData: { [genomeId: string]: ChrLocation }) =>
      resolve(chrLocationData);
  }
);

export const setChrLocation: ActionCreator<
  ThunkAction<any, any, null, Action<string>>
> = (chrLocation: ChrLocation) => {
  return (dispatch: Dispatch, getState: () => RootState) => {
    const genomeId = getBrowserActiveGenomeId(getState());
    const currentChrLocation = getChrLocation(getState());
    const updatedChrLocation = { ...currentChrLocation };
    updatedChrLocation[genomeId] = chrLocation;

    dispatch(updateChrLocation(updatedChrLocation));
    browserStorageService.updateChrLocation(updatedChrLocation);
  };
};

export const updateDefaultChrLocation = createAction(
  'browser/update-default-chromosome-location',
  (resolve) => {
    return (chrLocationData: { [genomeId: string]: ChrLocation }) =>
      resolve(chrLocationData, getBrowserAnalyticsObject('User Interaction'));
  }
);

export const changeBrowserLocation: ActionCreator<
  ThunkAction<any, any, null, Action<string>>
> = (chrLocation: ChrLocation, browserEl: HTMLDivElement) => {
  return (dispatch: Dispatch, getState: () => RootState) => {
    const [chrCode, startBp, endBp] = chrLocation;
    const genomeId = getBrowserActiveGenomeId(getState());

    const stickEvent = new CustomEvent('bpane', {
      bubbles: true,
      detail: {
        stick: `${genomeId}:${chrCode}`
      }
    });

    browserEl.dispatchEvent(stickEvent);

    if (startBp > 0 && endBp > 0) {
      const gotoEvent = new CustomEvent('bpane', {
        bubbles: true,
        detail: {
          goto: `${startBp}-${endBp}`
        }
      });

      browserEl.dispatchEvent(gotoEvent);
    }

    const currentChrLocation = getChrLocation(getState());
    const updatedChrLocation = { ...currentChrLocation };
    updatedChrLocation[genomeId] = chrLocation;

    dispatch(updateChrLocation(updatedChrLocation));
    browserStorageService.updateChrLocation(updatedChrLocation);

    const currentDefaultChrLocation = getDefaultChrLocation(getState());

    if (!currentDefaultChrLocation[genomeId]) {
      const updatedDefaultChrLocation = { ...currentDefaultChrLocation };
      updatedDefaultChrLocation[genomeId] = chrLocation;
      dispatch(updateDefaultChrLocation(updatedDefaultChrLocation));
      browserStorageService.updateDefaultChrLocation(updatedDefaultChrLocation);
    }
  };
};

export const updateCogList = createAction(
  'browser/update-cog-list',
  (resolve) => {
    return (cogList: number) => {
      return resolve(cogList, getBrowserAnalyticsObject('User Interaction'));
    };
  }
);

export const updateCogTrackList = createAction(
  'browser/update-cog-track-list',
  (resolve) => {
    return (trackY: CogList) => {
      return resolve(trackY, getBrowserAnalyticsObject('User Interaction'));
    };
  }
);

export const updateSelectedCog = createAction(
  'browser/update-selected-cog',
  (resolve) => {
    return (index: string) => {
      return resolve(index, getBrowserAnalyticsObject('User Interaction'));
    };
  }
);

export const updateTrackConfigNames = createAction(
  'browser/update-track-config-names',
  (resolve) => {
    return (selectedCog: any, sense: boolean) => {
      return resolve(
        [selectedCog, sense],
        getBrowserAnalyticsObject('User Interaction')
      );
    };
  }
);

export const updateTrackConfigLabel = createAction(
  'browser/update-track-config-label',
  (resolve) => {
    return (selectedCog: any, sense: boolean) => {
      return resolve(
        [selectedCog, sense],
        getBrowserAnalyticsObject('User Interaction')
      );
    };
  }
);

export const updateApplyToAll = createAction(
  'browser/update-apply-to-all',
  (resolve) => {
    return (yn: boolean) => {
      return resolve(yn, getBrowserAnalyticsObject('User Interaction'));
    };
  }
);

export const toggleGenomeSelector = createAction(
  'toggle-genome-selector',
  (resolve) => {
    return (genomeSelectorActive: boolean) =>
      resolve(
        genomeSelectorActive,
        getBrowserAnalyticsObject('User Interaction')
      );
  }
);

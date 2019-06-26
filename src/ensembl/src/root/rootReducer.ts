import { combineReducers } from 'redux';
import { connectRouter } from 'connected-react-router';

import browser from '../content/app/browser/browserReducer';
import drawer from '../content/app/browser/drawer/drawerReducer';
import genome from '../genome/genomeReducer';
import customDownload from '../content/app/custom-download/state/customDownloadReducer';
import global from '../global/globalReducer';
import header from '../header/headerReducer';
import ensObject from '../ens-object/ensObjectReducer';
import trackPanel from '../content/app/browser/track-panel/trackPanelReducer';
import speciesSelector from '../content/app/species-selector/state/speciesSelectorReducer';

const rootReducer = (history: any) =>
  combineReducers({
    browser,
    drawer,
    customDownload,
    ensObject,
    genome,
    global,
    header,
    router: connectRouter(history),
    trackPanel,
    speciesSelector
  });

export default rootReducer;

import { combineEpics } from 'redux-observable';
import values from 'lodash/values';

import * as speciesSelectorEpics from 'src/content/app/species-selector/state/speciesSelectorEpics';

export default combineEpics(...values(speciesSelectorEpics));

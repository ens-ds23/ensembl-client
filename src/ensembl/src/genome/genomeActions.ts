import { createAsyncAction } from 'typesafe-actions';
import { ThunkAction } from 'redux-thunk';
import { Action, ActionCreator, Dispatch } from 'redux';

import apiService from 'src/services/api-service';
import { RootState } from 'src/store';
import { GenomeInfoData, GenomeTrackCategories } from './genomeTypes';

import { getCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import {
  getGenomeInfo,
  getGenomeTrackCategories
} from 'src/genome/genomeSelectors';

export const fetchGenomeInfoAsyncActions = createAsyncAction(
  'genome/fetch_genome_info_request',
  'genome/fetch_genome_info_success',
  'genome/fetch_genome_info_failure'
)<undefined, GenomeInfoData, Error>();

export const fetchGenomeInfo: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = () => (dispatch: Dispatch, getState: () => RootState) => {
  try {
    const genomeInfo: GenomeInfoData = getGenomeInfo(getState());

    const committedSpecies = getCommittedSpecies(getState());

    committedSpecies.map(async (species) => {
      if (!genomeInfo[species.genome_id]) {
        dispatch(fetchGenomeInfoAsyncActions.request());

        const url = `/api/genome/info?genome_id=${species.genome_id}`;
        const response = await apiService.fetch(url);

        dispatch(
          fetchGenomeInfoAsyncActions.success({
            [species.genome_id]: response.genome_info[0]
          })
        );
      }
    });
  } catch (error) {
    dispatch(fetchGenomeInfoAsyncActions.failure(error));
  }
};

export const fetchGenomeTrackCategoriesAsyncActions = createAsyncAction(
  'genome/fetch_genome_track_categories_request',
  'genome/fetch_genome_track_categories_success',
  'genome/fetch_genome_track_categories_failure'
)<string, GenomeTrackCategories, Error>();

// TODO: switch to using APIs when available
export const fetchGenomeTrackCategories: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (genomeId: string) => async (
  dispatch: Dispatch,
  getState: () => RootState
) => {
  try {
    const currentGenomeTrackCategories: GenomeTrackCategories = getGenomeTrackCategories(
      getState()
    );

    if (currentGenomeTrackCategories[genomeId]) {
      return;
    }

    const updatedGenomeTrackCategories: GenomeTrackCategories = {
      ...currentGenomeTrackCategories
    };

    dispatch(fetchGenomeTrackCategoriesAsyncActions.request(genomeId));

    const url = `/api/genome/track_categories?genome_id=${genomeId}`;
    const response = await apiService.fetch(url);
    updatedGenomeTrackCategories[genomeId] = response.track_categories;

    dispatch(
      fetchGenomeTrackCategoriesAsyncActions.success(
        updatedGenomeTrackCategories
      )
    );
  } catch (error) {
    dispatch(fetchGenomeTrackCategoriesAsyncActions.failure(error));
  }
};

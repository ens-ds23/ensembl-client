import { createAsyncAction } from 'typesafe-actions';
import { ThunkAction } from 'redux-thunk';
import { Action, ActionCreator, Dispatch } from 'redux';

// import apiService from 'src/services/api-service';

import { RootState } from 'src/store';
import { getCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import { CommittedItem } from 'src/content/app/species-selector/types/species-search';
import {
  GenomeInfoResponse,
  GenomeTrackCategoriesResponse
} from './genomeTypes';
import { EnsObjectResponse } from 'src/ens-object/ensObjectTypes';
import {
  humanGenomeInfoResponse,
  mouseGenomeInfoResponse,
  wheatGenomeInfoResponse
} from 'tests/data/genome/genome-info';

import {
  humanTrackCategoriesResponse,
  mouseTrackCategoriesResponse,
  wheatTrackCategoriesResponse
} from 'tests/data/genome/genome-track-categories';

import {
  humanGeneResponse,
  humanRegionResponse,
  mouseGeneResponse,
  mouseRegionResponse,
  wheatGeneResponse,
  wheatRegionResponse
} from 'tests/data/ens-object/ens-objects';

export const fetchGenomeInfoAsyncActions = createAsyncAction(
  'genome/fetch_genome_info_request',
  'genome/fetch_genome_info_success',
  'genome/fetch_genome_info_failure'
)<string, GenomeInfoResponse, Error>();

// TODO: switch to using APIs when available
export const fetchGenomeInfo: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (genomeId: string) => (dispatch: Dispatch) => {
  try {
    dispatch(fetchGenomeInfoAsyncActions.request(genomeId));

    let genomeInfoResponse: GenomeInfoResponse = {
      genome_info: []
    };

    switch (genomeId) {
      case 'homo_sapiens38':
        genomeInfoResponse = humanGenomeInfoResponse;
        break;
      case 'mus_musculus_bdc':
        genomeInfoResponse = mouseGenomeInfoResponse;
        break;
      case 'triticum_aestivum':
        genomeInfoResponse = wheatGenomeInfoResponse;
        break;
    }

    dispatch(fetchGenomeInfoAsyncActions.success(genomeInfoResponse));
  } catch (error) {
    dispatch(fetchGenomeInfoAsyncActions.failure(error));
  }
};

export const fetchGenomeExampleEnsObjectsAsyncActions = createAsyncAction(
  'ens-object/fetch_genome_example_ens_objects_request',
  'ens-object/fetch_genome_example_ens_objects_success',
  'ens-object/fetch_genome_example_ens_objects_failure'
)<null, EnsObjectResponse[], Error>();

// TODO: switch to using APIs when available
export const fetchGenomeExampleEnsObjects: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = () => (dispatch: Dispatch, getState: () => RootState) => {
  try {
    dispatch(fetchGenomeExampleEnsObjectsAsyncActions.request(null));

    const committedSpecies = getCommittedSpecies(getState());
    let ensObjectResponses: EnsObjectResponse[] = [];

    committedSpecies.map((species: CommittedItem) => {
      switch (species.genome_id) {
        case 'homo_sapiens38':
          ensObjectResponses.push(humanGeneResponse, humanRegionResponse);
          break;
        case 'mus_musculus_bdc':
          ensObjectResponses.push(mouseGeneResponse, mouseRegionResponse);
          break;
        case 'triticum_aestivum':
          ensObjectResponses.push(wheatGeneResponse, wheatRegionResponse);
          break;
      }
    });

    dispatch(
      fetchGenomeExampleEnsObjectsAsyncActions.success(
        ensObjectResponses.flat()
      )
    );
  } catch (error) {
    dispatch(fetchGenomeExampleEnsObjectsAsyncActions.failure(error));
  }
};

export const fetchGenomeTrackCategoriesAsyncActions = createAsyncAction(
  'genome/fetch_genome_track_categories_request',
  'genome/fetch_genome_track_categories_success',
  'genome/fetch_genome_track_categories_failure'
)<string, GenomeTrackCategoriesResponse, Error>();

// TODO: switch to using APIs when available
export const fetchGenomeTrackCategories: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (genomeId: string) => (dispatch: Dispatch) => {
  try {
    dispatch(fetchGenomeTrackCategoriesAsyncActions.request(genomeId));

    let genomeTrackCategoriesResponse: GenomeTrackCategoriesResponse = {
      genome_id: '',
      track_categories: []
    };

    switch (genomeId) {
      case 'homo_sapiens38':
        genomeTrackCategoriesResponse = humanTrackCategoriesResponse;
        break;
      case 'mus_musculus_bdc':
        genomeTrackCategoriesResponse = mouseTrackCategoriesResponse;
        break;
      case 'triticum_aestivum':
        genomeTrackCategoriesResponse = wheatTrackCategoriesResponse;
        break;
    }

    dispatch(
      fetchGenomeTrackCategoriesAsyncActions.success(
        genomeTrackCategoriesResponse
      )
    );
  } catch (error) {
    dispatch(fetchGenomeTrackCategoriesAsyncActions.failure(error));
  }
};
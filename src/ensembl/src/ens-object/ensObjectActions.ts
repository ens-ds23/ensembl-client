import { createAsyncAction } from 'typesafe-actions';
import { Action, ActionCreator, Dispatch } from 'redux';
import { ThunkAction } from 'redux-thunk';
import { RootState } from 'src/store';
import apiService from 'src/services/api-service';

import { GenomeInfoData } from 'src/genome/genomeTypes';
import { getGenomeInfo } from 'src/genome/genomeSelectors';
import { getExampleEnsObjects } from 'src/ens-object/ensObjectSelectors';
import {
  EnsObjectResponse,
  EnsObjectTracksResponse,
  ExampleEnsObjectsData
} from './ensObjectTypes';

type FetchEnsObjectRequestType = {
  ensObjectId: string;
  genomeId: string;
};

export const fetchEnsObjectAsyncActions = createAsyncAction(
  'ens-object/fetch_ens_object_request',
  'ens-object/fetch_ens_object_success',
  'ens-object/fetch_ens_object_failure'
)<FetchEnsObjectRequestType, EnsObjectResponse, Error>();

export const fetchEnsObject: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (ensObjectId: string, genomeId: string) => async (dispatch: Dispatch) => {
  const splitEnsObjectId = ensObjectId.split(':');
  // Do not send the request for regions
  if (splitEnsObjectId[1] === 'region') {
    const regionExample = {
      label: `${splitEnsObjectId[2]}:${splitEnsObjectId[3]}`,
      ensembl_object_id: splitEnsObjectId,
      genome_id: splitEnsObjectId[0],
      location: {
        chromosome: splitEnsObjectId[2],
        end: splitEnsObjectId[3].split('-')[1],
        start: splitEnsObjectId[3].split('-')[0]
      },
      object_type: 'region'
    };
    dispatch(
      fetchEnsObjectAsyncActions.success({
        ensembl_object: regionExample
      })
    );
    return;
  }

  try {
    dispatch(fetchEnsObjectAsyncActions.request({ ensObjectId, genomeId }));

    const url = `/api/ensembl_object/info?object_id=${ensObjectId}`;
    const response = await apiService.fetch(url);
    dispatch(
      fetchEnsObjectAsyncActions.success({
        ensembl_object: response
      })
    );
  } catch (error) {
    dispatch(fetchEnsObjectAsyncActions.failure(error));
  }
};

export const fetchEnsObjectTracksAsyncActions = createAsyncAction(
  'ens-object/fetch_ens_object_tracks_request',
  'ens-object/fetch_ens_object_tracks_success',
  'ens-object/fetch_ens_object_tracks_failure'
)<FetchEnsObjectRequestType, EnsObjectTracksResponse, Error>();

export const fetchEnsObjectTracks = (
  ensObjectId: string,
  genomeId: string
) => async (dispatch: Dispatch) => {
  try {
    // Do not send the request for regions
    if (ensObjectId.split(':')[1] !== 'gene') {
      return;
    }
    dispatch(fetchEnsObjectAsyncActions.request({ ensObjectId, genomeId }));

    const url = `/api/ensembl_object/track_list?object_id=${ensObjectId}`;
    const response = await apiService.fetch(url, { preserveEndpoint: true });

    dispatch(
      fetchEnsObjectTracksAsyncActions.success({
        object_tracks: response
      })
    );
  } catch (error) {
    dispatch(fetchEnsObjectTracksAsyncActions.failure(error));
  }
};

export const fetchExampleEnsObjectsAsyncActions = createAsyncAction(
  'ens-object/fetch_example_ens_objects_request',
  'ens-object/fetch_example_ens_objects_success',
  'ens-object/fetch_example_ens_objects_failure'
)<null, ExampleEnsObjectsData, Error>();

export const fetchExampleEnsObjects: ActionCreator<
  ThunkAction<void, any, null, Action<string>>
> = (genomeId?: string) => async (
  dispatch: Dispatch,
  getState: () => RootState
) => {
  try {
    const genomeInfoData: GenomeInfoData = getGenomeInfo(getState());

    const exampleObjects: ExampleEnsObjectsData = getExampleEnsObjects(
      getState()
    );

    if (genomeId && !exampleObjects[genomeId]) {
      dispatch(fetchExampleEnsObjectsAsyncActions.request(null));
      const exampleObjectUrls = genomeInfoData[genomeId].example_objects.map(
        (exampleObjectId) =>
          `/api/ensembl_object/info?object_id=${exampleObjectId}`
      );
      const responses = await Promise.all(
        exampleObjectUrls.map((url) => apiService.fetch(url))
      );
      const exampleObjects = responses.filter((response) => !response.error);
      dispatch(
        fetchExampleEnsObjectsAsyncActions.success({
          [genomeId]: exampleObjects
        })
      );
    }
  } catch (error) {
    dispatch(fetchExampleEnsObjectsAsyncActions.failure(error));
  }
};

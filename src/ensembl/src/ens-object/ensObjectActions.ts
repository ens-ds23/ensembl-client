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
  try {
    dispatch(fetchEnsObjectAsyncActions.request({ ensObjectId, genomeId }));
    let response;

    // FIXME: the if-branch is temporary, until backend learns to respond with region object data
    if (isRegionObject(ensObjectId)) {
      response = await parseRegionObjectId(ensObjectId);
    } else {
      const url = `/api/ensembl_object/info?object_id=${ensObjectId}`;
      response = await apiService.fetch(url);
    }

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
    if (isRegionObject(ensObjectId)) {
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
    const genomeInfo = genomeId && genomeInfoData[genomeId];
    const exampleObjects: ExampleEnsObjectsData = getExampleEnsObjects(
      getState()
    );

    if (genomeId && genomeInfo && !exampleObjects[genomeId]) {
      dispatch(fetchExampleEnsObjectsAsyncActions.request(null));
      const requests = genomeInfo.example_objects.map((exampleObjectId) => {
        if (isRegionObject(exampleObjectId)) {
          return parseRegionObjectId(exampleObjectId);
        } else {
          const url = `/api/ensembl_object/info?object_id=${exampleObjectId}`;
          return apiService.fetch(url);
        }
      });
      const responses = await Promise.all(requests);
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

// FIXME: remove when backend learns to return info about a region object
const isRegionObject = (objectId: string) => {
  return /:region:/.test(objectId);
};

// FIXME: the function below is horrible and should have never been written
// Remove when backend learns to return info about a region object
// (writing this as async function so that it has the same promise interface as apiService.fetch)
const parseRegionObjectId = async (objectId: string) => {
  const [genomeId, , chromosome, region] = objectId.split(':');
  const [start, end] = region.split('-');

  return {
    label: `${chromosome}:${region}`,
    ensembl_object_id: objectId,
    genome_id: genomeId,
    location: {
      chromosome,
      start,
      end
    },
    object_type: 'region'
  };
};

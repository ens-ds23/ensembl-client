import { ActionType, getType } from 'typesafe-actions';
import { combineReducers } from 'redux';

import * as objectActions from './ensObjectActions';
import {
  EnsObjectInfoState,
  defaultEnsObjectInfoState,
  EnsObjectTracksState,
  defaultEnsObjectTracksState,
  ExampleEnsObjectsState,
  defaultExampleEnsObjectsState
} from './ensObjectState';
import { ExampleEnsObjectsData } from './ensObjectTypes';

function ensObjectInfo(
  state: EnsObjectInfoState = defaultEnsObjectInfoState,
  action: ActionType<typeof objectActions>
): EnsObjectInfoState {
  switch (action.type) {
    case getType(objectActions.fetchEnsObjectAsyncActions.failure):
      return {
        ...state,
        ensObjectInfoFetchFailed: true,
        ensObjectInfoFetching: false
      };
    case getType(objectActions.fetchEnsObjectAsyncActions.request):
      return {
        ...state,
        ensObjectInfoFetchFailed: false,
        ensObjectInfoFetching: true
      };
    case getType(objectActions.fetchEnsObjectAsyncActions.success):
      return {
        ...state,
        ensObjectInfoData: action.payload.ensembl_object,
        ensObjectInfoFetchFailed: false,
        ensObjectInfoFetching: false
      };
    default:
      return state;
  }
}

function ensObjectTracks(
  state: EnsObjectTracksState = defaultEnsObjectTracksState,
  action: ActionType<typeof objectActions>
): EnsObjectTracksState {
  switch (action.type) {
    case getType(objectActions.fetchEnsObjectTracksAsyncActions.failure):
      return {
        ...state,
        ensObjectTracksFetchFailed: true,
        ensObjectTracksFetching: false
      };
    case getType(objectActions.fetchEnsObjectTracksAsyncActions.request):
      return {
        ...state,
        ensObjectTracksFetchFailed: false,
        ensObjectTracksFetching: true
      };
    case getType(objectActions.fetchEnsObjectTracksAsyncActions.success):
      return {
        ...state,
        ensObjectTracksData: action.payload.object_tracks,
        ensObjectTracksFetchFailed: false,
        ensObjectTracksFetching: false
      };
    default:
      return state;
  }
}

function exampleEnsObjects(
  state: ExampleEnsObjectsState = defaultExampleEnsObjectsState,
  action: ActionType<typeof objectActions>
): ExampleEnsObjectsState {
  switch (action.type) {
    case getType(objectActions.fetchExampleEnsObjectsAsyncActions.failure):
      return {
        ...state,
        exampleEnsObjectsFetchFailed: true,
        exampleEnsObjectsFetching: false
      };
    case getType(objectActions.fetchExampleEnsObjectsAsyncActions.request):
      return {
        ...state,
        exampleEnsObjectsFetchFailed: false,
        exampleEnsObjectsFetching: true
      };
    case getType(objectActions.fetchExampleEnsObjectsAsyncActions.success):
      return {
        ...state,
        exampleEnsObjectsData: exampleEnsObjectsData(
          state.exampleEnsObjectsData,
          action
        ),
        exampleEnsObjectsFetchFailed: false,
        exampleEnsObjectsFetching: false
      };
    default:
      return state;
  }
}

function exampleEnsObjectsData(
  state: ExampleEnsObjectsData = {},
  action: ActionType<typeof objectActions>
): ExampleEnsObjectsData {
  switch (action.type) {
    case getType(objectActions.fetchExampleEnsObjectsAsyncActions.success):
      return {
        ...state,
        ...action.payload
      };
    default:
      return state;
  }
}

export default combineReducers({
  ensObjectInfo,
  ensObjectTracks,
  exampleEnsObjects
});

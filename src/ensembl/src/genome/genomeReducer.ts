import { ActionType, getType } from 'typesafe-actions';
import { combineReducers } from 'redux';

import * as genomeActions from './genomeActions';

import {
  GenomeInfoState,
  defaultGenomeInfoState,
  GenomeTrackCategoriesState,
  defaultGenomeTrackCategoriesState
} from './genomeState';
import { GenomeInfoData } from './genomeTypes';

function genomeInfo(
  state: GenomeInfoState = defaultGenomeInfoState,
  action: ActionType<typeof genomeActions>
): GenomeInfoState {
  switch (action.type) {
    case getType(genomeActions.fetchGenomeInfoAsyncActions.failure):
      return {
        ...state,
        genomeInfoFetchFailed: true,
        genomeInfoFetching: false
      };
    case getType(genomeActions.fetchGenomeInfoAsyncActions.request):
      return {
        ...state,
        genomeInfoFetchFailed: false,
        genomeInfoFetching: true
      };
    case getType(genomeActions.fetchGenomeInfoAsyncActions.success):
      return {
        ...state,
        genomeInfoData: genomeInfoData(state.genomeInfoData, action),
        genomeInfoFetchFailed: false,
        genomeInfoFetching: false
      };
    default:
      return state;
  }
}

function genomeInfoData(
  state: GenomeInfoData = {},
  action: ActionType<typeof genomeActions>
): GenomeInfoData {
  switch (action.type) {
    case getType(genomeActions.fetchGenomeInfoAsyncActions.success):
      return {
        ...state,
        ...action.payload
      };
    default:
      return state;
  }
}

function genomeTrackCategories(
  state: GenomeTrackCategoriesState = defaultGenomeTrackCategoriesState,
  action: ActionType<typeof genomeActions>
): GenomeTrackCategoriesState {
  switch (action.type) {
    case getType(genomeActions.fetchGenomeTrackCategoriesAsyncActions.failure):
      return {
        ...state,
        genomeTrackCategoriesFetchFailed: true,
        genomeTrackCategoriesFetching: false
      };
    case getType(genomeActions.fetchGenomeTrackCategoriesAsyncActions.request):
      return {
        ...state,
        genomeTrackCategoriesFetchFailed: false,
        genomeTrackCategoriesFetching: true
      };
    case getType(genomeActions.fetchGenomeTrackCategoriesAsyncActions.success):
      return {
        ...state,
        genomeTrackCategoriesData: action.payload,
        genomeTrackCategoriesFetchFailed: false,
        genomeTrackCategoriesFetching: false
      };
    default:
      return state;
  }
}

export default combineReducers({
  genomeInfo,
  genomeTrackCategories
});

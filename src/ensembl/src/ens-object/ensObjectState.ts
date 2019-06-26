import {
  EnsObject,
  EnsObjectTrack,
  ExampleEnsObjectsData
} from './ensObjectTypes';

export type EnsObjectInfoState = Readonly<{
  ensObjectInfoData: EnsObject | {};
  ensObjectInfoFetchFailed: boolean;
  ensObjectInfoFetching: boolean;
}>;

export const defaultEnsObjectInfoState: EnsObjectInfoState = {
  ensObjectInfoData: {},
  ensObjectInfoFetchFailed: false,
  ensObjectInfoFetching: false
};

export type EnsObjectTracksState = Readonly<{
  ensObjectTracksData: EnsObjectTrack | {};
  ensObjectTracksFetchFailed: boolean;
  ensObjectTracksFetching: boolean;
}>;

export const defaultEnsObjectTracksState: EnsObjectTracksState = {
  ensObjectTracksData: {},
  ensObjectTracksFetchFailed: false,
  ensObjectTracksFetching: false
};

export type ExampleEnsObjectsState = Readonly<{
  exampleEnsObjectsData: ExampleEnsObjectsData;
  exampleEnsObjectsFetchFailed: boolean;
  exampleEnsObjectsFetching: boolean;
}>;

export const defaultExampleEnsObjectsState: ExampleEnsObjectsState = {
  exampleEnsObjectsData: {},
  exampleEnsObjectsFetchFailed: false,
  exampleEnsObjectsFetching: false
};

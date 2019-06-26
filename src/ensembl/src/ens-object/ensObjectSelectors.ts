import { RootState } from '../store';
import { EnsObject, EnsObjectTrack } from './ensObjectTypes';

export const getEnsObjectInfoFetchFailed = (state: RootState) =>
  state.ensObject.ensObjectInfo.ensObjectInfoFetchFailed;

export const getEnsObjectInfoFetching = (state: RootState) =>
  state.ensObject.ensObjectInfo.ensObjectInfoFetching;

export const getEnsObjectInfo = (state: RootState) =>
  state.ensObject.ensObjectInfo.ensObjectInfoData as EnsObject;

export const getEnsObjectTracksFetchFailed = (state: RootState) =>
  state.ensObject.ensObjectTracks.ensObjectTracksFetchFailed;

export const getEnsObjectTracksFetching = (state: RootState) =>
  state.ensObject.ensObjectTracks.ensObjectTracksFetching;

export const getEnsObjectTracks = (state: RootState) =>
  state.ensObject.ensObjectTracks.ensObjectTracksData as EnsObjectTrack;

export const getExampleEnsObjectsFetchFailed = (state: RootState) =>
  state.ensObject.exampleEnsObjects.exampleEnsObjectsFetchFailed;

export const getExampleEnsObjectsFetching = (state: RootState) =>
  state.ensObject.exampleEnsObjects.exampleEnsObjectsFetching;

export const getExampleEnsObjects = (state: RootState) =>
  state.ensObject.exampleEnsObjects.exampleEnsObjectsData;

import { RootState } from 'src/store';
import { GenomeInfoData } from './genomeTypes';

export const getGenomeInfo = (state: RootState) =>
  state.genome.genomeInfo.genomeInfoData as GenomeInfoData;

export const getGenomeInfoFetching = (state: RootState) =>
  state.genome.genomeInfo.genomeInfoFetching;

export const getGenomeInfoFetchFailed = (state: RootState) =>
  state.genome.genomeInfo.genomeInfoFetchFailed;

export const getGenomeTrackCategories = (state: RootState) =>
  state.genome.genomeTrackCategories.genomeTrackCategoriesData;

export const getGenomeTrackCategoriesFetching = (state: RootState) =>
  state.genome.genomeTrackCategories.genomeTrackCategoriesFetching;

export const getGenomeTrackCategoriesFetchFailed = (state: RootState) =>
  state.genome.genomeTrackCategories.genomeTrackCategoriesFetchFailed;

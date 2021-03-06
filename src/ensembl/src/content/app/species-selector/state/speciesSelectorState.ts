import speciesSelectorStorageService from 'src/content/app/species-selector/services/species-selector-storage-service';

import { LoadingState } from 'src/content/app/species-selector/types/loading-state';
import {
  SearchMatches,
  Strain,
  Assembly,
  CommittedItem,
  PopularSpecies
} from 'src/content/app/species-selector/types/species-search';

export type CurrentItem = {
  genome_id: string; // changes every time we update strain or assembly
  reference_genome_id: string | null;
  common_name: string | null;
  scientific_name: string;
  assembly_name: string | null; // name of the selected assembly
  selectedStrainId: string | null; // genome_id of selected strain
  strains: Strain[];
  assemblies: Assembly[];
};

export type SpeciesSelectorState = {
  loadingStates: {
    search: LoadingState;
  };
  ui: {
    isSelectingStrain: boolean;
    isSelectingAssembly: boolean;
  };
  search: {
    results: SearchMatches[] | null;
  };
  currentItem: CurrentItem | null;
  committedItems: CommittedItem[];
  popularSpecies: PopularSpecies[];
};

const storedSelectedSpecies = speciesSelectorStorageService.getSelectedSpecies();

const initialState: SpeciesSelectorState = {
  loadingStates: {
    search: LoadingState.NOT_REQUESTED
  },
  ui: {
    isSelectingStrain: false,
    isSelectingAssembly: false
  },
  search: {
    results: null
  },
  currentItem: null,
  committedItems: storedSelectedSpecies || [],
  popularSpecies: []
};

export default initialState;

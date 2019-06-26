import { getType, ActionType } from 'typesafe-actions';
import find from 'lodash/find';
import get from 'lodash/get';

import * as speciesSelectorActions from './speciesSelectorActions';

import initialState, {
  SpeciesSelectorState,
  CurrentItem
} from './speciesSelectorState';

import {
  SearchMatch,
  PopularSpecies,
  CommittedItem,
  Assembly
} from 'src/content/app/species-selector/types/species-search';

// NOTE: CurrentItem can be built from a search match or from a popular species
const buildCurrentItem = (data: SearchMatch | PopularSpecies): CurrentItem => {
  return {
    genome_id: data.genome_id,
    reference_genome_id: data.reference_genome_id,
    common_name: data.common_name,
    scientific_name: data.scientific_name,
    assembly_name: data.assembly_name,
    selectedStrainId: null,
    strains: [],
    assemblies: [buildAssembly(data)]
  };
};

const buildAssembly = (data: SearchMatch | PopularSpecies): Assembly => ({
  genome_id: data.genome_id,
  assembly_name: data.assembly_name
});

const buildCommittedItem = (data: CurrentItem): CommittedItem => ({
  genome_id: data.genome_id,
  reference_genome_id: data.reference_genome_id,
  common_name: data.common_name,
  scientific_name: data.scientific_name,
  assembly_name: get(
    find(data.assemblies, ({ genome_id }) => genome_id === data.genome_id),
    'assembly_name'
  ) as string,
  isEnabled: true
});

export default function speciesSelectorReducer(
  state: SpeciesSelectorState = initialState,
  action: ActionType<typeof speciesSelectorActions>
): SpeciesSelectorState {
  switch (action.type) {
    case getType(speciesSelectorActions.fetchSpeciesSearchResults.success):
      return {
        ...state,
        search: action.payload
      };
    case getType(speciesSelectorActions.setSelectedSpecies):
      return {
        ...state,
        currentItem: buildCurrentItem(action.payload)
      };
    // TODO: wait for strains
    // case getType(speciesSelectorActions.fetchStrainsAsyncActions.success):
    //   return {
    //     ...state,
    //     currentItem: {
    //       ...(state.currentItem as CurrentItem),
    //       strains: action.payload.strains,
    //       selectedStrainId: action.payload.strains.length
    //         ? (state.currentItem as CurrentItem).genome_id
    //         : null
    //     }
    //   };
    case getType(speciesSelectorActions.fetchAssembliesAsyncActions.success):
      return {
        ...state,
        currentItem: {
          ...(state.currentItem as CurrentItem),
          assemblies: [
            ...(state.currentItem as CurrentItem).assemblies,
            ...action.payload.assemblies
          ]
        }
      };
    case getType(speciesSelectorActions.changeAssembly):
      return {
        ...state,
        currentItem: {
          ...(state.currentItem as CurrentItem),
          ...action.payload
        }
      };
    case getType(
      speciesSelectorActions.fetchPopularSpeciesAsyncActions.success
    ):
      return {
        ...state,
        popularSpecies: action.payload.popularSpecies
      };
    case getType(speciesSelectorActions.commitSelectedSpecies):
      return {
        ...state,
        currentItem: null,
        committedItems: [
          ...state.committedItems,
          buildCommittedItem(state.currentItem as CurrentItem)
        ]
      };
    case getType(speciesSelectorActions.toggleSpeciesUse):
      return {
        ...state,
        committedItems: state.committedItems.map((item) => {
          if (item.genome_id === action.payload) {
            return {
              ...item,
              isEnabled: !item.isEnabled
            };
          } else {
            return item;
          }
        })
      };
    case getType(speciesSelectorActions.deleteSpecies):
      return {
        ...state,
        committedItems: state.committedItems.filter(
          (item) => item.genome_id !== action.payload
        )
      };
    case getType(speciesSelectorActions.clearSearchResults):
      return {
        ...state,
        search: initialState.search
      };
    case getType(speciesSelectorActions.clearSelectedSearchResult):
      return {
        ...state,
        currentItem: null
      };
    default:
      return state;
  }
}

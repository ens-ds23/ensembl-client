import storageService, {
  StorageServiceInterface
} from 'src/services/storage-service';

import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

export enum StorageKeys {
  SELECTED_SPECIES = 'speciesSelector.selectedSpecies'
}

// named export is for testing purposes
// for development, use default export
export class SpeciesSelectorStorageService {
  private storageService: StorageServiceInterface;

  public constructor(storageService: StorageServiceInterface) {
    this.storageService = storageService;
  }

  public getSelectedSpecies(): CommittedItem[] {
    return this.storageService.get(StorageKeys.SELECTED_SPECIES) || [];
  }

  public saveSelectedSpecies(species: CommittedItem[]) {
    this.storageService.save(StorageKeys.SELECTED_SPECIES, species);
  }
}

export default new SpeciesSelectorStorageService(storageService);

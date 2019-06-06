import {
  SpeciesSelectorStorageService,
  StorageKeys
} from './species-selector-storage-service';

import { createSelectedSpecies } from 'tests/fixtures/selected-species';

const mockStorageService = {
  get: jest.fn(),
  save: jest.fn(),
  update: jest.fn(),
  remove: jest.fn(),
  clearAll: jest.fn()
};

describe('SpeciesSelectorStorageService', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  describe('.getSelectedSpecies()', () => {
    it('gets saved selected species from storage service', () => {
      const savedSpecies = [createSelectedSpecies()];
      jest
        .spyOn(mockStorageService, 'get')
        .mockImplementation(() => savedSpecies);
      const speciesSelectorStorageService = new SpeciesSelectorStorageService(
        mockStorageService
      );
      const result = speciesSelectorStorageService.getSelectedSpecies();

      expect(mockStorageService.get).toHaveBeenCalledWith(
        StorageKeys.SELECTED_SPECIES
      );
      expect(result).toEqual(savedSpecies);

      mockStorageService.get.mockRestore();
    });

    it('returns an empty array if there are no saved species', () => {
      jest.spyOn(mockStorageService, 'get').mockImplementation(() => null);
      const speciesSelectorStorageService = new SpeciesSelectorStorageService(
        mockStorageService
      );
      const result = speciesSelectorStorageService.getSelectedSpecies();

      expect(result).toEqual([]);

      mockStorageService.get.mockRestore();
    });
  });

  describe('.saveSelectedSpecies()', () => {
    it('saves selected species via storage service', () => {
      const speciesSelectorStorageService = new SpeciesSelectorStorageService(
        mockStorageService
      );
      const selectedSpecies = [createSelectedSpecies()];
      speciesSelectorStorageService.saveSelectedSpecies(selectedSpecies);

      expect(mockStorageService.save).toHaveBeenCalledWith(
        StorageKeys.SELECTED_SPECIES,
        selectedSpecies
      );
    });
  });
});

import { StorageService, StorageType } from 'src/services/storage-service';
import { WindowServiceInterface } from 'src/services/window-service';
import faker from 'faker';

const mockLocalStorage: any = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn()
};

const mockSessionStorage: any = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn()
};

const mockWindowService: WindowServiceInterface = {
  getLocalStorage: () => mockLocalStorage,
  getSessionStorage: () => mockSessionStorage
};

describe('storageService', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  describe('using local storage', () => {
    // This is the default behaviour of storageService

    describe('.get()', () => {
      it('gets data from local storage if the data exists', () => {
        const key = faker.lorem.word();
        const storedValue = faker.lorem.words();

        jest
          .spyOn(mockLocalStorage, 'getItem')
          .mockImplementation(() => JSON.stringify(storedValue));

        const storageService = new StorageService(mockWindowService);
        const result = storageService.get(key);

        expect(mockLocalStorage.getItem).toHaveBeenCalledWith(key);
        expect(result).toBe(storedValue);

        mockLocalStorage.getItem.mockRestore();
      });

      it('returns null if nothing is stored under provided key', () => {
        const key = faker.lorem.word();
        jest.spyOn(mockLocalStorage, 'getItem').mockImplementation(() => null);

        const storageService = new StorageService(mockWindowService);
        const result = storageService.get(key);

        expect(mockLocalStorage.getItem).toHaveBeenCalledWith(key);
        expect(result).toBe(null);

        mockLocalStorage.getItem.mockRestore();
      });
    });

    describe('.save()', () => {
      it('saves JSON-stringified value in local storage', () => {
        const key = faker.lorem.word();
        const value = faker.lorem.words();

        const storageService = new StorageService(mockWindowService);
        storageService.save(key, value);

        const [passedKey, passedValue] = mockLocalStorage.setItem.mock.calls[0];

        expect(passedKey).toBe(key);
        expect(passedValue).toBe(JSON.stringify(value));
      });
    });

    describe('.update()', () => {
      it('saves an updated object under the same key', () => {
        const key = faker.lorem.word();
        const key1 = faker.lorem.word();
        const key2 = faker.lorem.word();
        const value1 = faker.lorem.words();
        const value2 = faker.lorem.words();
        const value3 = faker.lorem.words();
        const obj = { [key1]: value1 };
        const updateObj = { [key1]: value3, [key2]: value2 };

        jest
          .spyOn(mockLocalStorage, 'getItem')
          .mockImplementation(() => JSON.stringify(obj));

        const storageService = new StorageService(mockWindowService);
        storageService.update(key, updateObj);

        const [passedKey, passedValue] = mockLocalStorage.setItem.mock.calls[0];

        expect(passedKey).toBe(key);
        expect(passedValue).toBe(JSON.stringify(updateObj));

        mockLocalStorage.getItem.mockRestore();
      });
    });

    describe('.remove()', () => {
      it('removes value stored under provided key', () => {
        const key = faker.lorem.word();

        const storageService = new StorageService(mockWindowService);
        storageService.remove(key);

        expect(mockLocalStorage.removeItem).toHaveBeenCalledWith(key);
      });
    });
  });

  describe('using session storage', () => {
    // Session storage will be used if the appropriate option is provided

    describe('.get()', () => {
      it('gets data from session storage if the data exists', () => {
        const key = faker.lorem.word();
        const storedValue = faker.lorem.words();

        jest
          .spyOn(mockSessionStorage, 'getItem')
          .mockImplementation(() => JSON.stringify(storedValue));

        const storageService = new StorageService(mockWindowService);
        const result = storageService.get(key, {
          storage: StorageType.SESSION_STORAGE
        });

        expect(mockSessionStorage.getItem).toHaveBeenCalledWith(key);
        expect(result).toBe(storedValue);

        mockSessionStorage.getItem.mockRestore();
      });

      it('returns null if nothing is stored under provided key', () => {
        const key = faker.lorem.word();
        jest
          .spyOn(mockSessionStorage, 'getItem')
          .mockImplementation(() => null);

        const storageService = new StorageService(mockWindowService);
        const result = storageService.get(key, {
          storage: StorageType.SESSION_STORAGE
        });

        expect(mockSessionStorage.getItem).toHaveBeenCalledWith(key);
        expect(result).toBe(null);

        mockSessionStorage.getItem.mockRestore();
      });
    });

    describe('.save()', () => {
      it('saves JSON-stringified value in session storage', () => {
        const key = faker.lorem.word();
        const value = faker.lorem.words();

        const storageService = new StorageService(mockWindowService);
        storageService.save(key, value, {
          storage: StorageType.SESSION_STORAGE
        });

        const [
          passedKey,
          passedValue
        ] = mockSessionStorage.setItem.mock.calls[0];

        expect(passedKey).toBe(key);
        expect(passedValue).toBe(JSON.stringify(value));
      });
    });

    describe('.update()', () => {
      it('saves an updated object under the same key', () => {
        const key = faker.lorem.word();
        const key1 = faker.lorem.word();
        const key2 = faker.lorem.word();
        const value1 = faker.lorem.words();
        const value2 = faker.lorem.words();
        const value3 = faker.lorem.words();
        const obj = { [key1]: value1 };
        const updateObj = { [key1]: value3, [key2]: value2 };

        jest
          .spyOn(mockSessionStorage, 'getItem')
          .mockImplementation(() => JSON.stringify(obj));

        const storageService = new StorageService(mockWindowService);
        storageService.update(key, updateObj, {
          storage: StorageType.SESSION_STORAGE
        });

        const [
          passedKey,
          passedValue
        ] = mockSessionStorage.setItem.mock.calls[0];

        expect(passedKey).toBe(key);
        expect(passedValue).toBe(JSON.stringify(updateObj));
      });
    });

    describe('.remove()', () => {
      it('removes value stored under provided key', () => {
        const key = faker.lorem.word();

        const storageService = new StorageService(mockWindowService);
        storageService.remove(key, { storage: StorageType.SESSION_STORAGE });

        expect(mockSessionStorage.removeItem).toHaveBeenCalledWith(key);
      });
    });
  });

  describe('clearAll', () => {
    it('clears both localStorage and sessionStorage', () => {
      const storageService = new StorageService(mockWindowService);
      storageService.clearAll();

      expect(mockLocalStorage.clear).toHaveBeenCalled();
      expect(mockSessionStorage.clear).toHaveBeenCalled();
    });
  });
});

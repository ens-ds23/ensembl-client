import windowService, {
  WindowServiceInterface
} from 'src/services/window-service';
import merge from 'lodash/merge';

export enum StorageType {
  LOCAL_STORAGE = 'localstorage',
  SESSION_STORAGE = 'sessionstorage'
}

type PrimitiveValue = string | number | boolean | null | undefined;
type ArrayValue = PrimitiveValue[] | ObjectValue[];
type ObjectValue = {
  [key: string]: PrimitiveValue | ArrayValue | ObjectValue;
};

type ValueForSaving = PrimitiveValue | ArrayValue | ArrayValue[] | ObjectValue;

type options = {
  storage: StorageType;
};

export interface StorageServiceInterface {
  get: (key: string, options?: options) => any;
  save: (key: string, value: ValueForSaving, options?: options) => void;
  update: (key: string, fragment: ObjectValue, options?: options) => void;
  remove: (key: string, options?: options) => void;
  clearAll: () => void;
}

const defaultOptions: options = {
  storage: StorageType.LOCAL_STORAGE
};

// named export is for testing;
// for development, use default export
export class StorageService implements StorageServiceInterface {
  private localStorage: Storage;
  private sessionStorage: Storage;

  public constructor(windowService: WindowServiceInterface) {
    this.localStorage = windowService.getLocalStorage();
    this.sessionStorage = windowService.getSessionStorage();
  }

  private chooseStorage(storage: StorageType) {
    return storage === StorageType.SESSION_STORAGE
      ? this.sessionStorage
      : this.localStorage;
  }

  public get(key: string, options = defaultOptions) {
    const storage = this.chooseStorage(options.storage);
    const savedValue = storage.getItem(key);
    return savedValue ? JSON.parse(savedValue) : null;
  }

  public save(key: string, value: ValueForSaving, options = defaultOptions) {
    const storage = this.chooseStorage(options.storage);
    storage.setItem(key, JSON.stringify(value));
  }

  // intended only for updating part of the saved object
  public update(key: string, fragment: ObjectValue, options = defaultOptions) {
    const storedData = this.get(key, options);
    if (storedData) {
      this.save(key, merge(storedData, fragment), options);
    } else {
      this.save(key, fragment, options);
    }
  }

  public remove(key: string, options = defaultOptions) {
    const storage = this.chooseStorage(options.storage);
    storage.removeItem(key);
  }

  public clearAll() {
    this.localStorage.clear();
    this.sessionStorage.clear();
  }
}

export default new StorageService(windowService);

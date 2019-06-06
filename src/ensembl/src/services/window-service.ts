/**
 * This is a simple service whose purpose is to return objects from the window object.
 * This is helpful for mocking elements of the browser API during tests.
 */

export interface WindowServiceInterface {
  getLocalStorage: () => Storage;
  getSessionStorage: () => Storage;
}

class WindowService implements WindowServiceInterface {
  public getLocalStorage() {
    return window.localStorage;
  }

  public getSessionStorage() {
    return window.sessionStorage;
  }
}

export default new WindowService();

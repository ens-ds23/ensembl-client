// this service temporarily uses the native browser fetch API;
// we will modify it to use a library once we decide which one to choose

import config from 'config';

export enum HTTPMethod {
  GET = 'GET',
  POST = 'POST',
  PATCH = 'PATCH',
  PUT = 'PUT',
  DELETE = 'DELETE'
}

type FetchOptions = {
  method?: HTTPMethod;
  headers?: { [key: string]: string };
  body?: string; // stringified json
};

class ApiService {
  private host: string;

  constructor() {
    this.host = config.apiHost;
  }

  // temporary method, for easy testing, until we choose a library
  getFetch() {
    return fetch;
  }

  async fetch(
    endpoint: string,
    options: FetchOptions = {
      method: HTTPMethod.GET,
      headers: { 'Content-Type': 'application/json' }
    }
  ) {
    const fetch = this.getFetch();
    const url = `${endpoint}`;

    try {
      const response = await fetch(url, options);
      return await response.json();
    } catch (error) {
      throw error;
    }
  }
}

export default new ApiService();

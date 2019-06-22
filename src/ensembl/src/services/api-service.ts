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

type ApiServiceConfig = {
  host: string;
};

type FetchOptions = {
  host?: string;
  method?: HTTPMethod;
  headers?: { [key: string]: string };
  body?: string; // stringified json
  preserveEndpoint?: boolean;
};

const defaultMethod = HTTPMethod.GET;
const defaultHeaders = { Accept: 'application/json' };

class ApiService {
  private host: string;

  public constructor(config: ApiServiceConfig) {
    this.host = config.host;
  }

  // temporary method, for easy testing, until we choose a library
  public getFetch() {
    return fetch;
  }

  private buildFetchOptions(options: FetchOptions) {
    const headers: { [key: string]: string } = {
      ...defaultHeaders,
      ...options.headers
    };
    if (
      options.method &&
      [HTTPMethod.POST, HTTPMethod.PUT, HTTPMethod.PATCH].includes(
        options.method
      ) &&
      !headers.hasOwnProperty('Content-Type')
    ) {
      headers['Content-Type'] = 'application/json';
    }
    return {
      method: options.method || defaultMethod,
      headers: { ...defaultHeaders, ...options.headers },
      body: options.body
    };
  }

  public async fetch(endpoint: string, options: FetchOptions = {}) {
    const host = options.host || this.host;
    const fetch = this.getFetch();
    const url = options.preserveEndpoint ? endpoint : `${host}${endpoint}`;
    const fetchOptions = this.buildFetchOptions(options);

    try {
      const response = await fetch(url, fetchOptions);
      return await this.handleResponse(response, fetchOptions);
    } catch (error) {
      throw error;
    }
  }

  private async handleResponse(response: Response, options: FetchOptions) {
    if (response.status >= 400) {
      return {
        error: await response.json()
      };
    }
    if (options.headers && options.headers['Accept'] === 'application/json') {
      return await response.json();
    } else {
      return await response.text();
    }
  }
}

export default new ApiService({
  host: config.apiHost
});

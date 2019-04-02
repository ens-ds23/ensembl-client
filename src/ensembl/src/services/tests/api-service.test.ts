import apiService, { HTTPMethod } from '../api-service';
import config from 'config';

describe('api service', () => {
  let mockFetch: any;
  let mockResponse = { foo: 'foo' };

  beforeEach(() => {
    const mockJsonResolver = jest.fn(() => Promise.resolve(mockResponse));
    mockFetch = jest.fn(() =>
      Promise.resolve({
        json: mockJsonResolver
      })
    );
    jest
      .spyOn(apiService, 'getFetch')
      .mockImplementation(() => mockFetch as any);
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  describe('.fetch', () => {
    const endpoint = '/foo';

    test.skip('calls fetch passing it the endpoint', async () => {
      await apiService.fetch(endpoint);

      expect(mockFetch).toHaveBeenCalled();

      const mockFetchCall: any[] = mockFetch.mock.calls[0];
      const [url] = mockFetchCall;

      expect(url).toEqual(`${config.apiHost}${endpoint}`);
    });

    test('passes options to fetch', async () => {
      const options = {
        method: HTTPMethod.POST,
        body: JSON.stringify({ foo: 'bar' })
      };
      await apiService.fetch(endpoint, options);

      const mockFetchCall: any[] = mockFetch.mock.calls[0];
      const [, passedOptions] = mockFetchCall;

      expect(passedOptions).toEqual(options);
    });

    test('uses GET by default', async () => {
      await apiService.fetch(endpoint);

      const mockFetchCall: any[] = mockFetch.mock.calls[0];
      const [, options] = mockFetchCall;

      expect(options.method).toEqual(`GET`);
    });

    test('returns response from the api endpoint', async () => {
      const response = await apiService.fetch(endpoint);

      expect(response).toEqual(mockResponse);
    });
  });
});

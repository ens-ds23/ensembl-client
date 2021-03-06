import React from 'react';
import { mount } from 'enzyme';
import faker from 'faker';

import apiService from 'src/services/api-service';

import InlineSVG from './InlineSvg';

const sampleSVG = `
<svg viewBox="0 0 32 32">
  <rect width="1" height="1" />
</svg>
`;

describe('<InlineSVG />', () => {
  beforeEach(() => {
    jest
      .spyOn(apiService, 'fetch')
      .mockImplementation(() => Promise.resolve(sampleSVG));
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  it('fetches an svg and sets it as inline element', async () => {
    const src = faker.random.image();
    const wrapper = mount(<InlineSVG src={src} />);

    // FIXME: currently, the line below produces a warning (because of React state update)
    // it can be fixed by wrapping it into the asynchronous act function
    // introduced in react v.16.9.0 alpha, like so:
    // await act(async () => {
    //   await new Promise((resolve) => setTimeout(resolve, 0));
    // })
    await new Promise((resolve) => setTimeout(resolve, 0));

    wrapper.update();

    // expect the component to request the svg from the provided src
    const mockedFetch: any = apiService.fetch;
    const [requestedUrl] = mockedFetch.mock.calls[0];
    expect(requestedUrl).toBe(src);

    expect(wrapper.render().find('svg').length).toBe(1);
  });
});

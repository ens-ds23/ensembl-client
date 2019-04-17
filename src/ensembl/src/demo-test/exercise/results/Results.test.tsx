import React from 'react';
import { mount } from 'enzyme';
import faker from 'faker';
import times from 'lodash/times';

import Results from './Results';

const createResult = () => ({
  name: faker.lorem.word()
});

describe('<Results />', () => {
  it('renders results', () => {
    const results = times(5, () => createResult());
    const wrapper = mount(<Results results={results} />);

    const resultElements = wrapper.find('li');
    expect(resultElements.length).toBe(results.length);
  });
});

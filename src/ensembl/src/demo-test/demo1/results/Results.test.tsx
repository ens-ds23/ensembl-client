import React from 'react';
import { mount } from 'enzyme';
import faker from 'faker';
import times from 'lodash/times';

import Results from './Results';

const names = times(5, () => faker.name.findName());

const defaultProps = {
  names
};

describe('Results', () => {
  test('shows provided names', () => {
    const wrapper = mount(<Results {...defaultProps} />);
    expect(wrapper.find('li').length).toBe(defaultProps.names.length);
  });
});

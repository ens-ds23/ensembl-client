import React from 'react';
import { mount } from 'enzyme';

import Main from './Main';
import Field from '../field/Field';
import Results from '../results/Results';

describe('Main', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(<Main />);
  });

  test('contains the Field component', () => {
    expect(wrapper.find(Field).length).toBe(1);
  });

  test('does not show Results component when no characters have been fetched', () => {
    expect(wrapper.find(Results).length).toBe(0);
  });
});

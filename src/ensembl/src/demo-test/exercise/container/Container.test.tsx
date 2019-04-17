import React from 'react';
import { mount } from 'enzyme';

import apiService from 'src/services/api-service';

apiService.fetch = jest.fn(() => Promise.resolve([{ name: 'foo' }]));

import Container from './Container';
import Form from '../form/Form';
import Results from '../results/Results';

describe('<Container />', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(<Container />);
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('contains Results and Form components', () => {
    expect(wrapper.find(Form).length).toBe(1);
    expect(wrapper.find(Results).length).toBe(1);
  });

  it.only('requests github repositories upon submission', async () => {
    const form = wrapper.find(Form);
    const onSubmit = form.prop('onSubmit');
    const value = 'foo';

    await onSubmit(value);
    wrapper.update();
    const results = wrapper.find(Results);
    console.log(wrapper.html());

    expect(apiService.fetch).toHaveBeenCalledWith(
      `https://api.github.com/users/${value}/repos`
    );
    expect(results.length).toBeTruthy();
  });
});

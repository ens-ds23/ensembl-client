import React from 'react';
import { act } from 'react-dom/test-utils';
import { mount } from 'enzyme';
import { render, fireEvent, waitForElement } from 'react-testing-library';
import faker from 'faker';
import times from 'lodash/times';

import apiService from 'src/services/api-service';

import Main from './Main';
import Field from '../field/Field';
import Results from '../results/Results';

const mockRepo = () => ({
  name: faker.lorem.word()
});

const mockRepos = times(4, () => mockRepo());

apiService.fetch = jest.fn(() => Promise.resolve(mockRepos));

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

  test('shows repository names after they have been fetched', async () => {
    wrapper.find(Field).prop('onSubmit')('foo');

    await new Promise((resolve) => setTimeout(resolve, 0));
    wrapper.update();

    expect(wrapper.find(Results).length).toBe(1);
  });

  test('shows repository names after they have been fetched — Enzyme', async () => {
    const wrapper = mount(<Main />);
    const field = wrapper.find(Field);
    const onSubmit = field.prop('onSubmit');

    // works with react 16.9.0-alpha.0
    // await act(async () => {
    //   await onSubmit('foo');
    // });

    await onSubmit('foo');
    wrapper.update();
    const results = wrapper.find(Results);

    expect(results.length).toBeTruthy();
  });

  test('shows repository names after they have been fetched', async () => {
    const { container } = render(<Main />);
    const button = container.querySelector('button') as HTMLButtonElement;
    const input = container.querySelector('input') as HTMLInputElement;

    input.value = 'foo';

    fireEvent.submit(button);

    await waitForElement(() => container.querySelector('ul'), { container });
    expect(container.querySelector('ul')).toBeTruthy();
  });
});

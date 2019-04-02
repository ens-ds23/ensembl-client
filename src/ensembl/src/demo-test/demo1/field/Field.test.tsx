import React from 'react';
import { mount } from 'enzyme';
import { render, fireEvent } from 'react-testing-library';

import Field from './Field';

const onSubmit = jest.fn();

describe('Field', () => {
  // ENZYME
  test('calls onSubmit prop passing it the field value', () => {
    const value = 'foo';
    const wrapper = mount(<Field onSubmit={onSubmit} />);
    const form = wrapper.find('form');
    const input = wrapper.find('input').getDOMNode() as HTMLInputElement;

    input.value = value;
    form.simulate('submit');

    expect(onSubmit).toHaveBeenCalledWith(value);
  });

  // REACT-TESTING-LIBRARY
  test('calls onSubmit prop passing it the field value', () => {
    const value = 'foo';
    const { container } = render(<Field onSubmit={onSubmit} />);
    const button = container.querySelector('button');
    const input = container.querySelector('input') as HTMLInputElement;

    input.value = value;

    button && fireEvent.submit(button);
    expect(onSubmit).toHaveBeenCalledWith(value);
  });
});

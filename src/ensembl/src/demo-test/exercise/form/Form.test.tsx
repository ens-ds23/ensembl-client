import React from 'react';
import { mount } from 'enzyme';

import Form from './Form';

const onSubmit = jest.fn();

describe('<Form />', () => {
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(<Form onSubmit={onSubmit} />);
  });

  it('renders a form', () => {
    expect(wrapper.find('form').length).toBe(1);
  });

  it('calls onSubmit upon submission', () => {
    const value = 'foo';
    const form = wrapper.find('form');
    const button = wrapper.find('button');
    const input = wrapper.find('input').getDOMNode();
    input.value = value;

    form.simulate('submit');

    expect(onSubmit).toHaveBeenCalled();
  });
});

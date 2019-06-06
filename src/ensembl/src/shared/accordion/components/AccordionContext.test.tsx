import React from 'react';
import { mount } from 'enzyme';
import { Consumer, Provider } from './AccordionContext';

describe('ItemContext', () => {
  it('renders children props', () => {
    const wrapper = mount(
      <Provider>
        <Consumer>{(): string => 'Hello World'}</Consumer>
      </Provider>
    );

    expect(wrapper.text()).toBe('Hello World');
  });
});

import React from 'react';
import { mount } from 'enzyme';
import Accordion from './Accordion';
import { Consumer, Provider } from './ItemContext';

describe('ItemContext', () => {
  it('renders children props', () => {
    const wrapper = mount(
      <Accordion>
        <Provider uuid="FOO">
          <Consumer>{(): string => 'Hello World'}</Consumer>
        </Provider>
      </Accordion>
    );

    expect(wrapper.find(Accordion).text()).toBe('Hello World');
  });
});

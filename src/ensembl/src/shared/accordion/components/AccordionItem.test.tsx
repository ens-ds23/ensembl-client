import React from 'react';
import { mount } from 'enzyme';
import Accordion from './Accordion';
import AccordionItem from './AccordionItem';

describe('AccordionItem', () => {
  it('renders without erroring', () => {
    expect(() => {
      mount(<Accordion />);
    }).not.toThrow();
  });

  describe('className prop', () => {
    it('is "accordionItemDefault" by default', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem uuid={'FOO'} />
        </Accordion>
      );
      expect(
        wrapper
          .find(AccordionItem)
          .find('div')
          .hasClass('accordionItemDefault')
      ).toBe(true);
    });

    it('can be extended', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem uuid={'FOO'} className="foo" />
        </Accordion>
      );

      expect(
        wrapper
          .find(AccordionItem)
          .find('div')
          .hasClass('accordionItemDefault')
      ).toBe(true);
      expect(
        wrapper
          .find(AccordionItem)
          .find('div')
          .hasClass('foo')
      ).toBe(true);
    });
  });

  describe('children prop', () => {
    it('is respected', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem>Hello World</AccordionItem>
        </Accordion>
      );
      expect(wrapper.find(AccordionItem).text()).toBe('Hello World');
    });
  });
});

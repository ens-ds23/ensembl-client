import React from 'react';
import { mount } from 'enzyme';
import Accordion from './Accordion';
import AccordionItem from './AccordionItem';
import AccordionItemPanel from './AccordionItemPanel';

enum UUIDS {
  FOO = 'FOO',
  BAR = 'BAR'
}

describe('AccordionItemPanel', () => {
  it('renders without erroring', () => {
    expect(() => {
      mount(<AccordionItemPanel />);
    }).not.toThrow();
  });

  describe('className prop', () => {
    it('is "accordionPanelDefault" by default', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem uuid={UUIDS.FOO}>
            <AccordionItemPanel />
          </AccordionItem>
        </Accordion>
      );

      expect(
        wrapper
          .find(AccordionItemPanel)
          .find('div')
          .hasClass('accordionPanelDefault')
      ).toBe(true);
    });

    it('can be extended', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem uuid={UUIDS.FOO}>
            <AccordionItemPanel className="foo" />
          </AccordionItem>
        </Accordion>
      );

      expect(
        wrapper
          .find(AccordionItemPanel)
          .find('div')
          .hasClass('accordionPanelDefault')
      ).toBe(true);
      expect(
        wrapper
          .find(AccordionItemPanel)
          .find('div')
          .hasClass('foo')
      ).toBe(true);
    });
  });

  describe('children prop', () => {
    it('is respected', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem>
            <AccordionItemPanel>Hello World</AccordionItemPanel>
          </AccordionItem>
        </Accordion>
      );
      expect(wrapper.find(AccordionItemPanel).text()).toBe('Hello World');
    });
  });
});

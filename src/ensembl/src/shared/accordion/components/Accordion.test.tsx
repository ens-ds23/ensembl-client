import React from 'react';

import { mount } from 'enzyme';
import Accordion from './Accordion';
import AccordionItem from './AccordionItem';
import AccordionItemHeading from './AccordionItemHeading';
import AccordionItemButton from './AccordionItemButton';

enum UUIDS {
  FOO = 'FOO',
  BAR = 'Bar'
}

describe('Accordion', () => {
  it('renders without erroring', () => {
    expect(() => {
      mount(<Accordion />);
    }).not.toThrow();
  });

  describe('className', () => {
    it('is “accordionDefault” by default', () => {
      const wrapper = mount(<Accordion />);
      expect(wrapper.find('div').hasClass('accordionDefault')).toBe(true);
    });

    it('can be extended', () => {
      const wrapper = mount(<Accordion className="foo" />);

      expect(wrapper.find('div').props().className).toEqual(
        'accordionDefault foo'
      );
    });

    it('can also be overridden by using extendDefaultStyles === false', () => {
      const wrapper = mount(
        <Accordion className="foo" extendDefaultStyles={false} />
      );
      expect(wrapper.find('div').props().className).toEqual('foo');
    });
  });

  describe('expanding and collapsing: ', () => {
    it('permits multiple items to be expanded when allowMultipleExpanded is true', () => {
      const [FooHeader, BarHeader] = [
        (): JSX.Element => <AccordionItemButton />,
        (): JSX.Element => <AccordionItemButton />
      ];

      const wrapper = mount(
        <Accordion allowMultipleExpanded={true}>
          <AccordionItem>
            <AccordionItemHeading>
              <FooHeader />
            </AccordionItemHeading>
          </AccordionItem>
          <AccordionItem>
            <AccordionItemHeading>
              <BarHeader />
            </AccordionItemHeading>
          </AccordionItem>
        </Accordion>
      );

      wrapper.find(FooHeader).simulate('click');
      wrapper.find(BarHeader).simulate('click');

      expect(
        wrapper
          .find(FooHeader)
          .find('div')
          .props()['aria-expanded']
      ).toBe(true);
      expect(
        wrapper
          .find(BarHeader)
          .find('div')
          .props()['aria-expanded']
      ).toBe(true);
    });

    it('does not permit multiple items to be expanded when allowMultipleExpanded is false', () => {
      const [FooHeader, BarHeader] = [
        (): JSX.Element => <AccordionItemButton />,
        (): JSX.Element => <AccordionItemButton />
      ];

      const wrapper = mount(
        <Accordion>
          <AccordionItem>
            <AccordionItemHeading>
              <FooHeader />
            </AccordionItemHeading>
          </AccordionItem>
          <AccordionItem>
            <AccordionItemHeading>
              <BarHeader />
            </AccordionItemHeading>
          </AccordionItem>
        </Accordion>
      );

      wrapper.find(FooHeader).simulate('click');
      wrapper.find(BarHeader).simulate('click');

      expect(
        wrapper
          .find(FooHeader)
          .find('div')
          .props()['aria-expanded']
      ).toBe(false);
      expect(
        wrapper
          .find(BarHeader)
          .find('div')
          .props()['aria-expanded']
      ).toBe(true);
    });

    describe('allowZeroExpanded prop', () => {
      it('permits the last-expanded item to be collapsed when explicitly true', () => {
        const wrapper = mount(
          <Accordion allowZeroExpanded={true}>
            <AccordionItem>
              <AccordionItemHeading>
                <AccordionItemButton />
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        );

        wrapper.find(AccordionItemButton).simulate('click');
        wrapper.find(AccordionItemButton).simulate('click');

        expect(
          wrapper
            .find(AccordionItemButton)
            .find('div')
            .props()['aria-expanded']
        ).toEqual(false);
      });

      it('prevents the last-expanded item being collapsed by default', () => {
        const wrapper = mount(
          <Accordion allowZeroExpanded={false}>
            <AccordionItem>
              <AccordionItemHeading>
                <AccordionItemButton />
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        );

        wrapper.find(AccordionItemButton).simulate('click');
        wrapper.find(AccordionItemButton).simulate('click');

        expect(
          wrapper
            .find(AccordionItemButton)
            .find('div')
            .props()['aria-expanded']
        ).toEqual(true);
      });
    });

    describe('preExpanded prop', () => {
      it('expands items whose uuid props match those passed', () => {
        const wrapper = mount(
          <Accordion preExpanded={[UUIDS.FOO]}>
            <AccordionItem uuid={UUIDS.FOO}>
              <AccordionItemHeading>
                <AccordionItemButton />
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        );

        expect(
          wrapper
            .find(AccordionItemButton)
            .find('div')
            .props()['aria-expanded']
        ).toEqual(true);
      });

      it('collapses items by default', () => {
        const wrapper = mount(
          <Accordion>
            <AccordionItem>
              <AccordionItemHeading>
                <AccordionItemButton />
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        );

        expect(
          wrapper
            .find(AccordionItemButton)
            .find('div')
            .props()['aria-expanded']
        ).toEqual(false);
      });
    });

    describe('onChange prop', () => {
      it('is invoked with an array of expanded items’ uuids, if there are any', async () => {
        const onChange = jest.fn();
        const wrapper = mount(
          <Accordion onChange={onChange}>
            <AccordionItem uuid={UUIDS.FOO}>
              <AccordionItemHeading>
                <AccordionItemButton />
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        );

        wrapper.find(AccordionItemButton).simulate('click');

        // ugly hack: fall back to the end of event queue, giving priority to useEffect and useState
        await new Promise((resolve) => setTimeout(resolve, 0));

        expect(onChange).toHaveBeenCalledWith([UUIDS.FOO]);
      });

      it('is invoked with an empty array, if no items are expanded', async () => {
        const onChange = jest.fn();
        const wrapper = mount(
          <Accordion
            onChange={onChange}
            preExpanded={[UUIDS.FOO]}
            allowZeroExpanded={true}
          >
            <AccordionItem uuid={UUIDS.FOO}>
              <AccordionItemHeading>
                <AccordionItemButton />
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        );

        wrapper.find(AccordionItemButton).simulate('click');

        // ugly hack: fall back to the end of event queue, giving priority to useEffect and useState
        await new Promise((resolve) => setTimeout(resolve, 0));

        expect(onChange).toHaveBeenCalledWith([]);
      });
    });
  });
});

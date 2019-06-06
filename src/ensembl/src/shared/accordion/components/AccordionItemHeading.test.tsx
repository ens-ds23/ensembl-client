import React from 'react';
import { mount } from 'enzyme';
import Accordion from './Accordion';
import AccordionItem from './AccordionItem';
import AccordionItemButton from './AccordionItemButton';
import AccordionItemHeading, { SPEC_ERROR } from './AccordionItemHeading';

describe('AccordionItemHeading', () => {
  it('renders without erroring', () => {
    expect(() => {
      mount(<AccordionItemHeading />);
    }).not.toThrow();
  });

  describe('children prop', () => {
    it('is respected', () => {
      const wrapper = mount(
        <Accordion>
          <AccordionItem>
            <AccordionItemHeading>
              <AccordionItemButton>Hello World</AccordionItemButton>
            </AccordionItemHeading>
          </AccordionItem>
        </Accordion>
      );
      expect(wrapper.find(AccordionItemButton).text()).toEqual('Hello World');
    });
  });

  describe('validation', () => {
    // tslint:disable-next-line no-console
    const originalError = console.error;

    beforeEach(() => {
      // tslint:disable-next-line no-console
      console.error = jest.fn();
    });

    afterAll(() => {
      // tslint:disable-next-line no-console
      console.error = originalError;
    });

    it('permits a single AccordionItemButton as a child', () => {
      expect(() =>
        mount(
          <Accordion>
            <AccordionItem>
              <AccordionItemHeading>
                <AccordionItemButton>Hello World</AccordionItemButton>
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        )
      ).not.toThrowError(SPEC_ERROR);
    });

    it('permits a single AccordionItemButton as a child within an array', () => {
      expect(() =>
        mount(
          <Accordion>
            <AccordionItem>
              <AccordionItemHeading>
                [
                <AccordionItemButton key="foo">Hello World</AccordionItemButton>
                ]
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        )
      ).not.toThrowError(SPEC_ERROR);
    });

    it('does not permit multiple AccordionItemButton as children within an array', () => {
      expect(() =>
        mount(
          <Accordion>
            <AccordionItem>
              <AccordionItemHeading>
                [
                <AccordionItemButton key="foo">Hello World</AccordionItemButton>
                <AccordionItemButton key="bar">Hello World</AccordionItemButton>
                ]
              </AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        )
      ).toThrowError(SPEC_ERROR);
    });

    it('throws an error if you don’t nest an AccordionItemButton', () => {
      expect(() =>
        mount(
          <Accordion>
            <AccordionItem>
              <AccordionItemHeading />
            </AccordionItem>
          </Accordion>
        )
      ).toThrowError(SPEC_ERROR);
    });

    it('throws an error if you nest any non-AccordionItemButton element', () => {
      expect(() =>
        mount(
          <Accordion>
            <AccordionItem>
              <AccordionItemHeading>Foo</AccordionItemHeading>
            </AccordionItem>
          </Accordion>
        )
      ).toThrowError(SPEC_ERROR);
    });
  });
});

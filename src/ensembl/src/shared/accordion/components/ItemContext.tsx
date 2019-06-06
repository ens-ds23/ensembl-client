import React from 'react';
import {
  InjectedButtonAttributes,
  InjectedHeadingAttributes,
  InjectedPanelAttributes
} from '../helpers/AccordionStore';
import {
  AccordionContext,
  Consumer as AccordionContextConsumer
} from './AccordionContext';

export type UUID = string | number;

type ProviderProps = {
  children?: React.ReactNode;
  uuid: UUID;
  accordionContext: AccordionContext;
};

export type ProviderWrapperProps = Pick<
  ProviderProps,
  Exclude<keyof ProviderProps, 'accordionContext'>
>;

export type ItemContext = {
  uuid: UUID;
  expanded: boolean;
  disabled: boolean;
  panelAttributes: InjectedPanelAttributes;
  headingAttributes: InjectedHeadingAttributes;
  buttonAttributes: InjectedButtonAttributes;
  toggleExpanded(): void;
};

const Context = React.createContext(null as ItemContext | null);

const Provider = (props: ProviderProps) => {
  const toggleExpanded = (): void => {
    props.accordionContext.toggleExpanded(props.uuid);
  };

  const renderChildren = (accordionContext: AccordionContext): JSX.Element => {
    const { uuid } = props;

    const expanded = accordionContext.isItemExpanded(uuid);
    const disabled = accordionContext.isItemDisabled(uuid);
    const panelAttributes = accordionContext.getPanelAttributes(uuid);
    const headingAttributes = accordionContext.getHeadingAttributes(uuid);
    const buttonAttributes = accordionContext.getButtonAttributes(uuid);

    return (
      <Context.Provider
        value={{
          buttonAttributes,
          disabled,
          expanded,
          headingAttributes,
          panelAttributes,
          toggleExpanded: toggleExpanded,
          uuid
        }}
      >
        {props.children}
      </Context.Provider>
    );
  };

  return <AccordionContextConsumer>{renderChildren}</AccordionContextConsumer>;
};

const ProviderWrapper: React.SFC<ProviderWrapperProps> = (
  props: ProviderWrapperProps
): JSX.Element => (
  <AccordionContextConsumer>
    {(accordionContext: AccordionContext): JSX.Element => (
      <Provider {...props} accordionContext={accordionContext} />
    )}
  </AccordionContextConsumer>
);

export { ProviderWrapper as Provider };

type ConsumerProps = {
  children(container: ItemContext): React.ReactNode;
};

export const Consumer = (props: ConsumerProps) => {
  const renderChildren = (container: ItemContext | null): React.ReactNode => {
    return container ? props.children(container) : null;
  };

  return <Context.Consumer>{renderChildren}</Context.Consumer>;
};

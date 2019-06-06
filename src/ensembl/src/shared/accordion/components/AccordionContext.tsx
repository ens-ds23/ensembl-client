import React, { useState, useEffect } from 'react';
import AccordionStore, {
  InjectedButtonAttributes,
  InjectedHeadingAttributes,
  InjectedPanelAttributes
} from '../helpers/AccordionStore';
import { UUID } from './ItemContext';

export interface ProviderProps {
  preExpanded?: UUID[];
  allowMultipleExpanded?: boolean;
  allowZeroExpanded?: boolean;
  children?: React.ReactNode;
  onChange?(args: UUID[]): void;
}

export interface AccordionContext {
  allowMultipleExpanded: boolean;
  allowZeroExpanded: boolean;
  toggleExpanded(uuid: UUID): void;
  isItemDisabled(uuid: UUID): boolean;
  isItemExpanded(uuid: UUID): boolean;
  getPanelAttributes(uuid: UUID): InjectedPanelAttributes;
  getHeadingAttributes(uuid: UUID): InjectedHeadingAttributes;
  getButtonAttributes(uuid: UUID): InjectedButtonAttributes;
}

const Context = React.createContext(null as AccordionContext | null);

export const Provider = (props: ProviderProps) => {
  const [store, setStore] = useState(
    new AccordionStore({
      allowMultipleExpanded: props.allowMultipleExpanded,
      allowZeroExpanded: props.allowZeroExpanded,
      expanded: props.preExpanded
    })
  );

  useEffect(() => {
    if (props.onChange) {
      props.onChange(store.expanded);
    }
  }, [store]);

  const toggleExpanded = (key: UUID): void => {
    setStore(store.toggleExpanded(key));
  };

  const isItemDisabled = (key: UUID): boolean => {
    return store.isItemDisabled(key);
  };

  const isItemExpanded = (key: UUID): boolean => {
    return store.isItemExpanded(key);
  };

  const getPanelAttributes = (key: UUID): InjectedPanelAttributes => {
    return store.getPanelAttributes(key);
  };

  const getHeadingAttributes = (): InjectedHeadingAttributes => {
    return store.getHeadingAttributes();
  };

  const getButtonAttributes = (key: UUID): InjectedButtonAttributes => {
    return store.getButtonAttributes(key);
  };

  const { allowZeroExpanded, allowMultipleExpanded } = store;

  return (
    <Context.Provider
      value={{
        allowMultipleExpanded,
        allowZeroExpanded,
        getButtonAttributes,
        getHeadingAttributes,
        getPanelAttributes,
        isItemDisabled,
        isItemExpanded,
        toggleExpanded
      }}
    >
      {props.children || null}
    </Context.Provider>
  );
};

Provider.defaultProps = {
  allowMultipleExpanded: false,
  allowZeroExpanded: false
};
export class Consumer extends React.PureComponent<{
  children(container: AccordionContext): React.ReactNode;
}> {
  public renderChildren = (
    container: AccordionContext | null
  ): React.ReactNode => {
    return container ? this.props.children(container) : null;
  };

  public render(): JSX.Element {
    return <Context.Consumer>{this.renderChildren}</Context.Consumer>;
  }
}

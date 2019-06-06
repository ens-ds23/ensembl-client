import React from 'react';
import { InjectedButtonAttributes } from '../helpers/AccordionStore';
import { DivAttributes } from '../helpers/types';
import classNames from 'classnames';
import { Consumer as ItemConsumer, ItemContext } from './ItemContext';

import defaultStyles from '../css/Accordion.scss';

type Props = DivAttributes & {
  extendDefaultStyles: boolean;
  toggleExpanded(): void;
};

export const AccordionItemButton = (props: Props) => {
  const { className, extendDefaultStyles, toggleExpanded, ...rest } = props;

  let styles = className;

  if (extendDefaultStyles) {
    styles = classNames(defaultStyles.accordionButtonDefault, className);
  }

  return (
    <div
      {...rest}
      className={styles}
      onClick={toggleExpanded}
      data-accordion-component="AccordionItemButton"
    />
  );
};

AccordionItemButton.defaultProps = {
  extendDefaultStyles: true
};

type WrapperProps = Pick<
  DivAttributes,
  Exclude<keyof DivAttributes, keyof InjectedButtonAttributes>
>;

const AccordionItemButtonWrapper: React.SFC<WrapperProps> = (
  props: WrapperProps
): JSX.Element => (
  <ItemConsumer>
    {(itemContext: ItemContext): JSX.Element => {
      const { toggleExpanded, buttonAttributes } = itemContext;

      return (
        <AccordionItemButton
          toggleExpanded={toggleExpanded}
          {...props}
          {...buttonAttributes}
        />
      );
    }}
  </ItemConsumer>
);

export default AccordionItemButtonWrapper;

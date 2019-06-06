import React from 'react';
import { DivAttributes } from '../helpers/types';
import { nextUuid } from '../helpers/uuid';
import { Provider as ItemProvider, UUID } from './ItemContext';
import defaultStyles from '../css/Accordion.scss';
import classNames from 'classnames';

type Props = DivAttributes & {
  extendDefaultStyles: boolean;
  uuid?: UUID;
};

const AccordionItem = (props: Props) => {
  const instanceUuid: UUID = nextUuid();

  const {
    uuid = instanceUuid,
    extendDefaultStyles,
    className,
    ...rest
  } = props;

  let styles = className;

  if (extendDefaultStyles) {
    styles = classNames(defaultStyles.accordionItemDefault, className);
  }

  return (
    <ItemProvider uuid={uuid}>
      <div
        data-accordion-component="AccordionItem"
        {...rest}
        className={styles}
      />
    </ItemProvider>
  );
};

AccordionItem.defaultProps = {
  extendDefaultStyles: true
};

export default AccordionItem;

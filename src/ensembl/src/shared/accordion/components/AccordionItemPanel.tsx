import React from 'react';
import { DivAttributes } from '../helpers/types';
import { Consumer as ItemConsumer, ItemContext } from './ItemContext';
import defaultStyles from '../css/Accordion.scss';
import classNames from 'classnames';

type Props = DivAttributes & {
  extendDefaultStyles: boolean;
};

const AccordionItemPanel = (props: Props) => {
  const { className, extendDefaultStyles, ...rest } = props;

  let styles = className;

  if (extendDefaultStyles) {
    styles = classNames(defaultStyles.accordionPanelDefault, className);
  }

  return (
    <ItemConsumer>
      {({ panelAttributes }: ItemContext): JSX.Element => {
        return (
          <div
            data-accordion-component="AccordionItemPanel"
            {...rest}
            className={styles}
            {...panelAttributes}
          />
        );
      }}
    </ItemConsumer>
  );
};

AccordionItemPanel.defaultProps = {
  extendDefaultStyles: true
};

export default AccordionItemPanel;

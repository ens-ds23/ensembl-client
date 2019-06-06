import React from 'react';
import { DivAttributes } from '../helpers/types';
import { Consumer as ItemConsumer, ItemContext } from './ItemContext';

type Props = Pick<DivAttributes, Exclude<keyof DivAttributes, 'children'>> & {
  children(
    args: Partial<{ expanded: boolean; disabled: boolean }>
  ): React.ReactNode;
};

const AccordionItemState = (props: Props) => {
  return (
    <ItemConsumer>
      {(itemContext: ItemContext): JSX.Element => {
        const { expanded, disabled } = itemContext;
        return <>{props.children({ expanded, disabled })}</>;
      }}
    </ItemConsumer>
  );
};

export default AccordionItemState;

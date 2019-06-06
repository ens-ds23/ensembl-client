import React from 'react';

import styles from './Select.scss';

export enum Direction {
  UP = 'up',
  DOWN = 'down'
}

type Props = {
  direction: Direction;
};

const SelectArrowhead = (props: Props) => {
  const points =
    props.direction === Direction.DOWN ? '0,0 8,0 4,8' : '0,8 8,8 4,0';

  return (
    <svg className={styles.selectArrowhead} focusable="false" viewBox="0 0 8 8">
      <polygon points={points} />
    </svg>
  );
};

SelectArrowhead.defaultProps = {
  direction: Direction.DOWN
};

export default SelectArrowhead;

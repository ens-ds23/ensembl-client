import React, { ReactNode } from 'react';
import classNames from 'classnames';

import defaultStyles from './RoundButton.scss';

export enum RoundButtonStatus {
  ACTIVE = 'active',
  INACTIVE = 'inactive',
  DISABLED = 'disabled'
}

type Props = {
  onClick: () => void;
  status: RoundButtonStatus;
  classNames?: { [key in RoundButtonStatus]?: string };
  children: ReactNode;
};

const RoundButton = (props: Props) => {
  const handleClick = () => {
    if (props.status !== RoundButtonStatus.DISABLED) {
      props.onClick();
    }
  };

  const styles = props.classNames
    ? { ...defaultStyles, ...props.classNames }
    : defaultStyles;

  const className = classNames(defaultStyles.default, styles[props.status]);

  return (
    <button className={className} onClick={handleClick}>
      {props.children}
    </button>
  );
};

RoundButton.defaultProps = {
  status: RoundButtonStatus.INACTIVE
};

export default RoundButton;

import React from 'react';
import defaultStyles from './BadgedButton.scss';
import classNames from 'classnames';

type Props = {
  children: React.ReactChild;
  badgeContent?: string | number | undefined;
  className?: string;
};

const BadgedButton = (props: Props) => {
  const className = classNames(defaultStyles.badgeDefault, props.className);

  let badgeContent = props.badgeContent;

  if (typeof badgeContent === 'number' && badgeContent > 99) {
    badgeContent = '99+';
  } else if (typeof badgeContent === 'string') {
    // Limit the string to 3 characters
    badgeContent = badgeContent.substring(0, 3);
  }

  return (
    <div className={defaultStyles.badgedButton}>
      {props.children}
      {!!props.badgeContent && <div className={className}>{badgeContent}</div>}
    </div>
  );
};

export default BadgedButton;

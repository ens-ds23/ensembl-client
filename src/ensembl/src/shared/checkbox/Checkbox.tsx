import React from 'react';
import classNames from 'classnames';

import defaultStyles from './Checkbox.scss';

type WithoutLabelProps = {
  onChange: (status: boolean) => void;
  classNames?: any;
  disabled?: boolean;
  checked: boolean;
};

type WithLabelProps = WithoutLabelProps & {
  label: string;
  labelClassName?: string;
};

type Props = WithLabelProps | WithoutLabelProps;

const isWithLabel = (props: Props): props is WithLabelProps => {
  return 'label' in props;
};

const Checkbox = (props: Props) => {
  const handleOnChange = () => {
    // Do nothing if it is disabled
    if (props.disabled) {
      return;
    }

    // Call the onChange function
    props.onChange(!props.checked);
  };

  const styles = props.classNames
    ? { ...defaultStyles, ...props.classNames }
    : defaultStyles;

  const className = classNames(
    styles.defaultCheckbox,
    { [styles.checked]: props.checked },
    { [styles.unchecked]: !props.checked },
    { [styles.disabled]: props.disabled }
  );
  const labelClassName = classNames(
    defaultStyles.defaultLabel,
    isWithLabel(props) && props.labelClassName
  );

  return (
    <div className={defaultStyles.checkboxHolder}>
      <input
        type="checkbox"
        className={defaultStyles.hiddenInput}
        onChange={handleOnChange}
        checked={props.checked}
      />
      <div onClick={handleOnChange} className={className} />
      {isWithLabel(props) && (
        <label onClick={handleOnChange} className={labelClassName}>
          {props.label}
        </label>
      )}
    </div>
  );
};

export default Checkbox;

import React, { FunctionComponent, memo } from 'react';
import { NavLink } from 'react-router-dom';
import ImageButton from 'src/shared/image-button/ImageButton';

import styles from './Launchbar.scss';

type LaunchbarButtonProps = {
  app: string;
  description: string;
  icon: React.FunctionComponent<React.SVGProps<SVGSVGElement>> | string;
  enabled: boolean;
};

export const LaunchbarButton: FunctionComponent<LaunchbarButtonProps> = (
  props: LaunchbarButtonProps
) => {
  return props.enabled ? (
    <NavLink
      className={styles.launchbarButton}
      title={props.description}
      to={`/app/${props.app}`}
      activeClassName={styles.launchbarButtonSelected}
    >
      <ImageButton description={props.description} image={props.icon} />
    </NavLink>
  ) : (
    <div
      className={`${styles.launchbarButton} ${styles.launchbarButtonDisabled}`}
    >
      <ImageButton description={props.description} image={props.icon} />
    </div>
  );
};

export default memo(LaunchbarButton);

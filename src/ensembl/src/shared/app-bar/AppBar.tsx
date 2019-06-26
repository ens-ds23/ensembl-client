import React from 'react';

import SpeciesTabBar from '../species-tab-bar/SpeciesTabBar';

import chevronRightIcon from 'static/img/shared/chevron-right-grey.svg';
import styles from './AppBar.scss';

type AppBarProps = {
  currentAppName: string;
  activeGenomeId: string;
  onTabSelect: (genomeId: string) => void;
};

export const AppBar = (props: AppBarProps) => (
  <section className={styles.appBar}>
    <div className={styles.appBarTop}>
      <div>{props.currentAppName}</div>
    </div>
    <div className={styles.appBarBottom}>
      <SpeciesTabBar
        activeGenomeId={props.activeGenomeId}
        onTabSelect={props.onTabSelect}
      />
      <div className={styles.helpLink}>
        <a className="inactive">
          Help &amp; documentation <img src={chevronRightIcon} alt="" />
        </a>
      </div>
    </div>
  </section>
);

export default AppBar;

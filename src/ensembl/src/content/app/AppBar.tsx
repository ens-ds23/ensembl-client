import React from 'react';
import {
  withRouter,
  Route,
  RouteComponentProps,
  Switch
} from 'react-router-dom';

import SpeciesSelectorAppBar from 'src/content/app/species-selector/components/species-selector-app-bar/SpeciesSelectorAppBar';

import chevronRightIcon from 'static/img/shared/chevron-right-grey.svg';

import styles from './AppBar.scss';

type AppBarProps = RouteComponentProps;

const MockContent = () => (
  <>
    <div className={styles.top}>
      <div>Genome browser</div>
    </div>
    <div>
      <dl className={styles.selectedSpecies}>
        <dd>
          <strong>Human</strong> GRCh38.p12
        </dd>
        <dd>
          {/* <a className={`${styles.addSpecies} inactive`}>Change</a> */}
        </dd>
      </dl>
      <div className={styles.helpLink}>
        <a className="inactive">
          Help &amp; documentation <img src={chevronRightIcon} alt="" />
        </a>
      </div>
    </div>
  </>
);

export const AppBar = (props: AppBarProps) => {
  const { path } = props.match;

  return (
    <section className={styles.appBar}>
      <Switch>
        <Route
          path={`${path}/species-selector`}
          component={SpeciesSelectorAppBar}
        />
        <Route component={MockContent} />
      </Switch>
    </section>
  );
};

export default withRouter(AppBar);

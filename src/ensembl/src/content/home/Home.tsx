import React, { FunctionComponent, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';

import styles from './Home.scss';
import { fetchExampleEnsObjectsData } from 'src/ens-object/ensObjectActions';
import { getExampleEnsObjects } from 'src/ens-object/ensObjectSelectors';
import { RootState } from 'src/store';

type StateProps = {
  exampleEnsObjects: {};
};

type DispatchProps = {
  fetchExampleEnsObjectsData: () => void;
};

type OwnProps = {};

type HomeProps = StateProps & DispatchProps & OwnProps;

const Home: FunctionComponent<HomeProps> = (props: HomeProps) => {
  const [showPreviouslyViewed, toggleShowPreviouslyViewed] = useState(true);

  useEffect(() => {
    if (Object.values(props.exampleEnsObjects).length > 0) {
      toggleShowPreviouslyViewed(true);
    } else {
      toggleShowPreviouslyViewed(false);

      props.fetchExampleEnsObjectsData();
    }
  }, [props.exampleEnsObjects]);

  const getExampleObjectNode = (exampleObject: any) => {
    const {
      assembly,
      chromosome,
      display_name,
      location,
      object_type,
      species,
      stable_id
    } = exampleObject;
    const assemblyStr = `${assembly.name}_demo`;
    const regionStr = `${chromosome}:${location.start}-${location.end}`;
    const path = `/app/browser/${assemblyStr}/${stable_id}?region=${regionStr}`;

    return (
      <dd key={stable_id}>
        <Link to={path}>
          {species} {object_type} {display_name}
        </Link>
      </dd>
    );
  };

  return (
    <div className={styles.home}>
      <section className={styles.search}>
        <h2>Find</h2>
        <p>
          <input type="text" placeholder="Name, symbol or ID" disabled={true} />
          {/* <button disabled={true}>Go</button> */}
        </p>
        <div className={styles.filter}>
          <h2>Refine results</h2>
        </div>
      </section>
      {showPreviouslyViewed ? (
        <section className={styles.previouslyViewed}>
          <h2>Previously viewed</h2>
          <dl>
            {Object.values(props.exampleEnsObjects).map((exampleObject) =>
              getExampleObjectNode(exampleObject)
            )}
          </dl>
        </section>
      ) : null}
      <section className={styles.siteMessage}>
        <h4>Using the site</h4>
        <p>
          A very limited data set has been made available for this first
          release.
        </p>
        <p>
          Blue icons and text are clickable and will usually 'do' something.
        </p>
        <p>
          Grey icons indicate apps &amp; functionality that is planned, but not
          available yet.
        </p>
        <p className={styles.convoMessage}>
          It's very early days, but why not join the conversation:
        </p>
        <p>helpdesk@ensembl.org</p>
      </section>
    </div>
  );
};

const mapStateToProps = (state: RootState) => ({
  exampleEnsObjects: getExampleEnsObjects(state)
});

const mapDispatchToProps = {
  fetchExampleEnsObjectsData
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Home);

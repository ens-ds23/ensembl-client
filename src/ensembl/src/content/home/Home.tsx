import React, { FunctionComponent, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';

import * as urlFor from 'src/shared/helpers/urlHelper';
import { RootState } from 'src/store';

import { fetchExampleEnsObjects } from 'src/ens-object/ensObjectActions';
import {
  EnsObject,
  ExampleEnsObjectsData
} from 'src/ens-object/ensObjectTypes';
import { getExampleEnsObjects } from 'src/ens-object/ensObjectSelectors';
import { getGenomeInfo } from 'src/genome/genomeSelectors';
import { getCommittedSpecies } from '../app/species-selector/state/speciesSelectorSelectors';
import { CommittedItem } from '../app/species-selector/types/species-search';

import { fetchGenomeInfo } from 'src/genome/genomeActions';
import upperFirst from 'lodash/upperFirst';

import { GenomeInfoData } from 'src/genome/genomeTypes';
import styles from './Home.scss';

type StateProps = {
  activeSpecies: CommittedItem[];
  exampleEnsObjects: ExampleEnsObjectsData;
  genomeInfo: GenomeInfoData;
  totalSelectedSpecies: number;
};

type DispatchProps = {
  fetchExampleEnsObjects: () => void;
  fetchGenomeInfo: () => void;
};

type OwnProps = {};

type HomeProps = StateProps & DispatchProps & OwnProps;

const Home: FunctionComponent<HomeProps> = (props: HomeProps) => {
  const [showPreviouslyViewed, toggleShowPreviouslyViewed] = useState(false);

  useEffect(() => {
    props.fetchGenomeInfo();
  }, [props.activeSpecies]);

  useEffect(() => {
    props.fetchExampleEnsObjects();
  }, [props.genomeInfo]);

  useEffect(() => {
    if (Object.keys(props.exampleEnsObjects).length > 0) {
      toggleShowPreviouslyViewed(true);
    }
  }, [props.exampleEnsObjects]);

  const getPreviouslyViewed = () => {
    return props.activeSpecies.map((species) => {
      if (props.exampleEnsObjects[species.genome_id]) {
        return Object.values(props.exampleEnsObjects[species.genome_id]).map(
          (exampleObject: EnsObject) => {
            const location = `${exampleObject.location.chromosome}:${exampleObject.location.start}-${exampleObject.location.end}`;
            const path = urlFor.browser({
              genomeId: species.genome_id,
              focus: exampleObject.ensembl_object_id,
              location
            });

            return (
              <dd key={exampleObject.ensembl_object_id}>
                <Link to={path}>
                  {`${species.common_name} ${upperFirst(
                    exampleObject.object_type
                  )}: ${exampleObject.label}`}
                </Link>
              </dd>
            );
          }
        );
      }
    });
  };

  return (
    <div className={styles.home}>
      {!props.totalSelectedSpecies && (
        <>
          <span className={styles.speciesSelectorBannerText}>
            7 species now available
          </span>
          <Link
            className={styles.speciesSelectorBannerLink}
            to={urlFor.speciesSelector()}
          >
            Select a species to begin
          </Link>
        </>
      )}
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
      {showPreviouslyViewed && (
        <section className={styles.previouslyViewed}>
          <h2>Previously viewed</h2>
          {getPreviouslyViewed()}
        </section>
      )}
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
  activeSpecies: getCommittedSpecies(state),
  exampleEnsObjects: getExampleEnsObjects(state),
  totalSelectedSpecies: getCommittedSpecies(state).length,
  genomeInfo: getGenomeInfo(state)
});

const mapDispatchToProps = {
  fetchExampleEnsObjects,
  fetchGenomeInfo
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Home);

import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';

import { RootState } from 'src/store';
import { EnsObject } from 'src/ens-object/ensObjectTypes';
import { GenomeInfoData } from 'src/genome/genomeTypes';

import { getBrowserActiveGenomeId } from '../../../browserSelectors';
import { getGenomeInfo } from 'src/genome/genomeSelectors';
import { fetchExampleEnsObjects } from 'src/ens-object/ensObjectActions';
import { getExampleEnsObjects } from 'src/ens-object/ensObjectSelectors';
import * as urlFor from 'src/shared/helpers/urlHelper';

import { ExampleEnsObjectsData } from 'src/ens-object/ensObjectTypes';

import upperFirst from 'lodash/upperFirst';

import styles from '../TrackPanelModal.scss';

type StateProps = {
  activeGenomeId: string;
  genomeInfo: GenomeInfoData;
  exampleEnsObjects: ExampleEnsObjectsData;
};

type DispatchProps = {
  fetchExampleEnsObjects: (objectId: string) => void;
};

type OwnProps = {};

type TrackPanelBookmarksProps = StateProps & DispatchProps & OwnProps;

export const TrackPanelBookmarks = (props: TrackPanelBookmarksProps) => {
  useEffect(() => {
    props.fetchExampleEnsObjects(props.activeGenomeId);
  }, [props.activeGenomeId]);

  const getPreviouslyViewed = () => {
    return Object.values(props.exampleEnsObjects[props.activeGenomeId]).map(
      (exampleObject: EnsObject) => {
        const locationStr = `${exampleObject.location.chromosome}:${exampleObject.location.start}-${exampleObject.location.end}`;
        const path = urlFor.browser({
          genomeId: props.activeGenomeId,
          focus: exampleObject.ensembl_object_id,
          location: locationStr
        });

        return (
          <dd key={exampleObject.ensembl_object_id}>
            <Link to={path}>
              {upperFirst(exampleObject.object_type)}: {exampleObject.label}
            </Link>
          </dd>
        );
      }
    );
  };

  return (
    <section className="trackPanelBookmarks">
      <h3>Bookmarks</h3>
      <p>Save multiple browser configurations</p>
      {props.exampleEnsObjects[props.activeGenomeId] ? (
        <dl className={styles.previouslyViewed}>
          <dt>Previously viewed</dt>
          {getPreviouslyViewed()}
        </dl>
      ) : null}
    </section>
  );
};

const mapStateToProps = (state: RootState) => ({
  activeGenomeId: getBrowserActiveGenomeId(state),
  genomeInfo: getGenomeInfo(state),
  exampleEnsObjects: getExampleEnsObjects(state)
});

const mapDispatchToProps = {
  fetchExampleEnsObjects
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TrackPanelBookmarks);

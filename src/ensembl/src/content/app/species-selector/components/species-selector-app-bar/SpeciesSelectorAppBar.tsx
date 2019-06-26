import React from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router-dom';

import { getCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import {
  toggleSpeciesUseAndSave,
  deleteSpeciesAndSave
} from 'src/content/app/species-selector/state/speciesSelectorActions';
import * as urlFor from 'src/shared/helpers/urlHelper';

import SelectedSpecies from 'src/content/app/species-selector/components/selected-species/SelectedSpecies';

import { RootState } from 'src/store';
import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

import styles from './SpeciesSelectorAppBar.scss';
import appBarStyles from 'src/shared/app-bar/AppBar.scss';

type Props = {
  selectedSpecies: CommittedItem[];
  toggleSpeciesUse: (genomeId: string) => void;
  onSpeciesDelete: (genomeId: string) => void;
};

export const PlaceholderMessage = () => (
  <div className={styles.placeholderMessage}>
    Search for a species, select from popular species or browse species by data
    to manage your favourites
  </div>
);

export const SpeciesSelectorAppBar = (props: Props) => {
  return (
    <div className={appBarStyles.appBar}>
      <div className={appBarStyles.appBarTop}>
        <div className={appBarStyles.top}>Species Selector</div>
      </div>
      <div className={styles.main}>
        {props.selectedSpecies.length > 0 ? (
          <SelectedSpeciesList {...props} />
        ) : (
          <PlaceholderMessage />
        )}
      </div>
    </div>
  );
};

const SelectedSpeciesList = (props: Props) => {
  const shouldLinkToGenomeBrowser =
    props.selectedSpecies.filter(({ isEnabled }) => isEnabled).length > 0;
  return (
    <>
      {props.selectedSpecies.map((species) => (
        <SelectedSpecies
          key={species.genome_id}
          species={species}
          onToggleUse={props.toggleSpeciesUse}
          onRemove={props.onSpeciesDelete}
        />
      ))}
      {shouldLinkToGenomeBrowser && (
        <div className={styles.genomeBrowserLinkContainer}>
          <Link to={urlFor.browser()}>View in Genome Browser</Link>
        </div>
      )}
    </>
  );
};

const mapStateToProps = (state: RootState) => ({
  selectedSpecies: getCommittedSpecies(state)
});

const mapDispatchToProps = {
  toggleSpeciesUse: toggleSpeciesUseAndSave,
  onSpeciesDelete: deleteSpeciesAndSave
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(SpeciesSelectorAppBar);

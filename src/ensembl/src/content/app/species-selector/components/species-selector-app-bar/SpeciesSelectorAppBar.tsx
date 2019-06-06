import React from 'react';
import { connect } from 'react-redux';

import { getCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import {
  toggleSpeciesUse,
  deleteSpeciesAndSave
} from 'src/content/app/species-selector/state/speciesSelectorActions';

import SelectedSpecies from 'src/content/app/species-selector/components/selected-species/SelectedSpecies';

import { RootState } from 'src/store';
import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

import styles from './SpeciesSelectorAppBar.scss';
import appBarStyles from 'src/content/app/AppBar.scss';

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
    <div>
      <div className={appBarStyles.top}>Species Selector</div>
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
    </>
  );
};

const mapStateToProps = (state: RootState) => ({
  selectedSpecies: getCommittedSpecies(state)
});

const mapDispatchToProps = {
  toggleSpeciesUse,
  onSpeciesDelete: deleteSpeciesAndSave
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(SpeciesSelectorAppBar);

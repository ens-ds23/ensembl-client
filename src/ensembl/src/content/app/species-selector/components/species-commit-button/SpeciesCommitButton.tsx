import React from 'react';
import { connect } from 'react-redux';

import {
  hasCurrentSpecies,
  canCommitSpecies
} from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import { commitSelectedSpeciesAndSave } from 'src/content/app/species-selector/state/speciesSelectorActions';

import { PrimaryButton } from 'src/shared/button/Button';

import styles from './SpeciesCommitButton.scss';

import { RootState } from 'src/store';

type Props = {
  hasCurrentSpecies: boolean; // tells whether a species has been selected and can be committed
  disabled: boolean;
  onCommit: () => void;
};

export const SpeciesCommitButton = (props: Props) => {
  const handleClick = () => {
    props.onCommit();
  };

  return props.hasCurrentSpecies ? (
    <div className={styles.speciesCommitButton}>
      <PrimaryButton onClick={handleClick} isDisabled={props.disabled}>
        Add
      </PrimaryButton>
    </div>
  ) : null;
};

const mapStateToProps = (state: RootState) => ({
  hasCurrentSpecies: hasCurrentSpecies(state),
  disabled: !canCommitSpecies(state)
});

const mapDispatchToProps = {
  onCommit: commitSelectedSpeciesAndSave
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(SpeciesCommitButton);

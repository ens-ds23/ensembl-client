import React from 'react';
import { Link } from 'react-router-dom';
import { connect } from 'react-redux';

import SpeciesTab from 'src/shared/species-tab/SpeciesTab';

import { CommittedItem } from 'src/content/app/species-selector/types/species-search';
import { RootState } from 'src/store';
import { getEnabledCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';

import styles from './SpeciesTabBar.scss';

type StateProps = {
  species: CommittedItem[]; // list of species
};

type OwnProps = {
  activeGenomeId: string; // id of the species that is currently active
  onTabSelect: (genomeId: string) => void;
};

type SpeciesTabBarProps = StateProps & OwnProps;

export const SpeciesTabBar = (props: SpeciesTabBarProps) => {
  return (
    <div className={styles.speciesTabBar}>
      {props.species.map((species) => (
        <SpeciesTab
          key={species.genome_id}
          species={species}
          isActive={species.genome_id === props.activeGenomeId}
          onActivate={props.onTabSelect}
        />
      ))}
      <div className={styles.addSpeciesLink}>
        <Link to={'/app/species-selector'}>Change</Link>
      </div>
    </div>
  );
};

const mapStateToProps = (state: RootState) => ({
  species: getEnabledCommittedSpecies(state)
});

export default connect(mapStateToProps)(SpeciesTabBar);

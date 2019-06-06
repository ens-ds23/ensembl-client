import React from 'react';

import SpeciesTab from 'src/shared/species-tab/SpeciesTab';

import styles from './SpeciesTabBar.scss';

import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

type Props = {
  species: CommittedItem[]; // list of species
  activeGenomeId: string; // id of the species that is currently active
  onTabSelect: (genomeId: string) => void;
};

const SpeciesTabBar = (props: Props) => {
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
    </div>
  );
};

export default SpeciesTabBar;

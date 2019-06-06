import React, { useState } from 'react';
import { storiesOf } from '@storybook/react';

import speciesData from './speciesData';
import juneSpeciesData from './juneSpeciesData';

import SpeciesTabBar from 'src/shared/species-tab-bar/SpeciesTabBar';

import styles from './SpeciesTabBar.stories.scss';

import { CommittedItem } from 'src/content/app/species-selector/types/species-search';

type WrapperProps = {
  species: CommittedItem[];
};

const Wrapper = (props: WrapperProps) => {
  const [activeGenomeId, setActiveGenomeId] = useState(
    speciesData[0].genome_id
  );
  const onTabSelect = (genomeId: string) => {
    setActiveGenomeId(genomeId);
  };

  return (
    <div className={styles.wrapper}>
      <SpeciesTabBar
        species={props.species}
        activeGenomeId={activeGenomeId}
        onTabSelect={onTabSelect}
      />
    </div>
  );
};

storiesOf('Components|Shared Components/SpeciesTabBar', module)
  .add('few species', () => <Wrapper species={speciesData.slice(0, 3)} />)
  .add('more species', () => <Wrapper species={juneSpeciesData} />)
  .add('multiple species', () => <Wrapper species={speciesData} />);

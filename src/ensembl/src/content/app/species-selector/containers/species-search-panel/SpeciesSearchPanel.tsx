import React from 'react';

import SpeciesSearchField from 'src/content/app/species-selector/components/species-search-field/SpeciesSearchField';
import SpeciesCommitButton from 'src/content/app/species-selector/components/species-commit-button/SpeciesCommitButton';
import AssemblySelector from 'src/content/app/species-selector/components/assembly-selector/AssemblySelector';

import styles from './SpeciesSearchPanel.scss';

const SearchPanel = () => {
  return (
    <section className={styles.speciesSearchPanel}>
      <div className={styles.speciesSearchPanelRow}>
        <SpeciesSearchField />
        <SpeciesCommitButton />
      </div>
      <div className={styles.assemblyWrapper}>
        <AssemblySelector />
      </div>
    </section>
  );
};

export default SearchPanel;

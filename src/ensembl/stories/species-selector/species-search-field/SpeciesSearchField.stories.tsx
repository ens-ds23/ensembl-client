import React from 'react';
import { storiesOf } from '@storybook/react';
import { action } from '@storybook/addon-actions';

import { SpeciesSearchField } from 'src/content/app/species-selector/components/species-search-field/SpeciesSearchField';

import humanSearchResults from 'tests/data/species-selector/human-search';

import styles from './SpeciesSearchField.stories.scss';

const onSearchChange = action('search change');
const onMatchSelected = action('match selected');
const clearSearchResults = action('clear search');

storiesOf('Components|Species Selector/Species search field', module)
  .add('empty', () => (
    <div className={styles.searchFieldContainer}>
      <SpeciesSearchField
        matches={[]}
        onSearchChange={onSearchChange}
        onMatchSelected={onMatchSelected}
        clearSelectedSearchResult={clearSearchResults}
        selectedItemText={null}
      />
    </div>
  ))
  .add('with search matches', () => (
    <div className={styles.searchFieldContainer}>
      <SpeciesSearchField
        matches={humanSearchResults.matches}
        onSearchChange={onSearchChange}
        onMatchSelected={onMatchSelected}
        clearSelectedSearchResult={clearSearchResults}
        selectedItemText={null}
      />
    </div>
  ));

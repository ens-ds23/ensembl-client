import React, { useState } from 'react';
import { connect } from 'react-redux';

import {
  fetchSpeciesSearchResults,
  handleSelectedSearchResult,
  clearSelectedSearchResult
} from 'src/content/app/species-selector/state/speciesSelectorActions';

import {
  getSearchResults,
  getSelectedItemText
} from 'src/content/app/species-selector/state/speciesSelectorSelectors';

import SpeciesSearchMatch from '../species-search-match/SpeciesSearchMatch';

import AutosuggestSearchField from 'src/shared/autosuggest-search-field/AutosuggestSearchField';

import {
  SearchMatch,
  SearchMatches
} from 'src/content/app/species-selector/types/species-search';
import { RootState } from 'src/store';

import styles from './SpeciesSearchField.scss';

type Props = {
  onSearchChange: (search: string) => void;
  onMatchSelected: (match: SearchMatch) => void;
  clearSelectedSearchResult: () => void;
  matches: SearchMatches[];
  selectedItemText: string | null;
};

export const SpeciesSearchField = (props: Props) => {
  const [search, setSearch] = useState('');

  const handleSearchChange = (search: string) => {
    setSearch(search);
    if (props.selectedItemText) {
      props.clearSelectedSearchResult();
    }

    search = search.trim();
    if (search.length >= 3) {
      props.onSearchChange(search);
    }
  };

  const onMatchSelected = (match: SearchMatch) => {
    props.onMatchSelected(match);
    setSearch('');
  };

  return (
    <AutosuggestSearchField
      search={props.selectedItemText || search}
      placeholder="Common or scientific name..."
      className={styles.speciesSearchFieldWrapper}
      onChange={handleSearchChange}
      onSelect={onMatchSelected}
      matchGroups={buildMatchGroups(props.matches)}
      searchFieldClassName={styles.speciesSearchField}
      canShowSuggestions={!Boolean(props.selectedItemText)}
    />
  );
};

const buildMatchGroups = (groups: SearchMatches[]) => {
  return groups.map((group) => {
    const matches = group.map((match) => {
      const renderedMatch = <SpeciesSearchMatch match={match} />;
      return {
        data: match,
        element: renderedMatch
      };
    });
    return { matches };
  });
};

const mapStateToProps = (state: RootState) => ({
  matches: getSearchResults(state),
  selectedItemText: getSelectedItemText(state)
});

const mapDispatchToProps = {
  onSearchChange: fetchSpeciesSearchResults.request,
  onMatchSelected: handleSelectedSearchResult,
  clearSelectedSearchResult
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(SpeciesSearchField);

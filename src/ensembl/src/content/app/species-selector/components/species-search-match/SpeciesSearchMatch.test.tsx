import React from 'react';
import { render } from 'enzyme';

import SpeciesSearchMatch from './SpeciesSearchMatch';

import styles from './SpeciesSearchMatch.scss';

import { MatchedFieldName } from 'src/content/app/species-selector/types/species-search';

const matchTemplate = {
  genome_id: 'homo_sapiens_38',
  reference_genome_id: null,
  common_name: 'Human',
  scientific_name: 'Homo sapiens',
  subtype: 'GRCh38.p12',
  matched_substrings: [
    {
      length: 3,
      offset: 0,
      match: MatchedFieldName.COMMON_NAME
    }
  ]
};

describe('<SpeciesSearchMatch />', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test('highlights a single match in the description field', () => {
    const renderedMatch = render(<SpeciesSearchMatch match={matchTemplate} />);
    const highlightedFragments = renderedMatch.find(`.${styles.matched}`);
    expect(highlightedFragments.length).toBe(1);
    // highlighting Hum in Human
    expect(highlightedFragments.first().text()).toBe('Hum');
  });

  test('highlights a single match in the scientific_name field', () => {
    const match = {
      ...matchTemplate,
      matched_substrings: [
        {
          length: 3,
          offset: 0,
          match: MatchedFieldName.SCIENTIFIC_NAME
        }
      ]
    };
    const renderedMatch = render(<SpeciesSearchMatch match={match} />);
    const highlightedFragments = renderedMatch.find(
      `.${styles.scientificName}
      .${styles.matched}`
    );
    expect(highlightedFragments.length).toBe(1);
    // highlighting Hom in Homo sapiens
    expect(highlightedFragments.first().text()).toBe('Hom');
  });

  test('highlights multiple matches in the description field', () => {
    const match = {
      ...matchTemplate,
      common_name: null,
      scientific_name: 'Bacillus subtilis',
      subtype: null,
      matched_substrings: [
        {
          length: 3,
          offset: 0,
          match: MatchedFieldName.SCIENTIFIC_NAME
        },
        {
          length: 3,
          offset: 9,
          match: MatchedFieldName.SCIENTIFIC_NAME
        }
      ]
    };

    const renderedMatch = render(<SpeciesSearchMatch match={match} />);

    const highlightedFragments = renderedMatch.find(`.${styles.matched}`);
    expect(highlightedFragments.length).toBe(2);
    // highlighting Bac in Bacillus and sub in subtilis
    expect(highlightedFragments.first().text()).toBe('Bac');
    expect(highlightedFragments.last().text()).toBe('sub');
  });
});

import React, { useState, useEffect } from 'react';
import { storiesOf } from '@storybook/react';
import { action } from '@storybook/addon-actions';

import AutosuggestSearchField from 'src/shared/autosuggest-search-field/AutosuggestSearchField';
import QuestionButton from 'src/shared/question-button/QuestionButton';

import notes from './autosuggestSearchField.md';

import * as matches from 'tests/data/species-selector/species-search';

import styles from './AutosuggestSearchField.stories.scss';

const ItemWrapper = (props: any) => <span>{props.description}</span>;

const groupedMatches = [
  {
    matches: [
      {
        data: matches.human38Match,
        element: <ItemWrapper {...matches.human38Match} />
      },
      {
        data: matches.human37Match,
        element: <ItemWrapper {...matches.human37Match} />
      }
    ]
  },
  {
    matches: [
      {
        data: matches.azospirillumMatch,
        element: <ItemWrapper {...matches.azospirillumMatch} />
      }
    ]
  }
];

const Wrapper = (props: any) => {
  const [value, setValue] = useState('');
  const [isSelected, setIsSelected] = useState(false);
  const { searchField: AutosuggestSearchField, ...otherProps } = props;

  useEffect(() => {
    setIsSelected(false);
  }, [value]);

  return (
    <div>
      <AutosuggestSearchField
        search={value}
        onChange={setValue}
        onSubmit={(value: string) => {
          setIsSelected(true);
          action('autosuggest-search-field-submit')(value);
        }}
        onSelect={(match: any) => {
          const { description } = match;
          setValue(description);
          // allow time for the isSelected state value to get updated
          setTimeout(() => setIsSelected(true), 0);
          action('autosuggest-search-field-select')(description);
        }}
        canShowSuggestions={!isSelected}
        className={styles.autosuggestSearchField}
        searchFieldClassName={styles.autosuggestSearchFieldInput}
        {...otherProps}
      />
    </div>
  );
};

const RightCorner = () => (
  <QuestionButton onHover={action('question-button-hover')} />
);

storiesOf('Components|Shared Components/AutosuggestSearchField', module)
  .add(
    'allowing only submission of matches',
    () => (
      <div className={styles.container}>
        <p>For description of the component's behaviour, see the Notes tab.</p>
        <div className={styles.example}>
          <p>
            Notice how the first match is immediately selected. Check the
            Actions panel to see what gets submitted.
          </p>
          <Wrapper
            searchField={AutosuggestSearchField}
            matchGroups={groupedMatches}
            rightCorner={<RightCorner />}
          />
        </div>
        <div className={styles.example}>
          <p>
            If no matches were found, pressing Enter will not submit the raw
            search string.
          </p>
          <Wrapper
            searchField={AutosuggestSearchField}
            rightCorner={<RightCorner />}
          />
        </div>
      </div>
    ),
    { notes }
  )
  .add(
    'allowing raw search submission',
    () => (
      <div className={styles.container}>
        <div className={styles.example}>
          <p>
            Notice that, as opposed to the other variant, first suggestion is
            not automatically pre-selected. Pressing enter when no suggestion is
            selected will submit current content of the search field (see the
            Actions panel).
          </p>
          <Wrapper
            searchField={AutosuggestSearchField}
            matchGroups={groupedMatches}
            allowRawInputSubmission={true}
            rightCorner={<RightCorner />}
          />
        </div>
        <div className={styles.example}>
          <p>
            When no matches are available, pressing enter will submit the
            current content of the field.
          </p>
          <Wrapper
            searchField={AutosuggestSearchField}
            allowRawInputSubmission={true}
            rightCorner={<RightCorner />}
          />
        </div>
      </div>
    ),
    { notes }
  );

import React, { useState, useEffect, useRef, ReactNode } from 'react';
import classNames from 'classnames';

import SearchField from 'src/shared/search-field/SearchField';
import AutosuggestionPanel, {
  GroupOfMatchesType,
  MatchIndex
} from './AutosuggestionPanel';

import * as keyCodes from 'src/shared/constants/keyCodes';

import styles from './AutosuggestSearchField.scss';

type CommonProps = {
  search: string;
  onChange: (value: string) => void;
  onSelect: (match: any) => void;
  matchGroups: GroupOfMatchesType[];
  canShowSuggestions: boolean;
  placeholder?: string;
  onFocus?: () => void;
  onBlur?: () => void;
  rightCorner?: ReactNode;
  className?: string;
  searchFieldClassName?: string;
};

// with this set of props user can submit raw content of the search field
// (not just one of suggested matches)
type PropsAllowingRawDataSubmission = {
  allowRawInputSubmission: true;
  onSubmit: (value: string) => void;
};

// with this set of props user can submit only one of suggested matches
// (notice no onSubmit prop; typescript is smart enough to know it won't be available)
type PropsDisallowingRawDataSubmission = {
  allowRawInputSubmission: false;
};

type Props =
  | (CommonProps & PropsAllowingRawDataSubmission)
  | (CommonProps & PropsDisallowingRawDataSubmission);

function getNextItemIndex(
  props: Props,
  currentItemIndex: MatchIndex | null
): MatchIndex {
  const { matchGroups, allowRawInputSubmission } = props;
  const [groupIndex, itemIndex] = currentItemIndex || [null, null];
  const currentGroup =
    typeof groupIndex === 'number' ? matchGroups[groupIndex] : null;
  const firstItemIndex: MatchIndex = [0, 0];

  if (itemIndex === null) {
    return firstItemIndex;
  } else if (currentGroup && itemIndex < currentGroup.matches.length - 1) {
    // move to the next item in the group
    return [groupIndex, itemIndex + 1] as MatchIndex;
  } else if (groupIndex === matchGroups.length - 1) {
    // this is the last item in the last group;
    // either return null if submitting raw input is allowed, or
    // cycle back to the first item in the list
    return allowRawInputSubmission ? null : firstItemIndex;
  } else if (typeof groupIndex === 'number') {
    // move to the next group in the list
    return [groupIndex + 1, 0];
  } else {
    return null; // should never happen, but makes Typescript happy
  }
}

function getPreviousItemIndex(
  props: Props,
  currentItemIndex: MatchIndex | null
): MatchIndex {
  const { matchGroups, allowRawInputSubmission } = props;
  const [groupIndex, itemIndex] = currentItemIndex || [null, null];
  const lastGroupIndex = matchGroups.length - 1;
  const lastGroupItemIndex = matchGroups[lastGroupIndex].matches.length - 1;
  const lastItemIndex: MatchIndex = [lastGroupIndex, lastGroupItemIndex];
  if (itemIndex === null) {
    return lastItemIndex;
  } else if (itemIndex > 0) {
    // move to the previous item
    return [groupIndex, itemIndex - 1] as MatchIndex;
  } else if (groupIndex === 0) {
    // this is the first item in the first group;
    // either return null if submitting raw input is allowed,
    // or cycle back to the very last item in the list
    return allowRawInputSubmission ? null : lastItemIndex;
  } else if (typeof groupIndex === 'number') {
    // move to the last item in the previous group
    const previousGroupIndex = groupIndex - 1;
    const lastItemIndex = matchGroups[previousGroupIndex].matches.length - 1;
    return [previousGroupIndex, lastItemIndex];
  } else {
    return null; // should never happen, but makes Typescript happy
  }
}

const AutosuggestSearchField = (props: Props) => {
  const initialHighlightedItemIndex: MatchIndex = props.allowRawInputSubmission
    ? null
    : [0, 0];
  const [highlightedItemIndex, setHighlightedItemIndex] = useState(
    initialHighlightedItemIndex
  );
  const [isSelected, setIsSelected] = useState(false);
  const [canShowSuggesions, setCanShowSuggestions] = useState(true);
  const element = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setIsSelected(false);
  }, [props.search]);

  useEffect(() => {
    const onClickOutside = (event: MouseEvent) => {
      const currentElement = element.current;
      if (!currentElement) return;

      if (
        event.target !== currentElement &&
        !currentElement.contains(event.target as HTMLElement)
      ) {
        setCanShowSuggestions(false);
      }
    };
    window.addEventListener('click', onClickOutside);
    return () => window.removeEventListener('click', onClickOutside);
  }, []);

  const handleSelect = (match: any) => {
    setIsSelected(true);
    props.onSelect(match);
  };

  const handleBlur = () => {
    props.onBlur && props.onBlur();
  };

  const handleFocus = () => {
    if (!canShowSuggesions) {
      setCanShowSuggestions(true);
    }
    props.onFocus && props.onFocus();
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (![keyCodes.UP, keyCodes.DOWN].includes(event.keyCode)) return;

    event.preventDefault();

    if (event.keyCode === keyCodes.UP) {
      setHighlightedItemIndex(
        getPreviousItemIndex(props, highlightedItemIndex)
      );
    } else {
      setHighlightedItemIndex(getNextItemIndex(props, highlightedItemIndex));
    }
  };

  const handleItemHover = (itemIndex: MatchIndex) => {
    setHighlightedItemIndex(itemIndex);
  };

  const handleChange = (value: string) => {
    if (value !== props.search) {
      props.onChange(value);
    }
  };

  const handleSubmit = (value: string) => {
    if (!highlightedItemIndex && props.allowRawInputSubmission) {
      props.onSubmit(value);
    } else if (highlightedItemIndex) {
      const [groupIndex, itemIndex] = highlightedItemIndex;
      const match = props.matchGroups[groupIndex].matches[itemIndex];
      props.onSelect(match.data);
    }
    setIsSelected(true);
  };

  const shouldShowSuggestions =
    props.search &&
    Boolean(props.matchGroups.length) &&
    canShowSuggesions &&
    !isSelected &&
    props.canShowSuggestions;

  const className = classNames(
    styles.autosuggestionSearchField,
    props.className
  );
  const searchFieldClassName = classNames(
    styles.searchFieldInput,
    props.searchFieldClassName
  );

  return (
    <div ref={element} className={className}>
      <SearchField
        search={props.search}
        placeholder={props.placeholder}
        rightCorner={props.rightCorner}
        onChange={handleChange}
        onFocus={handleFocus}
        onBlur={handleBlur}
        onKeyDown={handleKeyDown}
        onSubmit={handleSubmit}
        className={searchFieldClassName}
      />
      {shouldShowSuggestions && (
        <AutosuggestionPanel
          highlightedItemIndex={highlightedItemIndex}
          matchGroups={props.matchGroups}
          onSelect={handleSelect}
          allowRawInputSubmission={props.allowRawInputSubmission}
          onItemHover={handleItemHover}
        />
      )}
    </div>
  );
};

AutosuggestSearchField.defaultProps = {
  matchGroups: [],
  canShowSuggestions: true,
  allowRawInputSubmission: false
};

export default AutosuggestSearchField;

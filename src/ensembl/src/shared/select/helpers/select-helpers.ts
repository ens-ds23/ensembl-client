import cloneDeep from 'lodash/cloneDeep';
import isEqual from 'lodash/isEqual';

import { Option, OptionGroup, GroupedOptionIndex } from '../Select';

export const findSelectedIndexForOptionGroups = (
  optionGroups: OptionGroup[]
) => {
  for (let groupIndex = 0; groupIndex < optionGroups.length; groupIndex++) {
    const group = optionGroups[groupIndex];
    for (
      let optionIndex = 0;
      optionIndex < group.options.length;
      optionIndex++
    ) {
      const option = group.options[optionIndex];
      if (option.isSelected) {
        return [groupIndex, optionIndex];
      }
    }
  }

  return -1;
};

export const splitFromSelected = (
  optionGroups: OptionGroup[]
): [Option, OptionGroup[]] | [null, OptionGroup[]] => {
  const position = findSelectedIndexForOptionGroups(optionGroups);

  if (!Array.isArray(position)) {
    // no option selected; return unmodified option groups
    return [null, optionGroups];
  }

  const [groupIndex, optionIndex] = position;
  const selectedOption = optionGroups[groupIndex].options[optionIndex];
  const withoutSelected = cloneDeep(optionGroups);
  withoutSelected[groupIndex].options.splice(optionIndex, 1);
  return [selectedOption, withoutSelected];
};

export const getNextItemIndex = (
  currentIndex: GroupedOptionIndex | null,
  optionGroups: OptionGroup[]
): GroupedOptionIndex | null => {
  const [groupIndex, itemIndex] = currentIndex || [null, null];
  const currentGroup =
    typeof groupIndex === 'number' ? optionGroups[groupIndex] : null;
  const firstItemIndex: GroupedOptionIndex = [0, 0];

  if (currentIndex === null) {
    return firstItemIndex;
  } else if (
    currentGroup &&
    typeof itemIndex === 'number' &&
    itemIndex < currentGroup.options.length - 1
  ) {
    // move to the next item in the group
    return [groupIndex, itemIndex + 1] as GroupedOptionIndex;
  } else if (groupIndex === optionGroups.length - 1) {
    // this is the last item in the last group;
    // cycle back to the first item in the list
    return firstItemIndex;
  } else if (typeof groupIndex === 'number') {
    // move to the next group in the list
    return [groupIndex + 1, 0];
  } else {
    return null; // should never happen, but makes Typescript happy
  }
};

export const getPreviousItemIndex = (
  currentIndex: GroupedOptionIndex | null,
  optionGroups: OptionGroup[]
): GroupedOptionIndex | null => {
  const [groupIndex, itemIndex] = currentIndex || [null, null];
  const lastGroupIndex = optionGroups.length - 1;
  const lastGroupItemIndex = optionGroups[lastGroupIndex].options.length - 1;
  const lastItemIndex: GroupedOptionIndex = [
    lastGroupIndex,
    lastGroupItemIndex
  ];

  if (currentIndex === null) {
    return lastItemIndex;
  } else if (typeof itemIndex === 'number' && itemIndex > 0) {
    // move to the previous item
    return [groupIndex, itemIndex - 1] as GroupedOptionIndex;
  } else if (groupIndex === 0) {
    // this is the first item in the first group;
    // cycle back to the last item in the last group
    return lastItemIndex;
  } else if (typeof groupIndex === 'number') {
    // move to the last item in the previous group
    const previousGroupIndex = groupIndex - 1;
    const lastItemIndex = optionGroups[previousGroupIndex].options.length - 1;
    return [previousGroupIndex, lastItemIndex];
  } else {
    return null; // should never happen, but makes Typescript happy
  }
};

// check whether the options panel fits the allotted portion of the window,
// and if it does not, assign a height to it
export const setOptionsPanelHeight = (
  elementRef: React.MutableRefObject<HTMLDivElement | null>
) => {
  const panel = elementRef.current;
  if (!panel) {
    return;
  }

  const windowHeight = window.innerHeight;
  const { top: panelTop, height: panelHeight } = panel.getBoundingClientRect();

  if (panelTop + panelHeight > windowHeight) {
    const minHeight = 300;
    const bottomOffset = 10;
    const distanceToWindowEdge = windowHeight - panelTop - bottomOffset;
    const panelHeight = Math.max(distanceToWindowEdge, minHeight);
    panel.style.height = `${panelHeight}px`;
  }
};

export const getPanelScrollStatus = (panel: HTMLDivElement) => {
  return {
    isScrolledToTop: panel.scrollTop === 0,
    isScrolledToBottom:
      panel.scrollTop === panel.scrollHeight - panel.clientHeight
  };
};

export const scrollDown = (
  panel: HTMLDivElement,
  ref: React.MutableRefObject<number>
) => {
  const scroll = () => {
    panel.scrollTop = panel.scrollTop + 10;
    if (panel.scrollTop + panel.clientHeight < panel.scrollHeight) {
      ref.current = window.requestAnimationFrame(scroll);
    }
  };
  window.requestAnimationFrame(scroll);
};

export const scrollUp = (
  panel: HTMLDivElement,
  ref: React.MutableRefObject<number>
) => {
  const scroll = () => {
    panel.scrollTop = panel.scrollTop - 10;
    if (panel.scrollTop > 0) {
      ref.current = window.requestAnimationFrame(scroll);
    }
  };
  window.requestAnimationFrame(scroll);
};

type ScrollOptionIntoViewArgs = {
  container: HTMLDivElement;
  currentIndex: GroupedOptionIndex;
  optionGroups: OptionGroup[];
  selector: string; // CSS selector for option elements
};
export const scrollOptionIntoView = ({
  container,
  currentIndex,
  optionGroups,
  selector
}: ScrollOptionIntoViewArgs) => {
  const lastGroupIndex = optionGroups.length - 1;
  const lastItemIndex = [
    lastGroupIndex,
    optionGroups[lastGroupIndex].options.length - 1
  ];
  const currentOptionElement = container.querySelector(selector);
  if (!currentOptionElement) {
    return;
  }

  const containerRect = container.getBoundingClientRect();
  const currentOptionElementRect = currentOptionElement.getBoundingClientRect();

  if (isEqual(currentIndex, [0, 0]) && container.scrollTop !== 0) {
    container.scrollTop = 0;
  } else if (isEqual(currentIndex, lastItemIndex)) {
    container.scrollTop = container.scrollHeight - container.clientHeight;
  } else if (
    currentOptionElementRect.bottom + currentOptionElementRect.height >
    containerRect.bottom
  ) {
    container.scrollTop = container.scrollTop + currentOptionElementRect.height;
  } else if (
    currentOptionElementRect.top <
    containerRect.top + currentOptionElementRect.height
  ) {
    container.scrollTop = container.scrollTop - currentOptionElementRect.height;
  }
};

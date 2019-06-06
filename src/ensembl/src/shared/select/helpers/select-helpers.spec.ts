import faker from 'faker';
import random from 'lodash/random';
import times from 'lodash/times';

import { Option, OptionGroup } from '../Select';

import {
  findSelectedIndexForOptionGroups,
  splitFromSelected
} from './select-helpers';

const buildOption = (): Option => ({
  value: faker.random.number(),
  label: faker.lorem.words(),
  isSelected: false
});

const buildOptionGroup = (options: Option[]): OptionGroup => ({
  options
});

describe('findSelectedIndexForOptionGroups', () => {
  let numberOfOptionGroups: number;
  let numberOfOptionsPerGroup: number;
  let optionGroups: OptionGroup[];
  let selectedOptionIndex: [number, number];

  beforeEach(() => {
    numberOfOptionGroups = 5;
    numberOfOptionsPerGroup = 10;

    optionGroups = times(numberOfOptionGroups, () => {
      const options = times(numberOfOptionsPerGroup, () => buildOption());
      return buildOptionGroup(options);
    });

    selectedOptionIndex = [
      random(numberOfOptionGroups - 1),
      random(numberOfOptionsPerGroup - 1)
    ];
  });

  test('finds index of selected option in an array of option groups', () => {
    selectedOptionIndex = [4, 0]; // TODO: find out why this fails
    const [groupIndex, optionIndex] = selectedOptionIndex;

    optionGroups[groupIndex].options[optionIndex].isSelected = true;

    expect(findSelectedIndexForOptionGroups(optionGroups)).toEqual(
      selectedOptionIndex
    );
  });

  test('returns -1 if none of the options is selected', () => {
    expect(findSelectedIndexForOptionGroups(optionGroups)).toEqual(-1);
  });
});

describe('splitFromSelected', () => {
  let numberOfOptionGroups: number;
  let numberOfOptionsPerGroup: number;
  let optionGroups: OptionGroup[];
  let selectedOptionIndex: [number, number];

  beforeEach(() => {
    numberOfOptionGroups = 5;
    numberOfOptionsPerGroup = 10;

    optionGroups = times(numberOfOptionGroups, () => {
      const options = times(numberOfOptionsPerGroup, () => buildOption());
      return buildOptionGroup(options);
    });

    selectedOptionIndex = [
      random(numberOfOptionGroups - 1),
      random(numberOfOptionsPerGroup - 1)
    ];
  });

  test('finds index of selected option in an array of option groups', () => {
    const [groupIndex, optionIndex] = selectedOptionIndex;
    optionGroups[groupIndex].options[optionIndex].isSelected = true;

    const [selectedOption, withoutSelected] = splitFromSelected(optionGroups);

    expect(selectedOption).toEqual(
      optionGroups[groupIndex].options[optionIndex]
    );
    expect(withoutSelected[groupIndex].options.length).toBe(
      numberOfOptionsPerGroup - 1
    );
  });

  test('returns unmodified option groups if no option is selected', () => {
    // TODO: write test
  });
});

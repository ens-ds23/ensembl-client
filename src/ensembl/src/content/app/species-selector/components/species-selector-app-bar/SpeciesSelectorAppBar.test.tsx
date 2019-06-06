import React from 'react';
import { mount } from 'enzyme';
import times from 'lodash/times';

import { createSelectedSpecies } from 'tests/fixtures/selected-species';

import {
  SpeciesSelectorAppBar,
  PlaceholderMessage
} from './SpeciesSelectorAppBar';
import SelectedSpecies from 'src/content/app/species-selector/components/selected-species/SelectedSpecies';

const toggleSpeciesUse = jest.fn();
const onSpeciesDelete = jest.fn();

const defaultProps = {
  selectedSpecies: times(5, () => createSelectedSpecies()),
  toggleSpeciesUse,
  onSpeciesDelete
};

describe('<SpeciesSelectorAppBar />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('shows placeholder message if no species are selected', () => {
    const props = { ...defaultProps, selectedSpecies: [] };
    const wrapper = mount(<SpeciesSelectorAppBar {...props} />);
    expect(wrapper.find(PlaceholderMessage).length).toBe(1);
  });

  it('does not show placeholder message if there are selected species', () => {
    const wrapper = mount(<SpeciesSelectorAppBar {...defaultProps} />);
    expect(wrapper.find(PlaceholderMessage).length).toBe(0);
  });

  it('renders the list of selected species if there are some', () => {
    const wrapper = mount(<SpeciesSelectorAppBar {...defaultProps} />);
    expect(wrapper.find(SelectedSpecies).length).toBe(
      defaultProps.selectedSpecies.length
    );
  });
});

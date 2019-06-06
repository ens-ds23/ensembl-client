import React from 'react';
import { mount } from 'enzyme';
import times from 'lodash/times';
import random from 'lodash/random';

import SpeciesTabBar from 'src/shared/species-tab-bar/SpeciesTabBar';
import SpeciesTab from 'src/shared/species-tab/SpeciesTab';

import { createSelectedSpecies } from 'tests/fixtures/selected-species';

const speciesList = times(5, () => createSelectedSpecies());
const onTabSelect = jest.fn();

const defaultProps = {
  species: speciesList,
  activeGenomeId: speciesList[0].genome_id,
  onTabSelect
};

describe('SpeciesTabBar', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('renders species tabs', () => {
    const wrapper = mount(<SpeciesTabBar {...defaultProps} />);
    const tabs = wrapper.find(SpeciesTab);
    expect(tabs.length).toBe(defaultProps.species.length);
  });

  it('indicates which of the tabs is active', () => {
    const activeTabIndex = random(0, defaultProps.species.length - 1);
    const activeTabData = defaultProps.species[activeTabIndex];
    const activeGenomeId = activeTabData.genome_id;
    const props = { ...defaultProps, activeGenomeId };
    const wrapper = mount(<SpeciesTabBar {...props} />);
    const activeTab = wrapper
      .find(SpeciesTab)
      .findWhere((tabWrapper) => tabWrapper.prop('isActive') === true);

    expect(activeTab.length).toBe(1);
    expect(activeTab.prop('species').genome_id).toBe(activeGenomeId);
  });

  it('calls onTabSelect when a non-active tab is clicked', () => {
    // tests integration with the SpeciesTab component
    // NOTE: as set in defaultProps, active tab is the first one (i.e. with an index of 0)
    const newTabIndex = random(1, defaultProps.species.length - 1);
    const newTabId = defaultProps.species[newTabIndex].genome_id;
    const wrapper = mount(<SpeciesTabBar {...defaultProps} />);

    const newTabWrapper = wrapper.find(SpeciesTab).at(newTabIndex);
    newTabWrapper.simulate('click');

    expect(onTabSelect).toHaveBeenCalledWith(newTabId);
  });
});

import React from 'react';
import { render, mount } from 'enzyme';
import faker from 'faker';
import set from 'lodash/fp/set';
import sample from 'lodash/sample';

import { createSelectedSpecies } from 'tests/fixtures/selected-species';

import SelectedSpecies from './SelectedSpecies';

const onToggleUse = jest.fn();
const onRemove = jest.fn();

const defaultProps = {
  species: createSelectedSpecies(),
  onToggleUse,
  onRemove
};

describe('<SelectedSpecies />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('displays species’ common name (if present) and assembly name', () => {
    const commonName = faker.lorem.words();
    const props = set('species.common_name', commonName, defaultProps);
    const wrapper = render(<SelectedSpecies {...props} />);
    const nameElement = wrapper.find(`.name`);
    const assemblyElement = wrapper.find(`.assembly`);

    expect(nameElement.text()).toMatch(commonName);
    expect(assemblyElement.text()).toMatch(props.species.assembly_name);
  });

  it('displays species’ scientific name (if species does not have a common name), and assembly name', () => {
    const wrapper = render(<SelectedSpecies {...defaultProps} />);
    const nameElement = wrapper.find(`.name`);
    const assemblyElement = wrapper.find(`.assembly`);

    expect(nameElement.text()).toMatch(defaultProps.species.scientific_name);
    expect(assemblyElement.text()).toMatch(defaultProps.species.assembly_name);
  });

  it('shows overlay with "Do not use" message if moused over enabled species', () => {
    const wrapper = mount(<SelectedSpecies {...defaultProps} />);
    wrapper.simulate('mouseover');
    wrapper.update();

    const overlayElement = wrapper.find('.selectedSpeciesOverlay');

    expect(overlayElement.length).toBe(1);
    expect(overlayElement.text()).toBe('Do not use');
  });

  it('shows overlay with "Use" message if moused over disabled species', () => {
    const props = set('species.isEnabled', false, defaultProps);
    const wrapper = mount(<SelectedSpecies {...props} />);
    wrapper.simulate('mouseover');
    wrapper.update();

    const overlayElement = wrapper.find('.selectedSpeciesOverlay');

    expect(overlayElement.length).toBe(1);
    expect(overlayElement.text()).toBe('Use');
  });

  it('calls onToggleUse when the "Do not use" or "Use" message is clicked', () => {
    const isEnabled = sample([true, false]);
    const props = set('species.isEnabled', isEnabled, defaultProps);
    const wrapper = mount(<SelectedSpecies {...props} />);
    wrapper.simulate('mouseover');
    wrapper.update();

    const overlayText = wrapper.find('.overlayText');
    overlayText.simulate('click');

    expect(onToggleUse).toHaveBeenCalledWith(props.species.genome_id);
  });

  it('calls onRemove when the delete icon is clicked', () => {
    const isEnabled = sample([true, false]);
    const props = set('species.isEnabled', isEnabled, defaultProps);
    const wrapper = mount(<SelectedSpecies {...props} />);
    wrapper.simulate('mouseover');
    wrapper.update();

    const closeButtonContainer = wrapper.find('.closeButtonContainer');
    closeButtonContainer.simulate('click');

    expect(onRemove).toHaveBeenCalledWith(props.species.genome_id);
  });
});

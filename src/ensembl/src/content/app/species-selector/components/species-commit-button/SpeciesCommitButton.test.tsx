import React from 'react';
import { mount } from 'enzyme';

import { SpeciesCommitButton } from './SpeciesCommitButton';

import { PrimaryButton } from 'src/shared/button/Button';

const onCommit = jest.fn();
const defaultProps = {
  hasCurrentSpecies: true,
  disabled: false,
  onCommit
};

describe('<SpeciesCommitButton />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('shows PrimaryButton if a species has been selected', () => {
    const wrapper = mount(<SpeciesCommitButton {...defaultProps} />);

    expect(wrapper.find(PrimaryButton).length).toBe(1);
  });

  it('does not show any button if no species has been selected', () => {
    const props = {
      ...defaultProps,
      hasCurrentSpecies: false
    };
    const wrapper = mount(<SpeciesCommitButton {...props} />);
    expect(wrapper.find(PrimaryButton).length).toBe(0);
  });

  it('passes disabled prop to the button', () => {
    const enabledProps = defaultProps;
    const disabledProps = { ...defaultProps, disabled: true };
    const enabledButton = mount(<SpeciesCommitButton {...enabledProps} />).find(
      PrimaryButton
    );
    const disabledButton = mount(
      <SpeciesCommitButton {...disabledProps} />
    ).find(PrimaryButton);

    expect(enabledButton.prop('isDisabled')).toBe(false);
    expect(disabledButton.prop('isDisabled')).toBe(true);
  });

  it('calls the onCommit prop if clicked when enabled', () => {
    const wrapper = mount(<SpeciesCommitButton {...defaultProps} />);
    const button = wrapper.find(PrimaryButton);

    button.simulate('click');
    expect(onCommit).toHaveBeenCalled();
  });

  it('does not call the onCommit prop if clicked when disabled', () => {
    const props = { ...defaultProps, disabled: true };
    const wrapper = mount(<SpeciesCommitButton {...props} />);
    const button = wrapper.find(PrimaryButton);

    button.simulate('click');
    expect(onCommit).not.toHaveBeenCalled();
  });
});

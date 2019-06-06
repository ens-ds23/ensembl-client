import React from 'react';
import { mount } from 'enzyme';
import Checkbox from './Checkbox';
import faker from 'faker';

const onChange = jest.fn();

describe('<Checkbox />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('renders without error', () => {
    expect(() => {
      mount(<Checkbox checked={false} onChange={onChange} />);
    }).not.toThrow();
  });

  it('applies the style "defaultCheckbox" by default', () => {
    const wrapper = mount(<Checkbox checked={false} onChange={onChange} />);
    expect(wrapper.find('.defaultCheckbox')).toHaveLength(1);
  });

  describe('prop label', () => {
    it('does not display any label by default', () => {
      const wrapper = mount(<Checkbox checked={false} onChange={onChange} />);
      expect(wrapper.find('.defaultLabel')).toHaveLength(0);
    });

    it('displays the label if it is passed in', () => {
      const label = faker.lorem.words();
      const wrapper = mount(
        <Checkbox checked={false} onChange={onChange} label={label} />
      );

      expect(wrapper.find('.defaultLabel').text()).toBe(label);
    });

    it('can be extended', () => {
      const label = faker.lorem.words();
      const fakeClassName = faker.lorem.word();
      const wrapper = mount(
        <Checkbox
          onChange={onChange}
          checked={false}
          label={label}
          labelClassName={fakeClassName}
        />
      );

      expect(wrapper.find('.defaultLabel').hasClass(fakeClassName)).toBe(true);
    });
  });

  describe('prop onChange', () => {
    it('calls the onChange function when clicked', () => {
      const wrapper = mount(<Checkbox checked={false} onChange={onChange} />);

      wrapper.find('.defaultCheckbox').simulate('click');
      expect(onChange).toHaveBeenCalledWith(true);
    });

    it('does not call onChange function if the status is disabled', () => {
      const wrapper = mount(
        <Checkbox checked={false} onChange={onChange} disabled={true} />
      );

      wrapper.find('.defaultCheckbox').simulate('click');

      expect(onChange).not.toHaveBeenCalled();
    });

    it('calls the onChange function if the label is clicked', () => {
      const label = faker.lorem.words();
      const wrapper = mount(
        <Checkbox checked={false} label={label} onChange={onChange} />
      );

      wrapper.find('.defaultLabel').simulate('click');

      expect(onChange).toHaveBeenCalled();
    });
  });
});

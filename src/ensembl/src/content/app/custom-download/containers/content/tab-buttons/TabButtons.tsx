import React from 'react';
import { connect } from 'react-redux';
import RoundButton, {
  RoundButtonStatus
} from 'src/shared/round-button/RoundButton';
import BadgedButton from 'src/shared/badged-button/BadgedButton';

import { getSelectedTab } from '../../../state/customDownloadSelectors';

import { getAttributes } from '../attributes-accordion/state/attributesAccordionSelector';
import { getFilters } from '../filter-accordion/state/filterAccordionSelector';

import { toggleTab } from '../../../state/customDownloadActions';
import { RootState } from 'src/store';

import { CustomDownloadAttributes } from 'src/content/app/custom-download/types/Attributes';

import styles from './TabButtons.scss';

type Props = StateProps & DispatchProps;

const getTotalSelectedAttributes = (attributes: CustomDownloadAttributes) => {
  let totalSelectedAttributes = 0;
  Object.keys(attributes).forEach((section) => {
    Object.keys(attributes[section]).forEach((subSection) => {
      Object.keys(attributes[section][subSection]).forEach((attributeId) => {
        if (attributes[section][subSection][attributeId].isChecked === true) {
          totalSelectedAttributes++;
        }
      });
    });
  });

  return totalSelectedAttributes;
};

const getTotalSelectedFilters = (filters: any) => {
  let totalSelectedFilters = 0;
  Object.values(filters).forEach((filter: any) => {
    if (typeof filter === 'string') {
      if (filter !== '') totalSelectedFilters++;
    } else if (Array.isArray(filter)) {
      if (filter.length > 0) totalSelectedFilters += filter.length;
    } else if (typeof filter === 'object') {
      Object.keys(filter).forEach((subSection) => {
        Object.keys(filter[subSection]).forEach((attributeId) => {
          if (filter[subSection][attributeId].isChecked === true) {
            totalSelectedFilters++;
          }
        });
      });
    }
  });

  return totalSelectedFilters;
};

const TabButtons = (props: Props) => {
  const dataButtonStatus =
    props.selectedTab === 'attributes'
      ? RoundButtonStatus.ACTIVE
      : RoundButtonStatus.INACTIVE;
  const filterButtonStatus =
    props.selectedTab === 'filter'
      ? RoundButtonStatus.ACTIVE
      : RoundButtonStatus.INACTIVE;
  return (
    <div className={`${styles.wrapper}`}>
      <div>
        <BadgedButton
          badgeContent={getTotalSelectedAttributes(props.attributes)}
        >
          <RoundButton
            onClick={() => {
              props.toggleTab('attributes');
            }}
            status={dataButtonStatus}
          >
            Data to download
          </RoundButton>
        </BadgedButton>
      </div>

      <div className={`${styles.buttonPadding}`}>
        <BadgedButton badgeContent={getTotalSelectedFilters(props.filters)}>
          <RoundButton
            onClick={() => {
              props.toggleTab('filter');
            }}
            status={filterButtonStatus}
          >
            Filter results
          </RoundButton>
        </BadgedButton>
      </div>
    </div>
  );
};

type DispatchProps = {
  toggleTab: (toggleTab: string) => void;
};

const mapDispatchToProps: DispatchProps = {
  toggleTab
};

type StateProps = {
  selectedTab: string;
  attributes: {};
  filters: {};
};

const mapStateToProps = (state: RootState): StateProps => ({
  selectedTab: getSelectedTab(state),
  attributes: getAttributes(state),
  filters: getFilters(state)
});

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TabButtons);

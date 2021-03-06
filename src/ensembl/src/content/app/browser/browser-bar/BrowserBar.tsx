import React, { FunctionComponent, Fragment, useState, useEffect } from 'react';
import { connect } from 'react-redux';
import classNames from 'classnames';

import { browserInfoConfig, BrowserInfoItem } from '../browserConfig';
import { TrackType } from '../track-panel/trackPanelConfig';

import { toggleBrowserNav, toggleGenomeSelector } from '../browserActions';
import { ChrLocation } from '../browserState';
import {
  getBrowserNavOpened,
  getChrLocation,
  getBrowserActivated,
  getDefaultChrLocation,
  getGenomeSelectorActive
} from '../browserSelectors';
import { getDrawerOpened } from '../drawer/drawerSelectors';
import { getEnsObjectInfo } from 'src/ens-object/ensObjectSelectors';
import {
  getSelectedBrowserTab,
  getTrackPanelModalOpened,
  getTrackPanelOpened
} from '../track-panel/trackPanelSelectors';
import { selectBrowserTab } from '../track-panel/trackPanelActions';
import { toggleDrawer } from '../drawer/drawerActions';
import { RootState } from 'src/store';

import BrowserReset from '../browser-reset/BrowserReset';
import BrowserGenomeSelector from '../browser-genome-selector/BrowserGenomeSelector';
import BrowserTabs from '../browser-tabs/BrowserTabs';

import styles from './BrowserBar.scss';

type StateProps = {
  browserActivated: boolean;
  browserNavOpened: boolean;
  chrLocation: ChrLocation;
  defaultChrLocation: ChrLocation;
  drawerOpened: boolean;
  genomeSelectorActive: boolean;
  ensObjectInfo: any;
  selectedBrowserTab: TrackType;
  trackPanelModalOpened: boolean;
  trackPanelOpened: boolean;
};

type DispatchProps = {
  selectBrowserTab: (selectedBrowserTab: TrackType) => void;
  toggleBrowserNav: () => void;
  toggleDrawer: (drawerOpened: boolean) => void;
  toggleGenomeSelector: (genomeSelectorActive: boolean) => void;
};

type OwnProps = {
  dispatchBrowserLocation: (chrLocation: ChrLocation) => void;
};

type BrowserBarProps = StateProps & DispatchProps & OwnProps;

type BrowserInfoProps = {
  ensObjectInfo: any;
};

type BrowserNavigatorButtonProps = {
  toggleNavigator: () => void;
  navigator: BrowserInfoItem;
  icon: string; // TODO: use inline SVG
};

export const BrowserBar: FunctionComponent<BrowserBarProps> = (
  props: BrowserBarProps
) => {
  const shouldShowBrowserInfo = () => {
    const [, chrStart, chrEnd] = props.defaultChrLocation;
    const isLocationOfWholeChromosome = chrStart === 0 && chrEnd === 0;

    return !(props.genomeSelectorActive || isLocationOfWholeChromosome);
  };

  const { navigator } = browserInfoConfig;
  const [showBrowserInfo, toggleShowBrowserInfo] = useState(
    shouldShowBrowserInfo()
  );

  const setBrowserInfoVisibility = () => {
    const shouldToggleVisibility = showBrowserInfo !== shouldShowBrowserInfo();
    if (shouldToggleVisibility) {
      toggleShowBrowserInfo(!showBrowserInfo);
    }
  };

  useEffect(() => {
    setBrowserInfoVisibility();
  }, [props.defaultChrLocation, props.genomeSelectorActive]);

  const getBrowserNavIcon = () => {
    if (props.drawerOpened === true) {
      return navigator.icon.grey as string;
    } else if (props.browserNavOpened === true) {
      return navigator.icon.selected as string;
    } else {
      return navigator.icon.default;
    }
  };

  const toggleNavigator = () => {
    if (props.drawerOpened === true) {
      return;
    }

    props.toggleBrowserNav();
  };

  const className = classNames(styles.browserInfo, {
    [styles.browserInfoExpanded]: !props.trackPanelOpened,
    [styles.browserInfoGreyed]: props.drawerOpened
  });

  return (
    <div className={styles.browserBar}>
      <div className={className}>
        <dl className={styles.browserInfoLeft}>
          <BrowserReset
            dispatchBrowserLocation={props.dispatchBrowserLocation}
            chrLocation={props.chrLocation}
            defaultChrLocation={props.defaultChrLocation}
            drawerOpened={props.drawerOpened}
          />
          {showBrowserInfo && (
            <BrowserInfo ensObjectInfo={props.ensObjectInfo} />
          )}
        </dl>
        <dl className={styles.browserInfoRight}>
          <BrowserGenomeSelector
            browserActivated={props.browserActivated}
            dispatchBrowserLocation={props.dispatchBrowserLocation}
            chrLocation={props.chrLocation}
            drawerOpened={props.drawerOpened}
            genomeSelectorActive={props.genomeSelectorActive}
            toggleGenomeSelector={props.toggleGenomeSelector}
          />
          {!props.genomeSelectorActive && (
            <BrowserNavigatorButton
              navigator={navigator}
              toggleNavigator={toggleNavigator}
              icon={getBrowserNavIcon()}
            />
          )}
        </dl>
      </div>
      {props.trackPanelOpened && (
        <BrowserTabs
          drawerOpened={props.drawerOpened}
          genomeSelectorActive={props.genomeSelectorActive}
          selectBrowserTab={props.selectBrowserTab}
          selectedBrowserTab={props.selectedBrowserTab}
          toggleDrawer={props.toggleDrawer}
          trackPanelModalOpened={props.trackPanelModalOpened}
        />
      )}
    </div>
  );
};

export const BrowserInfo = ({ ensObjectInfo }: BrowserInfoProps) => (
  <Fragment>
    <dd className={styles.geneSymbol}>
      <label>Gene</label>
      <span className={styles.value}>{ensObjectInfo.obj_symbol}</span>
    </dd>
    <dd>
      <label>Stable ID</label>
      <span className={styles.value}>{ensObjectInfo.stable_id}</span>
    </dd>
    <dd className="show-for-large">
      <label>Spliced mRNA length</label>
      <span className={styles.value}>{ensObjectInfo.spliced_length}</span>
      <label>bp</label>
    </dd>
    <dd className={`show-for-large ${styles.nonLabelValue}`}>
      {ensObjectInfo.bio_type}
    </dd>
    <dd className={`show-for-large ${styles.nonLabelValue}`}>
      {ensObjectInfo.strand} strand
    </dd>
  </Fragment>
);

export const BrowserNavigatorButton = (props: BrowserNavigatorButtonProps) => (
  <dd className={styles.navigator}>
    <button title={props.navigator.description} onClick={props.toggleNavigator}>
      <img src={props.icon} alt={props.navigator.description} />
    </button>
  </dd>
);

const mapStateToProps = (state: RootState): StateProps => ({
  browserActivated: getBrowserActivated(state),
  browserNavOpened: getBrowserNavOpened(state),
  chrLocation: getChrLocation(state),
  defaultChrLocation: getDefaultChrLocation(state),
  drawerOpened: getDrawerOpened(state),
  ensObjectInfo: getEnsObjectInfo(state),
  genomeSelectorActive: getGenomeSelectorActive(state),
  selectedBrowserTab: getSelectedBrowserTab(state),
  trackPanelModalOpened: getTrackPanelModalOpened(state),
  trackPanelOpened: getTrackPanelOpened(state)
});

const mapDispatchToProps: DispatchProps = {
  selectBrowserTab,
  toggleBrowserNav,
  toggleDrawer,
  toggleGenomeSelector
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(BrowserBar);

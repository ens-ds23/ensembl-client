import React, { FunctionComponent, RefObject, useEffect } from 'react';
import { connect } from 'react-redux';
import { useSpring, animated } from 'react-spring';

import TrackPanelBar from './track-panel-bar/TrackPanelBar';
import TrackPanelList from './track-panel-list/TrackPanelList';
import TrackPanelModal from './track-panel-modal/TrackPanelModal';
import Drawer from '../drawer/Drawer';
import { RootState } from 'src/store';

import {
  toggleTrackPanel,
  closeTrackPanelModal,
  openTrackPanelModal
} from './trackPanelActions';
import { toggleDrawer, changeDrawerView } from '../drawer/drawerActions';
import {
  getTrackPanelOpened,
  getTrackPanelModalOpened,
  getTrackPanelModalView,
  getSelectedBrowserTab
} from './trackPanelSelectors';
import { getDrawerView, getDrawerOpened } from '../drawer/drawerSelectors';
import {
  getBrowserActivated,
  getDefaultChrLocation,
  getBrowserActiveGenomeId
} from '../browserSelectors';
import {
  getExampleEnsObjects,
  getEnsObjectInfo,
  getEnsObjectTracks
} from 'src/ens-object/ensObjectSelectors';
import { getLaunchbarExpanded } from 'src/header/headerSelectors';
import { getBreakpointWidth } from 'src/global/globalSelectors';
import { ChrLocation } from '../browserState';
import { BreakpointWidth } from 'src/global/globalConfig';
import { TrackType, TrackStates } from './trackPanelConfig';

import { GenomeTrackCategory } from 'src/genome/genomeTypes';
import { getGenomeTrackCategories } from 'src/genome/genomeSelectors';
import {
  EnsObject,
  EnsObjectTrack,
  ExampleEnsObjectsData
} from 'src/ens-object/ensObjectTypes';

import styles from './TrackPanel.scss';

type StateProps = {
  activeGenomeId: string;
  breakpointWidth: BreakpointWidth;
  browserActivated: boolean;
  defaultChrLocation: { [genomeId: string]: ChrLocation };
  drawerOpened: boolean;
  drawerView: string;
  ensObjectInfo: EnsObject;
  ensObjectTracks: EnsObjectTrack;
  exampleEnsObjects: ExampleEnsObjectsData;
  launchbarExpanded: boolean;
  selectedBrowserTab: { [genomeId: string]: TrackType };
  genomeTrackCategories: GenomeTrackCategory[];
  trackPanelModalOpened: boolean;
  trackPanelModalView: string;
  trackPanelOpened: boolean;
};

type DispatchProps = {
  changeDrawerView: (drawerView: string) => void;
  closeTrackPanelModal: () => void;
  openTrackPanelModal: (trackPanelModalView: string) => void;
  toggleDrawer: (drawerOpened: boolean) => void;
  toggleTrackPanel: (trackPanelOpened?: boolean) => void;
};

type OwnProps = {
  browserRef: RefObject<HTMLDivElement>;
  trackStates: TrackStates;
};

type TrackPanelProps = StateProps & DispatchProps & OwnProps;

const TrackPanel: FunctionComponent<TrackPanelProps> = (
  props: TrackPanelProps
) => {
  useEffect(() => {
    if (props.breakpointWidth !== BreakpointWidth.LARGE) {
      props.toggleTrackPanel(false);
    } else {
      props.toggleTrackPanel(true);
    }
  }, [props.breakpointWidth, props.toggleTrackPanel]);

  const [trackAnimation, setTrackAnimation] = useSpring(() => ({
    config: { tension: 280, friction: 45 },
    height: '100%',
    position: 'absolute' as 'absolute',
    display: 'block',
    left: 'calc(-356px + 100vw)'
  }));

  const getBrowserWidth = (): string => {
    if (props.drawerOpened) {
      return 'calc(41px + 0vw)';
    }
    return props.trackPanelOpened
      ? 'calc(-356px + 100vw)'
      : 'calc(-36px + 100vw)';
  };

  useEffect(() => {
    setTrackAnimation({
      left: getBrowserWidth()
    });
  }, [props.drawerOpened, props.trackPanelOpened]);

  return (
    <animated.div style={trackAnimation}>
      {props.browserActivated && props.ensObjectInfo.ensembl_object_id ? (
        <div className={styles.trackPanel}>
          <TrackPanelBar
            closeTrackPanelModal={props.closeTrackPanelModal}
            drawerOpened={props.drawerOpened}
            launchbarExpanded={props.launchbarExpanded}
            openTrackPanelModal={props.openTrackPanelModal}
            toggleDrawer={props.toggleDrawer}
            toggleTrackPanel={props.toggleTrackPanel}
            trackPanelModalOpened={props.trackPanelModalOpened}
            trackPanelModalView={props.trackPanelModalView}
            trackPanelOpened={props.trackPanelOpened}
          />
          <TrackPanelList
            activeGenomeId={props.activeGenomeId}
            browserRef={props.browserRef}
            defaultChrLocation={props.defaultChrLocation}
            drawerOpened={props.drawerOpened}
            drawerView={props.drawerView}
            launchbarExpanded={props.launchbarExpanded}
            ensObjectInfo={props.ensObjectInfo}
            ensObjectTracks={props.ensObjectTracks}
            selectedBrowserTab={props.selectedBrowserTab}
            toggleDrawer={props.toggleDrawer}
            trackStates={props.trackStates}
            genomeTrackCategories={props.genomeTrackCategories}
            updateDrawerView={props.changeDrawerView}
          />

          {props.trackPanelModalOpened ? (
            <TrackPanelModal
              closeTrackPanelModal={props.closeTrackPanelModal}
              launchbarExpanded={props.launchbarExpanded}
              trackPanelModalView={props.trackPanelModalView}
            />
          ) : null}
          {props.drawerOpened && <Drawer />}
        </div>
      ) : null}
    </animated.div>
  );
};

const mapStateToProps = (state: RootState): StateProps => ({
  activeGenomeId: getBrowserActiveGenomeId(state),
  breakpointWidth: getBreakpointWidth(state),
  browserActivated: getBrowserActivated(state),
  defaultChrLocation: getDefaultChrLocation(state),
  drawerOpened: getDrawerOpened(state),
  drawerView: getDrawerView(state),
  ensObjectInfo: getEnsObjectInfo(state),
  ensObjectTracks: getEnsObjectTracks(state),
  exampleEnsObjects: getExampleEnsObjects(state),
  launchbarExpanded: getLaunchbarExpanded(state),
  selectedBrowserTab: getSelectedBrowserTab(state),
  genomeTrackCategories: getGenomeTrackCategories(state)[
    getBrowserActiveGenomeId(state)
  ],
  trackPanelModalOpened: getTrackPanelModalOpened(state),
  trackPanelModalView: getTrackPanelModalView(state),
  trackPanelOpened: getTrackPanelOpened(state)
});

const mapDispatchToProps: DispatchProps = {
  changeDrawerView,
  closeTrackPanelModal,
  openTrackPanelModal,
  toggleDrawer,
  toggleTrackPanel
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(TrackPanel);

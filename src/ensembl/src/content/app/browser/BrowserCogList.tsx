import React, {
  FunctionComponent,
  RefObject,
  useEffect,
  useCallback
} from 'react';
import { connect } from 'react-redux';

import BrowserCog from './BrowserCog';
import {
  updateCogList,
  updateCogTrackList,
  updateSelectedCog
} from './browserActions';
import { RootState } from 'src/store';
import { CogList } from './browserState';
import {
  getBrowserActivated,
  getBrowserCogList,
  getBrowserCogTrackList,
  getBrowserSelectedCog
} from './browserSelectors';

import styles from './BrowserCogList.scss';

type StateProps = {
  browserActivated: boolean;
  browserCogList: number;
  browserCogTrackList: CogList;
  selectedCog: any;
};

type DispatchProps = {
  updateCogList: (cogList: number) => void;
  updateCogTrackList: (track_y: CogList) => void;
  updateSelectedCog: (index: string) => void;
};

type OwnProps = {
  browserRef: RefObject<HTMLDivElement>;
};

type BrowserCogListProps = StateProps & DispatchProps & OwnProps;

type BpaneScrollEvent = Event & {
  detail: {
    delta_y?: number;
    track_y?: CogList;
  };
};

const BrowserCogList: FunctionComponent<BrowserCogListProps> = (
  props: BrowserCogListProps
) => {
  const { selectedCog, browserCogTrackList } = props;
  const listenBpaneScroll = useCallback((event: Event) => {
    const bpaneScrollEvent = event as BpaneScrollEvent;
    if (
      bpaneScrollEvent.detail.delta_y ||
      bpaneScrollEvent.detail.delta_y === 0
    ) {
      props.updateCogList(bpaneScrollEvent.detail.delta_y);
    }
    if (bpaneScrollEvent.detail.track_y) {
      props.updateCogTrackList(bpaneScrollEvent.detail.track_y);
    }
  }, []);

  useEffect(() => {
    const currentEl: HTMLDivElement = props.browserRef
      .current as HTMLDivElement;
    currentEl.addEventListener('bpane-scroll', listenBpaneScroll);
  }, []);

  const cogs = Object.entries(browserCogTrackList).map(([name, pos]) => {
    const posStyle = { top: pos + 'px' };

    return (
      <div key={name} className={styles.browserCogOuter} style={posStyle}>
        <BrowserCog
          cogActivated={props.selectedCog === name}
          index={name}
          updateSelectedCog={props.updateSelectedCog}
        />
      </div>
    );
  });

  const transformStyle = {
    transform: 'translate(0,' + props.browserCogList + 'px)'
  };

  return props.browserActivated ? (
    <div className={styles.browserTrackConfigOuter}>
      <div className={styles.browserCogListOuter}>
        <div className={styles.browserCogListInner} style={transformStyle}>
          {cogs}
        </div>
      </div>
    </div>
  ) : null;
};

const mapStateToProps = (state: RootState): StateProps => ({
  browserActivated: getBrowserActivated(state),
  browserCogList: getBrowserCogList(state),
  browserCogTrackList: getBrowserCogTrackList(state),
  selectedCog: getBrowserSelectedCog(state)
});

const mapDispatchToProps: DispatchProps = {
  updateCogList,
  updateCogTrackList,
  updateSelectedCog
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(BrowserCogList);

import React, { FunctionComponent, useCallback } from 'react';
import { connect } from 'react-redux';

import {
  updateTrackConfigNames,
  updateTrackConfigLabel,
  updateApplyToAll
} from '../browserActions';

import {
  getTrackConfigNames,
  getTrackConfigLabel,
  getApplyToAll,
  getBrowserCogTrackList,
  getBrowserSelectedCog
} from '../browserSelectors';

import styles from './BrowserTrackConfig.scss';

import tracksSliderOnIcon from 'static/img/browser/icon_tracks_slider_on.svg';
import tracksSliderOffIcon from 'static/img/browser/icon_tracks_slider_off.svg';
import trackHeightBtn from 'static/img/browser/icon_tracks_height_grey.svg';
import trackLockBtn from 'static/img/browser/icon_tracks_lock_open_grey.svg';
import trackHighlightBtn from 'static/img/browser/icon_tracks_highlight_grey.svg';
import trackMoveBtn from 'static/img/browser/icon_tracks_move_grey.svg';
import { RootState } from 'src/store';
import { CogList } from '../browserState';

type StateProps = {
  applyToAll: boolean;
  browserCogTrackList: CogList;
  selectedCog: any;
  trackConfigLabel: any;
  trackConfigNames: any;
};

type DispatchProps = {
  updateApplyToAll: (yn: boolean) => void;
  updateTrackConfigLabel: (selectedCog: any, sense: boolean) => void;
  updateTrackConfigNames: (selectedCog: any, sense: boolean) => void;
};

type BrowserTrackConfigProps = StateProps & DispatchProps;

const BrowserTrackConfig: FunctionComponent<BrowserTrackConfigProps> = (
  props: BrowserTrackConfigProps
) => {
  const {
    applyToAll,
    browserCogTrackList,
    selectedCog,
    trackConfigNames,
    trackConfigLabel
  } = props;

  const trackOurConfigName = trackConfigNames[selectedCog];
  const trackOurConfigLabel = trackConfigLabel[selectedCog];

  const nameIcon = trackOurConfigName
    ? tracksSliderOnIcon
    : tracksSliderOffIcon;
  const labelIcon =
    trackOurConfigLabel !== false ? tracksSliderOnIcon : tracksSliderOffIcon;

  const toggleName = useCallback(() => {
    if (applyToAll) {
      Object.keys(browserCogTrackList).map((name) => {
        props.updateTrackConfigNames(name, !trackOurConfigName);
      });
    } else {
      props.updateTrackConfigNames(selectedCog, !trackOurConfigName);
    }
  }, [
    selectedCog,
    props.updateTrackConfigNames,
    trackOurConfigName,
    applyToAll,
    browserCogTrackList
  ]);

  const toggleLabel = useCallback(() => {
    if (applyToAll) {
      Object.keys(browserCogTrackList).map((name) => {
        props.updateTrackConfigLabel(name, !trackOurConfigLabel);
      });
    } else {
      props.updateTrackConfigLabel(selectedCog, !trackOurConfigLabel);
    }
  }, [
    selectedCog,
    updateTrackConfigLabel,
    trackOurConfigLabel,
    applyToAll,
    browserCogTrackList
  ]);

  const applyToAllToggle = useCallback(() => {
    props.updateApplyToAll(!applyToAll);
  }, [applyToAll, updateApplyToAll]);

  return (
    <section className={styles.trackConfig}>
      <dl className="category">
        <dd className={styles.allTracks}>
          <input
            type="checkbox"
            defaultChecked={applyToAll}
            onChange={applyToAllToggle}
          />
          <label htmlFor="">All tracks</label>
        </dd>
      </dl>
      <dl className="category">
        <dd className="trackName">
          <label htmlFor="">Track name</label>
          <button className={styles.trackConfigSlider} onClick={toggleName}>
            <img src={nameIcon} />
          </button>
        </dd>
        <dd className="featureLabels">
          <label htmlFor="">Feature labels</label>
          <button className={styles.trackConfigSlider} onClick={toggleLabel}>
            <img src={labelIcon} />
          </button>
        </dd>
        <dd className={styles.heightSwitcher}>
          <button className={styles.trackHeightBtn}>
            <img src={trackHeightBtn} />
          </button>
        </dd>
      </dl>
      <dl className="category">
        <dd className={styles.trackLock}>
          <button className={styles.trackLockBtn}>
            <img src={trackLockBtn} />
          </button>
        </dd>
        <dd className="trackHighlight disabled">
          <button className={styles.trackHighlightBtn}>
            <img src={trackHighlightBtn} />
          </button>
        </dd>
        <dd className="trackMove disabled">
          <button className={styles.trackMoveBtn}>
            <img src={trackMoveBtn} />
          </button>
        </dd>
      </dl>
    </section>
  );
};

const mapStateToProps = (state: RootState): StateProps => ({
  applyToAll: getApplyToAll(state),
  browserCogTrackList: getBrowserCogTrackList(state),
  selectedCog: getBrowserSelectedCog(state),
  trackConfigLabel: getTrackConfigLabel(state),
  trackConfigNames: getTrackConfigNames(state)
});

const mapDispatchToProps: DispatchProps = {
  updateApplyToAll,
  updateTrackConfigLabel,
  updateTrackConfigNames
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(BrowserTrackConfig);

import React, {
  FunctionComponent,
  MouseEvent,
  ReactNode,
  RefObject,
  useState,
  useCallback,
  useEffect
} from 'react';
import get from 'lodash/get';

import { TrackItemColour } from '../trackPanelConfig';

import chevronDownIcon from 'static/img/shared/chevron-down.svg';
import chevronUpIcon from 'static/img/shared/chevron-up.svg';
import { ReactComponent as Eye } from 'static/img/track-panel/eye.svg';
import { ReactComponent as Ellipsis } from 'static/img/track-panel/ellipsis.svg';

import ImageButton, {
  ImageButtonStatus
} from 'src/shared/image-button/ImageButton';
import browserStorageService from '../../browser-storage-service';
import { EnsObjectTrack } from 'src/ens-object/ensObjectTypes';

import styles from './TrackPanelListItem.scss';

type TrackPanelListItemProps = {
  activeGenomeId: string;
  browserRef: RefObject<HTMLDivElement>;
  categoryName: string;
  children?: ReactNode[];
  defaultTrackStatus: ImageButtonStatus;
  drawerOpened: boolean;
  drawerView: string;
  track: EnsObjectTrack;
  updateDrawerView: (drawerView: string) => void;
};

// delete this when there is a better place to put this
const trackPrefix = 'track:';

const TrackPanelListItem: FunctionComponent<TrackPanelListItemProps> = (
  props: TrackPanelListItemProps
) => {
  const [expanded, setExpanded] = useState(true);
  const [trackStatus, setTrackStatus] = useState(props.defaultTrackStatus);
  const { activeGenomeId, browserRef, categoryName, drawerView, track } = props;

  // FIXME: rather reading trackstates from localStorage (multiple times!), they should be passed as props
  // (and stored in redux store; localStorage should be used to store the relevant part of redux store between browser reloads)
  useEffect(() => {
    const trackStates = browserStorageService.getTrackStates();
    const storedTrackStatus: ImageButtonStatus = get(
      trackStates,
      `${activeGenomeId}.${categoryName}.${track.track_id}`,
      ImageButtonStatus.ACTIVE
    ) as ImageButtonStatus;

    if (storedTrackStatus && storedTrackStatus !== trackStatus) {
      setTrackStatus(storedTrackStatus);
    }
  }, [props.activeGenomeId]);

  useEffect(() => {
    const trackToggleStates = browserStorageService.getTrackListToggleStates();

    if (
      track.child_tracks &&
      trackToggleStates[activeGenomeId] &&
      trackToggleStates[activeGenomeId][track.track_id] !== undefined
    ) {
      setExpanded(trackToggleStates[activeGenomeId][track.track_id]);
    }
  }, []);

  const getListItemClasses = useCallback((): string => {
    let classNames: string = styles.listItem;

    if (track.track_id === 'gene') {
      classNames += ` ${styles.main}`;
    }

    if (drawerView === track.track_id) {
      classNames += ` ${styles.currentDrawerView}`;
    }

    return classNames;
  }, [drawerView]);

  const getBoxClasses = (colour: any) => {
    let classNames = styles.box;

    if (colour) {
      const colourValue = TrackItemColour[colour];
      classNames += ` ${styles[colourValue]}`;
    }

    return classNames;
  };

  const drawerViewListHandler = (event: MouseEvent) => {
    event.preventDefault();

    if (props.drawerOpened === false) {
      return;
    }

    const viewName = track.track_id;

    props.updateDrawerView(viewName);
  };

  const drawerViewButtonHandler = () => {
    const viewName = track.track_id;

    props.updateDrawerView(viewName);
  };

  const toggleExpand = () => {
    setExpanded(!expanded);

    browserStorageService.updateTrackListToggleStates({
      [activeGenomeId]: { [track.track_id]: !expanded }
    });
  };

  const toggleTrack = () => {
    const currentTrackStatus =
      trackStatus === ImageButtonStatus.ACTIVE ? 'off' : 'on';

    const trackEvent = new CustomEvent('bpane', {
      bubbles: true,
      detail: {
        [currentTrackStatus]: `${trackPrefix}${track.track_id}`
      }
    });

    if (browserRef.current) {
      browserRef.current.dispatchEvent(trackEvent);
    }

    const newImageButtonStatus =
      trackStatus === ImageButtonStatus.ACTIVE
        ? ImageButtonStatus.INACTIVE
        : ImageButtonStatus.ACTIVE;

    browserStorageService.saveTrackStates(
      activeGenomeId,
      categoryName,
      track.track_id,
      newImageButtonStatus
    );
    setTrackStatus(newImageButtonStatus);
  };

  return (
    <>
      <dd className={getListItemClasses()} onClick={drawerViewListHandler}>
        <label>
          {track.colour && <span className={getBoxClasses(track.colour)} />}
          <span className={styles.mainText}>{track.label}</span>
          {track.support_level && (
            <span className={styles.selectedInfo}>{track.support_level}</span>
          )}
          {track.additional_info && (
            <span className={styles.additionalInfo}>
              {track.additional_info}
            </span>
          )}
          {track.child_tracks && (
            <button onClick={toggleExpand} className={styles.expandBtn}>
              <img
                src={expanded ? chevronUpIcon : chevronDownIcon}
                alt={expanded ? 'collapse' : 'expand'}
              />
            </button>
          )}
        </label>
        <div className={styles.ellipsisHolder}>
          <ImageButton
            buttonStatus={ImageButtonStatus.ACTIVE}
            description={`Go to ${track.label}`}
            onClick={drawerViewButtonHandler}
            image={Ellipsis}
          />
        </div>
        <div className={styles.eyeHolder}>
          <ImageButton
            buttonStatus={trackStatus}
            description={'enable/disable track'}
            onClick={toggleTrack}
            image={Eye}
          />
        </div>
      </dd>
      {expanded && props.children}
    </>
  );
};

export default TrackPanelListItem;

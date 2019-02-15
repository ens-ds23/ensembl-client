import React, { FunctionComponent } from 'react';
import { TrackPanelItem, trackPanelIconConfig } from '../trackPanelConfig';

import chevronDownIcon from 'static/img/shared/chevron-down.svg';
import chevronUpIcon from 'static/img/shared/chevron-up.svg';

import styles from './TrackPanelListItem.scss';

type TrackPanelListItemProps = {
  className: string;
  track: TrackPanelItem;
  changeTrack: (name: string) => void;
  additionalInfo?: string;
};

const TrackPanelListItem: FunctionComponent<TrackPanelListItemProps> = (
  props: TrackPanelListItemProps
) => {
  const expanded: boolean = false;
  const { className, track, additionalInfo } = props;
  const listItemClass = styles[className] || '';

  const changeTrackHandler = () => {
    props.changeTrack(props.track.name);
  };

  return (
    <dd className={`${styles.listItem} ${listItemClass}`}>
      <label>
        {track.color && (
          <span className={`${styles.box} ${styles[track.color]}`} />
        )}
        <span className={styles.mainText}>{track.label}</span>
        {additionalInfo && (
          <span className={styles.additionalInfo}>{additionalInfo}</span>
        )}
        <button>
          <img
            className={styles.expandIcon}
            src={expanded ? chevronUpIcon : chevronDownIcon}
            alt={expanded ? 'collapse' : 'expand'}
          />
        </button>
      </label>
      <button onClick={changeTrackHandler}>
        <img
          src={trackPanelIconConfig.ellipsis.icon.on}
          alt={`Go to ${track.label}`}
        />
      </button>
      <button>
        <img
          src={trackPanelIconConfig.eye.icon.on}
          alt={trackPanelIconConfig.ellipsis.description}
        />
      </button>
    </dd>
  );
};

export default TrackPanelListItem;

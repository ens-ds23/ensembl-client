import React, {
  FunctionComponent,
  RefObject,
  useCallback,
  useState,
  useEffect
} from 'react';

import TrackPanelListItem from './TrackPanelListItem';

import {
  TrackPanelCategory,
  TrackPanelItem,
  TrackType
} from '../trackPanelConfig';
import { ChrLocation } from '../../browserState';

import styles from './TrackPanelList.scss';

type TrackPanelListProps = {
  browserRef: RefObject<HTMLDivElement>;
  defaultChrLocation: ChrLocation;
  drawerOpened: boolean;
  drawerView: string;
  launchbarExpanded: boolean;
  ensObjectInfo: any;
  selectedBrowserTab: TrackType;
  toggleDrawer: (drawerOpened: boolean) => void;
  trackCategories: [];
  updateDrawerView: (drawerView: string) => void;
};

const TrackPanelList: FunctionComponent<TrackPanelListProps> = (
  props: TrackPanelListProps
) => {
  const [currentTrackCategories, setCurrentTrackCategories] = useState([]);

  useEffect(() => {
    if (props.trackCategories.length > 0) {
      setCurrentTrackCategories(
        props.trackCategories.filter(
          (category: TrackPanelCategory) =>
            category.types.indexOf(props.selectedBrowserTab) > -1
        )
      );
    }
  }, [props.selectedBrowserTab]);

  const changeDrawerView = useCallback(
    (currentTrack: string) => {
      const { drawerView, toggleDrawer, updateDrawerView } = props;

      updateDrawerView(currentTrack);

      if (!drawerView) {
        toggleDrawer(true);
      }
    },
    [props.drawerView]
  );

  const getTrackPanelListClasses = () => {
    const heightClass: string = props.launchbarExpanded
      ? styles.shorter
      : styles.taller;

    return `${styles.trackPanelList} ${heightClass}`;
  };

  const getMainTracks = (): TrackPanelItem | null => {
    const { defaultChrLocation, ensObjectInfo } = props;
    const [, chrStart, chrEnd] = defaultChrLocation;

    if (chrStart === 0 && chrEnd === 0) {
      return null;
    }

    let geneLabel = ensObjectInfo.obj_symbol;
    let transcriptLabel = ensObjectInfo.associated_object.stable_id;

    if (ensObjectInfo.obj_type === 'transcript') {
      geneLabel = ensObjectInfo.associated_object.obj_symbol;
      transcriptLabel = ensObjectInfo.stable_id;
    }

    return {
      additionalInfo: ensObjectInfo.bio_type,
      childTrackList: [
        {
          additionalInfo: ensObjectInfo.bio_type,
          color: 'BLUE',
          drawerView: 'transcript',
          id: 0.1,
          label: transcriptLabel,
          name: 'gene-feat',
          selectedInfo: ensObjectInfo.associated_object.selected_info
        }
      ],
      drawerView: 'gene',
      id: 0,
      label: geneLabel,
      name: 'gene-feat'
    };
  };

  const getTrackListItem = (track: TrackPanelItem | null) => {
    if (!track) {
      return;
    }

    return (
      <TrackPanelListItem
        browserRef={props.browserRef}
        drawerOpened={props.drawerOpened}
        drawerView={props.drawerView}
        key={track.id}
        track={track}
        updateDrawerView={changeDrawerView}
      >
        {track.childTrackList &&
          track.childTrackList.map((childTrack: TrackPanelItem) =>
            getTrackListItem(childTrack)
          )}
      </TrackPanelListItem>
    );
  };

  return (
    <div className={getTrackPanelListClasses()}>
      <section>
        <dl>{getTrackListItem(getMainTracks())}</dl>
      </section>
      {currentTrackCategories.map((category: TrackPanelCategory) => (
        <section key={category.name}>
          <h4>{category.name}</h4>
          <dl>
            {category.trackList.length ? (
              category.trackList.map((track: TrackPanelItem) =>
                getTrackListItem(track)
              )
            ) : (
              <dd className={styles.emptyListMsg}>No data available</dd>
            )}
          </dl>
        </section>
      ))}
    </div>
  );
};

export default TrackPanelList;

import React, {
  FunctionComponent,
  RefObject,
  useCallback,
  useState,
  useEffect
} from 'react';

import TrackPanelListItem from './TrackPanelListItem';

import { TrackType, TrackStates } from '../trackPanelConfig';
import { ChrLocation } from '../../browserState';

import styles from './TrackPanelList.scss';
import { ImageButtonStatus } from 'src/shared/image-button/ImageButton';
import { GenomeTrackCategory } from 'src/genome/genomeTypes';
import { EnsObjectTrack, EnsObject } from 'src/ens-object/ensObjectTypes';

type TrackPanelListProps = {
  activeGenomeId: string;
  browserRef: RefObject<HTMLDivElement>;
  defaultChrLocation: { [genomeId: string]: ChrLocation };
  drawerOpened: boolean;
  drawerView: string;
  launchbarExpanded: boolean;
  ensObjectInfo: EnsObject;
  ensObjectTracks: EnsObjectTrack;
  selectedBrowserTab: { [genomeId: string]: TrackType };
  toggleDrawer: (drawerOpened: boolean) => void;
  genomeTrackCategories: GenomeTrackCategory[];
  trackStates: TrackStates;
  updateDrawerView: (drawerView: string) => void;
};

const TrackPanelList: FunctionComponent<TrackPanelListProps> = (
  props: TrackPanelListProps
) => {
  const [currentTrackCategories, setCurrentTrackCategories] = useState<
    GenomeTrackCategory[]
  >([]);

  useEffect(() => {
    const selectedBrowserTab =
      props.selectedBrowserTab[props.activeGenomeId] || TrackType.GENOMIC;

    if (props.genomeTrackCategories && props.genomeTrackCategories.length > 0) {
      setCurrentTrackCategories(
        props.genomeTrackCategories.filter((category: GenomeTrackCategory) =>
          category.types.includes(selectedBrowserTab)
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

  const getDefaultTrackStatus = (categoryName: string, trackName: string) => {
    let trackStatus = ImageButtonStatus.ACTIVE;

    if (!props.trackStates[props.activeGenomeId]) {
      return trackStatus;
    }
    const statesOfCategory =
      props.trackStates[props.activeGenomeId][categoryName];

    if (statesOfCategory && statesOfCategory[trackName]) {
      trackStatus =
        statesOfCategory[trackName] === 'active'
          ? ImageButtonStatus.ACTIVE
          : ImageButtonStatus.INACTIVE;
    }

    return trackStatus;
  };

  const getTrackListItem = (
    categoryName: string,
    track: EnsObjectTrack | null
  ) => {
    if (!track) {
      return;
    }

    return (
      <TrackPanelListItem
        activeGenomeId={props.activeGenomeId}
        browserRef={props.browserRef}
        categoryName={categoryName}
        defaultTrackStatus={getDefaultTrackStatus(categoryName, track.track_id)}
        drawerOpened={props.drawerOpened}
        drawerView={props.drawerView}
        key={track.track_id}
        track={track}
        updateDrawerView={changeDrawerView}
      >
        {track.child_tracks &&
          track.child_tracks.map((childTrack: EnsObjectTrack) =>
            getTrackListItem(categoryName, childTrack)
          )}
      </TrackPanelListItem>
    );
  };

  return (
    <div className={getTrackPanelListClasses()}>
      {props.ensObjectInfo.object_type === 'region' ? null : (
        <section>
          <dl>{getTrackListItem('main', props.ensObjectTracks)}</dl>
        </section>
      )}
      {currentTrackCategories.map((category: GenomeTrackCategory) => (
        <section key={category.track_category_id}>
          <h4>{category.label}</h4>
          <dl>
            {category.track_list.length ? (
              category.track_list.map((track: EnsObjectTrack) =>
                getTrackListItem(category.track_category_id, track)
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

import storageService, {
  StorageServiceInterface
} from 'src/services/storage-service';
import {
  TrackStates,
  TrackType,
  TrackToggleStates
} from './track-panel/trackPanelConfig';
import { ChrLocation } from './browserState';
import { ImageButtonStatus } from 'src/shared/image-button/ImageButton';

export enum StorageKeys {
  ACTIVE_GENOME_ID = 'browser.activeGenomeId',
  ACTIVE_ENS_OBJECT_ID = 'browser.activeEnsObjectId',
  CHR_LOCATION = 'browser.chrLocation',
  DEFAULT_CHR_LOCATION = 'browser.defaultChrLocation',
  TRACK_STATES = 'browser.trackStates',
  TRACK_LIST_TOGGLE_STATES = 'browser.trackListToggleStates',
  SELECTED_BROWSER_TAB = 'browser.selectedBrowserTab'
}

export class BrowserStorageService {
  private storageService: StorageServiceInterface;

  public constructor(storageService: StorageServiceInterface) {
    this.storageService = storageService;
  }

  public getActiveGenomeId(): string {
    return this.storageService.get(StorageKeys.ACTIVE_GENOME_ID);
  }

  public saveActiveGenomeId(activeGenomeId: string) {
    this.storageService.save(StorageKeys.ACTIVE_GENOME_ID, activeGenomeId);
  }

  public getActiveEnsObjectId() {
    return this.storageService.get(StorageKeys.ACTIVE_ENS_OBJECT_ID) || {};
  }

  public updateActiveEnsObjectId(activeEnsObjectId: {
    [genomeId: string]: string;
  }) {
    this.storageService.update(
      StorageKeys.ACTIVE_ENS_OBJECT_ID,
      activeEnsObjectId
    );
  }

  public getChrLocation() {
    return this.storageService.get(StorageKeys.CHR_LOCATION) || {};
  }

  public updateChrLocation(chrLocation: { [genomeId: string]: ChrLocation }) {
    this.storageService.update(StorageKeys.CHR_LOCATION, chrLocation);
  }

  public getDefaultChrLocation() {
    return this.storageService.get(StorageKeys.DEFAULT_CHR_LOCATION) || {};
  }

  public updateDefaultChrLocation(defaultChrLocation: {
    [genomeId: string]: ChrLocation;
  }) {
    this.storageService.update(
      StorageKeys.DEFAULT_CHR_LOCATION,
      defaultChrLocation
    );
  }

  public getTrackStates(): TrackStates {
    return this.storageService.get(StorageKeys.TRACK_STATES) || {};
  }

  public saveTrackStates(
    genomeId: string,
    categoryName: string,
    trackName: string,
    trackStatus: ImageButtonStatus
  ) {
    const trackStates = this.getTrackStates();

    if (!trackStates[genomeId]) {
      trackStates[genomeId] = {};
    }

    if (!trackStates[genomeId][categoryName]) {
      trackStates[genomeId][categoryName] = {};
    }

    trackStates[genomeId][categoryName][trackName] = trackStatus;

    this.storageService.save(StorageKeys.TRACK_STATES, trackStates);
  }

  public getTrackListToggleStates() {
    return this.storageService.get(StorageKeys.TRACK_LIST_TOGGLE_STATES) || {};
  }

  public updateTrackListToggleStates(toggleState: {
    [genomeId: string]: TrackToggleStates;
  }) {
    this.storageService.update(
      StorageKeys.TRACK_LIST_TOGGLE_STATES,
      toggleState
    );
  }

  public getSelectedBrowserTab() {
    return (
      this.storageService.get(StorageKeys.SELECTED_BROWSER_TAB) ||
      TrackType.GENOMIC
    );
  }

  public updateSelectedBrowserTab(selectedBrowserTabForGenome: {
    [genomeId: string]: TrackType;
  }) {
    this.storageService.update(
      StorageKeys.SELECTED_BROWSER_TAB,
      selectedBrowserTabForGenome
    );
  }
}

export default new BrowserStorageService(storageService);

import { ImageButtonStatus } from 'src/shared/image-button/ImageButton';

export enum TrackItemColour {
  BLUE = 'blue',
  DARK_GREY = 'darkGrey',
  GREY = 'grey',
  WHITE = 'white'
}

export enum TrackType {
  GENOMIC = 'Genomic',
  VARIATION = 'Variation',
  EXPRESSION = 'Expression'
}

export type TrackPanelIcon = {
  description: string;
  icon: {
    off: string;
    on: string;
  };
};

export type TrackPanelIcons = {
  [key: string]: TrackPanelIcon;
};

export type TrackStates = {
  [key: string]: {
    [key: string]: {
      [genomeId: string]: ImageButtonStatus;
    };
  };
};

export type TrackToggleStates = {
  [key: string]: boolean;
};

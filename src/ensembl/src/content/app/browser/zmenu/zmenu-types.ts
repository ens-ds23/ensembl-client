// relation of the point of interest to the central point of the canvas;
// a gentle hint by genome browser about where there is the most available space
// e.g. if the reported side of a point is 'left', zmenu will position itself
// to the right of this point
// TODO: check if this type is required
export enum Side {
  LEFT = 'left',
  RIGHT = 'right',
  MIDDLE = 'middle'
}

// coordinates of the point at which zmenu should be pointing
export type AnchorCoordinates = {
  x: number;
  y: number;
};

export enum Markup {
  STRONG = 'strong',
  EMPHASIS = 'emphasis',
  FOCUS = 'focus',
  LIGHT = 'light'
}

export enum ZmenuAction {
  CREATE = 'create_zmenu',
  DESTROY = 'destroy_zmenu',
  REPOSITION = 'update_zmenu_position',
  ENTER = 'zmenu-enter',
  LEAVE = 'zmenu-leave'
}

export type ZmenuContentItem = {
  text: string;
  markup: Markup[];
};

export type ZmenuContentBlock = ZmenuContentItem[];

export type ZmenuContentLine = ZmenuContentBlock[];

export type ZmenuContentFeature = {
  id: string;
  lines: ZmenuContentLine[];
};

// data that is sufficient to describe an instance of Zmenu
export type ZmenuData = {
  id: string;
  anchor_coordinates: AnchorCoordinates;
  content: ZmenuContentFeature[];
};

export type ZmenuCreatePayload = {
  action: ZmenuAction.CREATE;
  id: string;
  anchor_coordinates: AnchorCoordinates;
  content: ZmenuContentFeature[];
};

// Sent from Genome browser to React
export type ZmenuCreateEvent = Event & {
  detail: ZmenuCreatePayload;
};

export type ZmenuDestroyPayload = {
  id: string;
  action: ZmenuAction.DESTROY;
};

// Sent from Genome browser to React
export type ZmenuDestroyEvent = Event & {
  detail: ZmenuDestroyPayload;
};

// Sent from React to Genome browser
// (on mouseover; perhaps tap?)
export type ZmenuEnterEvent = {
  id: string;
  action: ZmenuAction.ENTER;
};

// Sent from React to Genome browser
// (on mouseleave, or on click outside)
export type ZmenuLeaveEvent = {
  id: string;
  action: ZmenuAction.LEAVE;
};

// Sent from Genome browser to React
export type ZmenuRepositionPayload = {
  id: string;
  action: ZmenuAction.REPOSITION;
  anchor_coordinates: {
    x: number;
    y: number;
  };
};

export type ZmenuRepositionEvent = Event & {
  detail: ZmenuRepositionPayload;
};

export type ZmenuIncomingEvent =
  | ZmenuCreateEvent
  | ZmenuDestroyEvent
  | ZmenuRepositionEvent;

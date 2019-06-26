import React, { useState, useEffect } from 'react';
import pickBy from 'lodash/pickBy';

import Zmenu from './Zmenu';

import mockEventEmitter from './mock-event-emitter';

import {
  ZmenuData,
  ZmenuAction,
  ZmenuIncomingEvent,
  ZmenuCreateEvent,
  ZmenuDestroyEvent,
  ZmenuRepositionEvent
} from './zmenu-types';

type Props = {
  browserRef: React.RefObject<HTMLDivElement>;
};

// when a zmenu is created, it’s assigned an id,
// and its data is saved in the state keyed by this id
// (just in case we need to show more than one zmenu at a time)
type StateZmenu = {
  [key: string]: ZmenuData;
};

const ZmenuController = (props: Props) => {
  const [zmenus, setZmenus] = useState<StateZmenu>({});

  useEffect(() => {
    const browserElement = props.browserRef.current as HTMLDivElement;
    const eventHandler = (event: Event) =>
      handleBpaneEvent(event as ZmenuIncomingEvent);

    browserElement.addEventListener('bpane-zmenu', eventHandler);

    return () =>
      browserElement.removeEventListener('bpane-zmenu', eventHandler);
  }, []);

  const handleBpaneEvent = (event: ZmenuIncomingEvent) => {
    if (event.detail.action === ZmenuAction.CREATE) {
      handleZmenuCreate(event as ZmenuCreateEvent);
    } else if (event.detail.action === ZmenuAction.DESTROY) {
      handleZmenuDestroy(event as ZmenuDestroyEvent);
    } else if (event.detail.action === ZmenuAction.REPOSITION) {
      handleZmenuReposition(event as ZmenuRepositionEvent);
    }
  };

  const handleZmenuCreate = (event: ZmenuCreateEvent) => {
    const zmenuData = event.detail;
    const newZmenu = {
      id: zmenuData.id,
      anchor_coordinates: zmenuData.anchor_coordinates,
      content: zmenuData.content
    };
    setZmenus({
      ...zmenus,
      [zmenuData.id]: newZmenu
    });
  };

  const handleZmenuDestroy = (event: ZmenuDestroyEvent) => {
    setZmenus(pickBy(zmenus, (value, key) => key !== event.detail.id));
  };

  const handleZmenuReposition = (event: ZmenuRepositionEvent) => {
    const zmenuData = event.detail;
    setZmenus({
      ...zmenus,
      [zmenuData.id]: {
        ...zmenus[zmenuData.id],
        anchor_coordinates: zmenuData.anchor_coordinates
      }
    });
  };

  const handleZmenuEnter = (id: string) => {
    const browserElement = props.browserRef.current as HTMLDivElement;
    const event = new CustomEvent('bpane-zmenu', {
      detail: {
        id,
        action: ZmenuAction.ENTER
      }
    });
    browserElement.dispatchEvent(event);
  };

  const handleZmenuLeave = (id: string) => {
    const browserElement = props.browserRef.current as HTMLDivElement;
    const event = new CustomEvent('bpane-zmenu', {
      detail: {
        id,
        action: ZmenuAction.LEAVE
      }
    });
    browserElement.dispatchEvent(event);
  };

  const zmenuElements = Object.keys(zmenus).map((id) => (
    <Zmenu
      key={id}
      browserRef={props.browserRef}
      onEnter={handleZmenuEnter}
      onLeave={handleZmenuLeave}
      {...zmenus[id]}
    />
  ));

  return <>{zmenuElements}</>;
};

export default React.memo(ZmenuController);

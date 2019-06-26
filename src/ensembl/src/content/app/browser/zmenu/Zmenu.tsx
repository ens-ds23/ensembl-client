import React from 'react';

import ZmenuContent from './ZmenuContent';

import styles from './Zmenu.scss';

import { ZmenuData, AnchorCoordinates } from './zmenu-types';

const TIP_WIDTH = 18;
const TIP_HEIGHT = 13;
// extra height makes the tip a bit higher and is used to anchor the tip in zmenu body (goes inside the body)
const TIP_EXTRA_HEIGHT = 2;
const TIP_OFFSET = 10;

enum Direction {
  LEFT = 'left',
  RIGHT = 'right'
}

type Props = ZmenuData & {
  browserRef: React.RefObject<HTMLDivElement>;
  onEnter: (id: string) => void;
  onLeave: (id: string) => void;
};

type InlineStyles = { [key: string]: string | number | undefined };
type TipProps = {
  direction: Direction; // where the tip is pointing
  style: InlineStyles;
};

type GetInlineStylesParams = {
  direction: Direction;
  anchorCoordinates: AnchorCoordinates;
};

const Zmenu = (props: Props) => {
  const direction = chooseDirection(props);
  const inlineStyles = getInlineStyles({
    direction,
    anchorCoordinates: props.anchor_coordinates
  });

  return (
    <div
      className={styles.zmenuWrapper}
      style={inlineStyles.body}
      onMouseEnter={() => props.onEnter(props.id)}
      onMouseLeave={() => props.onLeave(props.id)}
    >
      <div className={styles.zmenu}>
        <ZmenuContent content={props.content} />
      </div>
      <Tip
        direction={getInverseDirection(direction)}
        style={inlineStyles.tip}
      />
    </div>
  );
};

// choose how to position zmenu relative to its anchor point
const chooseDirection = (params: Props) => {
  const browserElement = params.browserRef.current as HTMLDivElement;
  const { width } = browserElement.getBoundingClientRect();
  const { x } = params.anchor_coordinates;
  return x > width / 2 ? Direction.LEFT : Direction.RIGHT;
};

const getInlineStyles = (params: GetInlineStylesParams) => {
  const { direction, anchorCoordinates } = params;
  if (direction === Direction.LEFT) {
    return {
      body: {
        // body is to the left of the anchor point
        left: `${anchorCoordinates.x - TIP_HEIGHT}px`,
        transform: `translateX(-100%)`,
        top: `${anchorCoordinates.y - TIP_OFFSET - TIP_WIDTH / 2}px`
      },
      tip: {
        // tip is on the right side of the body and points to the right
        right: `-${TIP_HEIGHT}px`,
        top: `${TIP_OFFSET}px`,
        width: `${TIP_HEIGHT + TIP_EXTRA_HEIGHT}px`,
        height: `${TIP_WIDTH}px`
      }
    };
  } else {
    // Direction.RIGHT
    return {
      body: {
        // body is to the right of the anchor point
        left: `${anchorCoordinates.x + TIP_HEIGHT}px`,
        top: `${anchorCoordinates.y - TIP_OFFSET - TIP_WIDTH / 2}px`
      },
      tip: {
        // tip is on the left side of the body and points to the left
        left: `-${TIP_HEIGHT}px`,
        top: `${TIP_OFFSET}px`,
        width: `${TIP_HEIGHT + TIP_EXTRA_HEIGHT}px`,
        height: `${TIP_WIDTH}px`
      }
    };
  }
};

const getInverseDirection = (direction: Direction) => {
  if (direction === Direction.LEFT) {
    return Direction.RIGHT;
  } else {
    return Direction.LEFT;
  }
};

const Tip = (props: TipProps) => {
  const halfBase = TIP_WIDTH / 2;
  let polygonPoints;
  const height = TIP_HEIGHT + TIP_EXTRA_HEIGHT;

  if (props.direction === Direction.LEFT) {
    polygonPoints = `0,${halfBase} ${height},0 ${height},${TIP_WIDTH}`;
  } else {
    polygonPoints = `0,0 ${height},${halfBase} 0,${TIP_WIDTH}`;
  }

  return (
    <svg
      className={styles.zmenuTip}
      style={props.style}
      xmlns="http://www.w3.org/2000/svg"
      xmlnsXlink="http://www.w3.org/1999/xlink"
      viewBox={`0 0 ${height} ${TIP_WIDTH}`}
    >
      <polygon points={polygonPoints} />
    </svg>
  );
};

export default Zmenu;

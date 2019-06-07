import React, {
  FunctionComponent,
  useCallback,
  CSSProperties,
  useState,
  useEffect
} from 'react';

import cogOnIcon from 'static/img/shared/cog-on.svg';
import cogOffIcon from 'static/img/shared/cog.svg';
import BrowserTrackConfig from './browser-track-config/BrowserTrackConfig';
import { useTransition, animated } from 'react-spring';

type BrowserCogProps = {
  cogActivated: boolean;
  index: string;
  updateSelectedCog: (index: string) => void;
};

const BrowserCog: FunctionComponent<BrowserCogProps> = (
  props: BrowserCogProps
) => {
  const { cogActivated, updateSelectedCog, index } = props;

  const toggleCog = useCallback(() => {
    if (cogActivated === false) {
      updateSelectedCog(index);
    } else {
      updateSelectedCog('');
    }
  }, [cogActivated]);

  const inline: CSSProperties = { position: 'relative' };
  const imgInline: CSSProperties = {
    height: '18px',
    width: '18px'
  };

  const cogIcon = props.cogActivated ? cogOnIcon : cogOffIcon;

  const [showTrackConfig, setTrackConfigAnimation] = useState(cogActivated);
  useEffect(() => {
    if (cogActivated) {
      setTrackConfigAnimation(true);
      return;
    }
    setTrackConfigAnimation(false);
  }, [cogActivated]);

  const transitions = useTransition(showTrackConfig, null, {
    config: { duration: 100 },
    enter: { opacity: 1 },
    from: { opacity: 0 },
    leave: { opacity: 0 }
  });

  return (
    <>
      <div style={inline}>
        <button onClick={toggleCog}>
          <img src={cogIcon} style={imgInline} alt="Configure track" />
        </button>
      </div>
      {transitions.map(({ item, key, props: style }) => {
        return (
          item && (
            <animated.div key={key} style={style}>
              <BrowserTrackConfig />
            </animated.div>
          )
        );
      })}
    </>
  );
};

export default BrowserCog;

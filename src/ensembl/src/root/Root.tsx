import React, {
  FunctionComponent,
  useEffect,
  useState,
  useCallback
} from 'react';
import { connect } from 'react-redux';
import { withCookies, ReactCookieProps, Cookies } from 'react-cookie';
import useResizeObserver from 'use-resize-observer';

import Header from '../header/Header';
import Content from '../content/Content';
import PrivacyBanner from '../shared/privacy-banner/PrivacyBanner';
import ErrorBoundary from 'src/shared/error-boundary/ErrorBoundary';
import { GeneralErrorScreen } from 'src/shared/error-screen';

import { updateBreakpointWidth } from '../global/globalActions';
import { getBreakpointWidth } from '../global/globalSelectors';
import { RootState } from '../store';
import { BreakpointWidth } from '../global/globalConfig';
import { getBreakpoint } from '../global/globalHelper';

import styles from './Root.scss';

type StateProps = {
  breakpointWidth: BreakpointWidth;
};

type DispatchProps = {
  updateBreakpointWidth: (breakpointWidth: BreakpointWidth) => void;
};

type OwnProps = {};

type RootProps = StateProps & DispatchProps & ReactCookieProps & OwnProps;

export const Root: FunctionComponent<RootProps> = (props: RootProps) => {
  const [ref, width] = useResizeObserver();
  const currentBreakpoint: BreakpointWidth = getBreakpoint(width);

  useEffect(() => {
    props.updateBreakpointWidth(currentBreakpoint);
  }, [props.updateBreakpointWidth, currentBreakpoint]);

  const [showPrivacyBanner, setShowPrivacyBanner] = useState(true);
  const cookies = props.cookies as Cookies;

  useEffect(() => {
    if (showPrivacyBanner && cookies.get('ENSEMBL_PRIVACY_POLICY') === 'true') {
      setShowPrivacyBanner(false);
    }
  }, [cookies]);

  const closeBanner = useCallback(() => {
    cookies.set('ENSEMBL_PRIVACY_POLICY', 'true');
    setShowPrivacyBanner(false);
  }, [cookies]);

  return (
    <div ref={ref as React.RefObject<HTMLDivElement>} className={styles.root}>
      <ErrorBoundary fallbackComponent={GeneralErrorScreen}>
        <Header />
        <Content />
        {showPrivacyBanner && <PrivacyBanner closeBanner={closeBanner} />}
      </ErrorBoundary>
    </div>
  );
};

const mapStateToProps = (state: RootState): StateProps => ({
  breakpointWidth: getBreakpointWidth(state)
});

const mapDispatchToProps: DispatchProps = { updateBreakpointWidth };

export default withCookies(
  connect(
    mapStateToProps,
    mapDispatchToProps
  )(Root)
);

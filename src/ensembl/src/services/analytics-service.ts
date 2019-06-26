import ReactGA from 'react-ga';
import { AnalyticsType } from 'src/analyticsHelper';

import config from 'config';

const { googleAnalyticsKey = '' } = config;

class AnalyticsTracking {
  private reactGA: typeof ReactGA;

  public constructor() {
    ReactGA.initialize(googleAnalyticsKey);
    this.reactGA = ReactGA;
  }

  // Track a pageview
  public trackPageView(pathToTrack: string) {
    this.reactGA.pageview(pathToTrack);
  }

  // Track an event
  public trackEvent(action: any) {
    const { ga } = action.meta as AnalyticsType;

    this.reactGA.event({
      action: ga.action ? ga.action : action.type,
      category: ga.category,
      label: ga.label,
      nonInteraction: ga.nonInteraction,
      transport: 'xhr',
      value: ga.value
    });
  }
}

const GoogleAnalyticsTracking = new AnalyticsTracking();

export default GoogleAnalyticsTracking;

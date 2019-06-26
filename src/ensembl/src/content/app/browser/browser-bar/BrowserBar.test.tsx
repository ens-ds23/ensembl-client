import React from 'react';
import { mount } from 'enzyme';

import { BrowserBar, BrowserInfo, BrowserNavigatorButton } from './BrowserBar';

import { TrackType } from '../track-panel/trackPanelConfig';

import BrowserReset from 'src/content/app/browser/browser-reset/BrowserReset';
import BrowserGenomeSelector from 'src/content/app/browser/browser-genome-selector/BrowserGenomeSelector';
import BrowserTabs from 'src/content/app/browser/browser-tabs/BrowserTabs';

jest.mock('src/content/app/browser/browser-reset/BrowserReset', () => () => (
  <div>BrowserReset</div>
));
jest.mock(
  'src/content/app/browser/browser-genome-selector/BrowserGenomeSelector',
  () => () => <div>BrowserGenomeSelector</div>
);
jest.mock('src/content/app/browser/browser-tabs/BrowserTabs', () => () => (
  <div>BrowserReset</div>
));

describe('<BrowserBar />', () => {
  const dispatchBrowserLocation: any = jest.fn();
  const selectBrowserTab: any = jest.fn();
  const toggleBrowserNav: any = jest.fn();
  const toggleDrawer: any = jest.fn();
  const toggleGenomeSelector: any = jest.fn();

  const objectInfo = {
    assembly: {
      name: 'GRCh38',
      patch: 'p12'
    },
    associated_object: {
      obj_type: 'transcript',
      selected_info: 'MANE Select',
      spliced_length: 84793,
      stable_id: 'ENST00000380152.7'
    },
    bio_type: 'Protein coding',
    obj_symbol: 'BRCA2',
    obj_type: 'gene',
    spliced_length: 84793,
    stable_id: 'ENSG00000139618',
    strand: 'forward',
    genome_id: 'homo_sapiens_grch38'
  };

  const defaultProps = {
    browserActivated: true,
    browserNavOpened: false,
    chrLocation: ['13', 32275301, 32433493] as [string, number, number],
    defaultChrLocation: ['13', 32271473, 32437359] as [string, number, number],
    drawerOpened: false,
    genomeSelectorActive: false,
    ensObjectInfo: objectInfo,
    selectedBrowserTab: TrackType.GENOMIC,
    trackPanelModalOpened: false,
    trackPanelOpened: false,
    dispatchBrowserLocation,
    selectBrowserTab,
    toggleBrowserNav,
    toggleDrawer,
    toggleGenomeSelector
  };

  const renderBrowserBar = (props?: any) => (
    <BrowserBar {...defaultProps} {...props} />
  );

  describe('general', () => {
    let renderedBrowserBar: any;

    beforeEach(() => {
      renderedBrowserBar = mount(renderBrowserBar());
    });

    test('contains a left bar', () => {
      expect(renderedBrowserBar.find('.browserInfoLeft')).toHaveLength(1);
    });

    test('contains a right bar', () => {
      expect(renderedBrowserBar.find('.browserInfoRight')).toHaveLength(1);
    });

    test('contains BrowserReset button', () => {
      expect(renderedBrowserBar.find(BrowserReset)).toHaveLength(1);
    });

    test('contains BrowserGenomeSelector', () => {
      expect(renderedBrowserBar.find(BrowserGenomeSelector)).toHaveLength(1);
    });
  });

  describe('behaviour', () => {
    test('does not show BrowserInfo panel by default', () => {
      const renderedBrowserBar = mount(renderBrowserBar());
      expect(renderedBrowserBar.find(BrowserInfo)).toHaveLength(0);
    });

    test('shows BrowserInfo panel if gene location is provided', () => {
      const renderedBrowserBar = mount(
        renderBrowserBar({
          activeGenomeId: 'homo_sapiens_grch38',
          defaultChrLocation: { homo_sapiens_grch38: ['13', 100, 100] }
        })
      );
      expect(renderedBrowserBar.find(BrowserInfo).length).toBe(1);
    });

    test('hides BrowserInfo panel if gene location is not provided', () => {
      const renderedBrowserBar = mount(
        renderBrowserBar({ defaultChrLocation: ['13', 0, 0] })
      );
      expect(renderedBrowserBar.find(BrowserInfo).length).toBe(0);
    });

    test('hides BrowserInfo panel if genome selector becomes active', async () => {
      const renderedBrowserBar = mount(renderBrowserBar());
      renderedBrowserBar.setProps({ genomeSelectorActive: true });

      // ugly hack: fall back to the end of event queue, giving priority to useEffect and useState
      await new Promise((resolve) => setTimeout(resolve, 0));

      renderedBrowserBar.update();
      expect(renderedBrowserBar.find(BrowserInfo).length).toBe(0);
    });

    test('shows BrowserNavigatorButton if genomeSelector is not active', () => {
      const renderedBrowserBar = mount(renderBrowserBar());
      expect(renderedBrowserBar.find(BrowserNavigatorButton).length).toBe(1);
    });

    test('hides BrowserNavigatorButton if genomeSelector is active', () => {
      const renderedBrowserBar = mount(
        renderBrowserBar({ genomeSelectorActive: true })
      );
      expect(renderedBrowserBar.find(BrowserNavigatorButton).length).toBe(0);
    });

    test('hides BrowserNavigatorButton if there is no focus object', () => {
      const renderedBrowserBar = mount(renderBrowserBar({ ensObjectInfo: {} }));
      expect(renderedBrowserBar.find(BrowserNavigatorButton).length).toBe(0);
    });

    test('shows BrowserTabs if TrackPanel is open', () => {
      const renderedBrowserBar = mount(
        renderBrowserBar({ trackPanelOpened: true })
      );
      expect(renderedBrowserBar.find(BrowserTabs).length).toBe(1);
    });

    test('hides BrowserTabs if TrackPanel is closed', () => {
      const renderedBrowserBar = mount(renderBrowserBar());
      expect(renderedBrowserBar.find(BrowserTabs).length).toBe(0);
    });
  });
});

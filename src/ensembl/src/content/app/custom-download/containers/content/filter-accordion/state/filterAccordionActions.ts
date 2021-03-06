import { createAction } from 'typesafe-actions';

import { getCustomDownloadAnalyticsObject } from 'src/analyticsHelper';

export const setFiltersAccordionExpandedPanel = createAction(
  'custom-download/set-filters-accordion-expanded-panels',
  (resolve) => {
    return (expandedPanel: string) =>
      resolve(
        expandedPanel,
        getCustomDownloadAnalyticsObject('Toggle filters accordion panel')
      );
  }
);

export const setFiltersAccordionExpandedGenePanels = createAction(
  'custom-download/set-filters-accordion-expanded-gene-panels',
  (resolve) => {
    return (expandedPanels: []) =>
      resolve(
        expandedPanels,
        getCustomDownloadAnalyticsObject('Toggle filters accordion gene panels')
      );
  }
);

export const setGencodeAnnotationFilters = createAction(
  'custom-download/set-gencode-basic-annotation-filters',
  (resolve) => {
    return (gencodeBasicAnnotation: string) =>
      resolve(
        gencodeBasicAnnotation,
        getCustomDownloadAnalyticsObject(
          'Gencode basic annotation filters updated'
        )
      );
  }
);

export const setGeneSourceFilters = createAction(
  'custom-download/set-gene-sourcefilters',
  (resolve) => {
    return (geneSource: {}) =>
      resolve(
        geneSource,
        getCustomDownloadAnalyticsObject('Gene source filters updated')
      );
  }
);

export const setGeneTypeFilters = createAction(
  'custom-download/set-gene-type-filters',
  (resolve) => {
    return (geneTypeFilters: {}) =>
      resolve(
        geneTypeFilters,
        getCustomDownloadAnalyticsObject('Gene type filters updated')
      );
  }
);

export const setTranscriptTypeFilters = createAction(
  'custom-download/set-transcript-type-filters',
  (resolve) => {
    return (transcriptTypeFilters: {}) =>
      resolve(
        transcriptTypeFilters,
        getCustomDownloadAnalyticsObject('Transcript type filters updated')
      );
  }
);

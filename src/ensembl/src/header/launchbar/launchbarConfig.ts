import ensemblIcon from 'static/img/launchbar/ensembl.svg';
import ensemblSelectedIcon from 'static/img/launchbar/ensembl-selected.svg';

import searchIcon from 'static/img/launchbar/search.svg';
import searchSelectedIcon from 'static/img/launchbar/search-selected.svg';
import searchGreyIcon from 'static/img/launchbar/search-grey.svg';

import speciesSelectorIcon from 'static/img/launchbar/species-selector.svg';
import speciesSelectorSelectedIcon from 'static/img/launchbar/species-selector-selected.svg';
import speciesSelectorGreyIcon from 'static/img/launchbar/species-selector-grey.svg';

import browserIcon from 'static/img/launchbar/browser.svg';
import browserSelectedIcon from 'static/img/launchbar/browser-selected.svg';

// import blastIcon from 'static/img/launchbar/blast.svg';
// import blastSelectedIcon from 'static/img/launchbar/blast-selected.svg';

import vepIcon from 'static/img/launchbar/vep.svg';
import vepSelectedIcon from 'static/img/launchbar/vep-selected.svg';

import toolsGreyIcon from 'static/img/launchbar/tools-grey.svg';

// import conversionsIcon from 'static/img/launchbar/conversions.svg';
// import conversionsSelectedIcon from 'static/img/launchbar/conversions-selected.svg';

// import ldIcon from 'static/img/launchbar/ld.svg';
// import ldSelectedIcon from 'static/img/launchbar/ld-selected.svg';

// import biomartIcon from 'static/img/launchbar/biomart.svg';
// import biomartSelectedIcon from 'static/img/launchbar/biomart-selected.svg';

// import bulkDownloadIcon from 'static/img/launchbar/bulk-download.svg';
// import bulkDownloadSelectedIcon from 'static/img/launchbar/bulk-download-selected.svg';

import customDownloadIcon from 'static/img/launchbar/custom-download.svg';
import customDownloadSelectedIcon from 'static/img/launchbar/custom-download-selected.svg';

import downloadGreyIcon from 'static/img/launchbar/download-grey.svg';

import helpIcon from 'static/img/launchbar/help.svg';
import helpSelectedIcon from 'static/img/launchbar/help-selected.svg';
import helpGreyIcon from 'static/img/launchbar/help-grey.svg';

// import glossaryIcon from 'static/img/launchbar/glossary.svg';
// import glossarySelectedIcon from 'static/img/launchbar/glossary-selected.svg';

// import newInfoIcon from 'static/img/launchbar/new-info.svg';
// import newInfoSelectedIcon from 'static/img/launchbar/new-info-selected.svg';

export type LaunchbarApp = {
  description: string;
  icon: {
    default: string;
    grey?: string;
    selected: string;
  };
  name: string;
};

export type LaunchbarCategory = {
  apps: LaunchbarApp[];
  name: string;
  separator: boolean;
};

export type LaunchbarDetails = {
  about: LaunchbarApp;
  categories: LaunchbarCategory[];
};

export const launchbarConfig: LaunchbarDetails = {
  about: {
    description: 'about ensembl',
    icon: {
      default: ensemblIcon,
      selected: ensemblSelectedIcon
    },
    name: 'about'
  },
  categories: [
    {
      apps: [
        {
          description: 'global search',
          icon: {
            default: searchIcon,
            grey: searchGreyIcon,
            selected: searchSelectedIcon
          },
          name: 'global-search'
        },
        {
          description: 'species selector',
          icon: {
            default: speciesSelectorIcon,
            grey: speciesSelectorGreyIcon,
            selected: speciesSelectorSelectedIcon
          },
          name: 'species-selector'
        }
      ],
      name: 'search',
      separator: true
    },
    {
      apps: [
        {
          description: 'browser',
          icon: {
            default: browserIcon,
            selected: browserSelectedIcon
          },
          name: 'browser'
        }
      ],
      name: 'browsers',
      separator: true
    },
    {
      apps: [
        // {
        //   description: 'BLAST/BLAT',
        //   icon: {
        //     default: blastIcon,
        //     selected: blastSelectedIcon
        //   },
        //   name: 'blast'
        // },
        // {
        //   description: 'VEP',
        //   icon: {
        //     default: vepIcon,
        //     selected: vepSelectedIcon
        //   },
        //   name: 'vep'
        // },
        // {
        //   description: 'Conversions',
        //   icon: {
        //     default: conversionsIcon,
        //     selected: conversionsSelectedIcon
        //   },
        //   name: 'conversions'
        // },
        // {
        //   description: 'LD calculator',
        //   icon: {
        //     default: ldIcon,
        //     selected: ldSelectedIcon
        //   },
        //   name: 'ld'
        // },
        // {
        //   description: 'biomart',
        //   icon: {
        //     default: biomartIcon,
        //     selected: biomartSelectedIcon
        //   },
        //   name: 'biomart'
        // }
        {
          description: 'tools',
          icon: {
            default: vepIcon,
            grey: toolsGreyIcon,
            selected: vepSelectedIcon
          },
          name: 'tools'
        }
      ],
      name: 'tools',
      separator: true
    },
    {
      apps: [
        // {
        //   description: 'bulk download',
        //   icon: {
        //     default: bulkDownloadIcon,
        //     selected: bulkDownloadSelectedIcon
        //   },
        //   name: 'bulk-download'
        // },
        // {
        //   description: 'custom download',
        //   icon: {
        //     default: customDownloadIcon,
        //     selected: customDownloadSelectedIcon
        //   },
        //   name: 'custom-download'
        // }
        {
          description: 'download',
          icon: {
            default: customDownloadIcon,
            grey: downloadGreyIcon,
            selected: customDownloadSelectedIcon
          },
          name: 'download'
        }
      ],
      name: 'downloads',
      separator: true
    },
    {
      apps: [
        {
          description: 'help and documentation',
          icon: {
            default: helpIcon,
            grey: helpGreyIcon,
            selected: helpSelectedIcon
          },
          name: 'help-docs'
        }
        // {
        //   description: 'glossary',
        //   icon: {
        //     default: glossaryIcon,
        //     selected: glossarySelectedIcon
        //   },
        //   name: 'glossary'
        // },
        // {
        //   description: 'what is new and what has changed',
        //   icon: {
        //     default: newInfoIcon,
        //     selected: newInfoSelectedIcon
        //   },
        //   name: 'new-info'
        // }
      ],
      name: 'learning',
      separator: false
    }
  ]
};
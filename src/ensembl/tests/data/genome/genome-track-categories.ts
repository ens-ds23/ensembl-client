import { TrackType } from 'src/content/app/browser/track-panel/trackPanelConfig';
import { GenomeTrackCategories } from 'src/genome/genomeTypes';

export const humanTrackCategoriesResponse: GenomeTrackCategories = {
  genome_id: 'homo_sapiens38',
  track_categories: [
    {
      label: 'Genes & transcripts',
      track_category_id: 'genes-transcripts',
      track_list: [
        {
          additional_info: 'Forward strand',
          colour: 'DARK_GREY',
          label: 'Protein coding genes',
          track_id: 'gene-pc-fwd'
        },
        {
          additional_info: 'Forward strand',
          colour: 'GREY',
          label: 'Other genes',
          track_id: 'gene-other-fwd'
        },
        {
          additional_info: 'Reverse strand',
          colour: 'DARK_GREY',
          label: 'Protein coding genes',
          track_id: 'gene-pc-rev'
        },
        {
          additional_info: 'Reverse strand',
          colour: 'GREY',
          label: 'Other genes',
          track_id: 'gene-other-rev'
        }
      ],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Assembly',
      track_category_id: 'assembly',
      track_list: [
        {
          label: 'Contigs',
          track_id: 'contig'
        },
        {
          label: '%GC',
          track_id: 'gc'
        }
      ],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Comparative genomics',
      track_category_id: 'comparative-genomics',
      track_list: [],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Short variants',
      track_category_id: 'short-variants',
      track_list: [
        {
          label: '1000 Genomes all SNPs & indels',
          track_id: 'snps-and-indels'
        }
      ],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Somatic variants',
      track_category_id: 'somatic-variants',
      track_list: [],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Structural variants',
      track_category_id: 'structural-variants',
      track_list: [],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Experiment design',
      track_category_id: 'experiment-design',
      track_list: [],
      types: [TrackType.GENOMIC, TrackType.EXPRESSION, TrackType.VARIATION]
    },
    {
      label: 'References & evidence',
      track_category_id: 'references-evidence',
      track_list: [],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Regulatory features',
      track_category_id: 'regulatory-features',
      track_list: [],
      types: [TrackType.EXPRESSION]
    },
    {
      label: 'Gene expression correlations',
      track_category_id: 'gene-expression',
      track_list: [],
      types: [TrackType.EXPRESSION]
    },
    {
      label: 'Other regulatory regions',
      track_category_id: 'regulatory-regions',
      track_list: [],
      types: [TrackType.EXPRESSION]
    }
  ]
};

export const mouseTrackCategoriesResponse: GenomeTrackCategories = {
  genome_id: 'mus_musculus_bdc',
  track_categories: [
    {
      label: 'Genes & transcripts',
      track_category_id: 'genes-transcripts',
      track_list: [
        {
          additional_info: 'Forward strand',
          colour: 'DARK_GREY',
          label: 'Protein coding genes',
          track_id: 'gene-pc-fwd'
        },
        {
          additional_info: 'Forward strand',
          colour: 'GREY',
          label: 'Other genes',
          track_id: 'gene-other-fwd'
        },
        {
          additional_info: 'Reverse strand',
          colour: 'DARK_GREY',
          label: 'Protein coding genes',
          track_id: 'gene-pc-rev'
        },
        {
          additional_info: 'Reverse strand',
          colour: 'GREY',
          label: 'Other genes',
          track_id: 'gene-other-rev'
        }
      ],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Assembly',
      track_category_id: 'assembly',
      track_list: [
        {
          label: 'Contigs',
          track_id: 'contig'
        },
        {
          label: '%GC',
          track_id: 'gc'
        }
      ],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Comparative genomics',
      track_category_id: 'comparative-genomics',
      track_list: [],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Short variants',
      track_category_id: 'short-variants',
      track_list: [
        {
          label: '1000 Genomes all SNPs & indels',
          track_id: 'snps-and-indels'
        }
      ],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Somatic variants',
      track_category_id: 'somatic-variants',
      track_list: [],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Structural variants',
      track_category_id: 'structural-variants',
      track_list: [],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Experiment design',
      track_category_id: 'experiment-design',
      track_list: [],
      types: [TrackType.GENOMIC, TrackType.EXPRESSION, TrackType.VARIATION]
    },
    {
      label: 'References & evidence',
      track_category_id: 'references-evidence',
      track_list: [],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Regulatory features',
      track_category_id: 'regulatory-features',
      track_list: [],
      types: [TrackType.EXPRESSION]
    },
    {
      label: 'Gene expression correlations',
      track_category_id: 'gene-expression',
      track_list: [],
      types: [TrackType.EXPRESSION]
    },
    {
      label: 'Other regulatory regions',
      track_category_id: 'regulatory-regions',
      track_list: [],
      types: [TrackType.EXPRESSION]
    }
  ]
};

export const wheatTrackCategoriesResponse: GenomeTrackCategories = {
  genome_id: 'triticum_aestivum_GCA_900519105_1',
  track_categories: [
    {
      label: 'Genes & transcripts',
      track_category_id: 'genes-transcripts',
      track_list: [
        {
          additional_info: 'Forward strand',
          colour: 'DARK_GREY',
          label: 'Protein coding genes',
          track_id: 'gene-pc-fwd'
        },
        {
          additional_info: 'Forward strand',
          colour: 'GREY',
          label: 'Other genes',
          track_id: 'gene-other-fwd'
        },
        {
          additional_info: 'Reverse strand',
          colour: 'DARK_GREY',
          label: 'Protein coding genes',
          track_id: 'gene-pc-rev'
        },
        {
          additional_info: 'Reverse strand',
          colour: 'GREY',
          label: 'Other genes',
          track_id: 'gene-other-rev'
        }
      ],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Assembly',
      track_category_id: 'assembly',
      track_list: [
        {
          label: 'Contigs',
          track_id: 'contig'
        },
        {
          label: '%GC',
          track_id: 'gc'
        }
      ],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Comparative genomics',
      track_category_id: 'comparative-genomics',
      track_list: [],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Short variants',
      track_category_id: 'short-variants',
      track_list: [
        {
          label: '1000 Genomes all SNPs & indels',
          track_id: 'snps-and-indels'
        }
      ],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Somatic variants',
      track_category_id: 'somatic-variants',
      track_list: [],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Structural variants',
      track_category_id: 'structural-variants',
      track_list: [],
      types: [TrackType.VARIATION]
    },
    {
      label: 'Experiment design',
      track_category_id: 'experiment-design',
      track_list: [],
      types: [TrackType.GENOMIC, TrackType.EXPRESSION, TrackType.VARIATION]
    },
    {
      label: 'References & evidence',
      track_category_id: 'references-evidence',
      track_list: [],
      types: [TrackType.GENOMIC]
    },
    {
      label: 'Regulatory features',
      track_category_id: 'regulatory-features',
      track_list: [],
      types: [TrackType.EXPRESSION]
    },
    {
      label: 'Gene expression correlations',
      track_category_id: 'gene-expression',
      track_list: [],
      types: [TrackType.EXPRESSION]
    },
    {
      label: 'Other regulatory regions',
      track_category_id: 'regulatory-regions',
      track_list: [],
      types: [TrackType.EXPRESSION]
    }
  ]
};

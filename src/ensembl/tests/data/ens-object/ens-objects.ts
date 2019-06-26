import {
  EnsObjectResponse,
  EnsObjectTracksResponse
} from '../../../src/ens-object/ensObjectTypes';

export const humanGeneResponse: EnsObjectResponse = {
  ensembl_object: {
    bio_type: 'protein coding',
    label: 'BRCA2',
    ensembl_object_id: 'homo_sapiens_GCA_000001405_27:gene:ENSG00000139618',
    genome_id: 'homo_sapiens_GCA_000001405_27',
    location: {
      chromosome: '13',
      end: 32437359,
      start: 32271473
    },
    object_type: 'gene',
    spliced_length: 84793,
    stable_id: 'ENSG00000139618',
    strand: 'forward'
  }
};

export const humanGeneTracksResponse: EnsObjectTracksResponse = {
  object_tracks: {
    additional_info: 'Protein coding',
    child_tracks: [
      {
        additional_info: 'Protein coding',
        colour: 'BLUE',
        label: 'ENST00000380152.7',
        ensembl_object_id: 'transcript-1:ENSG00000139618',
        support_level: 'MANE Select',
        track_id: 'gene-feat-1'
      },
      {
        additional_info: 'Nonsense mediated decay',
        colour: 'GREY',
        label: '	ENST00000470094.1',
        ensembl_object_id: 'transcript-2:ENSG00000139618',
        support_level: 'TSL:5',
        track_id: 'gene-feat-2'
      }
    ],
    label: 'BRCA2',
    ensembl_object_id: 'homo_sapiens_GCA_000001405_27:gene:ENSG00000139618',
    track_id: 'gene-feat'
  }
};

export const humanRegionResponse: EnsObjectResponse = {
  ensembl_object: {
    bio_type: null,
    label: '17:63992802-64038237',
    ensembl_object_id: 'homo_sapiens:region:17:63992802-64038237',
    genome_id: 'homo_sapiens_GCA_000001405_27',
    location: {
      chromosome: '17',
      end: 64038237,
      start: 63992802
    },
    object_type: 'region',
    spliced_length: null,
    stable_id: null,
    strand: null
  }
};

export const mouseGeneResponse: EnsObjectResponse = {
  ensembl_object: {
    bio_type: 'protein coding',
    label: 'Cntnap1',
    ensembl_object_id: 'mus_musculus_bdc:gene:ENSMUSG00000017167',
    genome_id: 'mus_musculus_bdc',
    location: {
      chromosome: '11',
      end: 101190724,
      start: 101170523
    },
    object_type: 'gene',
    spliced_length: 0,
    stable_id: 'ENSMUSG00000017167',
    strand: 'forward'
  }
};

export const mouseGeneTracksResponse: EnsObjectTracksResponse = {
  object_tracks: {
    additional_info: 'Protein coding',
    child_tracks: [
      {
        additional_info: 'Protein coding',
        colour: 'BLUE',
        label: 'ENSMUST00000103109.3',
        ensembl_object_id: 'transcript-1:ENSMUSG00000017167',
        support_level: 'TSL:1',
        track_id: 'gene-feat-1'
      },
      {
        additional_info: 'processed coding',
        colour: 'GREY',
        label: 'ENSMUST00000138942.1',
        ensembl_object_id: 'transcript-2:ENSMUSG00000017167',
        support_level: 'TSL:5',
        track_id: 'gene-feat-2'
      }
    ],
    label: 'Cntnap1',
    ensembl_object_id: 'mus_musculus_bdc:gene:ENSMUSG00000017167',
    track_id: 'gene-feat'
  }
};

export const mouseRegionResponse: EnsObjectResponse = {
  ensembl_object: {
    bio_type: null,
    label: '4:136366473-136547301',
    ensembl_object_id: 'mus_musculus_bdc:region:4:136366473-136547301',
    genome_id: 'mus_musculus_bdc',
    location: {
      chromosome: '4',
      end: 136547301,
      start: 136366473
    },
    object_type: 'region',
    spliced_length: null,
    stable_id: null,
    strand: null
  }
};

export const wheatGeneResponse: EnsObjectResponse = {
  ensembl_object: {
    bio_type: 'protein coding',
    label: 'TraesCS3D02G273600',
    ensembl_object_id:
      'triticum_aestivum_GCA_900519105_1:gene:TraesCS3D02G273600',
    genome_id: 'triticum_aestivum_GCA_900519105_1',
    location: {
      chromosome: '3D',
      end: 379539827,
      start: 379535906
    },
    object_type: 'gene',
    spliced_length: null,
    stable_id: 'TraesCS3D02G273600',
    strand: 'forward'
  }
};

export const wheatGeneTracksResponse: EnsObjectTracksResponse = {
  object_tracks: {
    additional_info: 'Protein coding',
    child_tracks: [
      {
        additional_info: 'Protein coding',
        colour: 'BLUE',
        label: 'TraesCS3D02G273600.1',
        ensembl_object_id: 'transcript-1:TraesCS3D02G273600',
        support_level: null,
        track_id: 'gene-feat-1'
      },
      {
        additional_info: 'Protein coding',
        colour: 'GREY',
        label: 'TraesCS3D02G273600.2',
        ensembl_object_id: 'transcript-2:TraesCS3D02G273600',
        support_level: null,
        track_id: 'gene-feat-2'
      }
    ],
    label: 'TraesCS3D02G273600',
    ensembl_object_id:
      'triticum_aestivum_GCA_900519105_1:gene:TraesCS3D02G273600',
    track_id: 'gene-feat'
  }
};

export const wheatRegionResponse: EnsObjectResponse = {
  ensembl_object: {
    bio_type: null,
    label: '3D:2585940-2634711',
    ensembl_object_id:
      'triticum_aestivum_GCA_900519105_1:region:3D:2585940-2634711',
    genome_id: 'triticum_aestivum_GCA_900519105_1',
    location: {
      chromosome: '3D',
      end: 2634711,
      start: 2585940
    },
    object_type: 'region',
    spliced_length: null,
    stable_id: null,
    strand: null
  }
};

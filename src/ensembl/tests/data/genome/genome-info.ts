import { GenomeInfoResponse } from 'src/genome/genomeTypes';

export const humanGenomeInfoResponse: GenomeInfoResponse = {
  genome_info: [
    {
      genome_id: 'homo_sapiens38',
      reference_genome_id: null,
      common_name: 'Human',
      assembly_name: 'GRCh37',
      scientific_name: 'Homo sapiens',
      example_objects: ['gene:ENSG00000139618', 'region:17:63992802-64038237']
    }
  ]
};

export const mouseGenomeInfoResponse: GenomeInfoResponse = {
  genome_info: [
    {
      genome_id: 'mus_musculus_bdc',
      reference_genome_id: null,
      common_name: 'Mouse',
      assembly_name: 'GRCm38',
      scientific_name: 'Mus musculus',
      example_objects: [
        'gene:ENSMUSG00000017167',
        'region:4:136366473-136547301'
      ]
    }
  ]
};

export const wheatGenomeInfoResponse: GenomeInfoResponse = {
  genome_info: [
    {
      genome_id: 'triticum_aestivum_GCA_900519105_1',
      reference_genome_id: null,
      common_name: 'Wheat',
      assembly_name: 'WIGSC',
      scientific_name: 'Triticum aestivum',
      example_objects: ['gene:TraesCS3D02G273600', 'region:3D:2585940-2634711']
    }
  ]
};

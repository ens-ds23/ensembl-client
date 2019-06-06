/*

REFERENCE - shape of a single match:

{
  "genome_id": string, // a unique identifier (provisional field name)
  "reference_genome_id": string | null, // nullable field; present if the match is a subgroup (e.g. strain) of a species
  "common_name": string | null, // common name (if present)
  "scientific_name": string, // should always be present
  "subtype": string | null, // any extra information about the species (e.g. assembly_name for Human)
  "matched_substrings" : [
      {
          "length" : number,
          "offset" : number,
          "match": "common_name" | "scientific_name" | "subtype" // name of the matched field
      }
    ]
}

*/

// sample response for typing "hum" in the search box

import { MatchedFieldName } from 'src/content/app/species-selector/types/species-search';

export default {
  // notice that it is an array of groups of matches
  matches: [
    [
      {
        genome_id: 'homo_sapiens38',
        reference_genome_id: null,
        common_name: 'Human',
        scientific_name: 'Homo sapiens',
        subtype: 'GRCh38',
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'homo_sapiens37',
        reference_genome_id: null,
        common_name: 'Human',
        scientific_name: 'Homo sapiens',
        subtype: 'GRCh37',
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.COMMON_NAME
          }
        ]
      },
      {
        genome_id: 'humibacillus_sp_dsm_29435',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Humibacillus sp. DSM 29435',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 0,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      }
    ],
    [
      {
        genome_id: 'azospirillum_humicireducens',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Azospirillum humicireducens',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 13,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'propionibacterium_humerusii_hl037pa3',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Propionibacterium humerusii HL037PA3',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 18,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'propionibacterium_humerusii_hl037pa2',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Propionibacterium humerusii HL037PA2',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 18,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'propionibacterium_humerusii_hl044pa1',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Propionibacterium humerusii HL044PA1',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 18,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'propionibacterium_humerusii_p08',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Propionibacterium humerusii P08',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 18,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'sinomonas_humi',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Sinomonas humi',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 10,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'stenotrophomonas_humi',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Stenotrophomonas humi',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 17,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      },
      {
        genome_id: 'thauera_humireducens',
        reference_genome_id: null,
        common_name: null,
        scientific_name: 'Thauera humireducens',
        subtype: null,
        matched_substrings: [
          {
            length: 3,
            offset: 8,
            match: MatchedFieldName.SCIENTIFIC_NAME
          }
        ]
      }
    ]
  ]
};

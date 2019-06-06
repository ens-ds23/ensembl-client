export const human38Match = {
  description: 'Human',
  scientific_name: 'Homo sapiens',
  assembly_name: 'GRCh38.p12',
  matched_substrings: [
    {
      length: 3,
      offset: 0,
      match: 'description'
    }
  ],
  genome: 'GRCh38_demo'
};

export const human37Match = {
  description: 'Human',
  scientific_name: 'Homo sapiens',
  assembly_name: 'GRCh37.p12',
  matched_substrings: [
    {
      length: 3,
      offset: 0,
      match: 'description'
    }
  ],
  self: 'http://whatever.com/GRCh37_demo',
  genome: 'GRCh37_demo'
};

export const azospirillumMatch = {
  description: 'Azospirillum humicireducens',
  scientific_name: null,
  assembly_name: null,
  matched_substrings: [
    {
      length: 3,
      offset: 13,
      match: 'description'
    }
  ],
  genome: 'SgZ-5T'
};

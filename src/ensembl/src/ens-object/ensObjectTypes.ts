export type EnsObjectLocation = {
  chromosome: string;
  end: number;
  start: number;
};

export type EnsObject = {
  bio_type: string | null;
  label: string;
  ensembl_object_id: string;
  genome_id: string;
  location: EnsObjectLocation;
  object_type: string;
  spliced_length: number | null;
  stable_id: string | null;
  strand: string | null;
};

export type EnsObjectTrack = {
  additional_info?: string;
  child_tracks?: EnsObjectTrack[];
  colour?: string;
  label: string;
  ensembl_object_id?: string;
  support_level?: string | null;
  track_id: string;
};

export type EnsObjectResponse = {
  ensembl_object: EnsObject | {};
};

export type EnsObjectTracksResponse = {
  object_tracks: EnsObjectTrack | {};
};

export type ExampleEnsObjectsData = {
  [key: string]: EnsObject[];
};

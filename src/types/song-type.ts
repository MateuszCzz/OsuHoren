export enum Mode {
  Standard = 0,
  Taiko = 1,
  CatchTheBeat = 2,
  Mania = 3,
}

export enum RankedStatus {
  Unknown = 0,
  Unsubmitted = 1,
  PendingWipGraveyard = 2,
  Ranked = 4,
  Approved = 5,
  Qualified = 6,
  Loved = 7,
}

export interface SongType {
  id: number; artist_ascii: string,
  artist_unicode: string | null,
  title_ascii: string,
  title_unicode: string | null,
  creator: string | null,
  song_source: string | null,
  tags: string | null,
  length: number | null,
  mode: Mode | null,
  ranking_status: RankedStatus | null,
  last_modified: Date | null,
}
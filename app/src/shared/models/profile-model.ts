export interface Profile {
  id: string;
  user_id: string;

  displayName: string;
  avatarUrl?: string;
  bio?: string;

  createdAt: Date;
  updatedAt: Date;
}

export interface ProfileCreate {
  displayName: string;
  avatarUrl?: string;
  bio?: string;
}

export interface ProfileUpdate {
  displayName?: string;
  avatarUrl?: string;
  bio?: string;
}

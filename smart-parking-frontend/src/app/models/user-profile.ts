export interface UserProfile {
    id: number;
    email: string;
    first_name: string;
    last_name: string;
    phone_number: string | null;
    is_active: boolean;
  }
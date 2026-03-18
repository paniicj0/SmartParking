export interface ParkingSpotStatus {
    id: string;
    label: string;
    floor: number | null;
    zone: string;
    spot_type: string;
    is_active: boolean;
    available: boolean;
  }
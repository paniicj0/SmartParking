export interface CreateReservationRequest {
    vehicle_id: number;
    parking_spot_id: string;
    start_time: string;
    end_time: string;
  }
  
  export interface MyReservation {
    id: string;
    parking_spot_id: string;
    parking_spot_label: string;
    zone: string;
    start_time: string;
    end_time: string;
    status: string;
  }
  
  export interface MessageResponse {
    message: string;
  }
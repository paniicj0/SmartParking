import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface EntryRequest {
  gate_id: string;
  vehicle_id: number;
}

export interface EntryResponse {
  message: string;
  session_id: string;
  reservation_id: string;
  parking_spot_id: string;
  entry_time: string;
  status: string;
}

export interface ActiveParkingSessionResponse {
  session_id: string;
  reservation_id: string;
  vehicle_id: number;
  parking_spot_id: string;
  entry_time: string;
  planned_start_time: string;
  planned_end_time: string;
  status: string;
}

export interface ExitRequest {
  gate_id: string;
}

export interface ExitResponse {
  message: string;
  session_id: string;
  entry_time: string;
  exit_time: string;
  duration_minutes: number;
  billable_hours: number;
  total_amount: number;
}

export interface ParkingHistoryItemResponse {
  session_id: string;
  reservation_id: string;
  vehicle_id: number;
  parking_spot_id: string;
  entry_time: string;
  exit_time?: string;
  planned_start_time: string;
  planned_end_time: string;
  status: string;
}

export interface ParkingHistoryResponse {
  items: ParkingHistoryItemResponse[];
}

export interface MessageResponse {
  message: string;
}

@Injectable({
  providedIn: 'root'
})
export class ParkingSessionService {
  private http = inject(HttpClient);

  private baseUrl = 'http://127.0.0.1:8083/parking-sessions';

  enterParking(body: EntryRequest): Observable<EntryResponse> {
    return this.http.post<EntryResponse>(`${this.baseUrl}/entry`, body);
  }

  getActiveSession(): Observable<ActiveParkingSessionResponse> {
    return this.http.get<ActiveParkingSessionResponse>(`${this.baseUrl}/active`);
  }

  exitParking(body: ExitRequest): Observable<ExitResponse> {
    return this.http.post<ExitResponse>(`${this.baseUrl}/exit`, body);
  }

  getHistory(status?: string): Observable<ParkingHistoryResponse> {
    let params = new HttpParams();

    if (status && status.trim() !== '') {
      params = params.set('status', status);
    }

    return this.http.get<ParkingHistoryResponse>(`${this.baseUrl}/history`, { params });
  }
}
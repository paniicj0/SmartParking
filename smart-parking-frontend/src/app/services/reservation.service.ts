import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { ParkingSpotStatus } from '../models/parking-spot-status';
import {
  CreateReservationRequest,
  MessageResponse,
  MyReservation
} from '../models/reservation';

@Injectable({
    providedIn: 'root'
  })
export class ReservationService {
  private baseUrl = 'http://localhost:8082/api/reservations';

  constructor(private http: HttpClient) {}

  private getAuthHeaders(): HttpHeaders {
    const token = localStorage.getItem('token') || '';

    return new HttpHeaders({
      Authorization: `Bearer ${token}`
    });
  }

  getSpotStatuses(startTime: string, endTime: string): Observable<ParkingSpotStatus[]> {
    return this.http.get<ParkingSpotStatus[]>(
      `${this.baseUrl}/spots/status?start_time=${encodeURIComponent(startTime)}&end_time=${encodeURIComponent(endTime)}`,
      { headers: this.getAuthHeaders() }
    );
  }

  createReservation(request: CreateReservationRequest): Observable<any> {
    return this.http.post<any>(this.baseUrl, request, {
      headers: this.getAuthHeaders()
    });
  }

  getMyReservations(): Observable<MyReservation[]> {
    return this.http.get<MyReservation[]>(`${this.baseUrl}/my`, {
      headers: this.getAuthHeaders()
    });
  }

  cancelReservation(id: string): Observable<MessageResponse> {
    return this.http.patch<MessageResponse>(
      `${this.baseUrl}/${id}/cancel`,
      {},
      { headers: this.getAuthHeaders() }
    );
  }
}
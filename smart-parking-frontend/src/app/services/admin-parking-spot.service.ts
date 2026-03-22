import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { CreateParkingSpotRequest, DailyOccupancy, ParkingOccupancy, ParkingSpot, UpdateParkingSpotRequest } from '../pages/admin-parking-spots/admin-parking-spots.component';

@Injectable({
  providedIn: 'root'
})
export class AdminParkingSpotService {
  private http = inject(HttpClient);

  private apiUrl = 'http://127.0.0.1:8082/admin/parking-spots';

  getAllSpots(): Observable<ParkingSpot[]> {
    return this.http.get<ParkingSpot[]>(this.apiUrl);
  }

  createSpot(request: CreateParkingSpotRequest): Observable<ParkingSpot> {
    return this.http.post<ParkingSpot>(this.apiUrl, request);
  }

  updateSpot(id: string, request: UpdateParkingSpotRequest): Observable<ParkingSpot> {
    return this.http.put<ParkingSpot>(`${this.apiUrl}/${id}`, request);
  }

  deleteSpot(id: string): Observable<void> {
    return this.http.delete<void>(`${this.apiUrl}/${id}`);
  }

  getOccupancySummary() {
    return this.http.get<ParkingOccupancy>(`${this.apiUrl}/occupancy`);
  }
  
  getDailyOccupancy() {
    return this.http.get<DailyOccupancy[]>(`${this.apiUrl}/analytics/daily-occupancy`);
  }

  
}
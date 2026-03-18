import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Vehicle } from '../models/vehicle';

@Injectable({
  providedIn: 'root'
})
export class VehicleService {
  private baseUrl = 'http://localhost:3000';

  constructor(private http: HttpClient) {}

  private getAuthHeaders(): HttpHeaders {
    const token = localStorage.getItem('token') || '';

    return new HttpHeaders({
      Authorization: `Bearer ${token}`
    });
  }

  getMyVehicles(): Observable<Vehicle[]> {
    return this.http.get<Vehicle[]>(`${this.baseUrl}/vehicles/me`, {
      headers: this.getAuthHeaders()
    });
  }
}
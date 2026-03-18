import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { UserProfile } from '../models/user-profile';
import { Vehicle } from '../models/vehicle';

export interface LoginRequest {
  email: string;
  password: string;
}

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private apiUrl = 'http://localhost:3000';

  constructor(private http: HttpClient) {}

  login(data: LoginRequest): Observable<any> {
    return this.http.post(`${this.apiUrl}/auth/login`, data);
  }

  register(data: any): Observable<any> {
    return this.http.post(`${this.apiUrl}/auth/register`, data);
  }

  saveToken(token: string): void {
    localStorage.setItem('token', token);
  }

  getToken(): string | null {
    return localStorage.getItem('token');
  }

  logout(): void {
    localStorage.removeItem('token');
  }

  getMe(): Observable<UserProfile> {
    return this.http.get<UserProfile>(`${this.apiUrl}/users/me`);
  }

  activateAccount(token: string) {
    return this.http.get(`${this.apiUrl}/auth/activate?token=${token}`, {
      responseType: 'text'
    });
  }

  updateMe(data: {
    first_name: string;
    last_name: string;
    phone_number: string | null;
  }) {
    return this.http.put<UserProfile>(`${this.apiUrl}/users/me`, data);
  }

  changePassword(data: { old_password: string; new_password: string }) {
    return this.http.put(`${this.apiUrl}/users/me/change-password`, data, {
      responseType: 'text'
    });
  }

  getMyVehicles() {
    return this.http.get<Vehicle[]>(`${this.apiUrl}/vehicles/me`);
  }
  
  createVehicle(data: { license_plate: string; name: string | null }) {
    return this.http.post<Vehicle>(`${this.apiUrl}/vehicles`, data);
  }
  
  updateVehicle(id: number, data: { license_plate: string; name: string | null }) {
    return this.http.put<Vehicle>(`${this.apiUrl}/vehicles/${id}`, data);
  }
  
  deleteVehicle(id: number) {
    return this.http.delete(`${this.apiUrl}/vehicles/${id}`, {
      responseType: 'text'
    });
  }
}
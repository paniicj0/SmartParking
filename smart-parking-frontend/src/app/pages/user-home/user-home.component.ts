import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormBuilder, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';

import { Vehicle } from '../../models/vehicle';
import { ParkingSpotStatus } from '../../models/parking-spot-status';
import { CreateReservationRequest, MyReservation } from '../../models/reservation';

import { VehicleService } from '../../services/vehicle.service';
import { ReservationService } from '../../services/reservation.service';
import { HeaderComponent } from '../../shared/header/header.component';

@Component({
  selector: 'app-user-home',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule,HeaderComponent],
  templateUrl: './user-home.component.html',
  styleUrls: ['./user-home.component.css']
})
export class UserHomeComponent implements OnInit {
  reservationForm!: FormGroup;

  vehicles: Vehicle[] = [];
  spots: ParkingSpotStatus[] = [];
  myReservations: MyReservation[] = [];

  selectedSpot: ParkingSpotStatus | null = null;

  loadingVehicles = false;
  loadingSpots = false;
  loadingReservations = false;
  submittingReservation = false;

  successMessage = '';
  errorMessage = '';

  constructor(
    private fb: FormBuilder,
    private vehicleService: VehicleService,
    private reservationService: ReservationService
  ) {}

  ngOnInit(): void {
    this.initForm();
    this.loadVehicles();
    this.loadMyReservations();
  }

  initForm(): void {
    this.reservationForm = this.fb.group({
      vehicle_id: ['', Validators.required],
      start_time: ['', Validators.required],
      end_time: ['', Validators.required]
    });
  }

  loadVehicles(): void {
    this.loadingVehicles = true;

    this.vehicleService.getMyVehicles().subscribe({
      next: (data) => {
        this.vehicles = data;
        this.loadingVehicles = false;
      },
      error: () => {
        this.errorMessage = 'Greška pri učitavanju vozila.';
        this.loadingVehicles = false;
      }
    });
  }

  loadMyReservations(): void {
    this.loadingReservations = true;

    this.reservationService.getMyReservations().subscribe({
      next: (data) => {
        this.myReservations = data;
        this.loadingReservations = false;
      },
      error: () => {
        this.errorMessage = 'Greška pri učitavanju rezervacija.';
        this.loadingReservations = false;
      }
    });
  }

  checkAvailability(): void {
    this.clearMessages();
    this.selectedSpot = null;
    this.spots = [];

    if (this.reservationForm.invalid) {
      this.errorMessage = 'Popuni sva polja za rezervaciju.';
      return;
    }

    const startTime = this.formatDateTimeForBackend(this.reservationForm.value.start_time);
    const endTime = this.formatDateTimeForBackend(this.reservationForm.value.end_time);

    if (startTime >= endTime) {
      this.errorMessage = 'Početno vreme mora biti pre krajnjeg vremena.';
      return;
    }

    this.loadingSpots = true;

    this.reservationService.getSpotStatuses(startTime, endTime).subscribe({
      next: (data) => {
        this.spots = data;
        this.loadingSpots = false;
      },
      error: (err) => {
        this.errorMessage = err?.error?.message || 'Greška pri proveri dostupnosti mesta.';
        this.loadingSpots = false;
      }
    });
  }

  selectSpot(spot: ParkingSpotStatus): void {
    if (!spot.is_active || !spot.available) {
      return;
    }

    this.selectedSpot = spot;
  }

  reserve(): void {
    this.clearMessages();

    if (this.reservationForm.invalid) {
      this.errorMessage = 'Popuni sva polja.';
      return;
    }

    if (!this.selectedSpot) {
      this.errorMessage = 'Izaberi parking mesto.';
      return;
    }

    const request: CreateReservationRequest = {
      vehicle_id: Number(this.reservationForm.value.vehicle_id),
      parking_spot_id: this.selectedSpot.id,
      start_time: this.formatDateTimeForBackend(this.reservationForm.value.start_time),
      end_time: this.formatDateTimeForBackend(this.reservationForm.value.end_time)
    };

    this.submittingReservation = true;

    this.reservationService.createReservation(request).subscribe({
      next: () => {
        this.successMessage = 'Rezervacija je uspešno kreirana.';
        this.submittingReservation = false;
        this.selectedSpot = null;
        this.spots = [];
        this.reservationForm.patchValue({
          start_time: '',
          end_time: ''
        });
        this.loadMyReservations();
      },
      error: (err) => {
        this.errorMessage = err?.error?.message || 'Greška pri kreiranju rezervacije.';
        this.submittingReservation = false;
      }
    });
  }

  cancelReservation(id: string): void {
    this.clearMessages();

    this.reservationService.cancelReservation(id).subscribe({
      next: (response) => {
        this.successMessage = response.message;
        this.loadMyReservations();
      },
      error: (err) => {
        this.errorMessage = err?.error?.message || 'Greška pri otkazivanju rezervacije.';
      }
    });
  }

  formatDateTimeForBackend(value: string): string {
    return value.length === 16 ? `${value}:00` : value;
  }

  formatDateForDisplay(value: string): string {
    return value.replace('T', ' ');
  }

  clearMessages(): void {
    this.successMessage = '';
    this.errorMessage = '';
  }
}
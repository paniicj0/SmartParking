import { Component, OnInit, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormBuilder, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';

import { Vehicle } from '../../models/vehicle';
import { ParkingSpotStatus } from '../../models/parking-spot-status';
import { CreateReservationRequest, MyReservation } from '../../models/reservation';

import { VehicleService } from '../../services/vehicle.service';
import { ReservationService } from '../../services/reservation.service';
import { HeaderComponent } from '../../shared/header/header.component';
import { ActiveParkingSessionResponse, ExitResponse, ParkingHistoryItemResponse, ParkingSessionService } from '../../services/parking-session.service';
import { FormsModule } from '@angular/forms';
import { HttpErrorResponse } from '@angular/common/http';

@Component({
  selector: 'app-user-home',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule,HeaderComponent, FormsModule],
  templateUrl: './user-home.component.html',
  styleUrls: ['./user-home.component.css']
})
export class UserHomeComponent implements OnInit {
  reservationForm!: FormGroup;
  selectedParkingVehicleId: number | null = null;

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

  private parkingSessionService = inject(ParkingSessionService);

  entryGateId: string = '11111111-1111-1111-1111-111111111111';
  exitGateId: string = '22222222-2222-2222-2222-222222222222';
  selectedVehicleId: number | null = null;

  activeSession: ActiveParkingSessionResponse | null = null;
  historyItems: ParkingHistoryItemResponse[] = [];
  historyStatusFilter: string = '';

  entryMessage: string = '';
  activeMessage: string = '';
  exitMessage: string = '';
  historyMessage: string = '';

  lastExitResult: ExitResponse | null = null;

  minDateTime: string = '';

constructor(
  private fb: FormBuilder,
  private vehicleService: VehicleService,
  private reservationService: ReservationService
  ) {}
  
  ngOnInit(): void {
    const now = new Date();
    this.initForm();
    this.loadVehicles();
    this.loadMyReservations();
    this.loadActiveSession();
    this.loadParkingHistory();
    this.minDateTime = now.toISOString().slice(0, 16);
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


  enterParking(): void {
    this.entryMessage = '';

    if (!this.selectedParkingVehicleId) {
      this.entryMessage = 'Izaberi vozilo.';
      return;
    }
    
    this.parkingSessionService.enterParking({
      gate_id: this.entryGateId,
      vehicle_id: this.selectedParkingVehicleId
    }).subscribe({
      next: (response) => {
        this.entryMessage = response.message;
        this.lastExitResult = null;
        this.loadActiveSession();
        this.loadParkingHistory();
      },
      error: (error: HttpErrorResponse) => {
        this.entryMessage = error.error?.message || 'Greška prilikom evidentiranja ulaska.';
      }
    });
  }

  loadActiveSession(): void {
    this.activeMessage = '';

    this.parkingSessionService.getActiveSession().subscribe({
      next: (response) => {
        this.activeSession = response;
      },
      error: (error: HttpErrorResponse) => {
        this.activeSession = null;
        this.activeMessage = error.error?.message || 'Korisnik nema aktivno parkiranje.';
      }
    });
  }

  exitParking(): void {
    this.exitMessage = '';

    this.parkingSessionService.exitParking({
      gate_id: this.exitGateId
    }).subscribe({
      next: (response) => {
        this.lastExitResult = response;
        this.exitMessage = response.message;
        this.activeSession = null;
        this.loadParkingHistory();
        alert('Račun je generisan i poslat na email!');
      },
      error: (error: HttpErrorResponse) => {
        this.exitMessage = error.error?.message || 'Greška prilikom evidentiranja izlaska.';
      }
    });
  }

  loadParkingHistory(): void {
    this.historyMessage = '';

    const status = this.historyStatusFilter.trim() || undefined;

    this.parkingSessionService.getHistory(status).subscribe({
      next: (response) => {
        this.historyItems = response.items;
      },
      error: (error: HttpErrorResponse) => {
        this.historyItems = [];
        this.historyMessage = error.error?.message || 'Greška prilikom učitavanja istorije.';
      }
    });
  }
}
import { Component, OnInit, inject, signal, AfterViewInit, ViewChild, ElementRef } from '@angular/core';

import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { AdminParkingSpotService } from '../../services/admin-parking-spot.service';
import { Router } from '@angular/router';
import { Chart, registerables } from 'chart.js';


export interface ParkingSpot {
  id: string;
  label: string;
  floor: number | null;
  zone: string;
  spot_type: string;
  is_active: boolean;
}

export interface CreateParkingSpotRequest {
  label: string;
  floor: number | null;
  zone: string;
  spot_type: string;
  is_active: boolean;
}

export interface UpdateParkingSpotRequest {
  label: string;
  floor: number | null;
  zone: string;
  spot_type: string;
  is_active: boolean;
}

export interface ParkingOccupancy {
  total_spots: number;
  active_spots: number;
  inactive_spots: number;
  reserved_spots: number;
  free_spots: number;
  occupancy_percentage: number;
}

export interface DailyOccupancy {
  date: string;
  occupied_count: number;
}
Chart.register(...registerables);


@Component({
  selector: 'app-admin-parking-spots',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './admin-parking-spots.component.html',
  styleUrls: ['./admin-parking-spots.component.css']
})

export class AdminParkingSpotsComponent implements OnInit, AfterViewInit {
  private parkingSpotService = inject(AdminParkingSpotService);
  private router = inject(Router);

  @ViewChild('occupancyChart') occupancyChartRef!: ElementRef<HTMLCanvasElement>;
  chart: Chart | null = null;

  spots = signal<ParkingSpot[]>([]);
  occupancy = signal<ParkingOccupancy | null>(null);
  dailyOccupancy = signal<DailyOccupancy[]>([]);

  errorMessage = signal('');
  successMessage = signal('');

  isEditMode = false;
  selectedSpotId: string | null = null;

  formData: CreateParkingSpotRequest = {
    label: '',
    floor: null,
    zone: '',
    spot_type: '',
    is_active: true
  };

  ngOnInit(): void {
    this.loadAllData();
  }

  ngAfterViewInit(): void {
    this.renderChart();
  }

  loadAllData(): void {
    this.loadSpots();
    this.loadOccupancy();
    this.loadDailyOccupancy();
  }

  loadSpots(): void {
    this.parkingSpotService.getAllSpots().subscribe({
      next: (data) => this.spots.set(data),
      error: () => this.errorMessage.set('Greška pri učitavanju parking mesta.')
    });
  }

  loadOccupancy(): void {
    this.parkingSpotService.getOccupancySummary().subscribe({
      next: (data) => this.occupancy.set(data),
      error: () => this.errorMessage.set('Greška pri učitavanju zauzetosti.')
    });
  }

  loadDailyOccupancy(): void {
    this.parkingSpotService.getDailyOccupancy().subscribe({
      next: (data) => {
        this.dailyOccupancy.set(data);
        this.renderChart();
      },
      error: () => this.errorMessage.set('Greška pri učitavanju analitike.')
    });
  }

  renderChart(): void {
    if (!this.occupancyChartRef) return;

    const data = this.dailyOccupancy();
    const labels = data.map(x => x.date);
    const values = data.map(x => x.occupied_count);

    if (this.chart) {
      this.chart.destroy();
    }

    this.chart = new Chart(this.occupancyChartRef.nativeElement, {
      type: 'bar',
      data: {
        labels,
        datasets: [
          {
            label: 'Dnevna zauzetost',
            data: values
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false
      }
    });
  }

  resetForm(): void {
    this.formData = {
      label: '',
      floor: null,
      zone: '',
      spot_type: '',
      is_active: true
    };
    this.isEditMode = false;
    this.selectedSpotId = null;
  }

  onSubmit(): void {
    this.errorMessage.set('');
    this.successMessage.set('');

    if (this.isEditMode && this.selectedSpotId) {
      const updateRequest: UpdateParkingSpotRequest = { ...this.formData };

      this.parkingSpotService.updateSpot(this.selectedSpotId, updateRequest).subscribe({
        next: () => {
          this.successMessage.set('Parking mesto je uspešno izmenjeno.');
          this.loadAllData();
          this.resetForm();
        },
        error: (err) => {
          this.errorMessage.set(err?.error || 'Greška pri izmeni parking mesta.');
        }
      });
    } else {
      this.parkingSpotService.createSpot(this.formData).subscribe({
        next: () => {
          this.successMessage.set('Parking mesto je uspešno kreirano.');
          this.loadAllData();
          this.resetForm();
        },
        error: (err) => {
          this.errorMessage.set(err?.error || 'Greška pri kreiranju parking mesta.');
        }
      });
    }
  }

  onEdit(spot: ParkingSpot): void {
    this.isEditMode = true;
    this.selectedSpotId = spot.id;
    this.formData = {
      label: spot.label,
      floor: spot.floor,
      zone: spot.zone,
      spot_type: spot.spot_type,
      is_active: spot.is_active
    };
    this.successMessage.set('');
    this.errorMessage.set('');
  }

  onDelete(id: string): void {
    const confirmed = confirm('Da li sigurno želiš da obrišeš parking mesto?');
    if (!confirmed) return;

    this.parkingSpotService.deleteSpot(id).subscribe({
      next: () => {
        this.successMessage.set('Parking mesto je uspešno obrisano.');
        this.loadAllData();

        if (this.selectedSpotId === id) {
          this.resetForm();
        }
      },
      error: (err) => {
        this.errorMessage.set(err?.error || 'Greška pri brisanju parking mesta.');
      }
    });
  }

  logout(): void {
    localStorage.removeItem('token');
    this.router.navigate(['/login']);
  }
}
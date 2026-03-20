import { Component, OnInit, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { AuthService } from '../../services/auth.service';
import { UserProfile } from '../../models/user-profile';
import { Vehicle } from '../../models/vehicle';
import { HeaderComponent } from '../../shared/header/header.component';
import { ParkingHistoryItemResponse, ParkingSessionService } from '../../services/parking-session.service';
import { HttpErrorResponse } from '@angular/common/http';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [CommonModule, FormsModule, HeaderComponent, ReactiveFormsModule],
  templateUrl: './profile.component.html',
  styleUrl: './profile.component.css'
})
export class ProfileComponent implements OnInit {
  user: UserProfile | null = null;
  loading = true;
  errorMessage = '';
  successMessage = '';
  editMode = false;

  formData = {
    first_name: '',
    last_name: '',
    phone_number: '' as string | null
  };

  passwordData = {
    old_password: '',
    new_password: '',
    confirm_password: ''
  };
  
  passwordErrorMessage = '';
  passwordSuccessMessage = '';
  passwordEditMode = false;

  vehicles: Vehicle[] = [];
  vehicleSectionVisible = false;
  vehicleFormVisible = false;
  vehicleEditId: number | null = null;

  vehicleFormData = {
    license_plate: '',
    name: ''
  };

  vehicleErrorMessage = '';
  vehicleSuccessMessage = '';

  historyItems: ParkingHistoryItemResponse[] = [];
  historyStatusFilter: string = '';
  private parkingSessionService = inject(ParkingSessionService);
  
  historyMessage: string = '';
  constructor(private authService: AuthService) {}

  ngOnInit(): void {
    this.loadProfile();
    this.loadParkingHistory();
  }

  loadProfile(): void {
    this.loading = true;
    this.authService.getMe().subscribe({
      next: (data) => {
        this.user = data;
        this.formData = {
          first_name: data.first_name,
          last_name: data.last_name,
          phone_number: data.phone_number
        };
        this.loading = false;
      },
      error: (err) => {
        console.error(err);
        this.errorMessage = 'Neuspešno učitavanje profila.';
        this.loading = false;
      }
    });
  }

  enableEdit(): void {
    this.editMode = true;
    this.successMessage = '';
    this.errorMessage = '';

    if (this.user) {
      this.formData = {
        first_name: this.user.first_name,
        last_name: this.user.last_name,
        phone_number: this.user.phone_number
      };
    }
  }

  cancelEdit(): void {
    this.editMode = false;
    this.errorMessage = '';
    this.successMessage = '';

    if (this.user) {
      this.formData = {
        first_name: this.user.first_name,
        last_name: this.user.last_name,
        phone_number: this.user.phone_number
      };
    }
  }

  saveChanges(): void {
    this.errorMessage = '';
    this.successMessage = '';

    if (!this.formData.first_name.trim() || !this.formData.last_name.trim()) {
      this.errorMessage = 'Ime i prezime su obavezni.';
      return;
    }

    const payload = {
      first_name: this.formData.first_name.trim(),
      last_name: this.formData.last_name.trim(),
      phone_number: this.formData.phone_number?.trim() || null
    };

    this.authService.updateMe(payload).subscribe({
      next: (updatedUser) => {
        this.user = updatedUser;
        this.editMode = false;
        this.successMessage = 'Profil je uspešno ažuriran.';
      },
      error: (err) => {
        console.error(err);
        this.errorMessage = err?.error || 'Greška pri ažuriranju profila.';
      }
    });
  }

  changePassword(): void {
    this.passwordErrorMessage = '';
    this.passwordSuccessMessage = '';
  
    if (!this.passwordData.old_password.trim() || !this.passwordData.new_password.trim()) {
      this.passwordErrorMessage = 'Sva polja za lozinku su obavezna.';
      return;
    }
  
    if (this.passwordData.new_password.length < 7) {
      this.passwordErrorMessage = 'Nova lozinka mora imati najmanje 7 karaktera.';
      return;
    }
  
    if (this.passwordData.new_password !== this.passwordData.confirm_password) {
      this.passwordErrorMessage = 'Nova lozinka i potvrda lozinke se ne poklapaju.';
      return;
    }
  
    this.authService.changePassword({
      old_password: this.passwordData.old_password,
      new_password: this.passwordData.new_password
    }).subscribe({
      next: () => {
        this.passwordSuccessMessage = 'Lozinka je uspešno promenjena.';
        this.passwordData = {
          old_password: '',
          new_password: '',
          confirm_password: ''
        };
        this.passwordEditMode = false;
      },
      error: (err) => {
        console.error(err);
        this.passwordErrorMessage = err?.error || 'Greška pri promeni lozinke.';
      }
    });
  }

  enablePasswordEdit(): void {
    this.passwordEditMode = true;
    this.passwordErrorMessage = '';
    this.passwordSuccessMessage = '';
  }
  
  cancelPasswordEdit(): void {
    this.passwordEditMode = false;
    this.passwordErrorMessage = '';
    this.passwordSuccessMessage = '';
    this.passwordData = {
      old_password: '',
      new_password: '',
      confirm_password: ''
    };
  }

  toggleVehicleSection(): void {
    console.log('Klik na vozila');
    this.vehicleSectionVisible = !this.vehicleSectionVisible;
  
    if (this.vehicleSectionVisible) {
      console.log('Pozivam loadVehicles');
      this.loadVehicles();
    }
  }

  openVehicleForm(): void {
    this.vehicleFormVisible = true;
    this.vehicleEditId = null;
    this.vehicleFormData = {
      license_plate: '',
      name: ''
    };
    this.vehicleErrorMessage = '';
    this.vehicleSuccessMessage = '';
  }

  cancelVehicleForm(): void {
    this.vehicleFormVisible = false;
    this.vehicleEditId = null;
    this.vehicleFormData = {
      license_plate: '',
      name: ''
    };
  }

  startEditVehicle(vehicle: Vehicle): void {
    this.vehicleFormVisible = true;
    this.vehicleEditId = vehicle.id;
    this.vehicleFormData = {
      license_plate: vehicle.license_plate,
      name: vehicle.name || ''
    };
  }

  loadVehicles(): void {
    console.log('Usla u loadVehicles');
    this.vehicleErrorMessage = '';
  
    this.authService.getMyVehicles().subscribe({
      next: (data) => {
        console.log('VOZILA SA BACKENDA:', data);
        this.vehicles = data;
      },
      error: (err) => {
        console.error(err);
        this.vehicleErrorMessage = 'Neuspešno učitavanje vozila.';
      }
    });
  }

  saveVehicle(): void {
    this.vehicleErrorMessage = '';
    this.vehicleSuccessMessage = '';
  
    const payload = {
      license_plate: this.vehicleFormData.license_plate.trim(),
      name: this.vehicleFormData.name.trim() || null
    };
  
    if (!payload.license_plate) {
      this.vehicleErrorMessage = 'Tablica je obavezna.';
      return;
    }
  
    if (this.vehicleEditId) {
      this.authService.updateVehicle(this.vehicleEditId, payload).subscribe({
        next: () => {
          this.vehicleSuccessMessage = 'Vozilo je uspešno izmenjeno.';
          this.vehicleFormVisible = false;
          this.vehicleEditId = null;
          this.vehicleFormData = {
            license_plate: '',
            name: ''
          };
          this.loadVehicles();
        },
        error: (err) => {
          console.error(err);
          this.vehicleErrorMessage = err?.error || 'Greška pri izmeni vozila.';
        }
      });
    } else {
      this.authService.createVehicle(payload).subscribe({
        next: () => {
          this.vehicleSuccessMessage = 'Vozilo je uspešno dodato.';
          this.vehicleFormVisible = false;
          this.vehicleFormData = {
            license_plate: '',
            name: ''
          };
          this.loadVehicles();
        },
        error: (err) => {
          console.error(err);
          this.vehicleErrorMessage = err?.error || 'Greška pri dodavanju vozila.';
        }
      });
    }
  }

  deleteVehicle(id: number): void {
    this.vehicleErrorMessage = '';
    this.vehicleSuccessMessage = '';
  
    this.authService.deleteVehicle(id).subscribe({
      next: () => {
        this.vehicleSuccessMessage = 'Vozilo je uspešno obrisano.';
        this.loadVehicles();
      },
      error: (err) => {
        console.error(err);
        this.vehicleErrorMessage = err?.error || 'Greška pri brisanju vozila.';
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
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../../services/auth.service';
import { Router, RouterLink } from '@angular/router';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [FormsModule, RouterLink],
  templateUrl: './register.component.html',
  styleUrl: './register.component.css'
})
export class RegisterComponent {
  email = '';
  password = '';
  confirmPassword = '';
  first_name = '';
  last_name = '';
  phone_number = '';

  errorMessage = '';
  successMessage = '';

  constructor(private authService: AuthService, private router: Router) {}

  onSubmit() {
    this.errorMessage = '';
    this.successMessage = '';

    if (this.password !== this.confirmPassword) {
      this.errorMessage = 'Lozinke se ne poklapaju.';
      return;
    }

    const registerData = {
      email: this.email,
      password: this.password,
      first_name: this.first_name,
      last_name: this.last_name,
      phone_number: this.phone_number
    };

    this.authService.register(registerData).subscribe({
      next: (response) => {
        console.log('Registracija uspešna:', response);
        this.successMessage =
          'Registracija uspešna. Proverite email i aktivirajte nalog u roku od 24h.';
      },
      error: (error) => {
        console.error('Greška pri registraciji:', error);
        this.errorMessage =
          error?.error || 'Došlo je do greške prilikom registracije.';
      }
    });
  }
}
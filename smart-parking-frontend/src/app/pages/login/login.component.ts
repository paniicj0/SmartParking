import { RouterLink } from '@angular/router';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../../services/auth.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule, RouterLink],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {
  email = '';
  password = '';

  constructor(private authService: AuthService, private router: Router) {}

  onSubmit() {
    const loginData = {
      email: this.email,
      password: this.password
    };

    this.authService.login(loginData).subscribe({
      next: (response) => {
        console.log('Uspešan login:', response);
        this.router.navigate(['/user-home']);

        this.authService.saveToken(response.token);
        console.log('Token sačuvan u localStorage');

        this.authService.getMe().subscribe({
          next: (userResponse) => {
            console.log('Podaci ulogovanog korisnika:', userResponse);
          },
          error: (userError) => {
            console.error('Greška pri /users/me:', userError);
          }
        });
      },
      error: (error) => {
        console.error('Greška pri loginu:', error);
      }
    });
  }
}
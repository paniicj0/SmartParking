import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { AuthService } from '../../services/auth.service';

@Component({
  selector: 'app-activate-account',
  standalone: true,
  imports: [RouterLink],
  templateUrl: './activate-account.component.html',
  styleUrl: './activate-account.component.css'
})
export class ActivateAccountComponent implements OnInit {
  message = 'Aktivacija naloga je u toku...';
  isSuccess = false;

  constructor(
    private route: ActivatedRoute,
    private authService: AuthService
  ) {}

  ngOnInit(): void {
    const token = this.route.snapshot.queryParamMap.get('token');

    if (!token) {
      this.message = 'Aktivacioni token nedostaje.';
      this.isSuccess = false;
      return;
    }

    this.authService.activateAccount(token).subscribe({
      next: (response) => {
        this.message = response;
        this.isSuccess = true;
      },
      error: (error) => {
        this.message = error?.error || 'Aktivacija nije uspela.';
        this.isSuccess = false;
      }
    });
  }
}
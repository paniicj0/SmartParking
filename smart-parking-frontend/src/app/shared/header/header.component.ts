import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterModule } from '@angular/router';

@Component({
  selector: 'app-header',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './header.component.html',
  styleUrls: ['./header.component.css']
})
export class HeaderComponent {
  constructor(private router: Router) {}

  goToProfile(): void {
    this.router.navigate(['/profile']);
  }

  logout(): void {
    localStorage.removeItem('token');
    this.router.navigate(['/login']);
  }
}
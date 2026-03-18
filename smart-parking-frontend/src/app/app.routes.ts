import { Routes } from '@angular/router';
import { LoginComponent } from './pages/login/login.component';
import { RegisterComponent } from './pages/register/register.component';
import { ActivateAccountComponent } from './pages/activate-account/activate-account.component';
import { ProfileComponent } from './pages/profile/profile.component';

export const routes: Routes = [ 
    
    { path: 'login', component: LoginComponent },
    { path: 'register', component: RegisterComponent },
    { path: 'activate', component: ActivateAccountComponent },
    { path: 'profile', component: ProfileComponent },
    { path: '', redirectTo: 'login', pathMatch: 'full' }
];
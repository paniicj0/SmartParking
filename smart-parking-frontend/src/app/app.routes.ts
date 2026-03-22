import { Routes } from '@angular/router';
import { LoginComponent } from './pages/login/login.component';
import { RegisterComponent } from './pages/register/register.component';
import { ActivateAccountComponent } from './pages/activate-account/activate-account.component';
import { ProfileComponent } from './pages/profile/profile.component';
import { UserHomeComponent } from './pages/user-home/user-home.component';
import { MyInvoicesComponent } from './pages/my-invoices/my-invoices.component';
import { AdminParkingSpotsComponent } from './pages/admin-parking-spots/admin-parking-spots.component';

export const routes: Routes = [ 
    
    { path: 'login', component: LoginComponent },
    { path: 'register', component: RegisterComponent },
    { path: 'activate', component: ActivateAccountComponent },
    { path: 'profile', component: ProfileComponent },
    { path: 'user-home', component: UserHomeComponent },
    { path: 'invoices', component: MyInvoicesComponent},
    { path: 'admin/parking-spots', component: AdminParkingSpotsComponent},
    { path: '', redirectTo: 'login', pathMatch: 'full' }
];
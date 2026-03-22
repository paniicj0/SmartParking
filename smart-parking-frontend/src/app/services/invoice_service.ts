import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface Invoice {
  id: number;
  session_id: string;
  invoice_number: string;
  qr_code_data: string;
  amount: number;
  status: string;
  issued_at: string;
  paid_at?: string | null;
}

@Injectable({
  providedIn: 'root'
})
export class InvoiceService {
  private baseUrl = 'http://localhost:8084';

  constructor(private http: HttpClient) {}

  getInvoicesByUser(userId: number): Observable<Invoice[]> {
    return this.http.get<Invoice[]>(`${this.baseUrl}/invoices/user/${userId}`);
  }

  payInvoice(invoiceId: number): Observable<Invoice> {
    return this.http.patch<Invoice>(`${this.baseUrl}/invoices/${invoiceId}/pay`, {});
  }
}
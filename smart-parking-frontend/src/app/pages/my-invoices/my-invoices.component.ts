import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import jsPDF from 'jspdf';
import QRCode from 'qrcode';
import { Invoice, InvoiceService } from '../../services/invoice_service';
import { HeaderComponent } from '../../shared/header/header.component';

@Component({
  selector: 'app-my-invoices',
  standalone: true,
  imports: [CommonModule, HeaderComponent],
  templateUrl: './my-invoices.component.html',
  styleUrls: ['./my-invoices.component.css']
})
export class MyInvoicesComponent implements OnInit {
  invoices: Invoice[] = [];
  loading = false;
  errorMessage = '';
  successMessage = '';
  payingInvoiceId: number | null = null;
  downloadingInvoiceId: number | null = null;

  constructor(
    private invoiceService: InvoiceService,
    private router: Router
  ) {}

  ngOnInit(): void {
    this.loadInvoices();
  }

  loadInvoices(): void {
    const userId = this.getUserId();

    if (!userId) {
      this.errorMessage = 'Korisnik nije prijavljen.';
      return;
    }

    this.loading = true;
    this.errorMessage = '';
    this.successMessage = '';

    this.invoiceService.getInvoicesByUser(userId).subscribe({
      next: (data) => {
        this.invoices = data;
        this.loading = false;
      },
      error: (err) => {
        console.error('Greška pri učitavanju računa:', err);
        this.errorMessage = 'Greška pri učitavanju računa.';
        this.loading = false;
      }
    });
  }

  payInvoice(invoiceId: number): void {
    this.payingInvoiceId = invoiceId;
    this.errorMessage = '';
    this.successMessage = '';

    this.invoiceService.payInvoice(invoiceId).subscribe({
      next: () => {
        this.successMessage = 'Račun je uspešno označen kao plaćen.';
        this.payingInvoiceId = null;
        this.loadInvoices();
      },
      error: (err) => {
        console.error('Greška pri plaćanju računa:', err);
        this.errorMessage = 'Greška pri simulaciji plaćanja.';
        this.payingInvoiceId = null;
      }
    });
  }

  async downloadPdf(invoice: Invoice): Promise<void> {
    try {
      this.downloadingInvoiceId = invoice.id;

      const doc = new jsPDF();

      const qrDataUrl = await QRCode.toDataURL(invoice.qr_code_data, {
        width: 200,
        margin: 1
      });

      doc.setFontSize(18);
      doc.text('Racun za parking', 20, 20);

      doc.setFontSize(12);
      doc.text(`Broj racuna: ${invoice.invoice_number}`, 20, 35);
      doc.text(`Status: ${invoice.status}`, 20, 45);
      doc.text(`Iznos: ${invoice.amount.toFixed(2)} RSD`, 20, 55);
      doc.text(`Datum izdavanja: ${this.formatDate(invoice.issued_at)}`, 20, 65);
      doc.text(`Datum placanja: ${this.formatDate(invoice.paid_at)}`, 20, 75);

      doc.text('QR kod za identifikaciju racuna:', 20, 95);

      doc.addImage(qrDataUrl, 'PNG', 20, 100, 60, 60);

      doc.setFontSize(10);


      doc.save(`${invoice.invoice_number}.pdf`);
    } catch (error) {
      console.error('Greška pri generisanju PDF-a:', error);
      this.errorMessage = 'Greška pri preuzimanju PDF računa.';
    } finally {
      this.downloadingInvoiceId = null;
    }
  }

  goBack(): void {
    this.router.navigate(['/user-home']);
  }

  isPaid(status: string): boolean {
    return status.toLowerCase() === 'paid';
  }

  formatDate(date: string | null | undefined): string {
    if (!date) return '-';

    const parsed = new Date(date);
    if (isNaN(parsed.getTime())) {
      return date;
    }

    return parsed.toLocaleString('sr-RS');
  }

  private getUserId(): number | null {
    const token = localStorage.getItem('token');
    if (!token) return null;

    try {
      const payload = token.split('.')[1]
        .replace(/-/g, '+')
        .replace(/_/g, '/');

      const decodedPayload = JSON.parse(atob(payload));
      const userId = Number(decodedPayload.sub);

      return Number.isNaN(userId) ? null : userId;
    } catch (error) {
      console.error('Greška pri dekodiranju tokena:', error);
      return null;
    }
  }
}
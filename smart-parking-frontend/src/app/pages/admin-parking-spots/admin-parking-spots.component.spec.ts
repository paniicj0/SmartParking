import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AdminParkingSpotsComponent } from './admin-parking-spots.component';

describe('AdminParkingSpotsComponent', () => {
  let component: AdminParkingSpotsComponent;
  let fixture: ComponentFixture<AdminParkingSpotsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AdminParkingSpotsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AdminParkingSpotsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

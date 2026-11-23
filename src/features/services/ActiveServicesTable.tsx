import type { ServiceRecordDto } from "../../shared/api";
import {
  formatExactMoney,
  formatMoney,
  formatNotes,
  formatServiceStatus,
} from "./formatServiceDisplay";

type ActiveServicesTableProps = {
  services: ServiceRecordDto[];
};

export function ActiveServicesTable({ services }: ActiveServicesTableProps) {
  return (
    <div className="services-table-scroll">
      <table className="services-table" aria-label="Servicios activos">
        <thead>
          <tr>
            <th scope="col">Nombre</th>
            <th scope="col">Moneda</th>
            <th scope="col">Costo total</th>
            <th scope="col">Cantidad de personas</th>
            <th scope="col">Mi parte</th>
            <th scope="col">Notas</th>
            <th scope="col">Estado</th>
          </tr>
        </thead>
        <tbody>
          {services.map((service) => (
            <tr key={service.id}>
              <th scope="row">{service.name}</th>
              <td>{service.totalCost.currencyCode}</td>
              <td className="services-table__number">
                {formatMoney(service.totalCost)}
              </td>
              <td className="services-table__number">
                {service.sharedPeopleCount}
              </td>
              <td className="services-table__number">
                {formatExactMoney(service.myShare)}
              </td>
              <td>{formatNotes(service.notes)}</td>
              <td>{formatServiceStatus(service.status)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

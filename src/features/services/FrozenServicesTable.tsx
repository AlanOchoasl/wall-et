import type { ServiceRecordDto } from "../../shared/api";
import {
  formatExactMoney,
  formatMoney,
  formatNotes,
  formatServiceStatus,
} from "./formatServiceDisplay";

type FrozenServicesTableProps = {
  services: ServiceRecordDto[];
  reactivatingId: string | null;
  onReactivate: (service: ServiceRecordDto) => void;
};

export function FrozenServicesTable({
  services,
  reactivatingId,
  onReactivate,
}: FrozenServicesTableProps) {
  return (
    <div className="services-table-scroll">
      <table
        className="services-table"
        aria-label="Servicios congelados"
        aria-busy={reactivatingId !== null ? "true" : undefined}
      >
        <thead>
          <tr>
            <th scope="col">Nombre</th>
            <th scope="col">Moneda</th>
            <th scope="col">Costo total</th>
            <th scope="col">Cantidad de personas</th>
            <th scope="col">Mi parte</th>
            <th scope="col">Notas</th>
            <th scope="col">Estado</th>
            <th scope="col">Acciones</th>
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
              <td>
                <button
                  type="button"
                  className="button button--secondary button--compact"
                  aria-label={`Reactivar ${service.name}`}
                  aria-busy={reactivatingId === service.id ? "true" : undefined}
                  disabled={reactivatingId !== null}
                  onClick={() => onReactivate(service)}
                >
                  {reactivatingId === service.id ? "Reactivando…" : "Reactivar"}
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

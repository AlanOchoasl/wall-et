import type { ServiceTotalsDto } from "../../shared/api";
import { formatExactMoney, formatMoney } from "./formatServiceDisplay";

type ServiceTotalsSummaryProps = {
  totals: ServiceTotalsDto | null;
  error: string | null;
};

export function ServiceTotalsSummary({
  totals,
  error,
}: ServiceTotalsSummaryProps) {
  return (
    <section className="service-totals" aria-label="Totales de servicios activos">
      {error ? (
        <p role="alert">No se pudieron cargar los totales: {error}</p>
      ) : null}

      {!error && !totals ? <p>Cargando totales…</p> : null}

      {!error && totals ? (
        <>
          <dl className="service-totals__group">
            <dt className="service-totals__label">
              Costo total de servicios activos
            </dt>
            <dd className="service-totals__value">
              {formatMoney(totals.ars.totalCost)}
            </dd>
            <dd className="service-totals__value">
              {formatMoney(totals.usd.totalCost)}
            </dd>
          </dl>

          <dl className="service-totals__group">
            <dt className="service-totals__label">Mi gasto</dt>
            <dd className="service-totals__value">
              {formatExactMoney(totals.ars.userShare)}
            </dd>
            <dd className="service-totals__value">
              {formatExactMoney(totals.usd.userShare)}
            </dd>
          </dl>
        </>
      ) : null}
    </section>
  );
}

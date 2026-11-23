import type { InvestmentSummaryDto } from "../../shared/api";
import { formatExactMoney, formatMoney } from "../../shared/moneyDisplay";
import {
  formatCoverageRoutes,
  formatFinancialRoutes,
  formatTna,
} from "./formatInvestmentDisplay";

export type InvestmentsTableProps = {
  investments: InvestmentSummaryDto[];
  loading?: boolean;
  error?: string | null;
};

const COLUMN_HEADERS = [
  "Mes",
  "Moneda",
  "Capital invertido",
  "TNA",
  "Plazo (días)",
  "Impuestos",
  "Rendimiento bruto",
  "Gastos del mes",
  "Rendimiento neto",
  "Cobertura",
] as const;

export function InvestmentsTable({
  investments,
  loading = false,
  error = null,
}: InvestmentsTableProps) {
  if (error) {
    return (
      <p role="alert">
        No se pudieron cargar las inversiones: {error}
      </p>
    );
  }

  if (loading) {
    return <p role="status">Cargando inversiones…</p>;
  }

  if (investments.length === 0) {
    return <p>No hay inversiones para el mes seleccionado</p>;
  }

  return (
    <div className="investments-table-scroll">
      <table className="investments-table" aria-label="Inversiones">
        <thead>
          <tr>
            {COLUMN_HEADERS.map((header) => (
              <th key={header} scope="col">
                {header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {investments.map(({ record, derived }) => (
            <tr key={record.id}>
              <th scope="row">{record.month}</th>
              <td>{record.currencyCode}</td>
              <td className="investments-table__number">
                {formatMoney(record.capital)}
              </td>
              <td className="investments-table__number">
                {formatTna(record.tna)}
              </td>
              <td className="investments-table__number">{record.termDays}</td>
              <td className="investments-table__number">
                {formatMoney(record.taxes)}
              </td>
              <td className="investments-table__number">
                {formatFinancialRoutes(derived.grossYield, formatExactMoney)}
              </td>
              <td className="investments-table__number">
                {formatExactMoney(derived.monthlyExpenses)}
              </td>
              <td className="investments-table__number">
                {formatFinancialRoutes(derived.netYield, formatExactMoney)}
              </td>
              <td className="investments-table__number">
                {formatCoverageRoutes(derived.coverage)}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

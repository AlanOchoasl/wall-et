import type {
  CalculationStatusDto,
  CoverageOutcomeDto,
  FinancialResultDto,
  FinancialRoutesDto,
} from "../../shared/api";
import { formatExactMoney } from "../../shared/moneyDisplay";

const STATUS_LABELS: Record<CalculationStatusDto, string> = {
  disponible: "Disponible",
  provisional: "Provisional",
  no_disponible: "No disponible",
};

const ROUTE_LABELS = {
  definitive: "Definitivo",
  provisional: "Provisional",
} as const;

export function formatTna(tna: string): string {
  return `${tna} %`;
}

export function formatFinancialResult<T>(
  result: FinancialResultDto<T>,
  formatValue: (value: T) => string,
): string {
  if (result.status === "no_disponible" || result.value === null) {
    return "No disponible";
  }

  return `${formatValue(result.value)} (${STATUS_LABELS[result.status].toLowerCase()})`;
}

export function formatFinancialRoutes<T>(
  routes: FinancialRoutesDto<T>,
  formatValue: (value: T) => string,
): string {
  return [
    `${ROUTE_LABELS.definitive}: ${formatFinancialResult(routes.definitive, formatValue)}`,
    `${ROUTE_LABELS.provisional}: ${formatFinancialResult(routes.provisional, formatValue)}`,
  ].join(" · ");
}

export function formatCoverageOutcome(outcome: CoverageOutcomeDto): string {
  if (outcome.outcome === "covered") {
    return "Cubierto";
  }

  return `Déficit: ${formatExactMoney(outcome.amount)}`;
}

export function formatCoverageRoutes(
  routes: FinancialRoutesDto<CoverageOutcomeDto>,
): string {
  return formatFinancialRoutes(routes, formatCoverageOutcome);
}

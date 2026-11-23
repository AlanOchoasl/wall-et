import type {
  CalculationReasonDto,
  CoverageOutcomeDto,
  FinancialResultDto,
  FinancialRoutesDto,
} from "../../shared/api";
import { formatExactMoney } from "../../shared/moneyDisplay";

const AVAILABLE_LABEL = "disponible";
const UNAVAILABLE_LABEL = "no disponible";
const GENERIC_PROVISIONAL_LABEL = "provisional";

const PROVISIONAL_REASON_LABELS: Partial<Record<CalculationReasonDto, string>> = {
  approved_demo_pv01: "PV-01 · Provisional / demostración",
  approved_demo_pv02: "PV-02 · Provisional / demostración",
  approved_demo_pv03: "PV-03 · Provisional / demostración",
};

const ROUTE_LABELS = {
  definitive: "Definitivo",
  provisional: "Provisional",
} as const;

export function formatTna(tna: string): string {
  return `${tna} %`;
}

function formatStatusLabel<T>(result: FinancialResultDto<T>): string {
  if (result.status === "no_disponible") {
    return UNAVAILABLE_LABEL;
  }

  if (result.status === "provisional") {
    const reasonLabel = result.reason
      ? PROVISIONAL_REASON_LABELS[result.reason]
      : undefined;

    return reasonLabel ?? GENERIC_PROVISIONAL_LABEL;
  }

  return AVAILABLE_LABEL;
}

export function formatFinancialResult<T>(
  result: FinancialResultDto<T>,
  formatValue: (value: T) => string,
): string {
  const label = formatStatusLabel(result);

  if (label === UNAVAILABLE_LABEL || result.value === null) {
    return UNAVAILABLE_LABEL;
  }

  return `${formatValue(result.value)} (${label})`;
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
  switch (outcome.outcome) {
    case "covered":
      return "Cubierto";
    case "deficit":
      return `Déficit de: ${formatExactMoney(outcome.amount)}`;
    case "incompatible_currencies":
      return "Monedas incompatibles";
    case "zero_expenses":
      return "Gastos en cero";
  }
}

function hasNoVerdict(outcome: CoverageOutcomeDto): boolean {
  return (
    outcome.outcome === "incompatible_currencies" || outcome.outcome === "zero_expenses"
  );
}

function formatCoverageResult(
  result: FinancialResultDto<CoverageOutcomeDto>,
): string {
  if (result.status === "no_disponible" || result.value === null) {
    return UNAVAILABLE_LABEL;
  }

  const text = formatCoverageOutcome(result.value);

  if (hasNoVerdict(result.value) && result.status === "disponible") {
    return text;
  }

  return `${text} (${formatStatusLabel(result)})`;
}

export function formatCoverageRoutes(
  routes: FinancialRoutesDto<CoverageOutcomeDto>,
): string {
  return [
    `${ROUTE_LABELS.definitive}: ${formatCoverageResult(routes.definitive)}`,
    `${ROUTE_LABELS.provisional}: ${formatCoverageResult(routes.provisional)}`,
  ].join(" · ");
}

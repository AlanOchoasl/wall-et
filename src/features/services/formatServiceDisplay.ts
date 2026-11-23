import type {
  CurrencyCode,
  ExactMoneyDto,
  MoneyDto,
  ServiceStatusDto,
} from "../../shared/api";

const DIGITS_PATTERN = /^\d+$/;

function normalizeDigits(value: string, field: string): string {
  if (!DIGITS_PATTERN.test(value)) {
    throw new Error(`${field} no es un valor entero válido.`);
  }

  return value.replace(/^0+(?=\d)/, "");
}

export function formatMinorUnits(
  minorUnits: string,
  currencyCode: CurrencyCode,
): string {
  const digits = normalizeDigits(minorUnits, "minorUnits");
  const padded = digits.padStart(3, "0");
  const integerPart = padded.slice(0, -2);
  const fractionPart = padded.slice(-2);

  return `${currencyCode} ${integerPart},${fractionPart}`;
}

export function formatExactMinor(
  numeratorMinor: string,
  denominator: string,
  currencyCode: CurrencyCode,
): string {
  const numerator = normalizeDigits(numeratorMinor, "numeratorMinor");
  const divisor = normalizeDigits(denominator, "denominator");

  if (divisor === "0") {
    throw new Error("denominator no es un valor entero válido.");
  }

  const amount = formatMinorUnits(numerator, currencyCode);

  if (divisor === "1") {
    return amount;
  }

  return `${amount} ÷ ${divisor}`;
}

export function formatMoney(value: MoneyDto): string {
  return formatMinorUnits(value.minorUnits, value.currencyCode);
}

export function formatExactMoney(value: ExactMoneyDto): string {
  return formatExactMinor(
    value.numeratorMinor,
    value.denominator,
    value.currencyCode,
  );
}

export function formatNotes(notes: string | null): string {
  return notes ?? "—";
}

export function formatServiceStatus(status: ServiceStatusDto): string {
  return status === "active" ? "Activo" : "Congelado";
}

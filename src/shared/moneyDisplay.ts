import type { CurrencyCode, ExactMoneyDto, MoneyDto } from "./api";

const SIGNED_INTEGER_PATTERN = /^-?\d+$/;
const DIGITS_PATTERN = /^\d+$/;

function normalizeSignedInteger(value: string, field: string): string {
  if (!SIGNED_INTEGER_PATTERN.test(value)) {
    throw new Error(`${field} no es un valor entero válido.`);
  }

  const isNegative = value.startsWith("-");
  const digits = value.replace(/^-/, "").replace(/^0+(?=\d)/, "");

  if (!isNegative || digits === "0") {
    return digits;
  }

  return `-${digits}`;
}

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
  const normalized = normalizeSignedInteger(minorUnits, "minorUnits");
  const sign = normalized.startsWith("-") ? "-" : "";
  const digits = normalized.replace(/^-/, "");
  const padded = digits.padStart(3, "0");
  const integerPart = padded.slice(0, -2);
  const fractionPart = padded.slice(-2);

  return `${currencyCode} ${sign}${integerPart},${fractionPart}`;
}

export function formatExactMinor(
  numeratorMinor: string,
  denominator: string,
  currencyCode: CurrencyCode,
): string {
  const numerator = normalizeSignedInteger(numeratorMinor, "numeratorMinor");
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

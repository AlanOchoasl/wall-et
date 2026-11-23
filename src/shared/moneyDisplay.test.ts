import { describe, expect, it } from "vitest";

import {
  formatExactMinor,
  formatExactMoney,
  formatMinorUnits,
  formatMoney,
} from "./moneyDisplay";

describe("formatMinorUnits", () => {
  it("conserva el comportamiento existente para importes positivos", () => {
    expect(formatMinorUnits("10000", "ARS")).toBe("ARS 100,00");
    expect(formatMinorUnits("0002000", "USD")).toBe("USD 20,00");
    expect(formatMinorUnits("0", "ARS")).toBe("ARS 0,00");
  });

  it("formatea importes negativos", () => {
    expect(formatMinorUnits("-10000", "ARS")).toBe("ARS -100,00");
    expect(formatMinorUnits("-1", "USD")).toBe("USD -0,01");
    expect(formatMinorUnits("-0", "ARS")).toBe("ARS 0,00");
  });

  it("conserva precisión en importes grandes", () => {
    expect(formatMinorUnits("123456789012345678901234", "ARS")).toBe(
      "ARS 1234567890123456789012,34",
    );
  });

  it("rechaza valores que no sean enteros con signo", () => {
    expect(() => formatMinorUnits("12,34", "ARS")).toThrow(/minorUnits/);
    expect(() => formatMinorUnits("+100", "ARS")).toThrow(/minorUnits/);
  });
});

describe("formatExactMinor", () => {
  it("muestra la división exacta sin redondear", () => {
    expect(formatExactMinor("10000", "3", "ARS")).toBe("ARS 100,00 ÷ 3");
    expect(formatExactMinor("00010000", "0003", "USD")).toBe(
      "USD 100,00 ÷ 3",
    );
  });

  it("muestra el importe convencional cuando el denominador es 1", () => {
    expect(formatExactMinor("10000", "1", "ARS")).toBe("ARS 100,00");
  });

  it("conserva el signo en fracciones exactas", () => {
    expect(formatExactMinor("-10000", "3", "ARS")).toBe("ARS -100,00 ÷ 3");
  });

  it("rechaza denominador cero y valores inválidos", () => {
    expect(() => formatExactMinor("10000", "0", "ARS")).toThrow(
      /denominator/,
    );
    expect(() => formatExactMinor("100,00", "3", "ARS")).toThrow(
      /numeratorMinor/,
    );
  });
});

describe("formatMoney y formatExactMoney", () => {
  it("formatea DTOs con su propia moneda", () => {
    expect(formatMoney({ minorUnits: "-500", currencyCode: "USD" })).toBe(
      "USD -5,00",
    );
    expect(
      formatExactMoney({
        numeratorMinor: "10000",
        denominator: "3",
        currencyCode: "ARS",
      }),
    ).toBe("ARS 100,00 ÷ 3");
  });
});

import { describe, expect, it } from "vitest";

import {
  formatExactMinor,
  formatExactMoney,
  formatMinorUnits,
  formatMoney,
  formatNotes,
  formatServiceStatus,
} from "./formatServiceDisplay";

describe("formatMinorUnits", () => {
  it("formatea centavos en ARS", () => {
    expect(formatMinorUnits("10000", "ARS")).toBe("ARS 100,00");
  });

  it("formatea centavos en USD", () => {
    expect(formatMinorUnits("2000", "USD")).toBe("USD 20,00");
  });

  it("muestra cero", () => {
    expect(formatMinorUnits("0", "ARS")).toBe("ARS 0,00");
  });

  it("normaliza ceros iniciales", () => {
    expect(formatMinorUnits("00010000", "ARS")).toBe("ARS 100,00");
  });

  it("conserva precisión en importes grandes", () => {
    expect(formatMinorUnits("123456789012345678901234", "ARS")).toBe(
      "ARS 1234567890123456789012,34",
    );
  });

  it("rechaza un valor que no sea entero", () => {
    expect(() => formatMinorUnits("12,34", "ARS")).toThrow(/minorUnits/);
  });
});

describe("formatExactMinor", () => {
  it("muestra explícitamente una fracción no entera en ARS", () => {
    expect(formatExactMinor("10000", "3", "ARS")).toBe("ARS 100,00 ÷ 3");
  });

  it("muestra explícitamente una fracción no entera en USD", () => {
    expect(formatExactMinor("2000", "3", "USD")).toBe("USD 20,00 ÷ 3");
  });

  it("muestra el importe convencional cuando el denominador es 1", () => {
    expect(formatExactMinor("10000", "1", "ARS")).toBe("ARS 100,00");
  });

  it("normaliza ceros iniciales del numerador y del denominador", () => {
    expect(formatExactMinor("00010000", "003", "ARS")).toBe(
      "ARS 100,00 ÷ 3",
    );
    expect(formatExactMinor("00010000", "001", "ARS")).toBe("ARS 100,00");
  });

  it("rechaza denominador cero", () => {
    expect(() => formatExactMinor("10000", "0", "ARS")).toThrow(
      /denominator/,
    );
  });

  it("rechaza numerador inválido", () => {
    expect(() => formatExactMinor("100,00", "3", "ARS")).toThrow(
      /numeratorMinor/,
    );
  });
});

describe("formatMoney y formatExactMoney", () => {
  it("formatea un DTO de importe", () => {
    expect(
      formatMoney({ minorUnits: "1000000", currencyCode: "ARS" }),
    ).toBe("ARS 10000,00");
  });

  it("formatea un DTO de fracción exacta", () => {
    expect(
      formatExactMoney({
        numeratorMinor: "10000",
        denominator: "3",
        currencyCode: "ARS",
      }),
    ).toBe("ARS 100,00 ÷ 3");
  });
});

describe("formatNotes", () => {
  it("muestra un guion cuando las notas son nulas", () => {
    expect(formatNotes(null)).toBe("—");
  });

  it("conserva las notas existentes", () => {
    expect(formatNotes("familiar")).toBe("familiar");
  });
});

describe("formatServiceStatus", () => {
  it("muestra Activo para servicios activos", () => {
    expect(formatServiceStatus("active")).toBe("Activo");
  });

  it("muestra Congelado para servicios congelados", () => {
    expect(formatServiceStatus("frozen")).toBe("Congelado");
  });
});

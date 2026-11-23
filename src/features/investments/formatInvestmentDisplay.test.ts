import type {
  CoverageOutcomeDto,
  ExactMoneyDto,
  FinancialRoutesDto,
} from "../../shared/api";
import { formatExactMoney } from "../../shared/moneyDisplay";
import {
  formatCoverageOutcome,
  formatCoverageRoutes,
  formatFinancialResult,
  formatFinancialRoutes,
  formatTna,
} from "./formatInvestmentDisplay";

function exact(
  numeratorMinor: string,
  denominator: string,
  currencyCode: ExactMoneyDto["currencyCode"],
): ExactMoneyDto {
  return { numeratorMinor, denominator, currencyCode };
}

describe("formatTna", () => {
  it("muestra la TNA recibida sin convertirla", () => {
    expect(formatTna("25,50")).toBe("25,50 %");
    expect(formatTna("0,00")).toBe("0,00 %");
  });
});

describe("formatFinancialResult", () => {
  it("incluye el estado en resultados disponibles", () => {
    expect(
      formatFinancialResult(
        {
          status: "disponible",
          reason: null,
          value: exact("36500", "1", "ARS"),
        },
        formatExactMoney,
      ),
    ).toBe("ARS 365,00 (disponible)");
  });

  it.each([
    ["approved_demo_pv01", "PV-01 · Provisional / demostración"],
    ["approved_demo_pv02", "PV-02 · Provisional / demostración"],
    ["approved_demo_pv03", "PV-03 · Provisional / demostración"],
  ] as const)("identifica %s como %s", (reason, label) => {
    expect(
      formatFinancialResult(
        {
          status: "provisional",
          reason,
          value: exact("36500", "1", "ARS"),
        },
        formatExactMoney,
      ),
    ).toBe(`ARS 365,00 (${label})`);
  });

  it("muestra no disponible cuando no hay valor o la ruta está bloqueada", () => {
    expect(
      formatFinancialResult(
        {
          status: "no_disponible",
          reason: "pending_gross_yield_formula",
          value: null,
        },
        formatExactMoney,
      ),
    ).toBe("no disponible");
    expect(
      formatFinancialResult(
        { status: "disponible", reason: null, value: null },
        formatExactMoney,
      ),
    ).toBe("no disponible");
  });

  it("conserva moneda y fracción exacta sin convertir ni redondear", () => {
    expect(
      formatFinancialResult(
        {
          status: "provisional",
          reason: "approved_demo_pv01",
          value: exact("300", "3", "USD"),
        },
        formatExactMoney,
      ),
    ).toBe("USD 3,00 ÷ 3 (PV-01 · Provisional / demostración)");
  });
});

describe("formatFinancialRoutes", () => {
  it("muestra ambas rutas con su estado sin presentar lo provisional como definitivo", () => {
    const routes: FinancialRoutesDto<ExactMoneyDto> = {
      definitive: {
        status: "no_disponible",
        reason: "pending_gross_yield_formula",
        value: null,
      },
      provisional: {
        status: "provisional",
        reason: "approved_demo_pv01",
        value: exact("36500", "3", "ARS"),
      },
    };

    expect(formatFinancialRoutes(routes, formatExactMoney)).toBe(
      "Definitivo: no disponible · Provisional: ARS 365,00 ÷ 3 (PV-01 · Provisional / demostración)",
    );
  });
});

describe("formatCoverageOutcome", () => {
  it("usa el texto definitivo para cobertura y déficit", () => {
    const deficit: CoverageOutcomeDto = {
      outcome: "deficit",
      amount: exact("150000", "1", "ARS"),
    };

    expect(formatCoverageOutcome({ outcome: "covered" })).toBe("Cubierto");
    expect(formatCoverageOutcome(deficit)).toBe("Déficit de: ARS 1500,00");
    expect(formatCoverageOutcome({ outcome: "covered" })).not.toBe(
      "¡Sí cubre!",
    );
  });

  it("muestra código de moneda y fracción exacta en el déficit", () => {
    expect(
      formatCoverageOutcome({
        outcome: "deficit",
        amount: exact("10000", "3", "ARS"),
      }),
    ).toBe("Déficit de: ARS 100,00 ÷ 3");
    expect(
      formatCoverageOutcome({
        outcome: "deficit",
        amount: exact("25050", "1", "USD"),
      }),
    ).toBe("Déficit de: USD 250,50");
  });

  it("muestra estados sin veredicto", () => {
    expect(
      formatCoverageOutcome({ outcome: "incompatible_currencies" }),
    ).toBe("Monedas incompatibles");
    expect(formatCoverageOutcome({ outcome: "zero_expenses" })).toBe(
      "Gastos en cero",
    );
  });
});

describe("formatCoverageRoutes", () => {
  it("muestra estado y resultado de cada ruta", () => {
    const routes: FinancialRoutesDto<CoverageOutcomeDto> = {
      definitive: {
        status: "no_disponible",
        reason: "pending_net_yield",
        value: null,
      },
      provisional: {
        status: "provisional",
        reason: "approved_demo_pv03",
        value: {
          outcome: "deficit",
          amount: exact("150000", "1", "ARS"),
        },
      },
    };

    expect(formatCoverageRoutes(routes)).toBe(
      "Definitivo: no disponible · Provisional: Déficit de: ARS 1500,00 (PV-03 · Provisional / demostración)",
    );
  });

  it("no agrega sufijo disponible a los estados sin veredicto", () => {
    const routes: FinancialRoutesDto<CoverageOutcomeDto> = {
      definitive: {
        status: "disponible",
        reason: null,
        value: { outcome: "incompatible_currencies" },
      },
      provisional: {
        status: "disponible",
        reason: null,
        value: { outcome: "zero_expenses" },
      },
    };

    expect(formatCoverageRoutes(routes)).toBe(
      "Definitivo: Monedas incompatibles · Provisional: Gastos en cero",
    );
  });

  it("conserva la etiqueta provisional en los estados sin veredicto", () => {
    const routes: FinancialRoutesDto<CoverageOutcomeDto> = {
      definitive: {
        status: "no_disponible",
        reason: "pending_net_yield",
        value: null,
      },
      provisional: {
        status: "provisional",
        reason: "approved_demo_pv03",
        value: { outcome: "zero_expenses" },
      },
    };

    expect(formatCoverageRoutes(routes)).toBe(
      "Definitivo: no disponible · Provisional: Gastos en cero (PV-03 · Provisional / demostración)",
    );
  });
});

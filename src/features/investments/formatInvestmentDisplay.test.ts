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
  it("incluye el estado en resultados disponibles y provisionales", () => {
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
    expect(
      formatFinancialResult(
        {
          status: "provisional",
          reason: "approved_demo_pv01",
          value: exact("36500", "1", "ARS"),
        },
        formatExactMoney,
      ),
    ).toBe("ARS 365,00 (provisional)");
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
    ).toBe("No disponible");
    expect(
      formatFinancialResult(
        { status: "disponible", reason: null, value: null },
        formatExactMoney,
      ),
    ).toBe("No disponible");
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
      "Definitivo: No disponible · Provisional: ARS 365,00 ÷ 3 (provisional)",
    );
  });
});

describe("formatCoverageOutcome", () => {
  it("usa una representación genérica para la cobertura", () => {
    const deficit: CoverageOutcomeDto = {
      outcome: "deficit",
      amount: exact("150000", "1", "ARS"),
    };

    expect(formatCoverageOutcome({ outcome: "covered" })).toBe("Cubierto");
    expect(formatCoverageOutcome(deficit)).toBe("Déficit: ARS 1500,00");
    expect(formatCoverageOutcome(deficit)).not.toContain("Déficit de:");
    expect(formatCoverageOutcome({ outcome: "covered" })).not.toBe(
      "¡Sí cubre!",
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
      "Definitivo: No disponible · Provisional: Déficit: ARS 1500,00 (provisional)",
    );
  });
});

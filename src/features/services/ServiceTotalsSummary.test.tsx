import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import type { ServiceTotalsDto } from "../../shared/api";
import { ServiceTotalsSummary } from "./ServiceTotalsSummary";

const totals: ServiceTotalsDto = {
  ars: {
    totalCost: { minorUnits: "1600000", currencyCode: "ARS" },
    userShare: {
      numeratorMinor: "20000",
      denominator: "3",
      currencyCode: "ARS",
    },
  },
  usd: {
    totalCost: { minorUnits: "2000", currencyCode: "USD" },
    userShare: {
      numeratorMinor: "1000",
      denominator: "1",
      currencyCode: "USD",
    },
  },
};

const zeroTotals: ServiceTotalsDto = {
  ars: {
    totalCost: { minorUnits: "0", currencyCode: "ARS" },
    userShare: {
      numeratorMinor: "0",
      denominator: "1",
      currencyCode: "ARS",
    },
  },
  usd: {
    totalCost: { minorUnits: "0", currencyCode: "USD" },
    userShare: {
      numeratorMinor: "0",
      denominator: "1",
      currencyCode: "USD",
    },
  },
};

describe("ServiceTotalsSummary", () => {
  it("muestra un estado de carga accesible", () => {
    render(<ServiceTotalsSummary totals={null} error={null} />);

    expect(
      screen.getByRole("region", { name: "Totales de servicios activos" }),
    ).toBeInTheDocument();
    expect(screen.getByText("Cargando totales…")).toBeInTheDocument();
  });

  it("muestra el error en un alert accesible", () => {
    render(
      <ServiceTotalsSummary totals={totals} error="sin conexión totales" />,
    );

    expect(screen.getByRole("alert")).toHaveTextContent(
      "No se pudieron cargar los totales: sin conexión totales",
    );
    expect(screen.queryByText("Mi gasto")).not.toBeInTheDocument();
  });

  it("muestra etiquetas distintas y montos por moneda sin redondear", () => {
    render(<ServiceTotalsSummary totals={totals} error={null} />);

    const region = within(
      screen.getByRole("region", { name: "Totales de servicios activos" }),
    );
    expect(
      region.getByText("Costo total de servicios activos"),
    ).toBeInTheDocument();
    expect(region.getByText("Mi gasto")).toBeInTheDocument();
    expect(region.getByText("ARS 16000,00")).toBeInTheDocument();
    expect(region.getByText("USD 20,00")).toBeInTheDocument();
    expect(region.getByText("ARS 200,00 ÷ 3")).toBeInTheDocument();
    expect(region.getByText("USD 10,00")).toBeInTheDocument();
  });

  it("muestra ambos totales en cero cuando no hay servicios activos", () => {
    render(<ServiceTotalsSummary totals={zeroTotals} error={null} />);

    const region = within(
      screen.getByRole("region", { name: "Totales de servicios activos" }),
    );
    expect(region.getAllByText("ARS 0,00")).toHaveLength(2);
    expect(region.getAllByText("USD 0,00")).toHaveLength(2);
  });
});

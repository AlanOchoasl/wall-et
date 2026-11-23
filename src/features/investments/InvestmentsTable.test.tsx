import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import type { InvestmentSummaryDto } from "../../shared/api";
import { InvestmentsTable } from "./InvestmentsTable";

const arsInvestment: InvestmentSummaryDto = {
  record: {
    id: "inv-ars",
    month: "2026-01",
    currencyCode: "ARS",
    capital: { minorUnits: "10000000", currencyCode: "ARS" },
    tna: "25,50",
    termDays: 30,
    taxes: { minorUnits: "50000", currencyCode: "ARS" },
    createdAt: "2026-01-01T00:00:00Z",
    updatedAt: "2026-01-01T00:00:00Z",
  },
  derived: {
    monthlyExpenses: {
      numeratorMinor: "550000",
      denominator: "1",
      currencyCode: "ARS",
    },
    grossYield: {
      definitive: {
        status: "no_disponible",
        reason: "pending_gross_yield_formula",
        value: null,
      },
      provisional: {
        status: "provisional",
        reason: "approved_demo_pv01",
        value: {
          numeratorMinor: "36500",
          denominator: "1",
          currencyCode: "ARS",
        },
      },
    },
    netYield: {
      definitive: {
        status: "no_disponible",
        reason: "pending_net_yield_treatment",
        value: null,
      },
      provisional: {
        status: "provisional",
        reason: "approved_demo_pv02",
        value: {
          numeratorMinor: "-10000",
          denominator: "1",
          currencyCode: "ARS",
        },
      },
    },
    coverage: {
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
          amount: {
            numeratorMinor: "150000",
            denominator: "1",
            currencyCode: "ARS",
          },
        },
      },
    },
  },
};

const usdInvestment: InvestmentSummaryDto = {
  record: {
    id: "inv-usd",
    month: "2026-02",
    currencyCode: "USD",
    capital: { minorUnits: "200000", currencyCode: "USD" },
    tna: "5,00",
    termDays: 15,
    taxes: { minorUnits: "0", currencyCode: "USD" },
    createdAt: "2026-02-01T00:00:00Z",
    updatedAt: "2026-02-01T00:00:00Z",
  },
  derived: {
    monthlyExpenses: {
      numeratorMinor: "10000",
      denominator: "1",
      currencyCode: "USD",
    },
    grossYield: {
      definitive: {
        status: "disponible",
        reason: null,
        value: {
          numeratorMinor: "200",
          denominator: "1",
          currencyCode: "USD",
        },
      },
      provisional: {
        status: "disponible",
        reason: null,
        value: {
          numeratorMinor: "300",
          denominator: "1",
          currencyCode: "USD",
        },
      },
    },
    netYield: {
      definitive: {
        status: "disponible",
        reason: null,
        value: {
          numeratorMinor: "100",
          denominator: "1",
          currencyCode: "USD",
        },
      },
      provisional: {
        status: "disponible",
        reason: null,
        value: {
          numeratorMinor: "150",
          denominator: "1",
          currencyCode: "USD",
        },
      },
    },
    coverage: {
      definitive: {
        status: "disponible",
        reason: null,
        value: { outcome: "covered" },
      },
      provisional: {
        status: "disponible",
        reason: null,
        value: { outcome: "covered" },
      },
    },
  },
};

const noVerdictInvestment: InvestmentSummaryDto = {
  ...usdInvestment,
  record: { ...usdInvestment.record, id: "inv-sin-veredicto", month: "2026-03" },
  derived: {
    ...usdInvestment.derived,
    coverage: {
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
    },
  },
};

describe("InvestmentsTable", () => {
  it("muestra las diez columnas definidas y el mes como encabezado de fila", () => {
    render(<InvestmentsTable investments={[arsInvestment]} />);

    const table = screen.getByRole("table", { name: "Inversiones" });
    const headers = within(table)
      .getAllByRole("columnheader")
      .map((header) => header.textContent);

    expect(headers).toEqual([
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
    ]);
    expect(
      within(table).getByRole("rowheader", { name: "2026-01" }),
    ).toBeInTheDocument();
  });

  it("muestra datos de origen de ARS y USD sin conversión", () => {
    render(
      <InvestmentsTable investments={[arsInvestment, usdInvestment]} />,
    );

    const table = within(
      screen.getByRole("table", { name: "Inversiones" }),
    );

    expect(table.getAllByText("ARS")).toHaveLength(1);
    expect(table.getAllByText("USD")).toHaveLength(1);
    expect(table.getByText("ARS 100000,00")).toBeInTheDocument();
    expect(table.getByText("USD 2000,00")).toBeInTheDocument();
    expect(table.getByText("25,50 %")).toBeInTheDocument();
    expect(table.getByText("5,00 %")).toBeInTheDocument();
    expect(table.getByText("ARS 500,00")).toBeInTheDocument();
    expect(table.getByText("USD 0,00")).toBeInTheDocument();
    expect(table.getAllByRole("row")).toHaveLength(3);
  });

  it("muestra resultados por ruta con estado y sin reemplazar ausencias por cero", () => {
    render(
      <InvestmentsTable investments={[arsInvestment, usdInvestment]} />,
    );

    const table = within(
      screen.getByRole("table", { name: "Inversiones" }),
    );

    expect(table.getByText("ARS 5500,00")).toBeInTheDocument();
    expect(
      table.getByText(
        "Definitivo: no disponible · Provisional: ARS 365,00 (PV-01 · Provisional / demostración)",
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(
        "Definitivo: no disponible · Provisional: ARS -100,00 (PV-02 · Provisional / demostración)",
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(
        "Definitivo: no disponible · Provisional: Déficit de: ARS 1500,00 (PV-03 · Provisional / demostración)",
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(
        "Definitivo: USD 2,00 (disponible) · Provisional: USD 3,00 (disponible)",
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(
        "Definitivo: Cubierto (disponible) · Provisional: Cubierto (disponible)",
      ),
    ).toBeInTheDocument();
    expect(table.queryByText(/Definitivo: (ARS|USD) 0,00/)).toBeNull();
  });

  it("muestra estados sin veredicto sin sufijo disponible", () => {
    render(<InvestmentsTable investments={[noVerdictInvestment]} />);

    const table = within(
      screen.getByRole("table", { name: "Inversiones" }),
    );

    expect(
      table.getByText(
        "Definitivo: Monedas incompatibles · Provisional: Gastos en cero",
      ),
    ).toBeInTheDocument();
    expect(
      table.queryByText(/Monedas incompatibles \(disponible\)/),
    ).toBeNull();
    expect(table.queryByText(/Gastos en cero \(disponible\)/)).toBeNull();
  });

  it("etiqueta PV-01 a PV-03 como provisionales y mantiene la ruta definitiva no disponible", () => {
    render(<InvestmentsTable investments={[arsInvestment]} />);

    const table = within(
      screen.getByRole("table", { name: "Inversiones" }),
    );

    expect(
      table.getByText(
        /Definitivo: no disponible · Provisional: ARS 365,00/,
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(
        /Definitivo: no disponible · Provisional: ARS -100,00/,
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(
        /Definitivo: no disponible · Provisional: Déficit de: ARS 1500,00/,
      ),
    ).toBeInTheDocument();
    expect(
      table.getByText(/PV-01 · Provisional \/ demostración/),
    ).toBeInTheDocument();
    expect(
      table.getByText(/PV-02 · Provisional \/ demostración/),
    ).toBeInTheDocument();
    expect(
      table.getByText(/PV-03 · Provisional \/ demostración/),
    ).toBeInTheDocument();
    expect(table.queryByText(/^Definitivo: (?!no disponible)/)).toBeNull();
    expect(table.queryByText(/¡Sí cubre!/)).toBeNull();
    expect(table.queryByText(/Déficit: ARS/)).toBeNull();
  });

  it("no permite editar los resultados derivados", () => {
    render(
      <InvestmentsTable investments={[arsInvestment, usdInvestment]} />,
    );

    const table = screen.getByRole("table", { name: "Inversiones" });
    const interactiveRoles = [
      "button",
      "checkbox",
      "combobox",
      "radio",
      "searchbox",
      "slider",
      "spinbutton",
      "textbox",
    ];

    for (const role of interactiveRoles) {
      expect(within(table).queryAllByRole(role)).toHaveLength(0);
    }
    expect(
      table.querySelectorAll("input, select, textarea, button, [contenteditable]"),
    ).toHaveLength(0);
    expect(
      within(table).getByText(
        "Definitivo: no disponible · Provisional: ARS 365,00 (PV-01 · Provisional / demostración)",
      ),
    ).toBeInTheDocument();
  });

  it("aplica prioridad a error, carga y estado vacío", () => {
    const view = render(
      <InvestmentsTable investments={[]} loading error="sin conexión" />,
    );

    expect(screen.getByRole("alert")).toHaveTextContent(
      "No se pudieron cargar las inversiones: sin conexión",
    );
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
    expect(
      screen.queryByText("No hay inversiones para el mes seleccionado"),
    ).not.toBeInTheDocument();

    view.rerender(<InvestmentsTable investments={[]} loading />);
    expect(screen.getByRole("status")).toHaveTextContent(
      "Cargando inversiones…",
    );
    expect(
      screen.queryByText("No hay inversiones para el mes seleccionado"),
    ).not.toBeInTheDocument();

    view.rerender(<InvestmentsTable investments={[]} />);
    expect(
      screen.getByText("No hay inversiones para el mes seleccionado"),
    ).toBeInTheDocument();
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
  });
});

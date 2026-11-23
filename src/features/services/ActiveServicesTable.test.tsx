import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import type { ServiceRecordDto } from "../../shared/api";
import { ActiveServicesTable } from "./ActiveServicesTable";

const arsService: ServiceRecordDto = {
  id: "00000000-0000-4000-8000-000000000001",
  name: "Luz",
  totalCost: { minorUnits: "10000", currencyCode: "ARS" },
  myShare: {
    numeratorMinor: "10000",
    denominator: "3",
    currencyCode: "ARS",
  },
  sharedPeopleCount: 3,
  notes: null,
  status: "active",
  createdAt: "2026-11-23T12:00:00.000Z",
  updatedAt: "2026-11-23T12:00:00.000Z",
};

const usdService: ServiceRecordDto = {
  id: "00000000-0000-4000-8000-000000000002",
  name: "Netflix",
  totalCost: { minorUnits: "2000", currencyCode: "USD" },
  myShare: {
    numeratorMinor: "2000",
    denominator: "1",
    currencyCode: "USD",
  },
  sharedPeopleCount: 2,
  notes: "familiar",
  status: "active",
  createdAt: "2026-11-23T12:00:00.000Z",
  updatedAt: "2026-11-23T12:00:00.000Z",
};

describe("ActiveServicesTable", () => {
  it("muestra los siete encabezados con semántica de columna", () => {
    render(<ActiveServicesTable services={[arsService]} />);

    const table = screen.getByRole("table", { name: "Servicios activos" });
    const headers = within(table)
      .getAllByRole("columnheader")
      .map((header) => header.textContent);

    expect(headers).toEqual([
      "Nombre",
      "Moneda",
      "Costo total",
      "Cantidad de personas",
      "Mi parte",
      "Notas",
      "Estado",
    ]);
  });

  it("muestra una fila por servicio", () => {
    render(<ActiveServicesTable services={[arsService, usdService]} />);

    const rows = within(screen.getByRole("table")).getAllByRole("row");

    expect(rows).toHaveLength(3);
    expect(screen.getByRole("rowheader", { name: "Luz" })).toBeInTheDocument();
    expect(
      screen.getByRole("rowheader", { name: "Netflix" }),
    ).toBeInTheDocument();
  });

  it("muestra moneda, importes, personas, mi parte, notas y estado", () => {
    render(<ActiveServicesTable services={[arsService, usdService]} />);

    expect(screen.getByText("ARS")).toBeInTheDocument();
    expect(screen.getByText("USD")).toBeInTheDocument();
    expect(screen.getByText("ARS 100,00")).toBeInTheDocument();
    expect(screen.getByText("ARS 100,00 ÷ 3")).toBeInTheDocument();
    expect(screen.getAllByText("USD 20,00")).toHaveLength(2);
    expect(screen.getByText("3")).toBeInTheDocument();
    expect(screen.getByText("2")).toBeInTheDocument();
    expect(screen.getByText("—")).toBeInTheDocument();
    expect(screen.getByText("familiar")).toBeInTheDocument();
    expect(screen.getAllByText("Activo")).toHaveLength(2);
  });

  it("no muestra acciones de modificación", () => {
    render(<ActiveServicesTable services={[arsService]} />);

    expect(screen.queryByRole("button")).not.toBeInTheDocument();
  });
});

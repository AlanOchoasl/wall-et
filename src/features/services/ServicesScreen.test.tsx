import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  createService,
  deleteService,
  freezeService,
  getServiceTotals,
  listActiveServices,
  listFrozenServices,
  reactivateService,
  updateService,
  type ServiceRecordDto,
} from "../../shared/api";
import { ServicesScreen } from "./ServicesScreen";

vi.mock("../../shared/api", () => ({
  createService: vi.fn(),
  deleteService: vi.fn(),
  freezeService: vi.fn(),
  getServiceTotals: vi.fn(),
  listActiveServices: vi.fn(),
  listFrozenServices: vi.fn(),
  reactivateService: vi.fn(),
  updateService: vi.fn(),
}));

const mockedListActiveServices = vi.mocked(listActiveServices);

const service: ServiceRecordDto = {
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

describe("ServicesScreen", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("muestra el título y el estado de carga", () => {
    mockedListActiveServices.mockReturnValue(
      new Promise<ServiceRecordDto[]>(() => {}),
    );

    render(<ServicesScreen />);

    expect(
      screen.getByRole("heading", { name: "Servicios" }),
    ).toBeInTheDocument();
    expect(screen.getByText("Cargando servicios activos…")).toBeInTheDocument();
  });

  it("consulta servicios activos al montar y muestra sus datos", async () => {
    mockedListActiveServices.mockResolvedValue([service]);

    render(<ServicesScreen />);

    expect(
      await screen.findByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(screen.getByText("Luz")).toBeInTheDocument();
    expect(screen.getByText("ARS 100,00 ÷ 3")).toBeInTheDocument();
  });

  it("muestra un estado vacío cuando no hay servicios activos", async () => {
    mockedListActiveServices.mockResolvedValue([]);

    render(<ServicesScreen />);

    expect(await screen.findByText("No hay servicios activos.")).toBeInTheDocument();
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
  });

  it("muestra errores de consulta en un alert accesible", async () => {
    mockedListActiveServices.mockRejectedValue(new Error("sin conexión"));

    render(<ServicesScreen />);

    expect(await screen.findByRole("alert")).toHaveTextContent("sin conexión");
  });

  it("no invoca comandos de modificación ni de totales", async () => {
    mockedListActiveServices.mockResolvedValue([service]);

    render(<ServicesScreen />);

    await screen.findByRole("table");

    expect(createService).not.toHaveBeenCalled();
    expect(updateService).not.toHaveBeenCalled();
    expect(deleteService).not.toHaveBeenCalled();
    expect(freezeService).not.toHaveBeenCalled();
    expect(reactivateService).not.toHaveBeenCalled();
    expect(listFrozenServices).not.toHaveBeenCalled();
    expect(getServiceTotals).not.toHaveBeenCalled();
  });
});

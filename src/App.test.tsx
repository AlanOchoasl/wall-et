import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";

import { App } from "./App";
import {
  createService,
  getAppStatus,
  listActiveServices,
  listFrozenServices,
  reactivateService,
  updateService,
  type ServiceRecordDto,
} from "./shared/api";

vi.mock("./shared/api", () => ({
  createService: vi.fn(),
  getAppStatus: vi.fn(),
  listActiveServices: vi.fn(),
  listFrozenServices: vi.fn(),
  reactivateService: vi.fn(),
  updateService: vi.fn(),
}));

const mockedGetAppStatus = vi.mocked(getAppStatus);
const mockedListActiveServices = vi.mocked(listActiveServices);
const mockedListFrozenServices = vi.mocked(listFrozenServices);
const mockedReactivateService = vi.mocked(reactivateService);

const activeService: ServiceRecordDto = {
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

describe("App", () => {
  beforeEach(() => {
    mockedGetAppStatus.mockReset();
    mockedListActiveServices.mockReset();
    mockedListActiveServices.mockResolvedValue([]);
    mockedListFrozenServices.mockReset();
    mockedListFrozenServices.mockResolvedValue([]);
    mockedReactivateService.mockReset();
  });

  it("muestra el título de la aplicación", () => {
    mockedGetAppStatus.mockResolvedValue({
      appName: "App de Finanzas",
      version: "0.1.0",
      databaseReady: true,
      schemaVersion: 1,
      currencyCode: "ARS",
    });

    render(<App />);

    expect(
      screen.getByRole("heading", { name: "App de Finanzas" }),
    ).toBeInTheDocument();
  });

  it("muestra el estado informado por el backend", async () => {
    mockedGetAppStatus.mockResolvedValue({
      appName: "App de Finanzas",
      version: "0.1.0",
      databaseReady: true,
      schemaVersion: 1,
      currencyCode: "ARS",
    });

    render(<App />);

    expect(await screen.findByText("lista (esquema 1)")).toBeInTheDocument();
    expect(screen.getByText("ARS")).toBeInTheDocument();
  });

  it("muestra un mensaje cuando el comando falla", async () => {
    mockedGetAppStatus.mockRejectedValue(new Error("sin conexión"));

    render(<App />);

    expect(await screen.findByRole("alert")).toHaveTextContent("sin conexión");
  });

  it("muestra la pantalla de Servicios con los datos activos", async () => {
    mockedGetAppStatus.mockResolvedValue({
      appName: "App de Finanzas",
      version: "0.1.0",
      databaseReady: true,
      schemaVersion: 1,
      currencyCode: "ARS",
    });
    mockedListActiveServices.mockResolvedValue([activeService]);

    render(<App />);

    expect(
      await screen.findByRole("heading", { name: "Servicios" }),
    ).toBeInTheDocument();
    expect(await screen.findByText("Luz")).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(createService).not.toHaveBeenCalled();
    expect(updateService).not.toHaveBeenCalled();
  });
});

import { fireEvent, render, screen, waitFor } from "@testing-library/react";
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
const mockedCreateService = vi.mocked(createService);
const mockedUpdateService = vi.mocked(updateService);

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

const updatedService: ServiceRecordDto = {
  ...service,
  name: "Internet",
  totalCost: { minorUnits: "12000", currencyCode: "ARS" },
  myShare: {
    numeratorMinor: "12000",
    denominator: "3",
    currencyCode: "ARS",
  },
};

function fillForm(name: string) {
  fireEvent.change(screen.getByLabelText("Nombre"), {
    target: { value: name },
  });
  fireEvent.change(screen.getByLabelText("Costo total"), {
    target: { value: "120,00" },
  });
  fireEvent.change(screen.getByLabelText("Cantidad de personas"), {
    target: { value: "3" },
  });
}

describe("ServicesScreen", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockedListActiveServices.mockResolvedValue([service]);
    mockedCreateService.mockResolvedValue(service);
    mockedUpdateService.mockResolvedValue(service);
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

  it("no invoca comandos de modificación ni de totales al montar", async () => {
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

  it("abre el modal de alta desde Nuevo servicio", async () => {
    render(<ServicesScreen />);
    await screen.findByRole("table");

    fireEvent.click(screen.getByRole("button", { name: "Nuevo servicio" }));

    expect(
      screen.getByRole("dialog", { name: "Nuevo servicio" }),
    ).toBeInTheDocument();
    expect(screen.getByLabelText("Moneda")).toHaveValue("ARS");
  });

  it("abre el modal de edición desde la fila correspondiente", async () => {
    render(<ServicesScreen />);
    await screen.findByRole("table");

    fireEvent.click(screen.getByRole("button", { name: "Editar Luz" }));

    expect(
      screen.getByRole("dialog", { name: "Editar servicio" }),
    ).toBeInTheDocument();
    expect(screen.getByLabelText("Nombre")).toHaveValue("Luz");
    expect(screen.getByLabelText("Moneda")).toBeDisabled();
  });

  it("recarga los servicios activos después de un alta exitosa", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([updatedService]);

    render(<ServicesScreen />);
    await screen.findByRole("table");

    fireEvent.click(screen.getByRole("button", { name: "Nuevo servicio" }));
    fillForm("Internet");
    fireEvent.click(screen.getByRole("button", { name: "Guardar" }));

    await waitFor(() => {
      expect(mockedCreateService).toHaveBeenCalledTimes(1);
    });
    expect(await screen.findByText("Internet")).toBeInTheDocument();
    await waitFor(() => {
      expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("recarga los servicios activos después de una edición exitosa", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([updatedService]);

    render(<ServicesScreen />);
    await screen.findByRole("table");

    fireEvent.click(screen.getByRole("button", { name: "Editar Luz" }));
    fillForm("Internet");
    fireEvent.click(screen.getByRole("button", { name: "Guardar" }));

    await waitFor(() => {
      expect(mockedUpdateService).toHaveBeenCalledWith(service.id, {
        name: "Internet",
        totalCost: "120,00",
        sharedPeopleCount: 3,
        notes: null,
      });
    });
    expect(await screen.findByText("Internet")).toBeInTheDocument();
    await waitFor(() => {
      expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    });
  });

  it("evita guardados duplicados durante el envío", async () => {
    let resolveSubmit: (value: ServiceRecordDto) => void = () => {};
    mockedCreateService.mockReturnValue(
      new Promise<ServiceRecordDto>((resolve) => {
        resolveSubmit = resolve;
      }),
    );

    render(<ServicesScreen />);
    await screen.findByRole("table");

    fireEvent.click(screen.getByRole("button", { name: "Nuevo servicio" }));
    fillForm("Internet");
    fireEvent.click(screen.getByRole("button", { name: "Guardar" }));
    fireEvent.click(screen.getByRole("button", { name: "Guardando…" }));

    expect(mockedCreateService).toHaveBeenCalledTimes(1);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);

    resolveSubmit(service);
    await waitFor(() => {
      expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    });
  });
});

import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react";
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
  type DeleteServiceResultDto,
  type ServiceRecordDto,
  type ServiceTotalsDto,
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

const mockedDeleteService = vi.mocked(deleteService);
const mockedGetServiceTotals = vi.mocked(getServiceTotals);
const mockedListActiveServices = vi.mocked(listActiveServices);
const mockedListFrozenServices = vi.mocked(listFrozenServices);
const mockedReactivateService = vi.mocked(reactivateService);
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

const frozenService: ServiceRecordDto = {
  ...service,
  id: "00000000-0000-4000-8000-000000000002",
  name: "Netflix",
  totalCost: { minorUnits: "5000", currencyCode: "USD" },
  myShare: {
    numeratorMinor: "5000",
    denominator: "2",
    currencyCode: "USD",
  },
  sharedPeopleCount: 2,
  notes: "Pago compartido",
  status: "frozen",
};

const reactivatedService: ServiceRecordDto = {
  ...frozenService,
  status: "active",
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

const totalsWithActiveServices: ServiceTotalsDto = {
  ars: {
    totalCost: { minorUnits: "10000", currencyCode: "ARS" },
    userShare: {
      numeratorMinor: "10000",
      denominator: "3",
      currencyCode: "ARS",
    },
  },
  usd: {
    totalCost: { minorUnits: "5000", currencyCode: "USD" },
    userShare: {
      numeratorMinor: "5000",
      denominator: "2",
      currencyCode: "USD",
    },
  },
};

const updatedTotals: ServiceTotalsDto = {
  ars: {
    totalCost: { minorUnits: "12000", currencyCode: "ARS" },
    userShare: {
      numeratorMinor: "12000",
      denominator: "3",
      currencyCode: "ARS",
    },
  },
  usd: zeroTotals.usd,
};

function totalsRegion() {
  return screen.getByRole("region", { name: "Totales de servicios activos" });
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((done) => {
    resolve = done;
  });

  return { promise, resolve };
}

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

function openFrozenView() {
  fireEvent.click(screen.getByRole("button", { name: "Congelados" }));
}

function openDeleteDialog(serviceName: string) {
  fireEvent.click(screen.getByRole("button", { name: `Eliminar ${serviceName}` }));

  return screen.getByRole("dialog", { name: "Eliminar servicio" });
}

describe("ServicesScreen", () => {
  beforeEach(() => {
    vi.resetAllMocks();
    mockedDeleteService.mockResolvedValue({
      outcome: "deleted",
      id: service.id,
    });
    mockedGetServiceTotals.mockResolvedValue(zeroTotals);
    mockedListActiveServices.mockResolvedValue([service]);
    mockedListFrozenServices.mockResolvedValue([]);
    mockedReactivateService.mockResolvedValue({ outcome: "reactivated" });
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
    expect(await screen.findByRole("region", {
      name: "Totales de servicios activos",
    })).toBeInTheDocument();
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

  it("consulta los totales al montar sin invocar comandos de modificación", async () => {
    render(<ServicesScreen />);

    await screen.findByRole("table");

    expect(createService).not.toHaveBeenCalled();
    expect(updateService).not.toHaveBeenCalled();
    expect(deleteService).not.toHaveBeenCalled();
    expect(freezeService).not.toHaveBeenCalled();
    expect(reactivateService).not.toHaveBeenCalled();
    expect(listFrozenServices).not.toHaveBeenCalled();
    expect(getServiceTotals).toHaveBeenCalledTimes(1);
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

  it("recarga los servicios activos y los totales después de un alta exitosa", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([updatedService]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(zeroTotals)
      .mockResolvedValueOnce(updatedTotals);

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
    expect(await within(totalsRegion()).findByText("ARS 120,00")).toBeInTheDocument();
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(2);
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("recarga los servicios activos y actualiza los totales después de una edición exitosa", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([updatedService]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(totalsWithActiveServices)
      .mockResolvedValueOnce(updatedTotals);

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
    expect(await within(totalsRegion()).findByText("ARS 120,00")).toBeInTheDocument();
    expect(within(totalsRegion()).queryByText("ARS 100,00")).not.toBeInTheDocument();
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(2);
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

  it("muestra el control interno y comienza en la vista Activos", () => {
    render(<ServicesScreen />);

    expect(screen.getByRole("button", { name: "Activos" })).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    expect(
      screen.getByRole("button", { name: "Congelados" }),
    ).toHaveAttribute("aria-pressed", "false");
    expect(
      screen.getByRole("group", { name: "Estado de los servicios" }),
    ).toBeInTheDocument();
  });

  it("muestra el estado de carga de congelados sin consultarlos al montar", async () => {
    mockedListFrozenServices.mockReturnValue(
      new Promise<ServiceRecordDto[]>(() => {}),
    );

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    expect(mockedListFrozenServices).not.toHaveBeenCalled();

    openFrozenView();

    expect(
      screen.getByText("Cargando servicios congelados…"),
    ).toBeInTheDocument();
    expect(mockedListFrozenServices).toHaveBeenCalledTimes(1);
  });

  it("consulta los servicios congelados bajo demanda y muestra sus datos", async () => {
    mockedListFrozenServices.mockResolvedValue([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();

    expect(
      await screen.findByRole("table", { name: "Servicios congelados" }),
    ).toBeInTheDocument();
    expect(mockedListFrozenServices).toHaveBeenCalledTimes(1);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(screen.getByText("Netflix")).toBeInTheDocument();
    expect(screen.getByText("USD 50,00")).toBeInTheDocument();
    expect(screen.getByText("USD 50,00 ÷ 2")).toBeInTheDocument();
    expect(screen.getByText("Pago compartido")).toBeInTheDocument();
    expect(screen.getByText("Congelado")).toBeInTheDocument();
    expect(
      screen.queryByRole("table", { name: "Servicios activos" }),
    ).not.toBeInTheDocument();
    expect(
      screen.queryByRole("region", { name: "Totales de servicios activos" }),
    ).not.toBeInTheDocument();
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(1);
  });

  it("evita consultas duplicadas al alternar repetidamente a Congelados", async () => {
    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    fireEvent.click(screen.getByRole("button", { name: "Congelados" }));

    await screen.findByText("No hay servicios congelados.");
    expect(mockedListFrozenServices).toHaveBeenCalledTimes(1);
  });

  it("muestra el estado vacío de servicios congelados", async () => {
    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();

    expect(
      await screen.findByText("No hay servicios congelados."),
    ).toBeInTheDocument();
    expect(screen.queryByRole("table")).not.toBeInTheDocument();
  });

  it("muestra errores de consulta de congelados en un alert accesible", async () => {
    mockedListFrozenServices.mockRejectedValue(
      new Error("sin conexión congelados"),
    );

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "sin conexión congelados",
    );
  });

  it("recarga servicios activos y totales al volver desde Congelados", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([service]);
    mockedListFrozenServices.mockResolvedValue([frozenService]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(zeroTotals)
      .mockResolvedValueOnce(updatedTotals);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });

    fireEvent.click(screen.getByRole("button", { name: "Activos" }));

    expect(
      await screen.findByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(await within(totalsRegion()).findByText("ARS 120,00")).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(2);
  });

  it("reactiva un servicio, vuelve a Activos y recarga la lista y los totales", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([service, reactivatedService]);
    mockedListFrozenServices.mockResolvedValue([frozenService]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(zeroTotals)
      .mockResolvedValueOnce(totalsWithActiveServices);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });
    fireEvent.click(screen.getByRole("button", { name: "Reactivar Netflix" }));

    expect(
      await screen.findByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(mockedReactivateService).toHaveBeenCalledTimes(1);
    expect(mockedReactivateService).toHaveBeenCalledWith(frozenService.id);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    expect(await within(totalsRegion()).findByText("USD 50,00")).toBeInTheDocument();
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(2);
    const activeTable = within(
      screen.getByRole("table", { name: "Servicios activos" }),
    );
    expect(activeTable.getByText("Netflix")).toBeInTheDocument();
    expect(activeTable.getByText("USD 50,00 ÷ 2")).toBeInTheDocument();
    expect(activeTable.getByText("Pago compartido")).toBeInTheDocument();
    expect(
      screen.queryByRole("table", { name: "Servicios congelados" }),
    ).not.toBeInTheDocument();
  });

  it("trata already_active como éxito sin mostrar un error", async () => {
    mockedReactivateService.mockResolvedValue({ outcome: "already_active" });
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([service, reactivatedService]);
    mockedListFrozenServices.mockResolvedValue([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });
    fireEvent.click(screen.getByRole("button", { name: "Reactivar Netflix" }));

    expect(
      await screen.findByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(2);
  });

  it("mantiene la fila congelada y permite reintentar cuando falla la reactivación", async () => {
    mockedReactivateService.mockRejectedValue(
      new Error("no se pudo reactivar"),
    );
    mockedListFrozenServices.mockResolvedValue([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });
    fireEvent.click(screen.getByRole("button", { name: "Reactivar Netflix" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "no se pudo reactivar",
    );
    expect(
      screen.getByRole("table", { name: "Servicios congelados" }),
    ).toBeInTheDocument();
    expect(screen.getByText("Netflix")).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(mockedReactivateService).toHaveBeenCalledTimes(1);
  });

  it("muestra un error cuando la reactivación devuelve un resultado inesperado", async () => {
    mockedReactivateService.mockResolvedValue({ outcome: "already_frozen" });
    mockedListFrozenServices.mockResolvedValue([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });
    fireEvent.click(screen.getByRole("button", { name: "Reactivar Netflix" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "No se pudo reactivar el servicio Netflix.",
    );
    expect(
      screen.getByRole("table", { name: "Servicios congelados" }),
    ).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
  });

  it("evita dobles envíos durante la reactivación", async () => {
    let resolveReactivate: (value: { outcome: "reactivated" }) => void = () => {};
    mockedReactivateService.mockReturnValue(
      new Promise<{ outcome: "reactivated" }>((resolve) => {
        resolveReactivate = resolve;
      }),
    );
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([service, reactivatedService]);
    mockedListFrozenServices.mockResolvedValue([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });

    const reactivateButton = screen.getByRole("button", {
      name: "Reactivar Netflix",
    });
    fireEvent.click(reactivateButton);
    fireEvent.click(reactivateButton);

    expect(mockedReactivateService).toHaveBeenCalledTimes(1);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);

    resolveReactivate({ outcome: "reactivated" });

    expect(
      await screen.findByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
  });

  it("no muestra éxito incompleto si falla la recarga de activos tras reactivar", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockRejectedValueOnce(new Error("falló recarga"));
    mockedListFrozenServices.mockResolvedValueOnce([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });
    fireEvent.click(screen.getByRole("button", { name: "Reactivar Netflix" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "El servicio se reactivó, pero no se pudieron cargar los servicios activos: falló recarga",
    );
    await waitFor(() => {
      expect(mockedListFrozenServices).toHaveBeenCalledTimes(2);
    });
    expect(screen.getByRole("button", { name: "Congelados" })).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    expect(
      screen.queryByRole("table", { name: "Servicios activos" }),
    ).not.toBeInTheDocument();
    expect(mockedReactivateService).toHaveBeenCalledTimes(1);
  });

  it("muestra el estado de carga de los totales sin ocultar la tabla de activos", async () => {
    mockedGetServiceTotals.mockReturnValue(
      new Promise<ServiceTotalsDto>(() => {}),
    );

    render(<ServicesScreen />);

    expect(screen.getByText("Cargando totales…")).toBeInTheDocument();
    expect(
      await screen.findByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
  });

  it("muestra ambos totales con etiquetas distintas y montos por moneda", async () => {
    mockedGetServiceTotals.mockResolvedValue(totalsWithActiveServices);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    const region = totalsRegion();
    expect(
      within(region).getByText("Costo total de servicios activos"),
    ).toBeInTheDocument();
    expect(within(region).getByText("Mi gasto")).toBeInTheDocument();
    expect(within(region).getByText("ARS 100,00")).toBeInTheDocument();
    expect(within(region).getByText("USD 50,00")).toBeInTheDocument();
    expect(within(region).getByText("ARS 100,00 ÷ 3")).toBeInTheDocument();
    expect(within(region).getByText("USD 50,00 ÷ 2")).toBeInTheDocument();
  });

  it("mantiene la tabla visible y no muestra totales cuando falla su consulta", async () => {
    mockedGetServiceTotals.mockRejectedValue(new Error("sin conexión totales"));

    render(<ServicesScreen />);

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "No se pudieron cargar los totales: sin conexión totales",
    );
    expect(
      screen.getByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(
      within(totalsRegion()).queryByText("Costo total de servicios activos"),
    ).not.toBeInTheDocument();
  });

  it("descarta una respuesta obsoleta de totales", async () => {
    const first = deferred<ServiceTotalsDto>();
    const second = deferred<ServiceTotalsDto>();
    mockedListFrozenServices.mockResolvedValue([frozenService]);
    mockedGetServiceTotals
      .mockReturnValueOnce(first.promise)
      .mockReturnValueOnce(second.promise);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });
    fireEvent.click(screen.getByRole("button", { name: "Activos" }));
    await screen.findByRole("table", { name: "Servicios activos" });

    await act(async () => {
      second.resolve(updatedTotals);
    });
    expect(
      await within(totalsRegion()).findByText("ARS 120,00"),
    ).toBeInTheDocument();

    await act(async () => {
      first.resolve(totalsWithActiveServices);
    });
    expect(
      within(totalsRegion()).queryByText("ARS 100,00"),
    ).not.toBeInTheDocument();
    expect(within(totalsRegion()).getByText("ARS 120,00")).toBeInTheDocument();
  });

  it("no conserva totales anteriores cuando falla una recarga", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([updatedService]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(totalsWithActiveServices)
      .mockRejectedValueOnce(new Error("falló totales"));

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    fireEvent.click(screen.getByRole("button", { name: "Editar Luz" }));
    fillForm("Internet");
    fireEvent.click(screen.getByRole("button", { name: "Guardar" }));

    expect(await screen.findByText("Internet")).toBeInTheDocument();
    expect(await screen.findByRole("alert")).toHaveTextContent(
      "No se pudieron cargar los totales: falló totales",
    );
    expect(
      within(totalsRegion()).queryByText("ARS 100,00"),
    ).not.toBeInTheDocument();
  });

  it("cancela la eliminación de un servicio activo sin modificar datos ni totales", async () => {
    mockedGetServiceTotals.mockResolvedValue(totalsWithActiveServices);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });
    expect(
      await within(totalsRegion()).findByText("ARS 100,00"),
    ).toBeInTheDocument();

    const dialog = openDeleteDialog("Luz");
    expect(dialog).toHaveTextContent("Luz");

    fireEvent.click(within(dialog).getByRole("button", { name: "Cancelar" }));

    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(mockedDeleteService).not.toHaveBeenCalled();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(1);
    expect(
      screen.getByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(within(totalsRegion()).getByText("ARS 100,00")).toBeInTheDocument();
  });

  it("confirma la eliminación de un servicio activo y recarga lista y totales", async () => {
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(totalsWithActiveServices)
      .mockResolvedValueOnce(zeroTotals);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });
    expect(
      await within(totalsRegion()).findByText("ARS 100,00"),
    ).toBeInTheDocument();

    const dialog = openDeleteDialog("Luz");
    fireEvent.click(within(dialog).getByRole("button", { name: "Eliminar" }));

    await waitFor(() => {
      expect(mockedDeleteService).toHaveBeenCalledWith(service.id, true);
    });
    expect(await screen.findByText("No hay servicios activos.")).toBeInTheDocument();
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(2);
    expect(
      await within(totalsRegion()).findAllByText("ARS 0,00"),
    ).toHaveLength(2);
  });

  it("cancela la eliminación de un servicio congelado sin recargar listas ni totales", async () => {
    mockedListFrozenServices.mockResolvedValue([frozenService]);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });
    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });

    const dialog = openDeleteDialog("Netflix");
    fireEvent.click(within(dialog).getByRole("button", { name: "Cancelar" }));

    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(mockedDeleteService).not.toHaveBeenCalled();
    expect(mockedListFrozenServices).toHaveBeenCalledTimes(1);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(1);
    expect(
      screen.getByRole("table", { name: "Servicios congelados" }),
    ).toBeInTheDocument();
    expect(screen.getByText("Netflix")).toBeInTheDocument();
  });

  it("confirma la eliminación de un servicio congelado sin recalcular totales", async () => {
    mockedListFrozenServices
      .mockResolvedValueOnce([frozenService])
      .mockResolvedValueOnce([]);
    mockedGetServiceTotals.mockResolvedValue(totalsWithActiveServices);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });
    openFrozenView();
    await screen.findByRole("table", { name: "Servicios congelados" });

    const dialog = openDeleteDialog("Netflix");
    fireEvent.click(within(dialog).getByRole("button", { name: "Eliminar" }));

    await waitFor(() => {
      expect(mockedDeleteService).toHaveBeenCalledWith(frozenService.id, true);
    });
    expect(
      await screen.findByText("No hay servicios congelados."),
    ).toBeInTheDocument();
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(mockedListFrozenServices).toHaveBeenCalledTimes(2);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(1);
  });

  it("muestra errores de eliminación en el diálogo y conserva la fila", async () => {
    mockedDeleteService.mockRejectedValue(
      new Error("sin conexión al eliminar"),
    );

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    const dialog = openDeleteDialog("Luz");
    fireEvent.click(within(dialog).getByRole("button", { name: "Eliminar" }));

    expect(await within(dialog).findByRole("alert")).toHaveTextContent(
      "sin conexión al eliminar",
    );
    expect(
      screen.getByRole("table", { name: "Servicios activos" }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Eliminar Luz" }),
    ).toBeInTheDocument();
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(1);
  });

  it("cierra el diálogo sin recargar cuando el backend informa cancelled", async () => {
    mockedDeleteService.mockResolvedValue({
      outcome: "cancelled",
      id: service.id,
    });

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    const dialog = openDeleteDialog("Luz");
    fireEvent.click(within(dialog).getByRole("button", { name: "Eliminar" }));

    await waitFor(() => {
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    });
    expect(mockedDeleteService).toHaveBeenCalledWith(service.id, true);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);
    expect(mockedGetServiceTotals).toHaveBeenCalledTimes(1);
  });

  it("evita dobles eliminaciones durante el envío", async () => {
    const pending = deferred<DeleteServiceResultDto>();
    mockedDeleteService.mockReturnValue(pending.promise);
    mockedListActiveServices
      .mockResolvedValueOnce([service])
      .mockResolvedValueOnce([]);
    mockedGetServiceTotals
      .mockResolvedValueOnce(zeroTotals)
      .mockResolvedValueOnce(zeroTotals);

    render(<ServicesScreen />);
    await screen.findByRole("table", { name: "Servicios activos" });

    const dialog = openDeleteDialog("Luz");
    fireEvent.click(within(dialog).getByRole("button", { name: "Eliminar" }));
    fireEvent.click(
      within(dialog).getByRole("button", { name: "Eliminando…" }),
    );

    expect(mockedDeleteService).toHaveBeenCalledTimes(1);
    expect(mockedListActiveServices).toHaveBeenCalledTimes(1);

    await act(async () => {
      pending.resolve({ outcome: "deleted", id: service.id });
    });
    await waitFor(() => {
      expect(mockedListActiveServices).toHaveBeenCalledTimes(2);
    });
  });
});

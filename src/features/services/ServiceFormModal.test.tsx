import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  createService,
  updateService,
  type ServiceRecordDto,
} from "../../shared/api";
import { ServiceFormModal } from "./ServiceFormModal";

vi.mock("../../shared/api", () => ({
  createService: vi.fn(),
  updateService: vi.fn(),
}));

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

const usdService: ServiceRecordDto = {
  ...service,
  id: "00000000-0000-4000-8000-000000000002",
  name: "Netflix",
  totalCost: { minorUnits: "2000", currencyCode: "USD" },
  myShare: {
    numeratorMinor: "2000",
    denominator: "1",
    currencyCode: "USD",
  },
  sharedPeopleCount: 1,
  notes: "familiar",
};

function fillValidForm({
  name = "Spotify",
  totalCost = "100,50",
  currencyCode = "ARS",
  people = "4",
  notes = "",
}: {
  name?: string;
  totalCost?: string;
  currencyCode?: "ARS" | "USD";
  people?: string;
  notes?: string;
} = {}) {
  fireEvent.change(screen.getByLabelText("Nombre"), {
    target: { value: name },
  });
  fireEvent.change(screen.getByLabelText("Costo total"), {
    target: { value: totalCost },
  });
  fireEvent.change(screen.getByLabelText("Moneda"), {
    target: { value: currencyCode },
  });
  fireEvent.change(screen.getByLabelText("Cantidad de personas"), {
    target: { value: people },
  });
  fireEvent.change(screen.getByLabelText("Notas"), {
    target: { value: notes },
  });
}

function submitForm() {
  fireEvent.click(screen.getByRole("button", { name: "Guardar" }));
}

function expectFieldError(label: string, message: string) {
  const field = screen.getByLabelText(label);
  const error = screen.getByText(message);

  expect(field).toHaveAttribute("aria-invalid", "true");
  expect(field).toHaveAttribute("aria-describedby", error.id);
  expect(error.id).not.toBe("");
}

describe("ServiceFormModal", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockedCreateService.mockResolvedValue(service);
    mockedUpdateService.mockResolvedValue(service);
  });

  it("renderiza el alta con etiquetas asociadas, ARS predeterminado y foco inicial", () => {
    const onClose = vi.fn();

    render(<ServiceFormModal onClose={onClose} onSaved={vi.fn()} />);

    expect(screen.getByRole("dialog", { name: "Nuevo servicio" })).toBeInTheDocument();
    expect(screen.getByLabelText("Nombre")).toHaveValue("");
    expect(screen.getByLabelText("Costo total")).toHaveValue("");
    expect(screen.getByLabelText("Moneda")).toHaveValue("ARS");
    expect(screen.getByLabelText("Moneda")).toBeEnabled();
    expect(screen.getByLabelText("Cantidad de personas")).toHaveValue("");
    expect(screen.getByLabelText("Notas")).toHaveValue("");
    expect(document.activeElement).toBe(screen.getByLabelText("Nombre"));

    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("renderiza la edición con los datos iniciales y la moneda deshabilitada", () => {
    render(
      <ServiceFormModal
        initialService={usdService}
        onClose={vi.fn()}
        onSaved={vi.fn()}
      />,
    );

    expect(screen.getByRole("dialog", { name: "Editar servicio" })).toBeInTheDocument();
    expect(screen.getByLabelText("Nombre")).toHaveValue("Netflix");
    expect(screen.getByLabelText("Costo total")).toHaveValue("20,00");
    expect(screen.getByLabelText("Moneda")).toHaveValue("USD");
    expect(screen.getByLabelText("Moneda")).toBeDisabled();
    expect(screen.getByLabelText("Cantidad de personas")).toHaveValue("1");
    expect(screen.getByLabelText("Notas")).toHaveValue("familiar");
  });

  it("cancela sin invocar comandos", () => {
    const onClose = vi.fn();

    render(<ServiceFormModal onClose={onClose} onSaved={vi.fn()} />);
    fireEvent.click(screen.getByRole("button", { name: "Cancelar" }));

    expect(onClose).toHaveBeenCalledTimes(1);
    expect(mockedCreateService).not.toHaveBeenCalled();
    expect(mockedUpdateService).not.toHaveBeenCalled();
  });

  it("envía el alta válida, cierra y no envía datos derivados", async () => {
    const onSaved = vi.fn();

    render(<ServiceFormModal onClose={vi.fn()} onSaved={onSaved} />);
    fillValidForm({ notes: "familiar" });
    submitForm();

    await waitFor(() => {
      expect(mockedCreateService).toHaveBeenCalledWith({
        name: "Spotify",
        totalCost: "100,50",
        currencyCode: "ARS",
        sharedPeopleCount: 4,
        notes: "familiar",
      });
    });
    expect(mockedUpdateService).not.toHaveBeenCalled();
    expect(onSaved).toHaveBeenCalledTimes(1);
  });

  it.each(["100,50", "1234.56"])(
    "conserva el importe exacto %s como cadena",
    async (totalCost) => {
      render(<ServiceFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
      fillValidForm({ totalCost });
      submitForm();

      await waitFor(() => {
        expect(mockedCreateService).toHaveBeenCalledWith(
          expect.objectContaining({ totalCost }),
        );
      });
    },
  );

  it("rechaza el nombre vacío antes de invocar el backend", () => {
    render(<ServiceFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ name: "" });
    submitForm();

    expectFieldError("Nombre", "El nombre del servicio es obligatorio.");
    expect(mockedCreateService).not.toHaveBeenCalled();
  });

  it("rechaza un costo inválido en el campo correspondiente", () => {
    render(<ServiceFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ totalCost: "100,999" });
    submitForm();

    expectFieldError(
      "Costo total",
      "Ingresá un importe válido con hasta dos decimales.",
    );
    expect(mockedCreateService).not.toHaveBeenCalled();
  });

  it("rechaza una cantidad de personas no entera", () => {
    render(<ServiceFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ people: "1,5" });
    submitForm();

    expectFieldError(
      "Cantidad de personas",
      "La cantidad de personas debe ser un entero mayor o igual a 1.",
    );
    expect(mockedCreateService).not.toHaveBeenCalled();
  });

  it("asocia el error de moneda del backend y conserva los datos", async () => {
    mockedCreateService.mockRejectedValue({
      code: "validation",
      message: "La moneda debe ser ARS o USD.",
      field: "currencyCode",
    });

    render(<ServiceFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ currencyCode: "ARS" });
    submitForm();

    await waitFor(() => {
      expectFieldError("Moneda", "La moneda debe ser ARS o USD.");
    });
    expect(screen.getByLabelText("Nombre")).toHaveValue("Spotify");
    expect(screen.getByLabelText("Costo total")).toHaveValue("100,50");
    expect(screen.getByLabelText("Cantidad de personas")).toHaveValue("4");
  });

  it("muestra errores sin campo como error general", async () => {
    mockedCreateService.mockRejectedValue({
      code: "internal",
      message: "No se pudo guardar el servicio.",
    });

    render(<ServiceFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm();
    submitForm();

    expect(
      await screen.findByText("No se pudo guardar el servicio."),
    ).toBeInTheDocument();
    expect(screen.getByLabelText("Nombre")).not.toHaveAttribute(
      "aria-invalid",
      "true",
    );
  });

  it("envía la edición sin currencyCode ni datos derivados", async () => {
    const onSaved = vi.fn();

    render(
      <ServiceFormModal
        initialService={service}
        onClose={vi.fn()}
        onSaved={onSaved}
      />,
    );
    submitForm();

    await waitFor(() => {
      expect(mockedUpdateService).toHaveBeenCalledWith(service.id, {
        name: "Luz",
        totalCost: "100,00",
        sharedPeopleCount: 3,
        notes: null,
      });
    });
    expect(mockedCreateService).not.toHaveBeenCalled();
    expect(onSaved).toHaveBeenCalledTimes(1);
  });

  it("asocia errores del backend durante la edición", async () => {
    mockedUpdateService.mockRejectedValue({
      code: "validation",
      message: "El costo total no puede ser negativo.",
      field: "totalCost",
    });

    render(
      <ServiceFormModal
        initialService={service}
        onClose={vi.fn()}
        onSaved={vi.fn()}
      />,
    );
    submitForm();

    await waitFor(() => {
      expectFieldError("Costo total", "El costo total no puede ser negativo.");
    });
    expect(screen.getByLabelText("Nombre")).toHaveValue("Luz");
    expect(screen.getByLabelText("Costo total")).toHaveValue("100,00");
  });

  it("evita envíos duplicados mientras el guardado está en curso", async () => {
    let resolveSubmit: (value: ServiceRecordDto) => void = () => {};
    mockedCreateService.mockReturnValue(
      new Promise<ServiceRecordDto>((resolve) => {
        resolveSubmit = resolve;
      }),
    );
    const onSaved = vi.fn();

    render(<ServiceFormModal onClose={vi.fn()} onSaved={onSaved} />);
    fillValidForm();
    const saveButton = screen.getByRole("button", { name: "Guardar" });
    fireEvent.click(saveButton);
    fireEvent.click(saveButton);

    expect(mockedCreateService).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("button", { name: "Guardando…" })).toBeDisabled();

    resolveSubmit(service);
    await waitFor(() => {
      expect(onSaved).toHaveBeenCalledTimes(1);
    });
  });
});

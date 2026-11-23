import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import {
  createInvestment,
  type InvestmentRecordDto,
  type InvestmentSummaryDto,
  updateInvestment,
} from "../../shared/api";
import { InvestmentFormModal } from "./InvestmentFormModal";

vi.mock("../../shared/api", () => ({
  createInvestment: vi.fn(),
  updateInvestment: vi.fn(),
}));

const mockedCreateInvestment = vi.mocked(createInvestment);
const mockedUpdateInvestment = vi.mocked(updateInvestment);

const investment: InvestmentRecordDto = {
  id: "00000000-0000-4000-8000-000000000001",
  month: "2026-03",
  currencyCode: "ARS",
  capital: { minorUnits: "100000", currencyCode: "ARS" },
  tna: "25,50",
  termDays: 30,
  taxes: { minorUnits: "5000", currencyCode: "ARS" },
  createdAt: "2026-11-23T12:00:00.000Z",
  updatedAt: "2026-11-23T12:00:00.000Z",
};

const usdInvestment: InvestmentRecordDto = {
  ...investment,
  id: "00000000-0000-4000-8000-000000000002",
  currencyCode: "USD",
  capital: { minorUnits: "100000", currencyCode: "USD" },
  taxes: { minorUnits: "5000", currencyCode: "USD" },
};

function fillValidForm({
  month = "2026-03",
  currencyCode = "ARS",
  capital = "1000,50",
  tna = "25,50",
  termDays = "30",
  taxes = "0,05",
}: {
  month?: string;
  currencyCode?: "ARS" | "USD";
  capital?: string;
  tna?: string;
  termDays?: string;
  taxes?: string;
} = {}) {
  fireEvent.change(screen.getByLabelText("Mes"), {
    target: { value: month },
  });
  fireEvent.change(screen.getByLabelText("Moneda"), {
    target: { value: currencyCode },
  });
  fireEvent.change(screen.getByLabelText("Capital"), {
    target: { value: capital },
  });
  fireEvent.change(screen.getByLabelText("TNA"), {
    target: { value: tna },
  });
  fireEvent.change(screen.getByLabelText("Plazo (días)"), {
    target: { value: termDays },
  });
  fireEvent.change(screen.getByLabelText("Impuestos"), {
    target: { value: taxes },
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

describe("InvestmentFormModal", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockedCreateInvestment.mockResolvedValue({} as InvestmentSummaryDto);
    mockedUpdateInvestment.mockResolvedValue({} as InvestmentSummaryDto);
  });

  it("renderiza el alta con seis campos, ARS predeterminado, ceros iniciales y foco inicial", () => {
    const onClose = vi.fn();

    render(<InvestmentFormModal onClose={onClose} onSaved={vi.fn()} />);

    const dialog = screen.getByRole("dialog", { name: "Nueva inversión" });
    expect(dialog).toHaveAttribute("aria-modal", "true");
    expect(screen.getByLabelText("Mes")).toHaveValue("");
    expect(screen.getByLabelText("Moneda")).toHaveValue("ARS");
    expect(screen.getByLabelText("Moneda")).toBeEnabled();
    expect(screen.getByLabelText("Capital")).toHaveValue("0");
    expect(screen.getByLabelText("TNA")).toHaveValue("0");
    expect(screen.getByLabelText("Plazo (días)")).toHaveValue("0");
    expect(screen.getByLabelText("Impuestos")).toHaveValue("0");
    expect(document.activeElement).toBe(screen.getByLabelText("Mes"));

    fireEvent.change(screen.getByLabelText("Moneda"), {
      target: { value: "USD" },
    });
    expect(screen.getByLabelText("Moneda")).toHaveValue("USD");

    fireEvent.keyDown(dialog, { key: "Escape" });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("renderiza la edición con datos precargados y moneda original deshabilitada", () => {
    render(
      <InvestmentFormModal
        initialInvestment={usdInvestment}
        onClose={vi.fn()}
        onSaved={vi.fn()}
      />,
    );

    expect(screen.getByRole("dialog", { name: "Editar inversión" })).toBeInTheDocument();
    expect(screen.getByLabelText("Mes")).toHaveValue("2026-03");
    expect(screen.getByLabelText("Moneda")).toHaveValue("USD");
    expect(screen.getByLabelText("Moneda")).toBeDisabled();
    expect(screen.getByLabelText("Capital")).toHaveValue("1000,00");
    expect(screen.getByLabelText("TNA")).toHaveValue("25,50");
    expect(screen.getByLabelText("Plazo (días)")).toHaveValue("30");
    expect(screen.getByLabelText("Impuestos")).toHaveValue("50,00");
    expect(screen.queryByLabelText("Rendimiento bruto")).not.toBeInTheDocument();
    expect(screen.queryByText("Rendimiento bruto")).not.toBeInTheDocument();
  });

  it("cancela sin invocar comandos", () => {
    const onClose = vi.fn();

    render(<InvestmentFormModal onClose={onClose} onSaved={vi.fn()} />);
    fireEvent.click(screen.getByRole("button", { name: "Cancelar" }));

    expect(onClose).toHaveBeenCalledTimes(1);
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
    expect(mockedUpdateInvestment).not.toHaveBeenCalled();
  });

  it("envía el alta válida con todos los datos de origen y notifica el guardado", async () => {
    const onSaved = vi.fn();

    render(<InvestmentFormModal onClose={vi.fn()} onSaved={onSaved} />);
    fillValidForm({ currencyCode: "USD" });
    submitForm();

    await waitFor(() => {
      expect(mockedCreateInvestment).toHaveBeenCalledWith({
        month: "2026-03",
        currencyCode: "USD",
        capital: "1000,50",
        tna: "25,50",
        termDays: "30",
        taxes: "0,05",
      });
    });
    expect(mockedUpdateInvestment).not.toHaveBeenCalled();
    expect(onSaved).toHaveBeenCalledTimes(1);
  });

  it.each(["1000,50", "1000.50"])(
    "conserva el importe decimal %s como cadena",
    async (capital) => {
      render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
      fillValidForm({ capital, tna: "25.50", taxes: "0.5" });
      submitForm();

      await waitFor(() => {
        expect(mockedCreateInvestment).toHaveBeenCalledWith(
          expect.objectContaining({ capital, tna: "25.50", taxes: "0.5" }),
        );
      });
    },
  );

  it("rechaza el mes vacío antes de invocar el backend", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ month: "" });
    submitForm();

    expectFieldError("Mes", "El mes es obligatorio.");
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it("rechaza un mes con formato inválido", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ month: "2026/03" });
    submitForm();

    expectFieldError("Mes", "El mes debe tener el formato YYYY-MM.");
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it("rechaza un capital inválido en el campo correspondiente", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ capital: "1000,999" });
    submitForm();

    expectFieldError(
      "Capital",
      "Ingresá un capital válido con hasta dos decimales.",
    );
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it("rechaza un capital negativo", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ capital: "-1" });
    submitForm();

    expectFieldError("Capital", "El capital no puede ser negativo.");
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it("rechaza una TNA no numérica", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ tna: "veinticinco" });
    submitForm();

    expectFieldError("TNA", "Ingresá una TNA válida con hasta dos decimales.");
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it("rechaza un plazo decimal", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ termDays: "30,5" });
    submitForm();

    expectFieldError("Plazo (días)", "El plazo debe ser un entero no negativo.");
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it("rechaza impuestos negativos", () => {
    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm({ taxes: "-1" });
    submitForm();

    expectFieldError("Impuestos", "Los impuestos no pueden ser negativos.");
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
  });

  it.each([
    { field: "month", label: "Mes", message: "Error del backend en mes." },
    {
      field: "currencyCode",
      label: "Moneda",
      message: "Error del backend en moneda.",
    },
    {
      field: "capital",
      label: "Capital",
      message: "Error del backend en capital.",
    },
    { field: "tna", label: "TNA", message: "Error del backend en TNA." },
    {
      field: "termDays",
      label: "Plazo (días)",
      message: "Error del backend en plazo.",
    },
    {
      field: "taxes",
      label: "Impuestos",
      message: "Error del backend en impuestos.",
    },
  ])("asocia el error del backend de $field al campo", async ({ field, label, message }) => {
    mockedCreateInvestment.mockRejectedValue({
      code: "validation",
      message,
      field,
    });

    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm();
    submitForm();

    await waitFor(() => {
      expectFieldError(label, message);
    });
    expect(screen.getByRole("dialog", { name: "Nueva inversión" })).toBeInTheDocument();
    expect(screen.getByLabelText("Capital")).toHaveValue("1000,50");
  });

  it("muestra errores sin campo como error general y mantiene el modal abierto", async () => {
    mockedCreateInvestment.mockRejectedValue({
      code: "internal",
      message: "No se pudo guardar la inversión.",
    });

    render(<InvestmentFormModal onClose={vi.fn()} onSaved={vi.fn()} />);
    fillValidForm();
    submitForm();

    expect(
      await screen.findByText("No se pudo guardar la inversión."),
    ).toBeInTheDocument();
    expect(screen.getByRole("dialog", { name: "Nueva inversión" })).toBeInTheDocument();
    expect(screen.getByLabelText("Capital")).not.toHaveAttribute(
      "aria-invalid",
      "true",
    );
  });

  it("envía la edición sin currencyCode ni resultados derivados", async () => {
    const onSaved = vi.fn();

    render(
      <InvestmentFormModal
        initialInvestment={usdInvestment}
        onClose={vi.fn()}
        onSaved={onSaved}
      />,
    );
    submitForm();

    await waitFor(() => {
      expect(mockedUpdateInvestment).toHaveBeenCalledWith(usdInvestment.id, {
        month: "2026-03",
        capital: "1000,00",
        tna: "25,50",
        termDays: "30",
        taxes: "50,00",
      });
    });
    expect(mockedCreateInvestment).not.toHaveBeenCalled();
    expect(onSaved).toHaveBeenCalledTimes(1);
  });

  it("evita envíos duplicados mientras el guardado está en curso", async () => {
    let resolveSubmit: (value: InvestmentSummaryDto) => void = () => {};
    mockedCreateInvestment.mockReturnValue(
      new Promise<InvestmentSummaryDto>((resolve) => {
        resolveSubmit = resolve;
      }),
    );
    const onSaved = vi.fn();

    render(<InvestmentFormModal onClose={vi.fn()} onSaved={onSaved} />);
    fillValidForm();
    const saveButton = screen.getByRole("button", { name: "Guardar" });
    fireEvent.click(saveButton);
    fireEvent.click(saveButton);

    expect(mockedCreateInvestment).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("button", { name: "Guardando…" })).toBeDisabled();
    expect(onSaved).not.toHaveBeenCalled();

    resolveSubmit({} as InvestmentSummaryDto);
    await waitFor(() => {
      expect(onSaved).toHaveBeenCalledTimes(1);
    });
  });
});

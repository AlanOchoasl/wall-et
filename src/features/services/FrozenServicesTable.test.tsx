import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { ServiceRecordDto } from "../../shared/api";
import { FrozenServicesTable } from "./FrozenServicesTable";

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
  status: "frozen",
  createdAt: "2026-11-23T12:00:00.000Z",
  updatedAt: "2026-11-23T12:00:00.000Z",
};

const usdService: ServiceRecordDto = {
  ...arsService,
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

describe("FrozenServicesTable", () => {
  it("muestra encabezados, una fila por servicio y estado congelado", () => {
    render(
      <FrozenServicesTable
        services={[arsService, usdService]}
        reactivatingId={null}
        onReactivate={vi.fn()}
        onDelete={vi.fn()}
      />,
    );

    expect(
      screen.getByRole("table", { name: "Servicios congelados" }),
    ).toBeInTheDocument();
    expect(screen.getAllByRole("row")).toHaveLength(3);
    expect(screen.getAllByText("Congelado")).toHaveLength(2);
    expect(
      screen.getByRole("columnheader", { name: "Cantidad de personas" }),
    ).toBeInTheDocument();
  });

  it("muestra los datos monetarios y las notas sin recalcularlos", () => {
    render(
      <FrozenServicesTable
        services={[arsService, usdService]}
        reactivatingId={null}
        onReactivate={vi.fn()}
        onDelete={vi.fn()}
      />,
    );

    expect(screen.getByText("Luz")).toBeInTheDocument();
    expect(screen.getByText("ARS")).toBeInTheDocument();
    expect(screen.getByText("ARS 100,00")).toBeInTheDocument();
    expect(screen.getByText("ARS 100,00 ÷ 3")).toBeInTheDocument();
    expect(screen.getByText("Netflix")).toBeInTheDocument();
    expect(screen.getByText("USD")).toBeInTheDocument();
    expect(screen.getByText("USD 50,00")).toBeInTheDocument();
    expect(screen.getByText("USD 50,00 ÷ 2")).toBeInTheDocument();
    expect(screen.getByText("Pago compartido")).toBeInTheDocument();
  });

  it("expone una acción accesible de reactivación por fila", () => {
    const onReactivate = vi.fn();

    render(
      <FrozenServicesTable
        services={[arsService, usdService]}
        reactivatingId={null}
        onReactivate={onReactivate}
        onDelete={vi.fn()}
      />,
    );

    const buttons = screen.getAllByRole("button", { name: /^Reactivar / });
    expect(buttons).toHaveLength(2);
    expect(buttons[0]).toHaveAccessibleName("Reactivar Luz");
    expect(buttons[1]).toHaveAccessibleName("Reactivar Netflix");

    fireEvent.click(buttons[1]);

    expect(onReactivate).toHaveBeenCalledTimes(1);
    expect(onReactivate).toHaveBeenCalledWith(usdService);
  });

  it("bloquea la reactivación mientras hay una operación en curso", () => {
    render(
      <FrozenServicesTable
        services={[arsService, usdService]}
        reactivatingId={arsService.id}
        onReactivate={vi.fn()}
        onDelete={vi.fn()}
      />,
    );

    const reactivateButtons = screen.getAllByRole("button", {
      name: /^Reactivar /,
    });

    expect(reactivateButtons[0]).toBeDisabled();
    expect(reactivateButtons[0]).toHaveAttribute("aria-busy", "true");
    expect(reactivateButtons[1]).toBeDisabled();
    expect(screen.getByText("Reactivando…")).toBeInTheDocument();
  });

  it("asocia la acción Eliminar con el servicio de cada fila", () => {
    const onDelete = vi.fn();

    render(
      <FrozenServicesTable
        services={[arsService, usdService]}
        reactivatingId={null}
        onReactivate={vi.fn()}
        onDelete={onDelete}
      />,
    );

    const buttons = screen.getAllByRole("button", { name: /^Eliminar / });
    expect(buttons).toHaveLength(2);
    expect(buttons[0]).toHaveAccessibleName("Eliminar Luz");
    expect(buttons[1]).toHaveAccessibleName("Eliminar Netflix");

    fireEvent.click(buttons[1]);

    expect(onDelete).toHaveBeenCalledTimes(1);
    expect(onDelete).toHaveBeenCalledWith(usdService);
  });

  it("bloquea la eliminación mientras hay una reactivación en curso", () => {
    render(
      <FrozenServicesTable
        services={[arsService]}
        reactivatingId={arsService.id}
        onReactivate={vi.fn()}
        onDelete={vi.fn()}
      />,
    );

    expect(screen.getByRole("button", { name: "Eliminar Luz" })).toBeDisabled();
  });

  it("no muestra acciones de congelación", () => {
    render(
      <FrozenServicesTable
        services={[arsService]}
        reactivatingId={null}
        onReactivate={vi.fn()}
        onDelete={vi.fn()}
      />,
    );

    expect(
      screen.queryByRole("button", { name: /congelar/i }),
    ).not.toBeInTheDocument();
  });
});

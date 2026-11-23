import { fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";

import type { ServiceRecordDto } from "../../shared/api";
import { ServiceDeleteConfirmationModal } from "./ServiceDeleteConfirmationModal";

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

describe("ServiceDeleteConfirmationModal", () => {
  it("renderiza un diálogo accesible con el nombre del servicio y foco en Cancelar", () => {
    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error={null}
        busy={false}
        onCancel={vi.fn()}
        onConfirm={vi.fn()}
      />,
    );

    const dialog = screen.getByRole("dialog", { name: "Eliminar servicio" });

    expect(dialog).toHaveAttribute("aria-modal", "true");
    expect(screen.getByText(/Luz/)).toBeInTheDocument();
    expect(document.activeElement).toBe(
      screen.getByRole("button", { name: "Cancelar" }),
    );
  });

  it("confirma la eliminación", () => {
    const onConfirm = vi.fn();

    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error={null}
        busy={false}
        onCancel={vi.fn()}
        onConfirm={onConfirm}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Eliminar" }));

    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("cancela sin confirmar", () => {
    const onCancel = vi.fn();
    const onConfirm = vi.fn();

    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error={null}
        busy={false}
        onCancel={onCancel}
        onConfirm={onConfirm}
      />,
    );

    fireEvent.click(screen.getByRole("button", { name: "Cancelar" }));

    expect(onCancel).toHaveBeenCalledTimes(1);
    expect(onConfirm).not.toHaveBeenCalled();
  });

  it("cancela con Escape", () => {
    const onCancel = vi.fn();

    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error={null}
        busy={false}
        onCancel={onCancel}
        onConfirm={vi.fn()}
      />,
    );

    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });

    expect(onCancel).toHaveBeenCalledTimes(1);
  });

  it("evita confirmaciones duplicadas", () => {
    const onConfirm = vi.fn();

    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error={null}
        busy={false}
        onCancel={vi.fn()}
        onConfirm={onConfirm}
      />,
    );

    const confirmButton = screen.getByRole("button", { name: "Eliminar" });
    fireEvent.click(confirmButton);
    fireEvent.click(confirmButton);

    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("bloquea las acciones mientras se elimina", () => {
    const onCancel = vi.fn();
    const onConfirm = vi.fn();

    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error={null}
        busy
        onCancel={onCancel}
        onConfirm={onConfirm}
      />,
    );

    const dialog = screen.getByRole("dialog", { name: "Eliminar servicio" });

    expect(screen.getByRole("button", { name: "Cancelar" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Eliminando…" })).toBeDisabled();

    fireEvent.click(screen.getByRole("button", { name: "Eliminando…" }));
    fireEvent.keyDown(dialog, { key: "Escape" });

    expect(onConfirm).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();
  });

  it("muestra errores en un alert accesible", () => {
    render(
      <ServiceDeleteConfirmationModal
        service={service}
        error="No se pudo eliminar el servicio Luz."
        busy={false}
        onCancel={vi.fn()}
        onConfirm={vi.fn()}
      />,
    );

    expect(screen.getByRole("alert")).toHaveTextContent(
      "No se pudo eliminar el servicio Luz.",
    );
  });

  it("devuelve el foco al botón que abrió el diálogo al cancelar", () => {
    function Harness() {
      const [open, setOpen] = useState(false);

      return (
        <>
          <button type="button" onClick={() => setOpen(true)}>
            Abrir
          </button>
          {open ? (
            <ServiceDeleteConfirmationModal
              service={service}
              error={null}
              busy={false}
              onCancel={() => setOpen(false)}
              onConfirm={vi.fn()}
            />
          ) : null}
        </>
      );
    }

    render(<Harness />);
    const trigger = screen.getByRole("button", { name: "Abrir" });
    trigger.focus();

    fireEvent.click(trigger);

    expect(document.activeElement).toBe(
      screen.getByRole("button", { name: "Cancelar" }),
    );

    fireEvent.click(screen.getByRole("button", { name: "Cancelar" }));

    expect(document.activeElement).toBe(trigger);
  });
});

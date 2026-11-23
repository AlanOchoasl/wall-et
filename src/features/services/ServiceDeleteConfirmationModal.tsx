import { useEffect, useId, useRef } from "react";
import type { FormEvent, KeyboardEvent } from "react";

import type { ServiceRecordDto } from "../../shared/api";

const FOCUSABLE_SELECTOR =
  'button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [href], [tabindex]:not([tabindex="-1"])';

type ServiceDeleteConfirmationModalProps = {
  service: ServiceRecordDto;
  error: string | null;
  busy: boolean;
  onCancel: () => void;
  onConfirm: () => void;
};

export function ServiceDeleteConfirmationModal({
  service,
  error,
  busy,
  onCancel,
  onConfirm,
}: ServiceDeleteConfirmationModalProps) {
  const generatedId = useId();
  const dialogRef = useRef<HTMLDivElement>(null);
  const cancelRef = useRef<HTMLButtonElement>(null);
  const submittingRef = useRef(false);

  const titleId = `${generatedId}-title`;
  const descriptionId = `${generatedId}-description`;

  useEffect(() => {
    const previouslyFocused =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;

    cancelRef.current?.focus();

    return () => {
      previouslyFocused?.focus();
    };
  }, []);

  useEffect(() => {
    if (!busy) {
      submittingRef.current = false;
    }
  }, [busy]);

  function handleKeyDown(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") {
      if (!busy) {
        event.preventDefault();
        onCancel();
      }

      return;
    }

    if (event.key !== "Tab") {
      return;
    }

    const focusable = Array.from(
      dialogRef.current?.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR) ?? [],
    );

    if (focusable.length === 0) {
      return;
    }

    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    const active = document.activeElement;

    if (event.shiftKey && active === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && active === last) {
      event.preventDefault();
      first.focus();
    }
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (busy || submittingRef.current) {
      return;
    }

    submittingRef.current = true;
    onConfirm();
  }

  return (
    <div className="modal-backdrop">
      <div
        className="service-modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        aria-describedby={descriptionId}
        aria-busy={busy ? "true" : undefined}
        ref={dialogRef}
        onKeyDown={handleKeyDown}
      >
        <h2 id={titleId}>Eliminar servicio</h2>

        <form
          className="service-form"
          noValidate
          onSubmit={handleSubmit}
          aria-busy={busy}
        >
          <p id={descriptionId}>
            ¿Eliminar &quot;{service.name}&quot; de forma permanente? Esta acción
            no se puede deshacer.
          </p>

          {error ? (
            <p className="form__error" role="alert">
              {error}
            </p>
          ) : null}

          <div className="service-form__actions">
            <button
              type="button"
              ref={cancelRef}
              className="button button--secondary"
              onClick={onCancel}
              disabled={busy}
            >
              Cancelar
            </button>
            <button
              type="submit"
              className="button button--danger"
              disabled={busy}
            >
              {busy ? "Eliminando…" : "Eliminar"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

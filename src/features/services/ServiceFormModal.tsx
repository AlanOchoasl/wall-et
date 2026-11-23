import { useEffect, useId, useRef, useState } from "react";
import type { FormEvent, KeyboardEvent } from "react";

import {
  createService,
  type CurrencyCode,
  type ServiceRecordDto,
  updateService,
} from "../../shared/api";
import { toCommandError } from "../../shared/commands";

const FIELD_NAMES = [
  "name",
  "totalCost",
  "currencyCode",
  "sharedPeopleCount",
] as const;

type FieldName = (typeof FIELD_NAMES)[number];
type FieldErrors = Partial<Record<FieldName, string>>;

type ServiceFormModalProps = {
  initialService?: ServiceRecordDto;
  onClose: () => void;
  onSaved: () => void;
};

type FormValues = {
  name: string;
  totalCost: string;
  currencyCode: CurrencyCode;
  sharedPeopleCount: string;
  notes: string;
};

const COST_PATTERN = /^-?\d+(?:[.,]\d{1,2})?$/;
const PEOPLE_PATTERN = /^\d+$/;
const FOCUSABLE_SELECTOR =
  'button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [href], [tabindex]:not([tabindex="-1"])';

function isFieldName(value: string): value is FieldName {
  return (FIELD_NAMES as readonly string[]).includes(value);
}

function parsePeopleCount(value: string): number | null {
  const normalized = value.trim();

  if (!PEOPLE_PATTERN.test(normalized)) {
    return null;
  }

  const parsed = Number(normalized);

  if (!Number.isSafeInteger(parsed) || parsed < 1) {
    return null;
  }

  return parsed;
}

function majorDecimalFromMinorUnits(minorUnits: string): string {
  const negative = minorUnits.startsWith("-");
  const digits = minorUnits.replace(/^-/, "").replace(/^0+(?=\d)/, "");

  if (!/^\d+$/.test(digits)) {
    return minorUnits;
  }

  const padded = digits.padStart(3, "0");

  return `${negative ? "-" : ""}${padded.slice(0, -2)},${padded.slice(-2)}`;
}

function validate(values: FormValues): FieldErrors {
  const errors: FieldErrors = {};
  const name = values.name.trim();
  const totalCost = values.totalCost.trim();

  if (!name) {
    errors.name = "El nombre del servicio es obligatorio.";
  }

  if (!totalCost) {
    errors.totalCost = "El costo total es obligatorio.";
  } else if (!COST_PATTERN.test(totalCost)) {
    errors.totalCost = "Ingresá un importe válido con hasta dos decimales.";
  } else if (totalCost.startsWith("-")) {
    errors.totalCost = "El costo total no puede ser negativo.";
  }

  if (values.currencyCode !== "ARS" && values.currencyCode !== "USD") {
    errors.currencyCode = "La moneda debe ser ARS o USD.";
  }

  if (parsePeopleCount(values.sharedPeopleCount) === null) {
    errors.sharedPeopleCount =
      "La cantidad de personas debe ser un entero mayor o igual a 1.";
  }

  return errors;
}

export function ServiceFormModal({
  initialService,
  onClose,
  onSaved,
}: ServiceFormModalProps) {
  const generatedId = useId();
  const dialogRef = useRef<HTMLDivElement>(null);
  const nameRef = useRef<HTMLInputElement>(null);
  const submittingRef = useRef(false);

  const [values, setValues] = useState<FormValues>(() => ({
    name: initialService?.name ?? "",
    totalCost: initialService
      ? majorDecimalFromMinorUnits(initialService.totalCost.minorUnits)
      : "",
    currencyCode: initialService?.totalCost.currencyCode ?? "ARS",
    sharedPeopleCount: String(initialService?.sharedPeopleCount ?? ""),
    notes: initialService?.notes ?? "",
  }));
  const [errors, setErrors] = useState<FieldErrors>({});
  const [generalError, setGeneralError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const titleId = `${generatedId}-title`;
  const nameId = `${generatedId}-name`;
  const nameErrorId = `${generatedId}-name-error`;
  const totalCostId = `${generatedId}-total-cost`;
  const totalCostErrorId = `${generatedId}-total-cost-error`;
  const currencyId = `${generatedId}-currency`;
  const currencyErrorId = `${generatedId}-currency-error`;
  const peopleId = `${generatedId}-people`;
  const peopleErrorId = `${generatedId}-people-error`;
  const notesId = `${generatedId}-notes`;

  useEffect(() => {
    const previouslyFocused =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;

    nameRef.current?.focus();

    return () => {
      previouslyFocused?.focus();
    };
  }, []);

  function updateValue<K extends keyof FormValues>(field: K, value: FormValues[K]) {
    setValues((current) => ({ ...current, [field]: value }));
    setErrors((current) => ({ ...current, [field]: undefined }));
    setGeneralError(null);
  }

  function handleKeyDown(event: KeyboardEvent<HTMLDivElement>) {
    if (event.key === "Escape") {
      if (!submitting) {
        event.preventDefault();
        onClose();
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

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (submittingRef.current) {
      return;
    }

    const validationErrors = validate(values);

    if (Object.keys(validationErrors).length > 0) {
      setErrors(validationErrors);
      setGeneralError(null);
      return;
    }

    const sharedPeopleCount = parsePeopleCount(values.sharedPeopleCount);

    if (sharedPeopleCount === null) {
      setErrors({
        sharedPeopleCount:
          "La cantidad de personas debe ser un entero mayor o igual a 1.",
      });
      return;
    }

    const notes = values.notes.trim() === "" ? null : values.notes;
    const name = values.name.trim();
    const totalCost = values.totalCost.trim();

    submittingRef.current = true;
    setSubmitting(true);
    setErrors({});
    setGeneralError(null);

    try {
      if (initialService) {
        await updateService(initialService.id, {
          name,
          totalCost,
          sharedPeopleCount,
          notes,
        });
      } else {
        await createService({
          name,
          totalCost,
          currencyCode: values.currencyCode,
          sharedPeopleCount,
          notes,
        });
      }

      setSubmitting(false);
      submittingRef.current = false;
      onSaved();
    } catch (reason: unknown) {
      const error = toCommandError(reason);

      if (error.field && isFieldName(error.field)) {
        setErrors({ [error.field]: error.message });
      } else {
        setGeneralError(error.message);
      }

      setSubmitting(false);
      submittingRef.current = false;
    }
  }

  return (
    <div className="modal-backdrop">
      <div
        className="service-modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        ref={dialogRef}
        onKeyDown={handleKeyDown}
      >
        <h2 id={titleId}>
          {initialService ? "Editar servicio" : "Nuevo servicio"}
        </h2>

        <form className="service-form" noValidate onSubmit={handleSubmit} aria-busy={submitting}>
          {generalError ? (
            <p className="form__error" role="alert">
              {generalError}
            </p>
          ) : null}

          <div className="field">
            <label htmlFor={nameId}>Nombre</label>
            <input
              id={nameId}
              ref={nameRef}
              value={values.name}
              onChange={(event) => updateValue("name", event.target.value)}
              aria-invalid={Boolean(errors.name)}
              aria-describedby={errors.name ? nameErrorId : undefined}
              disabled={submitting}
            />
            {errors.name ? (
              <p className="field__error" id={nameErrorId} role="alert">
                {errors.name}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={totalCostId}>Costo total</label>
            <input
              id={totalCostId}
              type="text"
              inputMode="decimal"
              value={values.totalCost}
              onChange={(event) => updateValue("totalCost", event.target.value)}
              aria-invalid={Boolean(errors.totalCost)}
              aria-describedby={errors.totalCost ? totalCostErrorId : undefined}
              disabled={submitting}
            />
            {errors.totalCost ? (
              <p className="field__error" id={totalCostErrorId} role="alert">
                {errors.totalCost}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={currencyId}>Moneda</label>
            <select
              id={currencyId}
              value={values.currencyCode}
              onChange={(event) =>
                updateValue("currencyCode", event.target.value as CurrencyCode)
              }
              aria-invalid={Boolean(errors.currencyCode)}
              aria-describedby={errors.currencyCode ? currencyErrorId : undefined}
              disabled={Boolean(initialService) || submitting}
            >
              <option value="ARS">ARS</option>
              <option value="USD">USD</option>
            </select>
            {errors.currencyCode ? (
              <p className="field__error" id={currencyErrorId} role="alert">
                {errors.currencyCode}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={peopleId}>Cantidad de personas</label>
            <input
              id={peopleId}
              type="text"
              inputMode="numeric"
              value={values.sharedPeopleCount}
              onChange={(event) =>
                updateValue("sharedPeopleCount", event.target.value)
              }
              aria-invalid={Boolean(errors.sharedPeopleCount)}
              aria-describedby={
                errors.sharedPeopleCount ? peopleErrorId : undefined
              }
              disabled={submitting}
            />
            {errors.sharedPeopleCount ? (
              <p className="field__error" id={peopleErrorId} role="alert">
                {errors.sharedPeopleCount}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={notesId}>Notas</label>
            <textarea
              id={notesId}
              rows={3}
              value={values.notes}
              onChange={(event) => updateValue("notes", event.target.value)}
              disabled={submitting}
            />
          </div>

          <div className="service-form__actions">
            <button
              type="button"
              className="button button--secondary"
              onClick={onClose}
              disabled={submitting}
            >
              Cancelar
            </button>
            <button
              type="submit"
              className="button button--primary"
              disabled={submitting}
            >
              {submitting ? "Guardando…" : "Guardar"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

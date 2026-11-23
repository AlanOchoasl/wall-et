import { useEffect, useId, useRef, useState } from "react";
import type { FormEvent, KeyboardEvent } from "react";

import {
  createInvestment,
  type CurrencyCode,
  type InvestmentRecordDto,
  updateInvestment,
} from "../../shared/api";
import { toCommandError } from "../../shared/commands";

const FIELD_NAMES = [
  "month",
  "currencyCode",
  "capital",
  "tna",
  "termDays",
  "taxes",
] as const;

type FieldName = (typeof FIELD_NAMES)[number];
type FieldErrors = Partial<Record<FieldName, string>>;

type InvestmentFormModalProps = {
  initialInvestment?: InvestmentRecordDto;
  onClose: () => void;
  onSaved: () => void;
};

type FormValues = {
  month: string;
  currencyCode: CurrencyCode;
  capital: string;
  tna: string;
  termDays: string;
  taxes: string;
};

const MONTH_PATTERN = /^\d{4}-\d{2}$/;
const DECIMAL_PATTERN = /^-?\d+(?:[.,]\d{1,2})?$/;
const TERM_PATTERN = /^\d+$/;
const FOCUSABLE_SELECTOR =
  'button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [href], [tabindex]:not([tabindex="-1"])';

function isFieldName(value: string): value is FieldName {
  return (FIELD_NAMES as readonly string[]).includes(value);
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
  const month = values.month.trim();
  const capital = values.capital.trim();
  const tna = values.tna.trim();
  const termDays = values.termDays.trim();
  const taxes = values.taxes.trim();

  if (!month) {
    errors.month = "El mes es obligatorio.";
  } else if (!MONTH_PATTERN.test(month)) {
    errors.month = "El mes debe tener el formato YYYY-MM.";
  }

  if (values.currencyCode !== "ARS" && values.currencyCode !== "USD") {
    errors.currencyCode = "La moneda debe ser ARS o USD.";
  }

  if (!capital) {
    errors.capital = "El capital es obligatorio.";
  } else if (!DECIMAL_PATTERN.test(capital)) {
    errors.capital = "Ingresá un capital válido con hasta dos decimales.";
  } else if (capital.startsWith("-")) {
    errors.capital = "El capital no puede ser negativo.";
  }

  if (!tna) {
    errors.tna = "La TNA es obligatoria.";
  } else if (!DECIMAL_PATTERN.test(tna)) {
    errors.tna = "Ingresá una TNA válida con hasta dos decimales.";
  } else if (tna.startsWith("-")) {
    errors.tna = "La TNA no puede ser negativa.";
  }

  if (!termDays) {
    errors.termDays = "El plazo es obligatorio.";
  } else if (!TERM_PATTERN.test(termDays)) {
    errors.termDays = "El plazo debe ser un entero no negativo.";
  }

  if (!taxes) {
    errors.taxes = "Los impuestos son obligatorios.";
  } else if (!DECIMAL_PATTERN.test(taxes)) {
    errors.taxes = "Ingresá impuestos válidos con hasta dos decimales.";
  } else if (taxes.startsWith("-")) {
    errors.taxes = "Los impuestos no pueden ser negativos.";
  }

  return errors;
}

export function InvestmentFormModal({
  initialInvestment,
  onClose,
  onSaved,
}: InvestmentFormModalProps) {
  const generatedId = useId();
  const dialogRef = useRef<HTMLDivElement>(null);
  const monthRef = useRef<HTMLInputElement>(null);
  const submittingRef = useRef(false);

  const [values, setValues] = useState<FormValues>(() => ({
    month: initialInvestment?.month ?? "",
    currencyCode: initialInvestment?.currencyCode ?? "ARS",
    capital: initialInvestment
      ? majorDecimalFromMinorUnits(initialInvestment.capital.minorUnits)
      : "0",
    tna: initialInvestment?.tna ?? "0",
    termDays: initialInvestment ? String(initialInvestment.termDays) : "0",
    taxes: initialInvestment
      ? majorDecimalFromMinorUnits(initialInvestment.taxes.minorUnits)
      : "0",
  }));
  const [errors, setErrors] = useState<FieldErrors>({});
  const [generalError, setGeneralError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const titleId = `${generatedId}-title`;
  const monthId = `${generatedId}-month`;
  const monthErrorId = `${generatedId}-month-error`;
  const currencyId = `${generatedId}-currency`;
  const currencyErrorId = `${generatedId}-currency-error`;
  const capitalId = `${generatedId}-capital`;
  const capitalErrorId = `${generatedId}-capital-error`;
  const tnaId = `${generatedId}-tna`;
  const tnaErrorId = `${generatedId}-tna-error`;
  const termDaysId = `${generatedId}-term-days`;
  const termDaysErrorId = `${generatedId}-term-days-error`;
  const taxesId = `${generatedId}-taxes`;
  const taxesErrorId = `${generatedId}-taxes-error`;

  useEffect(() => {
    const previouslyFocused =
      document.activeElement instanceof HTMLElement
        ? document.activeElement
        : null;

    monthRef.current?.focus();

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

    const month = values.month.trim();
    const capital = values.capital.trim();
    const tna = values.tna.trim();
    const termDays = values.termDays.trim();
    const taxes = values.taxes.trim();

    submittingRef.current = true;
    setSubmitting(true);
    setErrors({});
    setGeneralError(null);

    try {
      if (initialInvestment) {
        await updateInvestment(initialInvestment.id, {
          month,
          capital,
          tna,
          termDays,
          taxes,
        });
      } else {
        await createInvestment({
          month,
          currencyCode: values.currencyCode,
          capital,
          tna,
          termDays,
          taxes,
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
          {initialInvestment ? "Editar inversión" : "Nueva inversión"}
        </h2>

        <form className="service-form" noValidate onSubmit={handleSubmit} aria-busy={submitting}>
          {generalError ? (
            <p className="form__error" role="alert">
              {generalError}
            </p>
          ) : null}

          <div className="field">
            <label htmlFor={monthId}>Mes</label>
            <input
              id={monthId}
              ref={monthRef}
              type="text"
              inputMode="numeric"
              value={values.month}
              onChange={(event) => updateValue("month", event.target.value)}
              aria-invalid={Boolean(errors.month)}
              aria-describedby={errors.month ? monthErrorId : undefined}
              disabled={submitting}
            />
            {errors.month ? (
              <p className="field__error" id={monthErrorId} role="alert">
                {errors.month}
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
              disabled={Boolean(initialInvestment) || submitting}
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
            <label htmlFor={capitalId}>Capital</label>
            <input
              id={capitalId}
              type="text"
              inputMode="decimal"
              value={values.capital}
              onChange={(event) => updateValue("capital", event.target.value)}
              aria-invalid={Boolean(errors.capital)}
              aria-describedby={errors.capital ? capitalErrorId : undefined}
              disabled={submitting}
            />
            {errors.capital ? (
              <p className="field__error" id={capitalErrorId} role="alert">
                {errors.capital}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={tnaId}>TNA</label>
            <input
              id={tnaId}
              type="text"
              inputMode="decimal"
              value={values.tna}
              onChange={(event) => updateValue("tna", event.target.value)}
              aria-invalid={Boolean(errors.tna)}
              aria-describedby={errors.tna ? tnaErrorId : undefined}
              disabled={submitting}
            />
            {errors.tna ? (
              <p className="field__error" id={tnaErrorId} role="alert">
                {errors.tna}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={termDaysId}>Plazo (días)</label>
            <input
              id={termDaysId}
              type="text"
              inputMode="numeric"
              value={values.termDays}
              onChange={(event) => updateValue("termDays", event.target.value)}
              aria-invalid={Boolean(errors.termDays)}
              aria-describedby={errors.termDays ? termDaysErrorId : undefined}
              disabled={submitting}
            />
            {errors.termDays ? (
              <p className="field__error" id={termDaysErrorId} role="alert">
                {errors.termDays}
              </p>
            ) : null}
          </div>

          <div className="field">
            <label htmlFor={taxesId}>Impuestos</label>
            <input
              id={taxesId}
              type="text"
              inputMode="decimal"
              value={values.taxes}
              onChange={(event) => updateValue("taxes", event.target.value)}
              aria-invalid={Boolean(errors.taxes)}
              aria-describedby={errors.taxes ? taxesErrorId : undefined}
              disabled={submitting}
            />
            {errors.taxes ? (
              <p className="field__error" id={taxesErrorId} role="alert">
                {errors.taxes}
              </p>
            ) : null}
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

import { useEffect, useRef, useState } from "react";

import {
  getServiceTotals,
  listActiveServices,
  listFrozenServices,
  reactivateService,
  type ServiceRecordDto,
  type ServiceTotalsDto,
} from "../../shared/api";
import { ActiveServicesTable } from "./ActiveServicesTable";
import { FrozenServicesTable } from "./FrozenServicesTable";
import { ServiceFormModal } from "./ServiceFormModal";
import { ServiceTotalsSummary } from "./ServiceTotalsSummary";

type ModalState =
  | { mode: "create" }
  | { mode: "edit"; service: ServiceRecordDto }
  | null;

type ServiceView = "active" | "frozen";

function errorMessage(reason: unknown, fallback: string): string {
  return reason instanceof Error ? reason.message : fallback;
}

export function ServicesScreen() {
  const [view, setView] = useState<ServiceView>("active");
  const [activeServices, setActiveServices] = useState<ServiceRecordDto[] | null>(
    null,
  );
  const [frozenServices, setFrozenServices] = useState<ServiceRecordDto[] | null>(
    null,
  );
  const [totals, setTotals] = useState<ServiceTotalsDto | null>(null);
  const [totalsError, setTotalsError] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [modal, setModal] = useState<ModalState>(null);
  const [reloadToken, setReloadToken] = useState(0);
  const [frozenToken, setFrozenToken] = useState(0);
  const [reactivatingId, setReactivatingId] = useState<string | null>(null);
  const pendingReactivation = useRef<ServiceRecordDto | null>(null);

  useEffect(() => {
    if (view !== "active") {
      return;
    }

    let current = true;

    listActiveServices()
      .then((records) => {
        if (!current) {
          return;
        }

        setActiveServices(records);

        const reactivated = pendingReactivation.current;
        if (reactivated) {
          setFrozenServices((previous) =>
            previous
              ? previous.filter((service) => service.id !== reactivated.id)
              : previous,
          );
          pendingReactivation.current = null;
        }

        setError(null);
      })
      .catch((reason: unknown) => {
        if (!current) {
          return;
        }

        const message = errorMessage(
          reason,
          "No se pudieron cargar los servicios activos.",
        );
        const reactivated = pendingReactivation.current;

        if (reactivated) {
          pendingReactivation.current = null;
          setFrozenServices(null);
          setView("frozen");
          setFrozenToken((token) => token + 1);
          setError(
            `El servicio se reactivó, pero no se pudieron cargar los servicios activos: ${message}`,
          );
        } else {
          setError(message);
        }
      })
      .finally(() => {
        if (current) {
          setReactivatingId(null);
        }
      });

    return () => {
      current = false;
    };
  }, [view, reloadToken]);

  useEffect(() => {
    if (view !== "active") {
      return;
    }

    let current = true;

    getServiceTotals()
      .then((value) => {
        if (current) {
          setTotals(value);
        }
      })
      .catch((reason: unknown) => {
        if (current) {
          setTotals(null);
          setTotalsError(
            errorMessage(reason, "No se pudieron cargar los totales."),
          );
        }
      });

    return () => {
      current = false;
    };
  }, [view, reloadToken]);

  useEffect(() => {
    if (view !== "frozen" || frozenToken === 0) {
      return;
    }

    let current = true;

    listFrozenServices()
      .then((records) => {
        if (current) {
          setFrozenServices(records);
        }
      })
      .catch((reason: unknown) => {
        if (current) {
          setFrozenServices([]);
          setError(
            errorMessage(
              reason,
              "No se pudieron cargar los servicios congelados.",
            ),
          );
        }
      });

    return () => {
      current = false;
    };
  }, [view, frozenToken]);

  function handleViewChange(nextView: ServiceView) {
    if (nextView === view || reactivatingId !== null) {
      return;
    }

    setError(null);
    setTotals(null);
    setTotalsError(null);
    setView(nextView);

    if (nextView === "active") {
      setActiveServices(null);
    } else {
      setFrozenServices(null);
      setFrozenToken((token) => token + 1);
    }
  }

  async function handleReactivate(service: ServiceRecordDto) {
    if (reactivatingId !== null) {
      return;
    }

    setReactivatingId(service.id);
    setError(null);

    let result;
    try {
      result = await reactivateService(service.id);
    } catch (reason: unknown) {
      setError(
        errorMessage(
          reason,
          `No se pudo reactivar el servicio ${service.name}.`,
        ),
      );
      setReactivatingId(null);
      return;
    }

    if (
      result.outcome !== "reactivated" &&
      result.outcome !== "already_active"
    ) {
      setError(`No se pudo reactivar el servicio ${service.name}.`);
      setReactivatingId(null);
      return;
    }

    pendingReactivation.current = service;
    setTotals(null);
    setTotalsError(null);
    setView("active");
  }

  function handleSaved() {
    setModal(null);
    setError(null);
    setTotals(null);
    setTotalsError(null);
    setReloadToken((token) => token + 1);
  }

  const showingFrozen = view === "frozen";
  const services = showingFrozen ? frozenServices : activeServices;
  const loadingAfterReactivation = reactivatingId !== null && !showingFrozen;
  const loadingText = showingFrozen
    ? "Cargando servicios congelados…"
    : "Cargando servicios activos…";
  const emptyText = showingFrozen
    ? "No hay servicios congelados."
    : "No hay servicios activos.";

  return (
    <section className="services-screen" aria-labelledby="services-title">
      <div className="services-screen__header">
        <h2 id="services-title">Servicios</h2>
        <button
          type="button"
          className="button button--primary"
          onClick={() => setModal({ mode: "create" })}
        >
          Nuevo servicio
        </button>
      </div>

      {!showingFrozen ? (
        <ServiceTotalsSummary totals={totals} error={totalsError} />
      ) : null}

      <div
        className="services-view-switch"
        role="group"
        aria-label="Estado de los servicios"
      >
        <button
          type="button"
          className="button button--secondary button--compact"
          aria-pressed={!showingFrozen}
          disabled={reactivatingId !== null}
          onClick={() => handleViewChange("active")}
        >
          Activos
        </button>
        <button
          type="button"
          className="button button--secondary button--compact"
          aria-pressed={showingFrozen}
          disabled={reactivatingId !== null}
          onClick={() => handleViewChange("frozen")}
        >
          Congelados
        </button>
      </div>

      {error ? <p role="alert">{error}</p> : null}

      {!error && (!services || loadingAfterReactivation) ? (
        <p>{loadingText}</p>
      ) : null}

      {!error && !loadingAfterReactivation && services && services.length === 0 ? (
        <p>{emptyText}</p>
      ) : null}

      {!loadingAfterReactivation && services && services.length > 0 ? (
        showingFrozen ? (
          <FrozenServicesTable
            services={services}
            reactivatingId={reactivatingId}
            onReactivate={handleReactivate}
          />
        ) : (
          <ActiveServicesTable
            services={services}
            onEdit={(service) => setModal({ mode: "edit", service })}
          />
        )
      ) : null}

      {modal ? (
        <ServiceFormModal
          key={modal.mode === "edit" ? modal.service.id : "create"}
          initialService={modal.mode === "edit" ? modal.service : undefined}
          onClose={() => setModal(null)}
          onSaved={handleSaved}
        />
      ) : null}
    </section>
  );
}

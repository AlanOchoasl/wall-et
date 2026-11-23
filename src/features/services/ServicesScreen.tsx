import { useEffect, useState } from "react";

import { listActiveServices, type ServiceRecordDto } from "../../shared/api";
import { ActiveServicesTable } from "./ActiveServicesTable";
import { ServiceFormModal } from "./ServiceFormModal";

type ModalState =
  | { mode: "create" }
  | { mode: "edit"; service: ServiceRecordDto }
  | null;

export function ServicesScreen() {
  const [services, setServices] = useState<ServiceRecordDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [modal, setModal] = useState<ModalState>(null);
  const [reloadToken, setReloadToken] = useState(0);

  useEffect(() => {
    let active = true;

    listActiveServices()
      .then((records) => {
        if (active) {
          setServices(records);
        }
      })
      .catch((reason: unknown) => {
        if (active) {
          setError(
            reason instanceof Error
              ? reason.message
              : "No se pudieron cargar los servicios activos.",
          );
        }
      });

    return () => {
      active = false;
    };
  }, [reloadToken]);

  function handleSaved() {
    setModal(null);
    setError(null);
    setReloadToken((token) => token + 1);
  }

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

      {error ? <p role="alert">{error}</p> : null}

      {!error && !services ? <p>Cargando servicios activos…</p> : null}

      {services && services.length === 0 ? (
        <p>No hay servicios activos.</p>
      ) : null}

      {services && services.length > 0 ? (
        <ActiveServicesTable
          services={services}
          onEdit={(service) => setModal({ mode: "edit", service })}
        />
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

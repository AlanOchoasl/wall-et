import { useEffect, useState } from "react";

import { listActiveServices, type ServiceRecordDto } from "../../shared/api";
import { ActiveServicesTable } from "./ActiveServicesTable";

export function ServicesScreen() {
  const [services, setServices] = useState<ServiceRecordDto[] | null>(null);
  const [error, setError] = useState<string | null>(null);

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
  }, []);

  return (
    <section className="services-screen" aria-labelledby="services-title">
      <h2 id="services-title">Servicios</h2>

      {error ? <p role="alert">{error}</p> : null}

      {!error && !services ? <p>Cargando servicios activos…</p> : null}

      {services && services.length === 0 ? (
        <p>No hay servicios activos.</p>
      ) : null}

      {services && services.length > 0 ? (
        <ActiveServicesTable services={services} />
      ) : null}
    </section>
  );
}
